// Copyright 2026 Quantova Inc
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Constitutional negative tests (SPEC-governance section 5). These are written first and are RED

use qtv_constitution::{check_enactment, ConstitutionViolation, ProposedAction};

#[test]
fn over_ceiling_mint_is_unenactable() {
    let action = ProposedAction::Mint {
        amount: 3,
        epoch_minted: 99,
        epoch_ceiling: 100,
    };
    assert_eq!(
        check_enactment(&action),
        Err(ConstitutionViolation::OverMintCeiling)
    );
}

#[test]
fn justice_cannot_touch_validator_stake() {
    let action = ProposedAction::JusticeSeize {
        targets_validator_stake: true,
        within_bundle: true,
    };
    assert_eq!(
        check_enactment(&action),
        Err(ConstitutionViolation::JusticeTouchesConsensus)
    );
}

#[test]
fn justice_outside_the_bundle_is_unenactable() {
    let action = ProposedAction::JusticeSeize {
        targets_validator_stake: false,
        within_bundle: false,
    };
    assert_eq!(
        check_enactment(&action),
        Err(ConstitutionViolation::OutOfBundleScope)
    );
}

#[test]
fn emergency_cannot_move_value() {
    let action = ProposedAction::EmergencyPause { moves_value: true };
    assert_eq!(
        check_enactment(&action),
        Err(ConstitutionViolation::EmergencyMovesValue)
    );
}
