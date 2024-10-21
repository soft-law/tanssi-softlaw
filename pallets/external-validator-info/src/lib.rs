// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>

//! # Author Noting Pallet
//!
//! This pallet notes the author of the different containerChains that have registered:
//!
//! The set of container chains is retrieved thanks to the GetContainerChains trait
//! For each containerChain, we inspect the Header stored in the relayChain as
//! a generic header. This is the first requirement for containerChains.
//!
//! The second requirement is that an Aura digest with the slot number for the containerChains
//! needs to exist
//!  
//! Using those two requirements we can select who the author was based on the collators assigned
//! to that containerChain, by simply assigning the slot position.

#![cfg_attr(not(feature = "std"), no_std)]

use {
    cumulus_pallet_parachain_system::RelaychainStateProvider,
    cumulus_primitives_core::{
        relay_chain::{BlakeTwo256, BlockNumber, HeadData},
        ParaId,
    },
    dp_core::well_known_keys::PARAS_HEADS_INDEX,
    frame_support::{
        dispatch::PostDispatchInfo, pallet_prelude::*, traits::DefensiveSaturating, Hashable,
    },
    frame_system::pallet_prelude::*,
    log::log,
    nimbus_primitives::SlotBeacon,
    pallet_staking::SessionInterface,
    parity_scale_codec::FullCodec,
    parity_scale_codec::{Decode, Encode},
    sp_consensus_aura::{inherents::InherentType, Slot, AURA_ENGINE_ID},
    sp_inherents::{InherentIdentifier, IsFatalError},
    sp_runtime::traits::{CheckedAdd, Convert, Debug, One, Zero},
    sp_runtime::Perbill,
    sp_runtime::{traits::Header, DigestItem, DispatchResult, RuntimeString},
    sp_staking::{
        offence::{OffenceDetails, OnOffenceHandler},
        EraIndex, Exposure, SessionIndex,
    },
    tp_author_noting_inherent::INHERENT_IDENTIFIER,
    tp_traits::{
        AuthorNotingHook, ContainerChainBlockInfo, GenericStateProof, GenericStorageReader,
        GetContainerChainAuthor, GetCurrentContainerChains, LatestAuthorInfoFetcher,
        NativeStorageReader, ReadEntryErr,
    },
};

pub use pallet::*;

/// Information regarding the active era (era in used in session).
#[derive(Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct ActiveEraInfo {
    /// Index of era.
    pub index: EraIndex,
    /// Moment of start expressed as millisecond from `$UNIX_EPOCH`.
    ///
    /// Start can be none if start hasn't been set for the era yet,
    /// Start is set on the first on_finalize of the era to guarantee usage of `Time`.
    pub start: Option<u64>,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Removed author data
        SlashReported {
            validator: T::ValidatorId,
            fraction: Perbill,
            slash_era: EraIndex,
        },
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// A stable ID for a validator.
        type ValidatorId: Member
            + Parameter
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + TryFrom<Self::AccountId>;

        /// A conversion from account ID to validator ID.
        ///
        /// Its cost must be at most one storage read.
        type ValidatorIdOf: Convert<Self::AccountId, Option<Self::ValidatorId>>;

        /// Number of eras that slashes are deferred by, after computation.
        ///
        /// This should be less than the bonding duration. Set to 0 if slashes
        /// should be applied immediately, without opportunity for intervention.
        #[pallet::constant]
        type SlashDeferDuration: Get<EraIndex>;

        /// Number of eras that staked funds must remain bonded for.
        #[pallet::constant]
        type BondingDuration: Get<EraIndex>;

        type SlashId: Default
            + FullCodec
            + TypeInfo
            + Copy
            + Clone
            + Debug
            + Eq
            + CheckedAdd
            + One
            + Ord
            + MaxEncodedLen;

        /// Interface for interacting with a session pallet.
        type SessionInterface: SessionInterface<Self::AccountId>;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The new value for a configuration parameter is invalid.
        FailedReading,
        FailedDecodingHeader,
        AuraDigestFirstItem,
        AsPreRuntimeError,
        NonDecodableSlot,
        AuthorNotFound,
        NonAuraDigest,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(PhantomData<T>);

    /// All slashing events on validators, mapped by era to the highest slash proportion
    /// and slash value of the era.
    #[pallet::storage]
    pub(crate) type ValidatorSlashInEra<T: Config> =
        StorageDoubleMap<_, Twox64Concat, EraIndex, Twox64Concat, T::AccountId, Perbill>;

    /// A mapping from still-bonded eras to the first session index of that era.
    ///
    /// Must contains information for eras for the range:
    /// `[active_era - bounding_duration; active_era]`
    #[pallet::storage]
    #[pallet::unbounded]
    pub(crate) type BondedEras<T: Config> =
        StorageValue<_, Vec<(EraIndex, SessionIndex)>, ValueQuery>;

    #[pallet::storage]
    pub type NextSlashId<T: Config> = StorageValue<_, T::SlashId, ValueQuery>;

    /// All unapplied slashes that are queued for later.
    #[pallet::storage]
    #[pallet::unbounded]
    pub type Slashes<T: Config> =
        StorageMap<_, Twox64Concat, EraIndex, Vec<Slash<T::AccountId, T::SlashId>>, ValueQuery>;

    /// The session index at which the era start for the last [`Config::HistoryDepth`] eras.
    ///
    /// Note: This tracks the starting session (i.e. session index when era start being active)
    /// for the eras in `[CurrentEra - HISTORY_DEPTH, CurrentEra]`.
    #[pallet::storage]
    #[pallet::getter(fn eras_start_session_index)]
    pub type ErasStartSessionIndex<T> = StorageMap<_, Twox64Concat, EraIndex, SessionIndex>;

    /// Any validators that may never be slashed or forcibly kicked. It's a Vec since they're
    /// easy to initialize and the performance hit is minimal (we expect no more than four
    /// invulnerables) and restricted to testnets.
    #[pallet::storage]
    #[pallet::getter(fn invulnerables)]
    #[pallet::unbounded]
    pub type Invulnerables<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    /// The active era information, it holds index and start.
    ///
    /// The active era is the era being currently rewarded. Validator set of this era must be
    /// equal to [`SessionInterface::validators`].
    #[pallet::storage]
    #[pallet::getter(fn active_era)]
    pub type ActiveEra<T> = StorageValue<_, ActiveEraInfo>;
}

