// Copyright 2026 Quantova Inc
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The constitutional gate for QONCORD (SPEC-governance section 5).

/// An action proposed for enactment by a passed referendum.
pub enum ProposedAction {
    /// Mint of the native asset (Monetary track). Carries how much this epoch has already minted and
    Mint {
        amount: u128,
        epoch_minted: u128,
        epoch_ceiling: u128,
    },
    /// A Justice enactment. Must never reach validator stake or consensus, and must stay inside the
    JusticeSeize {
        targets_validator_stake: bool,
        within_bundle: bool,
    },
    /// An Emergency action. Pauses only; must never move value (SPEC-governance section 5).
    EmergencyPause { moves_value: bool },
}

/// Why the constitution refuses an action (SPEC-governance section 5).
#[derive(Debug, PartialEq, Eq)]
pub enum ConstitutionViolation {
    OverMintCeiling,
    JusticeTouchesConsensus,
    OutOfBundleScope,
    EmergencyMovesValue,
}

/// Refuse any action that crosses a constitutional invariant. Returns `Ok` only for actions the
pub fn check_enactment(_action: &ProposedAction) -> Result<(), ConstitutionViolation> {
    todo!("constitutional gate is implemented after SPEC-governance merges")
}
