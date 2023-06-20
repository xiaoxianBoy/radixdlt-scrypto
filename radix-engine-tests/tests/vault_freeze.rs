use radix_engine::errors::{RuntimeError, SystemModuleError};
use radix_engine::system::system_modules::auth::AuthError;
use radix_engine::types::*;
use scrypto::prelude::FromPublicKey;
use scrypto_unit::*;
use transaction::builder::ManifestBuilder;

#[test]
fn cannot_withdraw_from_frozen_vault() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let (key, _priv, account) = test_runner.new_account(true);
    let token_address = test_runner.create_freezeable_token(account);
    let vaults = test_runner.get_component_vaults(account, token_address);
    let vault_id = vaults[0];
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10u32.into())
        .freeze(InternalAddress::new_or_panic(vault_id.into()))
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    receipt.expect_commit_success();

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10u32.into())
        .withdraw_from_account(account, token_address, 1.into())
        .deposit_batch(account)
        .build();
    let receipt =
        test_runner.execute_manifest(manifest, vec![NonFungibleGlobalId::from_public_key(&key)]);

    // Assert
    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::SystemModuleError(SystemModuleError::AuthError(AuthError::Unauthorized(
                ..
            )))
        )
    });
}

#[test]
fn cannot_recall_from_frozen_vault() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let (key, _priv, account) = test_runner.new_account(true);
    let token_address = test_runner.create_freezeable_token(account);
    let vaults = test_runner.get_component_vaults(account, token_address);
    let vault_id = vaults[0];
    let vault_address = InternalAddress::new_or_panic(vault_id.into());
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10u32.into())
        .freeze(vault_address)
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    receipt.expect_commit_success();

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10u32.into())
        .recall(vault_address, 1.into())
        .deposit_batch(account)
        .build();
    let receipt =
        test_runner.execute_manifest(manifest, vec![NonFungibleGlobalId::from_public_key(&key)]);

    // Assert
    receipt.expect_commit_success();
}

#[test]
fn can_withdraw_from_unfrozen_vault() {
    // Arrange
    let mut test_runner = TestRunner::builder().build();
    let (key, _priv, account) = test_runner.new_account(true);
    let token_address = test_runner.create_freezeable_token(account);
    let vaults = test_runner.get_component_vaults(account, token_address);
    let vault_id = vaults[0];
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10u32.into())
        .freeze(InternalAddress::new_or_panic(vault_id.into()))
        .build();
    let receipt = test_runner.execute_manifest(manifest, vec![]);
    receipt.expect_commit_success();
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10u32.into())
        .unfreeze(InternalAddress::new_or_panic(vault_id.into()))
        .build();
    let receipt =
        test_runner.execute_manifest(manifest, vec![NonFungibleGlobalId::from_public_key(&key)]);
    receipt.expect_commit_success();

    // Act
    let manifest = ManifestBuilder::new()
        .lock_fee(test_runner.faucet_component(), 10u32.into())
        .withdraw_from_account(account, token_address, 1.into())
        .deposit_batch(account)
        .build();
    let receipt =
        test_runner.execute_manifest(manifest, vec![NonFungibleGlobalId::from_public_key(&key)]);

    // Assert
    receipt.expect_commit_success();
}