# QONCORD

The governance protocol of Quantova. Its ballots are post quantum signatures, its asset recovery is bound to the exact scope that was voted, and its constitution gate is enforced by the protocol rather than by convention. The guiding rule is that no vote is above the law.

Quantova is a sovereign post quantum Layer 1 with only NIST standardized schemes and no classical escape hatch anywhere. QONCORD is built to that same standard. Every ballot is a module lattice signature, and no classical cryptography appears anywhere in the pipeline.

## Overview

QONCORD is the governance design repository for Quantova. The governance that runs on chain lives in the `qtv-governance` crate and its ledger wiring in the Quantova-Chain repository, and those rules are the source of truth. This README states them as the chain enforces them. The crates and contracts in this repository are early stubs from an earlier design and are not what the chain runs.

## The five tracks

Every proposal is raised on exactly one of five parallel tracks. Each track has its own deposit, voting period, enactment delay, and pass threshold, all fixed in code.

| Track | Deposit | Voting period | Enactment delay | Pass threshold |
|---|---|---|---|---|
| Chain upgrades | 225,000 QTOV | 14 days | 7 days | 66.67% |
| Mint QTOV | 400,000 QTOV | 3 days | 7 days | 66.67% |
| Bridge pool migration | 150,000 QTOV | 5 days | 7 days | 66.67% |
| Freeze and asset recovery | 29,250 QTOV | 6 hours | 1 hour | 75% |
| Blacklist and kill address | 39,000 QTOV | 2 days | 1 day | 75% |

Chain upgrades carries runtime upgrades, feature activation, every parameter change, and guardian rotation. Mint QTOV is the only way to create QTOV after genesis, and it also carries every spend from the grants account and the stake treasury. Bridge pool migration moves the bridge custody pool to a new vault, and it also carries bridge committee rotation, bridged asset registration, operator revocation, and the bridge epoch advance. Freeze and asset recovery freezes a thief and returns the stolen amount to the victim, scoped to the exact seizures that were voted. Blacklist and kill address neutralises a malicious address, freezes and unfreezes accounts, and carries the governance lift of a bridge freeze.

A proposal passes only when three bars hold at once. The aye weight must reach the track threshold of the whole staked electorate, turnout must reach at least 25 percent of that electorate, and aye must exceed nay. The deposit is returned in full when the proposal passes and is not killed, and is otherwise forfeited to the treasury.

## Voting

A voter locks QTOV behind a ballot, up to the size of their bonded stake, and picks a conviction. Conviction one times locks for 1 month, one and a half times for 1 year, and two and a half times for 2 years. The weight of a ballot is the locked amount times its conviction. There is no delegation, so every ballot is cast by the holder who locks the stake. Every ballot is an ML-DSA-65 signed transaction.

## Mint cap

Minting exists only through the Mint QTOV track, and it is capped at 2 percent of supply per year, and never less than 100,000 QTOV. A mint above the cap is refused at enactment.

## Emergency powers

The guardian caucus is a threshold multisig whose members and threshold are set and rotated only by a Chain upgrades referendum. A caucus is well formed only when its threshold is at least two and is a majority of its members, so no single key can act. Under its threshold the caucus can freeze a batch of accounts for up to 7 days while a recovery or blacklist vote runs, and it can lift a bridge freeze early. It never moves funds, and it can never freeze a protected core account.

The bridge freeze is a bonded action rather than a vote, so it halts every bridge transfer on the next block. Any account that is not blacklisted can post a 39,000 QTOV bond to freeze the bridge for up to 7 days, with a 1 day cooldown after any lift before the next freeze. The bond is refunded only when the depositor lifts the freeze early. When the freeze expires, or the caucus or governance lifts it, the bond is forfeited to the treasury.

## The constitution gate

Before any approved action runs, the chain checks it against its track and its scope. An action raised on the wrong track is refused. An asset recovery must match the exact seizure set that was voted, and it can never take from a protected core account. A freeze or a blacklist can never target a protected core account. Protected core accounts are the keyless network pots, such as the treasury and the grants account. Every enacted referendum stores an enactment receipt with the proposal hash, the scope, and the tally.

## Repository layout and build state

- `crates/qtv-tracks`, `crates/qtv-conviction`, and `crates/qtv-tally` are empty stubs.
- `crates/qtv-constitution` carries an early constitutional gate with negative tests in `tests/constitution_gate.rs`. It models an earlier design and is not the gate the chain runs.
- `contracts/evidence_registry.qs` and `contracts/recovery_escrow.qs` are stub system contracts from the earlier design, written in the Quanta language.

The hostile governance vectors in `tests/hostile` describe referenda that passed their vote but cross a constitutional invariant, and they are mirrored into the Quantova-Conformance repository.

## Cryptography

Signatures are ML-DSA-65 from FIPS 204, hashing is SHA-3 and SHAKE from FIPS 202. There is no elliptic curve anywhere. The stack cryptography is a from scratch reference implementation validated against the NIST vectors. It has not been independently audited, and the chain is at testnet.

## Governance and license

Governed by the crypto policy, POLICY-crypto, in the Quantova-Specs repository. Commits are authored by the owner only. Dual licensed under Apache 2.0 and MIT.
