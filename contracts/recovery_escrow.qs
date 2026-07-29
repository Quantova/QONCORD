// Recovery Escrow for the QONCORD Justice Protocol. Implements SPEC-governance section 4.3.
// STUB: awaiting the Quanta compiler (SPEC-quanta-language Step 7). It uses Quanta native guarantees
// only: `conserves` on every asset flow, `signed by` for victim claims, `Quorum` for the guardian
// caucus, and `after` for the claim and residue windows. Clawback is cryptographically scoped to the
// cited Evidence Bundle, so nothing outside the bundle is reachable.
// Each claim is bound to the claimant's own entitlement and is paid at most once.

import { Q_Asset, Q_Sig, Quorum } from "quantova/primitives";
import { Registry } from "quantova/stdlib";

contract RecoveryEscrow {
  asset RECOVERED;

  state {
    bundle_hash: Q_Commit<EvidenceBundle>;
    escrow: Q_Asset<RECOVERED>;
    claimants: Registry<Q_Address>;
    claimed: Registry<Q_Address>;
    opened_at: Time;
  }

  genesis {
    bundle_hash = deploy_params.bundle_hash;
    opened_at = deploy_params.opened_at;
  }

  entry claim(proof: ClaimProof signed by claimant)
    writes(escrow, claimed)
    conserves RECOVERED
    denies !claimants.contains(claimant)
  {
    guard !claimed.contains(claimant);
    guard bundle_hash.opens(claimant, proof.amount, proof.path);
    guard escrow.amount >= proof.amount;
    claimed.insert(claimant);
    let payout = escrow.split(proof.amount);
    send(claimant, payout);
    emit Claimed(claimant, proof.amount);
  }

  entry sweep_residue(approvals: Quorum<7 of 11, guardians>)
    writes(escrow)
    conserves RECOVERED
    after 2 years from opened_at
  {
    send(insurance_fund, escrow);
    emit ResidueSwept(approvals.digest);
  }
}
