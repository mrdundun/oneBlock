use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};

// 测试创建claim是否能成功
#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0,1]).unwrap();

        assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()));

        //
        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1,frame_system::Pallet::<Test>::block_number()))
        );
    })
}

// 测试重复创建claim是否成功
#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        // 创建存证
        let claim = BoundedVec::try_from(vec![0,1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone()),// 重复创建存证应该失败
            Error::<Test>::ProofAlreadyExist
        );
    })
}

// 测试删除存证
#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0,1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()));
    })
}

// 测试删除不存在的存证是否成功
#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0,1]).unwrap();
        // 这里没有调用PoeModule::create_claim，也就是没有创建存证，以便引发错误

        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    })
}

// 测试删除不属于owner的存证是否成功
#[test]
fn revoke_claim_failed_with_wrong_owner() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0,1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::revoke_claim(RuntimeOrigin::signed(2), claim.clone()),// 用不同的AccountID签名，也就是从错误的owner中移除存证
            Error::<Test>::NotClaimOwner
        );
    })
}

// 测试存证能不能转移给自己
#[test]
fn transfer_claim_failed_when_transfer_claim_to_self() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0,1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
        let receiver_account_id: <Test as frame_system::Config>::AccountId = 1;

        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(1), receiver_account_id, claim),
            Error::<Test>::CanNotTransferToSelf
        );
    })
}

// 测试是否能转移不属于自己的存证
#[test]
fn transfer_claim_failed_when_claim_not_belone_to_sender() {
    new_test_ext().execute_with(|| {
        let claim = BoundedVec::try_from(vec![0,1]).unwrap();
        let _ = PoeModule::create_claim(RuntimeOrigin::signed(1), claim.clone());
        let receiver_account_id: <Test as frame_system::Config>::AccountId = 1;

        assert_noop!(
            PoeModule::transfer_claim(RuntimeOrigin::signed(2), receiver_account_id, claim),
            Error::<Test>::NotClaimOwner
        );
    })
}