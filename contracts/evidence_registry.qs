// Evidence Registry for the QONCORD Justice Protocol. Implements SPEC-governance sections 4.1 and 4.2.
// STUB: awaiting the Quanta compiler (SPEC-quanta-language Step 7). Stores hash committed Evidence
// Bundles and gates freeze enactment behind the guardian caucus via `Quorum`. A freeze is scope
// locked to the bundle hash and cannot widen.
// Filing escrows a real reporter bond on chain, not a self declared field.

import { Q_Asset, Q_Sig, Quorum } from "quantova/primitives";
import { Registry, Map } from "quantova/stdlib";

contract EvidenceRegistry {
  state {
    bundles: Map<Q_Hash, BundleHeader>;
    bonds: Q_Asset<QTOV>;
    bonded: Map<Q_Hash, u128>;
    frozen: Registry<Q_Address>;
    guardians: GuardianSet<11>;
  }

  genesis {
    guardians = deploy_params.guardians;
  }

  entry file_bundle(bundle: EvidenceBundle signed by reporter, posted: Q_Asset<QTOV>)
    conserves QTOV
    writes(bundles, bonds, bonded)
  {
    guard posted.amount >= 1_000;
    bonds.merge(posted);
    bonded.credit(bundle.hash, posted.amount);
    bundles.insert(bundle.hash, bundle.header);
    emit BundleFiled(bundle.hash, reporter);
  }

  entry enact_freeze(bundle_hash: Q_Hash, approvals: Quorum<7 of 11, guardians>)
    writes(frozen)
  {
    guard bundles.contains(bundle_hash);
    freeze_scope(bundle_hash);
    emit FreezeEnacted(bundle_hash, approvals.digest);
  }
}
