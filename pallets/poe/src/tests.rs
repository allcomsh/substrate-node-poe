use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;
use frame_system::pallet_prelude::*;

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let proof = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), proof.clone()));
        assert_eq!(
            Proofs::<Test>::get(&proof),
            Some((1u64, frame_system::Pallet::<Test>::block_number()))
        );
    })
}

#[test]
fn create_claim_failed_when_proof_overflow_nottrue() {
    new_test_ext().execute_with(|| {
        let proof = vec![0, 1, 2, 3]; // For test, the Bound of proof is set to 4.
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), proof.clone()),
            Error::<Test>::ProofOverflow
        );
    })
}

#[test]
fn create_claim_failed_when_proof_overflow() {
    new_test_ext().execute_with(|| {
        let proof = vec![0, 1, 2, 3, 4, 5]; // For test, the Bound of proof is set to 4.
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), proof.clone()),
            Error::<Test>::ProofOverflow
        );
    })
}

#[test]
fn create_claim_failed_when_proof_already_exist() {
    new_test_ext().execute_with(|| {
        let proof = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), proof.clone());
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), proof.clone()),
            Error::<Test>::ProofAlreadyClaimed
        );
    })
}

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let proof = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), proof.clone());
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), proof.clone()));
        assert_eq!(
            Proofs::<Test>::get(&proof),
            None
        );
    })
}

#[test]
fn revoke_claim_failed_when_proof_not_exist() {
    new_test_ext().execute_with(|| {
        let proof = vec![0, 1];
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), proof.clone()),
            Error::<Test>::NoSuchProof
        );
    })
}

#[test]
fn revoke_claim_failed_when_not_owner() {
    new_test_ext().execute_with(|| {
        let proof = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), proof.clone());
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), proof.clone()),
            Error::<Test>::NotProofOwner
        );
    })
}

#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let proof = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), proof.clone());
        let dest = ensure_signed(Origin::signed(2)).unwrap();
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), proof.clone(), dest));
        assert_eq!(
            Proofs::<Test>::get(&proof),
            Some((2u64, frame_system::Pallet::<Test>::block_number()))
        )
    })
}

#[test]
fn transfer_claim_failed_when_not_owner() {
    new_test_ext().execute_with(|| {
        let proof = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), proof.clone());
        let dest = ensure_signed(Origin::signed(2)).unwrap();
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(3), proof.clone(), dest),
            Error::<Test>::NotProofOwner
        );
    })
}

#[test]
fn transfer_claim_failed_when_proof_not_exist() {
    new_test_ext().execute_with(|| {
        let proof = vec![0, 1];
        let dest = ensure_signed(Origin::signed(2)).unwrap();
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), proof.clone(), dest),
            Error::<Test>::NoSuchProof
        );
    })
}