/// This is intended to be used with `FilterHistoricalOffences`.
impl<T: Config>
    OnOffenceHandler<T::AccountId, pallet_session::historical::IdentificationTuple<T>, Weight>
    for Pallet<T>
where
    T: Config<ValidatorId = <T as frame_system::Config>::AccountId>,
    T: pallet_session::Config<ValidatorId = <T as frame_system::Config>::AccountId>,
    T: pallet_session::historical::Config,
    T::SessionHandler: pallet_session::SessionHandler<<T as frame_system::Config>::AccountId>,
    T::SessionManager: pallet_session::SessionManager<<T as frame_system::Config>::AccountId>,
    <T as pallet::Config>::ValidatorIdOf: Convert<
        <T as frame_system::Config>::AccountId,
        Option<<T as frame_system::Config>::AccountId>,
    >,
{
    fn on_offence(
        offenders: &[OffenceDetails<
            T::AccountId,
            pallet_session::historical::IdentificationTuple<T>,
        >],
        slash_fraction: &[Perbill],
        slash_session: SessionIndex,
    ) -> Weight {
        let mut consumed_weight = Weight::from_parts(0, 0);
        let mut add_db_reads_writes = |reads, writes| {
            consumed_weight += T::DbWeight::get().reads_writes(reads, writes);
        };

        let active_era = {
            let active_era = Self::active_era();
            add_db_reads_writes(1, 0);
            if active_era.is_none() {
                // This offence need not be re-submitted.
                return consumed_weight;
            }
            active_era
                .expect("value checked not to be `None`; qed")
                .index
        };
        let active_era_start_session_index = Self::eras_start_session_index(active_era)
            .unwrap_or_else(|| {
                frame_support::print("Error: start_session_index must be set for current_era");
                0
            });
        add_db_reads_writes(1, 0);

        let window_start = active_era.saturating_sub(T::BondingDuration::get());

        // Fast path for active-era report - most likely.
        // `slash_session` cannot be in a future active era. It must be in `active_era` or before.
        let slash_era = if slash_session >= active_era_start_session_index {
            active_era
        } else {
            let eras = BondedEras::<T>::get();
            add_db_reads_writes(1, 0);

            // Reverse because it's more likely to find reports from recent eras.
            match eras.iter().rev().find(|&(_, sesh)| sesh <= &slash_session) {
                Some((slash_era, _)) => *slash_era,
                // Before bonding period. defensive - should be filtered out.
                None => return consumed_weight,
            }
        };

        add_db_reads_writes(1, 1);

        let slash_defer_duration = T::SlashDeferDuration::get();

        let invulnerables = Self::invulnerables();
        add_db_reads_writes(1, 0);

        let mut next_slash_id = NextSlashId::<T>::get();

        for (details, slash_fraction) in offenders.iter().zip(slash_fraction) {
            let (stash, _) = &details.offender;

            // Skip if the validator is invulnerable.
            if invulnerables.contains(stash) {
                continue;
            }

            let mut slash = compute_slash::<T>(
                slash_fraction.clone(),
                next_slash_id,
                slash_era,
                stash.clone(),
                slash_defer_duration,
            );

            Self::deposit_event(Event::<T>::SlashReported {
                validator: stash.clone(),
                fraction: *slash_fraction,
                slash_era,
            });

            if let Some(mut slash) = slash {
                slash.reporters = details.reporters.clone();

                // Defer to end of some `slash_defer_duration` from now.
                log!(
                    log::Level::Debug,
                    "deferring slash of {:?}% happened in {:?} (reported in {:?}) to {:?}",
                    slash_fraction,
                    slash_era,
                    active_era,
                    slash_era + slash_defer_duration + 1,
                );
                Slashes::<T>::mutate(
                    slash_era
                        .saturating_add(slash_defer_duration)
                        .saturating_add(One::one()),
                    move |for_later| for_later.push(slash),
                );

                // Fix unwrap
                next_slash_id = next_slash_id.checked_add(&One::one()).unwrap();
                add_db_reads_writes(1, 1);
            } else {
                add_db_reads_writes(4 /* fetch_spans */, 5 /* kick_out_if_recent */)
            }
        }
        NextSlashId::<T>::put(next_slash_id);
        consumed_weight
    }
}

