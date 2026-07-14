//! Ballot verification and tally for QONCORD (SPEC-governance section 6).
//!
//! Each ballot is an ML-DSA signature over the referendum identifier, the choice, and the conviction.
//! Ballots are aggregated once per epoch and each referendum yields exactly one STARK certificate,
//! built from the q-prover circuits and carried on the same certificate wrapper as consensus. No
//! classical aggregation appears anywhere in this pipeline.
