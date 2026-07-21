# QONCORD

The governance protocol of Quantova. Its ballots are post quantum signatures, its tallies are proven by hash based STARK certificates, its judicial powers are bound to signed evidence, and its constitution is enforced by the protocol rather than by convention. The guiding rule is that no vote is above the law.

Quantova is a sovereign post quantum Layer 1 with only NIST standardized schemes and no classical escape hatch anywhere. QONCORD is built to that same standard. Every ballot is a module lattice signature, every tally is proven on the same certificate wrapper as consensus, and no classical aggregation appears anywhere in the pipeline.

## Overview

QONCORD follows the open governance shape of parallel referendum tracks, conviction voting, and per track delegation, and it rebuilds that shape on post quantum foundations. Governance is bounded by a constitution that no track can cross, not even the track that amends the constitution.

## The seven tracks

Governance runs as seven parallel tracks. Each track has its own deposit, decision period, enactment delay, and thresholds. The genesis values below are frozen starting points and can be changed later only through the Constitution track.

Constitution. Amends the constitution, the track parameters, and the node logic. Deposit of 100 thousand QTOV, a 28 day decision period, and a 14 day enactment delay. Passes at 60 percent approval with support of at least 25 percent of all staked value. Even this track is bounded by the five invariants.

Crypto Transition. Adds or retires approved cryptographic schemes and sets key rotation windows. Deposit of 50 thousand QTOV, a 28 day decision period, a 30 day enactment delay, passing at 66 percent approval with 20 percent support. A proposal here is invalid without an external cryptanalysis report. This is the only path that can change the algorithm set, and it can never introduce a classical primitive.

Monetary. The only path that can mint the native asset, and it governs the fee split. Deposit of 50 thousand QTOV, a 21 day decision period, a 7 day enactment delay, passing at 66 percent approval with 20 percent support. Minting is capped by a hard ceiling, described below.

Treasury. Spends from the treasury across small, medium, and large lanes with a sliding scale of deposit, decision, and enactment. Deposits range from 1 thousand to 25 thousand QTOV. This track cannot mint.

Justice. Handles freeze extensions and clawback, and only on the basis of signed evidence. Deposit of 25 thousand QTOV, a 14 day decision period, a 7 day enactment delay followed by an appeal window, passing at 75 percent approval with 25 percent support. Justice can never reach validator stake, consensus parameters, or governance locks.

Emergency. A guardian caucus of 7 of 11 can pause a contract, a corridor, or a module within hours. It never moves funds. Any pause expires automatically after 72 hours unless a referendum confirms it.

Standards. Accepts or deprecates standards proposals and carries signaling votes. Deposit of 500 QTOV, a 14 day decision period, passing by simple majority.

## Voting

A voter strengthens a vote by locking stake. The multiplier runs from one, with no lock, to six, with a lock of 32 weeks. Delegation is chosen for each track, so a holder can delegate one track and vote directly on another. Validators vote as ordinary stakers with no extra weight.

## Monetary law

Minting the native asset exists only through the Monetary track, and it is capped by a hard ceiling for each epoch, set at genesis so that cumulative minting stays at or below 2 percent per year. A referendum that would mint above the ceiling is not merely outvoted. It is unenactable, refused the way the protocol refuses a malformed transaction. Every mint records the referendum identifier and the tally certificate.

## The Justice Protocol

Justice has two powers and both are bound to an Evidence Bundle. An Evidence Bundle is a hash committed record holding exploit traces, signed victim attestations, and an explicit list of addresses and amounts. The bundle hash locks the scope of any action that cites it, so enactment can never reach an address or an amount outside the bundle.

The freeze power is fast and reversible. A reporter posts a bond of 1 thousand QTOV, slashed if the report is frivolous, and files an Evidence Bundle. The guardian caucus of 7 of 11 then enacts a temporary freeze on exactly the listed addresses. The freeze is locked to the bundle and cannot widen. It expires automatically after 72 hours unless a Justice referendum opens. Counter evidence can be filed at any time.

The clawback power is slow and evidentiary. A Justice referendum opens by citing the bundle, runs for 14 days, then holds a 7 day appeal window in which counter evidence triggers one further vote. On enactment the assets move to the Recovery Escrow contract, scoped strictly to the bundle, so anything outside the bundle stays untouched. Victims claim with signed proofs, and any unclaimed remainder moves to the insurance fund after 2 years.

## The Constitution

Five invariants are enforced by the protocol, and no track can cross them, not even the Constitution track.

1. No track may introduce classical or non approved cryptography, because the crypto policy outranks governance itself.
2. Justice can never touch validator stake, consensus parameters, or governance locks.
3. Emergency pauses and never moves value, and every pause expires.
4. Mint ceilings, freeze expiry, appeal windows, and scope locks are protocol invariants, so any referendum that violates them is unenactable.
5. Every enacted referendum permanently stores the proposal hash, the evidence hash where the action is judicial, the STARK tally certificate, and the enactment receipt.

## The tally pipeline

Each ballot is a module lattice signature over the referendum identifier, the choice, and the conviction. Ballots are aggregated once for each epoch, and each referendum produces exactly one STARK certificate that proves the tally, built from the q-prover circuits and carried on the same certificate wrapper as consensus. No classical aggregation appears anywhere in the pipeline.

## Repository layout and build state

The governance specification, SPEC-governance, lives in the Quantova-Specs repository and is the source of truth. This repository implements it, and it is early.

- `crates/qtv-tracks` fixes the referendum lifecycle and the enactment queue for the seven tracks.
- `crates/qtv-conviction` fixes the conviction ledger and per track delegation.
- `crates/qtv-tally` fixes the ballot verification and tally, one STARK certificate per referendum on the consensus wrapper.
- `crates/qtv-constitution` carries the constitutional gate. Its `check_enactment` refuses any action that crosses an invariant, over the mint ceiling, a Justice action reaching consensus or stepping outside its bundle, or an Emergency action that moves value. The negative tests in `tests/constitution_gate.rs` are written first and stay red until the gate enforces each invariant, so a violating action must be proven unenactable before the gate is called done.
- `contracts/evidence_registry.qs` and `contracts/recovery_escrow.qs` are the system contracts for the Justice Protocol, written in the Quanta language and awaiting its compiler. They lean on Quanta native guarantees only, `conserves` on every asset flow, `signed by` for claims, `Quorum` for the guardian caucus, and `after` for the timed windows.

The hostile governance vectors in `tests/hostile` describe referenda that passed their vote but cross a constitutional invariant, and they are mirrored into the Quantova-Conformance repository. The required result for every one of them is the same. The referendum is unenactable.

## Cryptography

Signatures are ML-DSA-65 from FIPS 204, hashing is SHA-3 and SHAKE from FIPS 202, and tallies are proven with hash based STARKs that rest on hashing alone. There is no elliptic curve anywhere. The stack cryptography is a from scratch reference implementation validated against the NIST vectors. It has not been independently audited, and the chain is at testnet.

## Governance and license

Governed by the crypto policy, POLICY-crypto, in the Quantova-Specs repository. Commits are authored by the owner only. Dual licensed under Apache 2.0 and MIT.