impl<T: Config> Pallet<T> {
    /// Start a new era. It does:
    /// * Increment `active_era.index`,
    /// * reset `active_era.start`,
    /// * update `BondedEras` and apply slashes.
    fn start_era(start_session: SessionIndex) {
        let active_era = ActiveEra::<T>::mutate(|active_era| {
            let new_index = active_era.as_ref().map(|info| info.index + 1).unwrap_or(0);
            *active_era = Some(ActiveEraInfo {
                index: new_index,
                // Set new active era start in next `on_finalize`. To guarantee usage of `Time`
                start: None,
            });
            new_index
        });

        let bonding_duration = T::BondingDuration::get();

        BondedEras::<T>::mutate(|bonded| {
            bonded.push((active_era, start_session));

            if active_era > bonding_duration {
                let first_kept = active_era.defensive_saturating_sub(bonding_duration);

                // Prune out everything that's from before the first-kept index.
                let n_to_prune = bonded
                    .iter()
                    .take_while(|&&(era_idx, _)| era_idx < first_kept)
                    .count();

                // Kill slashing metadata.
                for (pruned_era, _) in bonded.drain(..n_to_prune) {
                    #[allow(deprecated)]
                    ValidatorSlashInEra::<T>::remove_prefix(&pruned_era, None);
                    #[allow(deprecated)]
                    Slashes::<T>::remove(&pruned_era);
                }

                if let Some(&(_, first_session)) = bonded.first() {
                    T::SessionInterface::prune_historical_up_to(first_session);
                }
            }
        });

        Self::apply_unapplied_slashes(active_era);
    }

    /// Apply previously-unapplied slashes on the beginning of a new era, after a delay.
    fn apply_unapplied_slashes(active_era: EraIndex) {
        let mut era_slashes = Slashes::<T>::take(&active_era);
        log!(
            log::Level::Debug,
            "found {} slashes scheduled to be confirmed in era {:?}",
            era_slashes.len(),
            active_era,
        );
        for mut slash in &mut era_slashes {
            slash.confirmed = true;
        }
        Slashes::<T>::insert(active_era, &era_slashes);
    }
}

/// A pending slash record. The value of the slash has been computed but not applied yet,
/// rather deferred for several eras.
#[derive(Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct Slash<AccountId, SlashId> {
    /// The stash ID of the offending validator.
    validator: AccountId,
    /// Reporters of the offence; bounty payout recipients.
    reporters: Vec<AccountId>,
    /// The amount of payout.
    slash_id: SlashId,
    percentage: Perbill,
    // Whether the slash is confirmed or still needs to go through deferred period
    confirmed: bool,
}

/// Computes a slash of a validator and nominators. It returns an unapplied
/// record to be applied at some later point. Slashing metadata is updated in storage,
/// since unapplied records are only rarely intended to be dropped.
///
/// The pending slash record returned does not have initialized reporters. Those have
/// to be set at a higher level, if any.
pub(crate) fn compute_slash<T: Config>(
    slash_fraction: Perbill,
    slash_id: T::SlashId,
    slash_era: EraIndex,
    stash: T::AccountId,
    slash_defer_duration: EraIndex,
) -> Option<Slash<T::AccountId, T::SlashId>> {
    let prior_slash_p = ValidatorSlashInEra::<T>::get(&slash_era, &stash).unwrap_or(Zero::zero());

    // compare slash proportions rather than slash values to avoid issues due to rounding
    // error.
    if slash_fraction.deconstruct() > prior_slash_p.deconstruct() {
        ValidatorSlashInEra::<T>::insert(&slash_era, &stash, &slash_fraction);
    } else {
        // we slash based on the max in era - this new event is not the max,
        // so neither the validator or any nominators will need an update.
        //
        // this does lead to a divergence of our system from the paper, which
        // pays out some reward even if the latest report is not max-in-era.
        // we opt to avoid the nominator lookups and edits and leave more rewards
        // for more drastic misbehavior.
        return None;
    }

    let confirmed = slash_defer_duration.is_zero();
    Some(Slash {
        validator: stash.clone(),
        percentage: slash_fraction,
        slash_id,
        reporters: Vec::new(),
        confirmed,
    })
}
