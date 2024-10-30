import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@moonwall/util";
import { Keyring } from "@polkadot/keyring";
import { u8aToHex } from "@polkadot/util";
import { jumpToSession } from "../../../util/block";
import { generateBabeEquivocationProof } from "../../../util/slashes";

describeSuite({
    id: "DTR1302",
    title: "Babe offences invulnerables",
    foundationMethods: "dev",
    testCases: ({ it, context }) => {
        let polkadotJs: ApiPromise;
        let alice: KeyringPair;
        let aliceBabePair: KeyringPair;
        beforeAll(async () => {
            const keyringBabe = new Keyring({ type: "sr25519" });
            aliceBabePair = keyringBabe.addFromUri("//Alice");
            polkadotJs = context.polkadotJs();
            alice = context.keyring.alice;
        });
        it({
            id: "E01",
            title: "Babe offences do not trigger a slash to invulnerables",
            test: async function () {
                // we crate one block so that we at least have one seal.
                await jumpToSession(context, 1);

                // let's inject the equivocation proof
                const doubleVotingProof = await generateBabeEquivocationProof(polkadotJs, aliceBabePair);

                // generate key ownership proof
                const keyOwnershipProof = (
                    await polkadotJs.call.babeApi.generateKeyOwnershipProof(
                        doubleVotingProof.slotNumber,
                        u8aToHex(aliceBabePair.publicKey)
                    )
                ).unwrap();
                const keyOwnershipProofHex = `0x${keyOwnershipProof.toHuman().toString().slice(8)}`;

                const tx = polkadotJs.tx.sudo.sudoUncheckedWeight(
                    polkadotJs.tx.utility.dispatchAs(
                        {
                            system: { Signed: alice.address },
                        } as any,
                        polkadotJs.tx.babe.reportEquivocation(doubleVotingProof, keyOwnershipProofHex)
                    ),
                    {
                        refTime: 1n,
                        proofSize: 1n,
                    }
                );

                const signedTx = await tx.signAsync(alice);
                await context.createBlock(signedTx);

                // Slash item should be there
                const DeferPeriod = await polkadotJs.consts.externalValidatorSlashes.slashDeferDuration;

                // Alice is an invulnerable, therefore she should not be slashed
                const expectedSlashes = await polkadotJs.query.externalValidatorSlashes.slashes(DeferPeriod.toNumber() + 1);
                expect(expectedSlashes.length).to.be.eq(0);
            },
        });
    },
});
