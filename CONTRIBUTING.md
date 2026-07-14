# Contributing

This repository implements SPEC-governance.md from the Quantova Specs repository. Read POLICY-crypto.md (the supreme law) and SPEC-governance.md before contributing. Any conflict with the crypto policy means stop and report.

## Cryptography
Ballots are ML-DSA signatures and tallies are STARK certificates. Only NIST post-quantum algorithms exist here. Banned crates are enforced by `cargo deny check` (see deny.toml). No classical cryptography and no pairing based aggregation.

## Commits and PRs
- Author only as the owner: quantova-inc / Quantovaorg@gmail.com. No AI attribution anywhere.
- Never push to main. Branch feat/<crate>, open a PR, merge only on green CI.
- Every PR cites the SPEC-governance.md section it implements. Cross-repo dependencies pin git tags.
- Genesis governance parameters are frozen. Changes arrive only as founder approved spec PRs.

## Claims discipline
Say STARK-proven tallies, evidence-bound enactment, machine-enforced constitution, and no vote above the law. Never say capture-proof, censorship-proof, or unhackable.

