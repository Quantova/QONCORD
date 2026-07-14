# Hostile governance vectors

Each vector describes a referendum that PASSED its vote but crosses a constitutional invariant
(SPEC-governance section 5). The required result is the same for all: **unenactable**. The runtime
refuses the action the way it refuses a malformed transaction.

These vectors are mirrored into `Quantova-Conformance/vectors/hostile/` and are enforced by the
negative tests in `crates/qtv-constitution/tests/constitution_gate.rs`.
