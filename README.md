QONCORD

QONCORD is the governance protocol of the Quantova network. It is the first governance system with tallies proven by STARK certificates, judicial powers bound to signed evidence, and a constitution enforced by the runtime itself. Its guiding rule is that no vote is above the law.

Overview

QONCORD follows the familiar open governance layout of parallel referendum tracks, conviction voting, and delegation, and it rebuilds all of that on post quantum foundations. Every ballot is a lattice signature. Every tally is a STARK certificate that travels the same wrapper as consensus. Governance is bounded by a constitution that no track can cross.

The seven tracks

Governance runs as seven parallel tracks. Each track has its own deposit, decision period, enactment delay, and thresholds. The genesis values below are frozen starting points and can be changed later only through the Constitution track.

Constitution. This track amends the constitution, the track parameters, and the runtime. It carries the largest deposit of 100 thousand QTOV, a decision period of 28 days, and an enactment delay of 14 days. It passes at 60 percent approval with support of at least 25 percent of all staked value. Even this track is bounded by the five invariants.

Crypto Transition. This track adds or retires approved cryptographic schemes and sets key rotation windows. Its deposit is 50 thousand QTOV, its decision period is 28 days, its enactment delay is 30 days, and it passes at 66 percent approval with 20 percent support. A proposal on this track must include an external cryptanalysis report or it is invalid. This is the only path that can change the algorithm set, and it can never introduce a classical primitive.

Monetary. This track is the only path that can mint the native asset, and it governs the fee burn and the fee split. Its deposit is 50 thousand QTOV, its decision period is 21 days, its enactment delay is 7 days, and it passes at 66 percent approval with 20 percent support. Minting is capped by a hard ceiling described further below.

Treasury. This track spends from the treasury across small, medium, and large lanes with a sliding scale of deposit, decision, and enactment. Deposits range from 1 thousand to 25 thousand QTOV. This track cannot mint.

Justice. This track handles freeze extensions and clawback, and only on the basis of signed evidence. Its deposit is 25 thousand QTOV, its decision period is 14 days, its enactment delay is 7 days followed by an appeal window, and it passes at 75 percent approval with 25 percent support. Justice can never reach validator stake, consensus parameters, or governance locks.

Emergency. A guardian caucus of 7 of 11 can pause a contract, a corridor, or a module within hours. It never moves funds. Any pause expires automatically after 72 hours unless a referendum confirms it.

Standards. This track accepts or deprecates standards proposals and carries signaling votes. Its deposit is 500 QTOV, its decision period is 14 days, and it passes by simple majority.

Voting

Voters strengthen a vote by locking stake. The multiplier runs from one time with no lock up to six times with a lock of 32 weeks. Delegation is chosen for each track, so a holder can delegate one track and vote directly on another. Validators vote as ordinary stakers with no extra weight.

Monetary law

Minting the native asset exists only through the Monetary track. It is capped by a hard ceiling for each epoch, set at genesis so that cumulative minting stays at or below 2 percent per year. A referendum that would mint above the ceiling is not merely outvoted. It is unenactable. The runtime refuses it the way it refuses a malformed transaction. Every mint records the referendum identifier and the tally certificate.

The Justice Protocol

Justice has two powers and both are bound to an Evidence Bundle. An Evidence Bundle is a hash committed record that holds exploit traces, signed victim attestations, and an explicit list of addresses and amounts. The bundle hash locks the scope of any action that cites it, so enactment can never reach an address or an amount outside the bundle.

The freeze power is fast and reversible. A reporter posts a bond of 1 thousand QTOV, which is slashed if the report is frivolous, and files an Evidence Bundle. The guardian caucus of 7 of 11 then enacts a temporary freeze on exactly the listed addresses. The freeze is locked to the bundle and cannot widen. It expires automatically after 72 hours unless a Justice referendum opens. Counter evidence can be filed at any time.

The clawback power is slow and evidentiary. A Justice referendum opens by citing the bundle, runs for 14 days, and then holds a 7 day appeal window in which counter evidence triggers one further vote. On enactment the assets move to the Recovery Escrow contract, scoped strictly to the bundle, so anything outside the bundle stays untouched. Victims claim with signed proofs, and any unclaimed remainder moves to the insurance fund after 2 years.

The Constitution

Five invariants are enforced by the runtime, and no track can cross them, not even the Constitution track. First, no track may introduce classical or non approved cryptography, because the crypto policy outranks governance itself. Second, Justice can never touch validator stake, consensus parameters, or governance locks. Third, Emergency pauses and never moves value, and every pause expires. Fourth, mint ceilings, freeze expiry, appeal windows, and scope locks are runtime invariants, so any referendum that violates them is unenactable. Fifth, every enacted referendum permanently stores the proposal hash, the evidence hash where the action is judicial, the STARK tally certificate, and the enactment receipt.

The tally pipeline

Each ballot is a lattice signature over the referendum identifier, the choice, and the conviction. Ballots are aggregated once for each epoch. Each referendum then produces exactly one STARK certificate that proves the tally, built from the proving circuits and carried on the same certificate wrapper as consensus. No classical aggregation appears anywhere in the pipeline.

Repository contents

This repository holds the runtime crates and the system contracts for QONCORD. The runtime crates cover the referendum lifecycle, the conviction and delegation ledger, the ballot verification and tally, and the constitutional gate that refuses any action which crosses an invariant. The system contracts hold the Recovery Escrow and the Evidence Registry, written in the Quanta language once its compiler is ready.

Status and governance

The governance specification lives in the Quantova Specs repository and is the source of truth. This repository implements it. Commits are authored by the owner only. This work is dual licensed under Apache 2.0 and MIT.
