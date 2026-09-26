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
fn justice_cannot_take_stake_from_a_holder_never_frozen() {
    let action = ProposedAction::JusticeSeize {
        targets_validator_stake: true,
        holder_frozen: false,
        within_bundle: true,
    };
    assert_eq!(
        check_enactment(&action),
        Err(ConstitutionViolation::StakeNotFrozen)
    );
}

#[test]
fn a_bundled_justice_seizure_of_frozen_validator_stake_is_enactable() {
    let action = ProposedAction::JusticeSeize {
        targets_validator_stake: true,
        holder_frozen: true,
        within_bundle: true,
    };
    assert_eq!(check_enactment(&action), Ok(()));
}

#[test]
fn frozen_validator_stake_outside_the_bundle_is_unenactable() {
    let action = ProposedAction::JusticeSeize {
        targets_validator_stake: true,
        holder_frozen: true,
        within_bundle: false,
    };
    assert_eq!(
        check_enactment(&action),
        Err(ConstitutionViolation::OutOfBundleScope)
    );
}

#[test]
fn justice_outside_the_bundle_is_unenactable() {
    let action = ProposedAction::JusticeSeize {
        targets_validator_stake: false,
        holder_frozen: false,
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

#[test]
fn a_mint_within_the_ceiling_is_enactable() {
    let action = ProposedAction::Mint {
        amount: 1,
        epoch_minted: 99,
        epoch_ceiling: 100,
    };
    assert_eq!(check_enactment(&action), Ok(()));
}

#[test]
fn a_bundled_justice_seizure_off_validator_stake_is_enactable() {
    let action = ProposedAction::JusticeSeize {
        targets_validator_stake: false,
        holder_frozen: false,
        within_bundle: true,
    };
    assert_eq!(check_enactment(&action), Ok(()));
}

#[test]
fn an_emergency_pause_that_moves_no_value_is_enactable() {
    let action = ProposedAction::EmergencyPause { moves_value: false };
    assert_eq!(check_enactment(&action), Ok(()));
}
