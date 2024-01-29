// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(PSP22, PSP22Vault, PSP22Metadata)]
#[ink::contract]
pub mod my_psp22_vault {
    use pendzl::contracts::token::psp22::extensions::vault::implementation::PSP22VaultInternalDefaultImpl;
    use pendzl::traits::String;
    #[ink(storage)]
    #[derive(Default, StorageFieldGetter)]
    pub struct Contract {
        #[storage_field]
        psp22: PSP22Data,
        #[storage_field]
        vault: PSP22VaultData,
        #[storage_field]
        metadata: PSP22MetadataData,

        decimals_offset: u8,

        max_deposit_and_mint: Option<u128>,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(
            asset: AccountId,
            decimals_offset: u8,
            max_deposit_and_mint: Option<u128>,
        ) -> Self {
            let mut instance = Self::default();
            let psp22: PSP22Ref = asset.into();
            instance.vault.asset.set(&psp22);
            let (success, asset_decimals) = instance._try_get_asset_decimals();
            let decimals_to_set = if success { asset_decimals } else { 12 };
            instance.vault.underlying_decimals.set(&decimals_to_set);

            instance.decimals_offset = decimals_offset;
            instance.max_deposit_and_mint = max_deposit_and_mint;

            instance
        }
    }

    #[overrider(PSP22VaultInternal)]
    fn _decimals_offset(&self) -> u8 {
        self.decimals_offset
    }

    #[overrider(PSP22VaultInternal)]
    fn _max_deposit(&self, to: &AccountId) -> Balance {
        if let Some(v) = self.max_deposit_and_mint {
            v
        } else {
            PSP22VaultInternalDefaultImpl::_max_deposit_default_impl(self, to)
        }
    }
    #[overrider(PSP22VaultInternal)]
    fn _max_mint(&self, to: &AccountId) -> Balance {
        if let Some(v) = self.max_deposit_and_mint {
            v
        } else {
            PSP22VaultInternalDefaultImpl::_max_mint_default_impl(self, to)
        }
    }
}

#[cfg(all(test, feature = "e2e-tests"))]
pub mod tests {
    use super::my_psp22_vault::{Contract as VaultContract, ContractRef as VaultRef, *};
    use ink::{scale::Decode as _, ToAccountId};
    use ink_e2e::{
        account_id,
        events::ContractEmitted,
        AccountKeyring::{Alice, Bob, Charlie, Dave},
        ContractsBackend,
    };
    use my_psp22_metadata::my_psp22_metadata::{
        Contract as PSP22MetadataContract, ContractRef as PSP22MetadataRef, PSP22Metadata,
    };
    use my_psp22_mintable::my_psp22_mintable::{
        Contract as PSP22MintableContract, ContractRef as PSP22MintableRef, *,
    };

    use pendzl::{
        contracts::token::psp22::{Transfer, PSP22},
        traits::{AccountId, Balance},
    };
    use test_helpers::{
        approve, assert_eq_msg, balance_of, balance_of2, mint, mint2, run_if_test_debug,
    };

    type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

    const DECIMALS: u8 = 12;

    fn assert_psp22_transfer_event<E: ink::env::Environment<AccountId = AccountId>>(
        event: &ContractEmitted<E>,
        expected_from: Option<AccountId>,
        expected_to: Option<AccountId>,
        expected_value: Balance,
        expected_asset: AccountId,
    ) {
        let decoded_event = <Transfer>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");

        let Transfer { from, to, value } = decoded_event;

        assert_eq_msg!("Transfer.from", from, expected_from);
        assert_eq_msg!("Transfer.to", to, expected_to);
        assert_eq_msg!("Transfer.value", value, expected_value);
        assert_eq_msg!("Transfer.asset", event.contract, expected_asset);
    }

    fn assert_deposit_event<E: ink::env::Environment<AccountId = AccountId>>(
        event: &ContractEmitted<E>,
        expected_sender: AccountId,
        expected_owner: AccountId,
        expected_assets: Balance,
        expected_shares: Balance,
    ) {
        let decoded_event = <Deposit>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");

        let Deposit {
            sender,
            owner,
            assets,
            shares,
        } = decoded_event;

        assert_eq_msg!("Deposit.sender", sender, expected_sender);
        assert_eq_msg!("Deposit.owner", owner, expected_owner);
        assert_eq_msg!("Deposit.assets", assets, expected_assets);
        assert_eq_msg!("Deposit.shares", shares, expected_shares);
    }

    fn assert_withdraw_event<E: ink::env::Environment<AccountId = AccountId>>(
        event: &ContractEmitted<E>,
        expected_sender: AccountId,
        expected_receiver: AccountId,
        expected_owner: AccountId,
        expected_assets: Balance,
        expected_shares: Balance,
    ) {
        let decoded_event = <Withdraw>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");

        let Withdraw {
            sender,
            receiver,
            owner,
            assets,
            shares,
        } = decoded_event;

        assert_eq_msg!("Withdraw.sender", sender, expected_sender);
        assert_eq_msg!("Withdraw.receiver", receiver, expected_receiver);
        assert_eq_msg!("Withdraw.owner", owner, expected_owner);
        assert_eq_msg!("Withdraw.assets", assets, expected_assets);
        assert_eq_msg!("Withdraw.shares", shares, expected_shares);
    }

    #[ink_e2e::test]
    async fn inherit_decimals_if_from_asset(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        for decimals in vec![0_u8, 4, 8, 12].iter() {
            for offset in vec![0_u8, 4, 8, 12].iter() {
                let mut constructor = PSP22MetadataRef::new(1000, None, None, *decimals);
                let psp22_metadata = client
                    .instantiate("my_psp22_metadata", &ink_e2e::alice(), &mut constructor)
                    .submit()
                    .await
                    .expect("instantiate failed")
                    .call::<PSP22MetadataContract>();

                let mut constructor = VaultRef::new(psp22_metadata.to_account_id(), *offset, None);
                let vault = client
                    .instantiate("my_psp22_vault", &ink_e2e::alice(), &mut constructor)
                    .submit()
                    .await
                    .expect("instantiate failed")
                    .call::<VaultContract>();

                let vault_decimals = client
                    .call(&ink_e2e::alice(), &vault.token_decimals())
                    .dry_run()
                    .await?
                    .return_value();
                assert_eq!(
                    vault_decimals,
                    *decimals + *offset,
                    "Vault decimals should be equal to decimals of the asset plus offset"
                );
            }
        }
        Ok(())
    }

    #[ink_e2e::test]
    async fn asset_has_not_yet_been_created(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        let mut constructor = VaultRef::new(account_id(Bob), 0, None);
        let vault = client
            .instantiate("my_psp22_vault", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed")
            .call::<VaultContract>();
        let vault_decimals = client
            .call(&ink_e2e::alice(), &vault.token_decimals())
            .dry_run()
            .await?
            .return_value();
        assert_eq!(
            vault_decimals, DECIMALS,
            "Vault decimals should be equal to 12"
        );
        Ok(())
    }

    #[ink_e2e::test]
    async fn underlying_maximal_decimal(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        let mut constructor = PSP22MetadataRef::new(1000, None, None, u8::MAX);
        let psp22_metadata = client
            .instantiate("my_psp22_metadata", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed")
            .call::<PSP22MetadataContract>();

        let mut constructor = VaultRef::new(psp22_metadata.to_account_id(), 0, None);
        let vault = client
            .instantiate("my_psp22_vault", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed")
            .call::<VaultContract>();
        let vault_decimals = client
            .call(&ink_e2e::alice(), &vault.token_decimals())
            .dry_run()
            .await?
            .return_value();
        assert_eq!(
            vault_decimals,
            u8::MAX,
            "Vault decimals should be equal to u8::MAX"
        );
        Ok(())
    }

    #[ink_e2e::test]
    async fn decimals_overflow(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        let mut constructor = PSP22MetadataRef::new(1000, None, None, u8::MAX);
        let psp22_metadata = client
            .instantiate("my_psp22_metadata", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed")
            .call::<PSP22MetadataContract>();

        let mut constructor = VaultRef::new(psp22_metadata.to_account_id(), 1, None);
        let vault = client
            .instantiate("my_psp22_vault", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed")
            .call::<VaultContract>();
        let vault_decimals = client
            .call(&ink_e2e::alice(), &vault.token_decimals())
            .dry_run()
            .await;

        assert!(vault_decimals.is_err(), "should panic with \"overflow\"");
        Ok(())
    }

    #[ink_e2e::test]
    async fn limits_reverts_on_deposit_above_max_deposit(
        client: ink_e2e::Client<C, E>,
    ) -> E2EResult<()> {
        let mut constructor = PSP22MintableRef::new(0);
        let mut psp22_mintable = client
            .instantiate("my_psp22_mintable", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed")
            .call::<PSP22MintableContract>();

        let max_deposit = 1000;
        let mut constructor = VaultRef::new(psp22_mintable.to_account_id(), 0, Some(max_deposit));
        let mut vault = client
            .instantiate("my_psp22_vault", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed")
            .call::<VaultContract>();

        let deposit_amount = max_deposit + 1;
        let _ = client
            .call(
                &ink_e2e::alice(),
                &psp22_mintable.mint(account_id(Alice), deposit_amount),
            )
            .submit()
            .await
            .expect("mint failed")
            .return_value();
        let _ = client
            .call(
                &ink_e2e::alice(),
                &psp22_mintable.increase_allowance(vault.to_account_id(), deposit_amount),
            )
            .submit()
            .await
            .expect("increase allowance failed")
            .return_value();

        let deposit = client
            .call(
                &ink_e2e::alice(),
                &vault.deposit(deposit_amount, account_id(Alice)),
            )
            .dry_run()
            .await?
            .return_value();

        assert_eq!(
            deposit,
            Err(PSP22Error::Custom("Vault: Max".to_string())),
            "should return Vault: Max err"
        );
        Ok(())
    }

    #[ink_e2e::test]
    async fn limits_reverts_on_mint_above_max_mint(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        let mut constructor = PSP22MintableRef::new(0);
        let mut psp22_mintable = client
            .instantiate("my_psp22_mintable", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed")
            .call::<PSP22MintableContract>();

        let max_mint = 1000;
        let mut constructor = VaultRef::new(psp22_mintable.to_account_id(), 0, Some(max_mint));
        let mut vault = client
            .instantiate("my_psp22_vault", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed")
            .call::<VaultContract>();

        let mint_amount = max_mint + 1;
        let _ = client
            .call(
                &ink_e2e::alice(),
                &psp22_mintable.mint(account_id(Alice), mint_amount),
            )
            .submit()
            .await
            .expect("mint failed")
            .return_value();
        let _ = client
            .call(
                &ink_e2e::alice(),
                &psp22_mintable.increase_allowance(vault.to_account_id(), mint_amount),
            )
            .submit()
            .await
            .expect("increase allowance failed")
            .return_value();

        let mint = client
            .call(
                &ink_e2e::alice(),
                &vault.mint(mint_amount, account_id(Alice)),
            )
            .dry_run()
            .await?
            .return_value();

        assert_eq!(
            mint,
            Err(PSP22Error::Custom("Vault: Max".to_string())),
            "should return Vault: Max err"
        );
        Ok(())
    }

    #[ink_e2e::test]
    async fn limits_reverts_on_redeem_above_balance(
        client: ink_e2e::Client<C, E>,
    ) -> E2EResult<()> {
        let mut constructor = PSP22MintableRef::new(0);
        let mut psp22_mintable = client
            .instantiate("my_psp22_mintable", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed")
            .call::<PSP22MintableContract>();

        let balance = 1000;
        let mut constructor = VaultRef::new(psp22_mintable.to_account_id(), 0, None);
        let mut vault = client
            .instantiate("my_psp22_vault", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed")
            .call::<VaultContract>();

        let _ = client
            .call(
                &ink_e2e::alice(),
                &psp22_mintable.mint(account_id(Alice), balance),
            )
            .submit()
            .await
            .expect("mint failed")
            .return_value();
        let _ = client
            .call(
                &ink_e2e::alice(),
                &psp22_mintable.increase_allowance(vault.to_account_id(), balance),
            )
            .submit()
            .await
            .expect("increase allowance failed")
            .return_value();

        let mint_amount = balance - 1;
        let _ = client
            .call(
                &ink_e2e::alice(),
                &vault.mint(mint_amount, account_id(Alice)),
            )
            .submit()
            .await?
            .return_value();

        let redeem_amount = mint_amount + 1;

        let redeem = client
            .call(
                &ink_e2e::alice(),
                &vault.redeem(redeem_amount, account_id(Alice), account_id(Alice)),
            )
            .dry_run()
            .await?
            .return_value();

        run_if_test_debug(|| {
            println!("{:?}", redeem);
        });

        assert_eq!(
            redeem,
            Err(PSP22Error::Custom("Vault: Max".to_string())),
            "should return Vault: Max err"
        );
        Ok(())
    }

    #[ink_e2e::test]
    async fn limits_reverts_on_withdraw_above_max_withdraw(
        client: ink_e2e::Client<C, E>,
    ) -> E2EResult<()> {
        let mut constructor = PSP22MintableRef::new(0);
        let mut psp22_mintable = client
            .instantiate("my_psp22_mintable", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed")
            .call::<PSP22MintableContract>();

        let max_withdraw = 1000;
        let mut constructor = VaultRef::new(psp22_mintable.to_account_id(), 0, None);
        let mut vault = client
            .instantiate("my_psp22_vault", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed")
            .call::<VaultContract>();

        let _ = client
            .call(
                &ink_e2e::alice(),
                &psp22_mintable.mint(account_id(Alice), max_withdraw),
            )
            .submit()
            .await
            .expect("mint failed")
            .return_value();
        let _ = client
            .call(
                &ink_e2e::alice(),
                &psp22_mintable.increase_allowance(vault.to_account_id(), max_withdraw),
            )
            .submit()
            .await
            .expect("increase allowance failed")
            .return_value();

        let _ = client
            .call(
                &ink_e2e::alice(),
                &vault.deposit(max_withdraw, account_id(Alice)),
            )
            .submit()
            .await?
            .return_value();

        let withdraw_amount = max_withdraw + 1;

        let withdraw = client
            .call(
                &ink_e2e::alice(),
                &vault.withdraw(withdraw_amount, account_id(Alice), account_id(Alice)),
            )
            .dry_run()
            .await?
            .return_value();

        assert_eq!(
            withdraw,
            Err(PSP22Error::Custom("Vault: Max".to_string())),
            "should return Vault: Max err"
        );
        Ok(())
    }

    fn parse_token(token: Balance) -> Balance {
        token * 10_u128.pow(DECIMALS as u32)
    }
    fn parse_share(share: Balance, offset: &u8) -> Balance {
        share * 10_u128.pow((DECIMALS + *offset) as u32)
    }

    #[ink_e2e::test]
    async fn deposit_1_token_in_empty_vault(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        for offset in vec![0_u8, 4, 8, 12].iter() {
            run_if_test_debug(|| {
                // print whole line separator
                println!("deposit_1_token_in_empty_vault");
                println!("====================");
                println!("testing with offset: {}", offset);
                println!("====================");
            });

            //before each
            let mut constructor = PSP22MintableRef::new(0);
            let mut psp22_mintable = client
                .instantiate("my_psp22_mintable", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<PSP22MintableContract>();

            let mut constructor = VaultRef::new(psp22_mintable.to_account_id(), *offset, None);
            let mut vault = client
                .instantiate("my_psp22_vault", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<VaultContract>();
            let vault_id = vault.to_account_id();
            let expected_token_amount = parse_token(1);
            let expected_shares_amount = parse_share(1, offset);
            let _ = mint!(client, psp22_mintable, Alice, expected_token_amount);
            let _ = approve!(
                client,
                psp22_mintable,
                alice,
                vault_id,
                expected_token_amount
            );
            //end of before each

            let deposit_res = client
                .call(
                    &ink_e2e::alice(),
                    &vault.deposit(expected_token_amount, account_id(Bob)),
                )
                .submit()
                .await?;

            //verify

            let contract_emitted_events = deposit_res.contract_emitted_events()?;
            let vault_events: Vec<_> = contract_emitted_events
                .iter()
                .filter(|event_with_topics| {
                    event_with_topics.event.contract == vault.to_account_id()
                })
                .collect();
            let psp22_events: Vec<_> = contract_emitted_events
                .iter()
                .filter(|event_with_topics| {
                    event_with_topics.event.contract == psp22_mintable.to_account_id()
                })
                .collect();
            assert_eq!(deposit_res.return_value(), Ok(expected_shares_amount));

            assert_psp22_transfer_event(
                &psp22_events[1].event,
                Some(account_id(Alice)),
                Some(vault.to_account_id()),
                expected_token_amount,
                psp22_mintable.to_account_id(),
            );

            //mint shares
            assert_psp22_transfer_event(
                &vault_events[0].event,
                None,
                Some(account_id(Bob)),
                expected_shares_amount,
                vault.to_account_id(),
            );

            assert_deposit_event(
                &vault_events[1].event,
                account_id(Alice),
                account_id(Bob),
                expected_token_amount,
                expected_shares_amount,
            );

            assert_eq!(
                balance_of2!(client, psp22_mintable, vault_id),
                expected_token_amount
            );
            assert_eq!(balance_of2!(client, psp22_mintable, account_id(Alice)), 0);
            assert_eq!(
                balance_of2!(client, vault, account_id(Bob)),
                expected_shares_amount
            );
        }
        Ok(())
    }

    #[ink_e2e::test]
    async fn mint_1_token_in_empty_vault(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        for offset in vec![0_u8, 4, 8, 12].iter() {
            run_if_test_debug(|| {
                // print whole line separator
                println!("mint_1_token_in_empty_vault");
                println!("====================");
                println!("testing with offset: {}", offset);
                println!("====================");
            });

            //before each
            let mut constructor = PSP22MintableRef::new(0);
            let mut psp22_mintable = client
                .instantiate("my_psp22_mintable", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<PSP22MintableContract>();

            let mut constructor = VaultRef::new(psp22_mintable.to_account_id(), *offset, None);
            let mut vault = client
                .instantiate("my_psp22_vault", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<VaultContract>();
            let vault_id = vault.to_account_id();

            let expected_token_amount = parse_token(1);
            let expected_shares_amount = parse_share(1, offset);

            let _ = mint!(client, psp22_mintable, Alice, expected_token_amount);
            let _ = approve!(
                client,
                psp22_mintable,
                alice,
                vault_id,
                expected_token_amount
            );
            //end of before each

            let mint_res = client
                .call(
                    &ink_e2e::alice(),
                    &vault.mint(expected_shares_amount, account_id(Bob)),
                )
                .submit()
                .await
                .expect("mint failed");

            //verify

            let contract_emitted_events = mint_res.contract_emitted_events()?;
            let vault_events: Vec<_> = contract_emitted_events
                .iter()
                .filter(|event_with_topics| {
                    event_with_topics.event.contract == vault.to_account_id()
                })
                .collect();
            let psp22_events: Vec<_> = contract_emitted_events
                .iter()
                .filter(|event_with_topics| {
                    event_with_topics.event.contract == psp22_mintable.to_account_id()
                })
                .collect();
            assert_eq!(mint_res.return_value(), Ok(expected_token_amount));

            assert_psp22_transfer_event(
                &psp22_events[1].event,
                Some(account_id(Alice)),
                Some(vault.to_account_id()),
                expected_token_amount,
                psp22_mintable.to_account_id(),
            );

            //mint shares
            assert_psp22_transfer_event(
                &vault_events[0].event,
                None,
                Some(account_id(Bob)),
                expected_shares_amount,
                vault.to_account_id(),
            );

            assert_deposit_event(
                &vault_events[1].event,
                account_id(Alice),
                account_id(Bob),
                expected_token_amount,
                expected_shares_amount,
            );

            assert_eq!(
                balance_of2!(client, psp22_mintable, vault_id),
                expected_token_amount
            );
            assert_eq!(balance_of2!(client, psp22_mintable, account_id(Alice)), 0);
            assert_eq!(
                balance_of2!(client, vault, account_id(Bob)),
                expected_shares_amount
            );
        }
        Ok(())
    }

    #[ink_e2e::test]
    async fn withdraw_token_in_empty_vault(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        for offset in vec![0_u8, 4, 8, 12].iter() {
            run_if_test_debug(|| {
                // print whole line separator
                println!("withdraw_token_in_empty_vault");
                println!("====================");
                println!("testing with offset: {}", offset);
                println!("====================");
            });

            //before each
            let mut constructor = PSP22MintableRef::new(0);
            let psp22_mintable = client
                .instantiate("my_psp22_mintable", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<PSP22MintableContract>();

            let mut constructor = VaultRef::new(psp22_mintable.to_account_id(), *offset, None);
            let mut vault = client
                .instantiate("my_psp22_vault", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<VaultContract>();
            let vault_id = vault.to_account_id();
            //end of before each

            let max_withdraw = client
                .call(&ink_e2e::alice(), &vault.max_withdraw(account_id(Alice)))
                .dry_run()
                .await?
                .return_value();

            let preview_withdraw = client
                .call(&ink_e2e::alice(), &vault.preview_withdraw(0))
                .dry_run()
                .await?
                .return_value()
                .unwrap();

            let withdraw_res = client
                .call(
                    &ink_e2e::alice(),
                    &vault.withdraw(max_withdraw, account_id(Charlie), account_id(Bob)),
                )
                .submit()
                .await?;

            //verify
            assert_eq!(max_withdraw, 0);
            assert_eq!(preview_withdraw, 0);

            let contract_emitted_events = withdraw_res.contract_emitted_events()?;
            let vault_events: Vec<_> = contract_emitted_events
                .iter()
                .filter(|event_with_topics| {
                    event_with_topics.event.contract == vault.to_account_id()
                })
                .collect();
            let psp22_events: Vec<_> = contract_emitted_events
                .iter()
                .filter(|event_with_topics| {
                    event_with_topics.event.contract == psp22_mintable.to_account_id()
                })
                .collect();
            assert_eq!(withdraw_res.return_value(), Ok(0));

            assert_psp22_transfer_event(
                &psp22_events[0].event,
                Some(vault.to_account_id()),
                Some(account_id(Charlie)),
                0,
                psp22_mintable.to_account_id(),
            );

            //burn shares
            assert_psp22_transfer_event(
                &vault_events[1].event,
                Some(account_id(Bob)),
                None,
                0,
                vault.to_account_id(),
            );

            assert_withdraw_event(
                &vault_events[2].event,
                account_id(Alice),
                account_id(Charlie),
                account_id(Bob),
                0,
                0,
            );

            assert_eq!(balance_of2!(client, psp22_mintable, vault_id), 0);
            assert_eq!(balance_of2!(client, psp22_mintable, account_id(Alice)), 0);
            assert_eq!(balance_of2!(client, vault, account_id(Alice)), 0);
        }
        Ok(())
    }

    #[ink_e2e::test]
    async fn redeem_token_in_empty_vault(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        for offset in vec![0_u8, 4, 8, 12].iter() {
            run_if_test_debug(|| {
                // print whole line separator
                println!("redeem_token_in_empty_vault");
                println!("====================");
                println!("testing with offset: {}", offset);
                println!("====================");
            });

            //before each
            let mut constructor = PSP22MintableRef::new(0);
            let psp22_mintable = client
                .instantiate("my_psp22_mintable", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<PSP22MintableContract>();

            let mut constructor = VaultRef::new(psp22_mintable.to_account_id(), *offset, None);
            let mut vault = client
                .instantiate("my_psp22_vault", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<VaultContract>();
            let vault_id = vault.to_account_id();
            //end of before each

            let max_redeem = client
                .call(&ink_e2e::alice(), &vault.max_redeem(account_id(Alice)))
                .dry_run()
                .await?
                .return_value();

            let preview_redeem = client
                .call(&ink_e2e::alice(), &vault.preview_redeem(0))
                .dry_run()
                .await?
                .return_value()
                .unwrap();

            assert_eq!(max_redeem, 0);
            assert_eq!(preview_redeem, 0);
            let redeem_res = client
                .call(
                    &ink_e2e::alice(),
                    &vault.redeem(max_redeem, account_id(Charlie), account_id(Bob)),
                )
                .submit()
                .await
                .expect("redeem failed");

            //verify
            let contract_emitted_events = redeem_res.contract_emitted_events()?;
            let vault_events: Vec<_> = contract_emitted_events
                .iter()
                .filter(|event_with_topics| {
                    event_with_topics.event.contract == vault.to_account_id()
                })
                .collect();
            let psp22_events: Vec<_> = contract_emitted_events
                .iter()
                .filter(|event_with_topics| {
                    event_with_topics.event.contract == psp22_mintable.to_account_id()
                })
                .collect();
            assert_eq!(redeem_res.return_value(), Ok(0));

            assert_psp22_transfer_event(
                &psp22_events[0].event,
                Some(vault.to_account_id()),
                Some(account_id(Charlie)),
                0,
                psp22_mintable.to_account_id(),
            );

            //burn shares
            assert_psp22_transfer_event(
                &vault_events[1].event,
                Some(account_id(Bob)),
                None,
                0,
                vault.to_account_id(),
            );

            assert_withdraw_event(
                &vault_events[2].event,
                account_id(Alice),
                account_id(Charlie),
                account_id(Bob),
                0,
                0,
            );

            assert_eq!(balance_of2!(client, psp22_mintable, vault_id), 0);
            assert_eq!(balance_of2!(client, psp22_mintable, account_id(Alice)), 0);
            assert_eq!(balance_of2!(client, vault, account_id(Alice)), 0);
        }
        Ok(())
    }

    #[ink_e2e::test]
    async fn inflation_attack_deposit(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        for offset in vec![0_u8, 4, 8, 12].iter() {
            let virtual_assets = 1_u128;
            let virtual_shares = 10_u128.pow(*offset as u32);

            run_if_test_debug(|| {
                // print whole line separator
                println!("inflation_attack_deposit");
                println!("====================");
                println!("testing with offset: {}", offset);
                println!("====================");
            });

            //before each
            let mut constructor = PSP22MintableRef::new(0);
            let mut psp22_mintable = client
                .instantiate("my_psp22_mintable", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<PSP22MintableContract>();

            let mut constructor = VaultRef::new(psp22_mintable.to_account_id(), *offset, None);
            let mut vault = client
                .instantiate("my_psp22_vault", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<VaultContract>();
            let vault_id = vault.to_account_id();
            let donated_balance = parse_token(1);
            let _ = mint2!(client, psp22_mintable, vault_id, donated_balance);

            let effective_assets = donated_balance + virtual_assets;
            let effective_shares = 0 + virtual_shares;

            let expected_token_amount = parse_token(1);
            let expected_shares_amount =
                expected_token_amount * effective_shares / effective_assets;

            let _ = mint!(client, psp22_mintable, Alice, expected_token_amount);
            let _ = approve!(
                client,
                psp22_mintable,
                alice,
                vault_id,
                expected_token_amount
            );
            //end of before each

            let preview_deposit = client
                .call(
                    &ink_e2e::alice(),
                    &vault.preview_deposit(expected_token_amount),
                )
                .dry_run()
                .await?
                .return_value()
                .unwrap();

            assert_eq!(preview_deposit, expected_shares_amount);

            let deposit_res = client
                .call(
                    &ink_e2e::alice(),
                    &vault.deposit(expected_token_amount, account_id(Bob)),
                )
                .submit()
                .await?;

            //verify

            let contract_emitted_events = deposit_res.contract_emitted_events()?;
            let vault_events: Vec<_> = contract_emitted_events
                .iter()
                .filter(|event_with_topics| {
                    event_with_topics.event.contract == vault.to_account_id()
                })
                .collect();
            let psp22_events: Vec<_> = contract_emitted_events
                .iter()
                .filter(|event_with_topics| {
                    event_with_topics.event.contract == psp22_mintable.to_account_id()
                })
                .collect();
            assert_eq!(deposit_res.return_value(), Ok(expected_shares_amount));

            assert_psp22_transfer_event(
                &psp22_events[1].event,
                Some(account_id(Alice)),
                Some(vault.to_account_id()),
                expected_token_amount,
                psp22_mintable.to_account_id(),
            );

            // mint shares
            assert_psp22_transfer_event(
                &vault_events[0].event,
                None,
                Some(account_id(Bob)),
                expected_shares_amount,
                vault.to_account_id(),
            );

            assert_deposit_event(
                &vault_events[1].event,
                account_id(Alice),
                account_id(Bob),
                expected_token_amount,
                expected_shares_amount,
            );

            assert_eq!(
                balance_of2!(client, psp22_mintable, vault_id),
                expected_token_amount + donated_balance
            );
            assert_eq!(balance_of2!(client, psp22_mintable, account_id(Alice)), 0);
            assert_eq!(
                balance_of2!(client, vault, account_id(Bob)),
                expected_shares_amount
            );
        }
        Ok(())
    }

    #[ink_e2e::test]
    async fn inflation_attack_mint(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        for offset in vec![0_u8, 4, 8, 12].iter() {
            let virtual_assets = 1_u128;
            let virtual_shares = 10_u128.pow(*offset as u32);

            run_if_test_debug(|| {
                // print whole line separator
                println!("inflation_attack_mint");
                println!("====================");
                println!("testing with offset: {}", offset);
                println!("====================");
            });

            //before each
            let mut constructor = PSP22MintableRef::new(0);
            let mut psp22_mintable = client
                .instantiate("my_psp22_mintable", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<PSP22MintableContract>();

            let mut constructor = VaultRef::new(psp22_mintable.to_account_id(), *offset, None);
            let mut vault = client
                .instantiate("my_psp22_vault", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<VaultContract>();
            let vault_id = vault.to_account_id();
            let donated_balance = parse_token(1);
            let _ = mint2!(client, psp22_mintable, vault_id, donated_balance);

            let effective_assets = donated_balance + virtual_assets;
            let effective_shares = 0 + virtual_shares;

            let expected_shares_amount = parse_share(1, offset);
            let expected_token_amount =
                (expected_shares_amount * effective_assets) / effective_shares;

            let _ = mint!(client, psp22_mintable, Alice, expected_token_amount);
            let _ = approve!(
                client,
                psp22_mintable,
                alice,
                vault_id,
                expected_token_amount
            );
            //end of before each

            let preview_mint = client
                .call(
                    &ink_e2e::alice(),
                    &vault.preview_mint(expected_shares_amount),
                )
                .dry_run()
                .await?
                .return_value()
                .unwrap();

            assert_eq!(preview_mint, expected_token_amount);

            let mint_res = client
                .call(
                    &ink_e2e::alice(),
                    &vault.mint(expected_shares_amount, account_id(Bob)),
                )
                .submit()
                .await?;

            //verify

            let contract_emitted_events = mint_res.contract_emitted_events()?;
            let vault_events: Vec<_> = contract_emitted_events
                .iter()
                .filter(|event_with_topics| {
                    event_with_topics.event.contract == vault.to_account_id()
                })
                .collect();
            let psp22_events: Vec<_> = contract_emitted_events
                .iter()
                .filter(|event_with_topics| {
                    event_with_topics.event.contract == psp22_mintable.to_account_id()
                })
                .collect();
            assert_eq!(mint_res.return_value(), Ok(expected_token_amount));

            assert_psp22_transfer_event(
                &psp22_events[1].event,
                Some(account_id(Alice)),
                Some(vault.to_account_id()),
                expected_token_amount,
                psp22_mintable.to_account_id(),
            );

            // mint shares
            assert_psp22_transfer_event(
                &vault_events[0].event,
                None,
                Some(account_id(Bob)),
                expected_shares_amount,
                vault.to_account_id(),
            );

            assert_deposit_event(
                &vault_events[1].event,
                account_id(Alice),
                account_id(Bob),
                expected_token_amount,
                expected_shares_amount,
            );

            assert_eq!(
                balance_of2!(client, psp22_mintable, vault_id),
                expected_token_amount + donated_balance
            );
            assert_eq!(balance_of2!(client, psp22_mintable, account_id(Alice)), 0);
            assert_eq!(
                balance_of2!(client, vault, account_id(Bob)),
                expected_shares_amount
            );
        }
        Ok(())
    }

    #[ink_e2e::test]
    async fn withdraw_token_in_donated_vault(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        for offset in vec![0_u8, 4, 8, 12].iter() {
            run_if_test_debug(|| {
                // print whole line separator
                println!("withdraw_token_in_donated_vault");
                println!("====================");
                println!("testing with offset: {}", offset);
                println!("====================");
            });

            //before each
            let mut constructor = PSP22MintableRef::new(0);
            let mut psp22_mintable = client
                .instantiate("my_psp22_mintable", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<PSP22MintableContract>();

            let mut constructor = VaultRef::new(psp22_mintable.to_account_id(), *offset, None);
            let mut vault = client
                .instantiate("my_psp22_vault", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<VaultContract>();
            let vault_id = vault.to_account_id();

            let donated_balance = parse_token(1);
            let _ = mint2!(client, psp22_mintable, vault_id, donated_balance);
            //end of before each

            let max_withdraw = client
                .call(&ink_e2e::alice(), &vault.max_withdraw(account_id(Alice)))
                .dry_run()
                .await?
                .return_value();

            let preview_withdraw = client
                .call(&ink_e2e::alice(), &vault.preview_withdraw(0))
                .dry_run()
                .await?
                .return_value()
                .unwrap();

            let withdraw_res = client
                .call(
                    &ink_e2e::alice(),
                    &vault.withdraw(max_withdraw, account_id(Charlie), account_id(Bob)),
                )
                .submit()
                .await?;

            //verify
            assert_eq!(max_withdraw, 0);
            assert_eq!(preview_withdraw, 0);

            let contract_emitted_events = withdraw_res.contract_emitted_events()?;
            let vault_events: Vec<_> = contract_emitted_events
                .iter()
                .filter(|event_with_topics| {
                    event_with_topics.event.contract == vault.to_account_id()
                })
                .collect();
            let psp22_events: Vec<_> = contract_emitted_events
                .iter()
                .filter(|event_with_topics| {
                    event_with_topics.event.contract == psp22_mintable.to_account_id()
                })
                .collect();
            assert_eq!(withdraw_res.return_value(), Ok(0));

            assert_psp22_transfer_event(
                &psp22_events[0].event,
                Some(vault.to_account_id()),
                Some(account_id(Charlie)),
                0,
                psp22_mintable.to_account_id(),
            );

            //burn shares
            assert_psp22_transfer_event(
                &vault_events[1].event,
                Some(account_id(Bob)),
                None,
                0,
                vault.to_account_id(),
            );

            assert_withdraw_event(
                &vault_events[2].event,
                account_id(Alice),
                account_id(Charlie),
                account_id(Bob),
                0,
                0,
            );

            assert_eq!(
                balance_of2!(client, psp22_mintable, vault_id),
                donated_balance
            );
            assert_eq!(balance_of2!(client, psp22_mintable, account_id(Alice)), 0);
            assert_eq!(balance_of2!(client, vault, account_id(Alice)), 0);
        }
        Ok(())
    }

    #[ink_e2e::test]
    async fn redeem_token_in_donated_vault(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        for offset in vec![0_u8, 4, 8, 12].iter() {
            run_if_test_debug(|| {
                // print whole line separator
                println!("redeem_token_in_donated_vault");
                println!("====================");
                println!("testing with offset: {}", offset);
                println!("====================");
            });

            //before each
            let mut constructor = PSP22MintableRef::new(0);
            let mut psp22_mintable = client
                .instantiate("my_psp22_mintable", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<PSP22MintableContract>();

            let mut constructor = VaultRef::new(psp22_mintable.to_account_id(), *offset, None);
            let mut vault = client
                .instantiate("my_psp22_vault", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<VaultContract>();
            let vault_id = vault.to_account_id();

            let donated_balance = parse_token(1);
            let _ = mint2!(client, psp22_mintable, vault_id, donated_balance);
            //end of before each

            let max_redeem = client
                .call(&ink_e2e::alice(), &vault.max_redeem(account_id(Alice)))
                .dry_run()
                .await?
                .return_value();

            let preview_redeem = client
                .call(&ink_e2e::alice(), &vault.preview_redeem(0))
                .dry_run()
                .await?
                .return_value()
                .unwrap();

            assert_eq!(max_redeem, 0);
            assert_eq!(preview_redeem, 0);
            let redeem_res = client
                .call(
                    &ink_e2e::alice(),
                    &vault.redeem(max_redeem, account_id(Charlie), account_id(Bob)),
                )
                .submit()
                .await
                .expect("redeem failed");

            //verify
            let contract_emitted_events = redeem_res.contract_emitted_events()?;
            let vault_events: Vec<_> = contract_emitted_events
                .iter()
                .filter(|event_with_topics| {
                    event_with_topics.event.contract == vault.to_account_id()
                })
                .collect();
            let psp22_events: Vec<_> = contract_emitted_events
                .iter()
                .filter(|event_with_topics| {
                    event_with_topics.event.contract == psp22_mintable.to_account_id()
                })
                .collect();
            assert_eq!(redeem_res.return_value(), Ok(0));

            assert_psp22_transfer_event(
                &psp22_events[0].event,
                Some(vault.to_account_id()),
                Some(account_id(Charlie)),
                0,
                psp22_mintable.to_account_id(),
            );

            //burn shares
            assert_psp22_transfer_event(
                &vault_events[1].event,
                Some(account_id(Bob)),
                None,
                0,
                vault.to_account_id(),
            );

            assert_withdraw_event(
                &vault_events[2].event,
                account_id(Alice),
                account_id(Charlie),
                account_id(Bob),
                0,
                0,
            );

            assert_eq!(
                balance_of2!(client, psp22_mintable, vault_id),
                donated_balance
            );
            assert_eq!(balance_of2!(client, psp22_mintable, account_id(Alice)), 0);
            assert_eq!(balance_of2!(client, vault, account_id(Alice)), 0);
        }
        Ok(())
    }

    #[ink_e2e::test]
    async fn deposit_token_in_full_vault(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        for offset in vec![0_u8, 4, 8, 12].iter() {
            let virtual_assets = 1_u128;
            let virtual_shares = 10_u128.pow(*offset as u32);

            run_if_test_debug(|| {
                // print whole line separator
                println!("deposit_1_token_in_empty_vault");
                println!("====================");
                println!("testing with offset: {}", offset);
                println!("====================");
            });

            //before each
            let mut constructor = PSP22MintableRef::new(0);
            let mut psp22_mintable = client
                .instantiate("my_psp22_mintable", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<PSP22MintableContract>();

            let mut constructor = VaultRef::new(psp22_mintable.to_account_id(), *offset, None);
            let mut vault = client
                .instantiate("my_psp22_vault", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<VaultContract>();
            let vault_id = vault.to_account_id();

            let init_deposit = parse_token(10);
            let _ = mint!(client, psp22_mintable, Dave, init_deposit);
            let _ = approve!(client, psp22_mintable, dave, vault_id, init_deposit);
            let _ = client
                .call(
                    &ink_e2e::dave(),
                    &vault.deposit(init_deposit, account_id(Dave)),
                )
                .submit()
                .await
                .expect("deposit failed");
            let init_shares = balance_of!(client, vault, Dave);

            let effective_assets = init_deposit + virtual_assets;
            let effective_shares = init_shares + virtual_shares;

            let expected_token_amount = parse_token(1);
            let expected_shares_amount =
                expected_token_amount * effective_shares / effective_assets;

            let _ = mint!(client, psp22_mintable, Alice, expected_token_amount);
            let _ = approve!(
                client,
                psp22_mintable,
                alice,
                vault_id,
                expected_token_amount
            );
            //end of before each

            let deposit_res = client
                .call(
                    &ink_e2e::alice(),
                    &vault.deposit(expected_token_amount, account_id(Bob)),
                )
                .submit()
                .await?;

            //verify

            let contract_emitted_events = deposit_res.contract_emitted_events()?;
            let vault_events: Vec<_> = contract_emitted_events
                .iter()
                .filter(|event_with_topics| {
                    event_with_topics.event.contract == vault.to_account_id()
                })
                .collect();
            let psp22_events: Vec<_> = contract_emitted_events
                .iter()
                .filter(|event_with_topics| {
                    event_with_topics.event.contract == psp22_mintable.to_account_id()
                })
                .collect();
            assert_eq!(deposit_res.return_value(), Ok(expected_shares_amount));

            assert_psp22_transfer_event(
                &psp22_events[1].event,
                Some(account_id(Alice)),
                Some(vault.to_account_id()),
                expected_token_amount,
                psp22_mintable.to_account_id(),
            );

            //mint shares
            assert_psp22_transfer_event(
                &vault_events[0].event,
                None,
                Some(account_id(Bob)),
                expected_shares_amount,
                vault.to_account_id(),
            );

            assert_deposit_event(
                &vault_events[1].event,
                account_id(Alice),
                account_id(Bob),
                expected_token_amount,
                expected_shares_amount,
            );

            assert_eq!(
                balance_of2!(client, psp22_mintable, vault_id),
                expected_token_amount + init_deposit
            );
            assert_eq!(balance_of2!(client, psp22_mintable, account_id(Alice)), 0);
            assert_eq!(
                balance_of2!(client, vault, account_id(Bob)),
                expected_shares_amount
            );
        }
        Ok(())
    }

    #[ink_e2e::test]
    async fn mint_token_in_full_vault(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        for offset in vec![0_u8, 4, 8, 12].iter() {
            let virtual_assets = 1_u128;
            let virtual_shares = 10_u128.pow(*offset as u32);

            run_if_test_debug(|| {
                // print whole line separator
                println!("deposit_1_token_in_empty_vault");
                println!("====================");
                println!("testing with offset: {}", offset);
                println!("====================");
            });

            //before each
            let mut constructor = PSP22MintableRef::new(0);
            let mut psp22_mintable = client
                .instantiate("my_psp22_mintable", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<PSP22MintableContract>();

            let mut constructor = VaultRef::new(psp22_mintable.to_account_id(), *offset, None);
            let mut vault = client
                .instantiate("my_psp22_vault", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed")
                .call::<VaultContract>();
            let vault_id = vault.to_account_id();

            let init_deposit = parse_token(10);
            let _ = mint!(client, psp22_mintable, Dave, init_deposit);
            let _ = approve!(client, psp22_mintable, dave, vault_id, init_deposit);
            let _ = client
                .call(
                    &ink_e2e::dave(),
                    &vault.deposit(init_deposit, account_id(Dave)),
                )
                .submit()
                .await
                .expect("deposit failed");
            let init_shares = balance_of!(client, vault, Dave);

            let effective_assets = init_deposit + virtual_assets;
            let effective_shares = init_shares + virtual_shares;

            let expected_shares_amount = parse_share(1, offset);
            let expected_token_amount =
                (expected_shares_amount * effective_assets) / effective_shares;

            let _ = mint!(client, psp22_mintable, Alice, expected_token_amount);
            let _ = approve!(
                client,
                psp22_mintable,
                alice,
                vault_id,
                expected_token_amount
            );
            //end of before each

            let preview_mint = client
                .call(
                    &ink_e2e::alice(),
                    &vault.preview_mint(expected_shares_amount),
                )
                .dry_run()
                .await?
                .return_value()
                .unwrap();

            assert_eq!(preview_mint, expected_token_amount);

            let mint_res = client
                .call(
                    &ink_e2e::alice(),
                    &vault.mint(expected_shares_amount, account_id(Bob)),
                )
                .submit()
                .await?;

            //verify

            let contract_emitted_events = mint_res.contract_emitted_events()?;
            let vault_events: Vec<_> = contract_emitted_events
                .iter()
                .filter(|event_with_topics| {
                    event_with_topics.event.contract == vault.to_account_id()
                })
                .collect();
            let psp22_events: Vec<_> = contract_emitted_events
                .iter()
                .filter(|event_with_topics| {
                    event_with_topics.event.contract == psp22_mintable.to_account_id()
                })
                .collect();
            assert_eq!(mint_res.return_value(), Ok(expected_token_amount));

            assert_psp22_transfer_event(
                &psp22_events[1].event,
                Some(account_id(Alice)),
                Some(vault.to_account_id()),
                expected_token_amount,
                psp22_mintable.to_account_id(),
            );

            // mint shares
            assert_psp22_transfer_event(
                &vault_events[0].event,
                None,
                Some(account_id(Bob)),
                expected_shares_amount,
                vault.to_account_id(),
            );

            assert_deposit_event(
                &vault_events[1].event,
                account_id(Alice),
                account_id(Bob),
                expected_token_amount,
                expected_shares_amount,
            );

            assert_eq!(
                balance_of2!(client, psp22_mintable, vault_id),
                expected_token_amount + init_deposit
            );
            assert_eq!(balance_of2!(client, psp22_mintable, account_id(Alice)), 0);
            assert_eq!(
                balance_of2!(client, vault, account_id(Bob)),
                expected_shares_amount
            );
        }
        Ok(())
    }

    #[ink_e2e::test]
    async fn multiple_mint_deposit_redeem_withdrawal(
        client: ink_e2e::Client<C, E>,
    ) -> E2EResult<()> {
        // test designed with both asset using similar decimals

        // deploy token & vault
        let mut constructor = PSP22MintableRef::new(0);
        let mut psp22_mintable = client
            .instantiate("my_psp22_mintable", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed")
            .call::<PSP22MintableContract>();

        let mut constructor = VaultRef::new(psp22_mintable.to_account_id(), 0, None);
        let mut vault = client
            .instantiate("my_psp22_vault", &ink_e2e::alice(), &mut constructor)
            .submit()
            .await
            .expect("instantiate failed")
            .call::<VaultContract>();
        let vault_id = vault.to_account_id();

        // mint 4000 tokens to Alice, 7001 to Bob
        let _ = mint!(client, psp22_mintable, Alice, 4000);
        let _ = mint!(client, psp22_mintable, Bob, 7001);

        // approve
        let _ = approve!(client, psp22_mintable, alice, vault_id, 4000);
        let _ = approve!(client, psp22_mintable, bob, vault_id, 7001);

        // 1. Alice mints 2000 shares (costs 2000 tokens)
        run_if_test_debug(|| {
            println!("1. Alice mints 2000 shares (costs 2000 tokens)");
        });
        let mint_res = client
            .call(&ink_e2e::alice(), &vault.mint(2000, account_id(Alice)))
            .submit()
            .await?;

        //verify
        let contract_emitted_events = mint_res.contract_emitted_events()?;
        let vault_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == vault.to_account_id())
            .collect();
        let psp22_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| {
                event_with_topics.event.contract == psp22_mintable.to_account_id()
            })
            .collect();
        assert_eq!(mint_res.return_value(), Ok(2000));

        assert_psp22_transfer_event(
            &psp22_events[1].event,
            Some(account_id(Alice)),
            Some(vault.to_account_id()),
            2000,
            psp22_mintable.to_account_id(),
        );
        assert_psp22_transfer_event(
            &vault_events[0].event,
            None,
            Some(account_id(Alice)),
            2000,
            vault.to_account_id(),
        );
        assert_deposit_event(
            &vault_events[1].event,
            account_id(Alice),
            account_id(Alice),
            2000,
            2000,
        );

        let preview_deposit = client
            .call(&ink_e2e::alice(), &vault.preview_deposit(2000))
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        let alice_vault_balance = balance_of2!(client, vault, account_id(Alice));
        let bob_vault_balance = balance_of2!(client, vault, account_id(Bob));
        assert_eq!(preview_deposit, 2000);
        assert_eq!(alice_vault_balance, 2000);
        assert_eq!(bob_vault_balance, 0);

        let balance_converted_to_assets = client
            .call(
                &ink_e2e::alice(),
                &vault.convert_to_assets(alice_vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_assets, 2000);

        let balance_converted_to_assets = client
            .call(
                &ink_e2e::bob(),
                &vault.convert_to_assets(bob_vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_assets, 0);

        let vault_balance = balance_of2!(client, psp22_mintable, vault_id);

        let balance_converted_to_shares = client
            .call(
                &ink_e2e::bob(),
                &vault.convert_to_shares(vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_shares, 2000);

        let total_supply = client
            .call(&ink_e2e::bob(), &vault.total_supply())
            .dry_run()
            .await?
            .return_value();

        assert_eq!(total_supply, 2000);

        let total_assets = client
            .call(&ink_e2e::bob(), &vault.total_assets())
            .dry_run()
            .await?
            .return_value();

        assert_eq!(total_assets, 2000);

        // 2. Bob deposits 4000 tokens (mints 4000 shares)

        run_if_test_debug(|| {
            println!("2. Bob deposits 4000 tokens (mints 4000 shares)");
        });

        let deposit_res = client
            .call(&ink_e2e::bob(), &vault.deposit(4000, account_id(Bob)))
            .submit()
            .await?;

        //verify
        let contract_emitted_events = deposit_res.contract_emitted_events()?;
        let vault_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == vault.to_account_id())
            .collect();
        let psp22_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| {
                event_with_topics.event.contract == psp22_mintable.to_account_id()
            })
            .collect();
        assert_eq!(deposit_res.return_value(), Ok(4000));

        assert_psp22_transfer_event(
            &psp22_events[1].event,
            Some(account_id(Bob)),
            Some(vault.to_account_id()),
            4000,
            psp22_mintable.to_account_id(),
        );
        assert_psp22_transfer_event(
            &vault_events[0].event,
            None,
            Some(account_id(Bob)),
            4000,
            vault.to_account_id(),
        );
        assert_deposit_event(
            &vault_events[1].event,
            account_id(Bob),
            account_id(Bob),
            4000,
            4000,
        );

        let preview_deposit = client
            .call(&ink_e2e::bob(), &vault.preview_deposit(4000))
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        let alice_vault_balance = balance_of2!(client, vault, account_id(Alice));
        let bob_vault_balance = balance_of2!(client, vault, account_id(Bob));
        assert_eq!(preview_deposit, 4000);
        assert_eq!(alice_vault_balance, 2000);
        assert_eq!(bob_vault_balance, 4000);

        let balance_converted_to_assets = client
            .call(
                &ink_e2e::alice(),
                &vault.convert_to_assets(alice_vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_assets, 2000);

        let balance_converted_to_assets = client
            .call(
                &ink_e2e::bob(),
                &vault.convert_to_assets(bob_vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_assets, 4000);

        let vault_balance = balance_of2!(client, psp22_mintable, vault_id);

        let balance_converted_to_shares = client
            .call(
                &ink_e2e::charlie(),
                &vault.convert_to_shares(vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_shares, 6000);

        let total_supply = client
            .call(&ink_e2e::bob(), &vault.total_supply())
            .dry_run()
            .await?
            .return_value();

        assert_eq!(total_supply, 6000);

        let total_assets = client
            .call(&ink_e2e::bob(), &vault.total_assets())
            .dry_run()
            .await?
            .return_value();

        assert_eq!(total_assets, 6000);

        // 3. Vault mutates by +3000 tokens (simulated yield returned from strategy)

        run_if_test_debug(|| {
            println!("3. Vault mutates by +3000 tokens (simulated yield returned from strategy)");
        });

        let amount_to_mint = 3000;
        let _ = mint2!(client, psp22_mintable, vault_id, amount_to_mint);

        let alice_vault_balance = balance_of2!(client, vault, account_id(Alice));
        let bob_vault_balance = balance_of2!(client, vault, account_id(Bob));
        assert_eq!(alice_vault_balance, 2000);
        assert_eq!(bob_vault_balance, 4000);
        let bault_asset_balance = balance_of2!(client, psp22_mintable, vault_id);
        assert_eq!(bault_asset_balance, 9000);

        let balance_converted_to_assets = client
            .call(
                &ink_e2e::alice(),
                &vault.convert_to_assets(alice_vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_assets, 2999); // used to be 3000

        let balance_converted_to_assets = client
            .call(
                &ink_e2e::bob(),
                &vault.convert_to_assets(bob_vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_assets, 5999); // used to be 6000

        let vault_balance = balance_of2!(client, psp22_mintable, vault_id);

        let balance_converted_to_shares = client
            .call(
                &ink_e2e::charlie(),
                &vault.convert_to_shares(vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_shares, 6000);

        let total_supply = client
            .call(&ink_e2e::bob(), &vault.total_supply())
            .dry_run()
            .await?
            .return_value();

        assert_eq!(total_supply, 6000);

        let total_assets = client
            .call(&ink_e2e::bob(), &vault.total_assets())
            .dry_run()
            .await?
            .return_value();

        assert_eq!(total_assets, 9000);

        // 4. Alice deposits 2000 tokens (mints 1333 shares)

        run_if_test_debug(|| {
            println!("4. Alice deposits 2000 tokens (mints 1333 shares)");
        });

        let mint_res = client
            .call(&ink_e2e::alice(), &vault.deposit(2000, account_id(Alice)))
            .submit()
            .await?;

        let contract_emitted_events = mint_res.contract_emitted_events()?;
        let vault_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == vault.to_account_id())
            .collect();
        let psp22_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| {
                event_with_topics.event.contract == psp22_mintable.to_account_id()
            })
            .collect();
        assert_eq!(mint_res.return_value(), Ok(1333));

        assert_psp22_transfer_event(
            &psp22_events[1].event,
            Some(account_id(Alice)),
            Some(vault.to_account_id()),
            2000,
            psp22_mintable.to_account_id(),
        );
        assert_psp22_transfer_event(
            &vault_events[0].event,
            None,
            Some(account_id(Alice)),
            1333,
            vault.to_account_id(),
        );
        assert_deposit_event(
            &vault_events[1].event,
            account_id(Alice),
            account_id(Alice),
            2000,
            1333,
        );

        let alice_vault_balance = balance_of2!(client, vault, account_id(Alice));
        let bob_vault_balance = balance_of2!(client, vault, account_id(Bob));
        assert_eq!(alice_vault_balance, 3333);
        assert_eq!(bob_vault_balance, 4000);

        let balance_converted_to_assets = client
            .call(
                &ink_e2e::alice(),
                &vault.convert_to_assets(alice_vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_assets, 4999);

        let balance_converted_to_assets = client
            .call(
                &ink_e2e::bob(),
                &vault.convert_to_assets(bob_vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_assets, 6000);

        let vault_balance = balance_of2!(client, psp22_mintable, vault_id);

        let balance_converted_to_shares = client
            .call(
                &ink_e2e::bob(),
                &vault.convert_to_shares(vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_shares, 7333);

        let total_supply = client
            .call(&ink_e2e::bob(), &vault.total_supply())
            .dry_run()
            .await?
            .return_value();
        let total_assets = client
            .call(&ink_e2e::bob(), &vault.total_assets())
            .dry_run()
            .await?
            .return_value();
        assert_eq!(total_supply, 7333);
        assert_eq!(total_assets, 11000);

        // 5. Bob mints 2000 shares (costs 3000 assets)
        // NOTE: Bob's assets spent got rounded towards infinity
        // NOTE: Alices's vault assets got rounded towards infinity

        run_if_test_debug(|| {
            println!("5. Bob mints 2000 shares (costs 3000 assets)");
        });

        let mint_res = client
            .call(&ink_e2e::bob(), &vault.mint(2000, account_id(Bob)))
            .submit()
            .await?;

        let contract_emitted_events = mint_res.contract_emitted_events()?;
        let vault_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == vault.to_account_id())
            .collect();
        let psp22_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| {
                event_with_topics.event.contract == psp22_mintable.to_account_id()
            })
            .collect();

        assert_psp22_transfer_event(
            &psp22_events[1].event,
            Some(account_id(Bob)),
            Some(vault.to_account_id()),
            3000,
            psp22_mintable.to_account_id(),
        );
        assert_psp22_transfer_event(
            &vault_events[0].event,
            None,
            Some(account_id(Bob)),
            2000,
            vault.to_account_id(),
        );
        assert_deposit_event(
            &vault_events[1].event,
            account_id(Bob),
            account_id(Bob),
            3000,
            2000,
        );

        let alice_vault_balance = balance_of2!(client, vault, account_id(Alice));
        let bob_vault_balance = balance_of2!(client, vault, account_id(Bob));
        assert_eq!(alice_vault_balance, 3333);
        assert_eq!(bob_vault_balance, 6000);

        let balance_converted_to_assets = client
            .call(
                &ink_e2e::alice(),
                &vault.convert_to_assets(alice_vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_assets, 4999); //used to be 5000

        let balance_converted_to_assets = client
            .call(
                &ink_e2e::bob(),
                &vault.convert_to_assets(bob_vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_assets, 9000);

        let vault_balance = balance_of2!(client, psp22_mintable, vault_id);

        let balance_converted_to_shares = client
            .call(
                &ink_e2e::bob(),
                &vault.convert_to_shares(vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_shares, 9333);

        let total_supply = client
            .call(&ink_e2e::bob(), &vault.total_supply())
            .dry_run()
            .await?
            .return_value();
        let total_assets = client
            .call(&ink_e2e::bob(), &vault.total_assets())
            .dry_run()
            .await?
            .return_value();

        assert_eq!(total_supply, 9333);
        assert_eq!(total_assets, 14000);

        // 6. Vault mutates by +3000 tokens
        // NOTE: Vault holds 17001 tokens, but sum of assetsOf() is 17000.

        run_if_test_debug(|| {
            println!("6. Vault mutates by +3000 tokens");
        });

        let mint_amount = 3000;
        let _ = mint2!(client, psp22_mintable, vault_id, mint_amount);

        let alice_vault_balance = balance_of2!(client, vault, account_id(Alice));
        let bob_vault_balance = balance_of2!(client, vault, account_id(Bob));
        assert_eq!(alice_vault_balance, 3333);
        assert_eq!(bob_vault_balance, 6000);

        let balance_converted_to_assets = client
            .call(
                &ink_e2e::alice(),
                &vault.convert_to_assets(alice_vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_assets, 6070); // used to be 6071

        let balance_converted_to_assets = client
            .call(
                &ink_e2e::bob(),
                &vault.convert_to_assets(bob_vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_assets, 10928); // used to be 10929

        let vault_balance = balance_of2!(client, psp22_mintable, vault_id);

        let balance_converted_to_shares = client
            .call(
                &ink_e2e::charlie(),
                &vault.convert_to_shares(vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_shares, 9333);

        let total_supply = client
            .call(&ink_e2e::bob(), &vault.total_supply())
            .dry_run()
            .await?
            .return_value();
        let total_assets = client
            .call(&ink_e2e::bob(), &vault.total_assets())
            .dry_run()
            .await?
            .return_value();

        assert_eq!(total_supply, 9333);
        assert_eq!(total_assets, 17000); // used to be 17001

        // 7. Alice redeem 1333 shares (2427 assets) // used to be 2428

        run_if_test_debug(|| {
            println!("7. Alice redeem 1333 shares (2427 assets)");
        });
        let redeem_res = client
            .call(
                &ink_e2e::alice(),
                &vault.redeem(1333, account_id(Alice), account_id(Alice)),
            )
            .submit()
            .await?;

        let contract_emitted_events = redeem_res.contract_emitted_events()?;
        let vault_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == vault.to_account_id())
            .collect();
        let psp22_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| {
                event_with_topics.event.contract == psp22_mintable.to_account_id()
            })
            .collect();

        assert_psp22_transfer_event(
            &psp22_events[0].event,
            Some(vault.to_account_id()),
            Some(account_id(Alice)),
            2427,
            psp22_mintable.to_account_id(),
        );
        assert_psp22_transfer_event(
            &vault_events[0].event,
            Some(account_id(Alice)),
            None,
            1333,
            vault.to_account_id(),
        );

        let alice_vault_balance = balance_of2!(client, vault, account_id(Alice));
        let bob_vault_balance = balance_of2!(client, vault, account_id(Bob));
        assert_eq!(alice_vault_balance, 2000);
        assert_eq!(bob_vault_balance, 6000);

        let balance_converted_to_assets = client
            .call(
                &ink_e2e::alice(),
                &vault.convert_to_assets(alice_vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_assets, 3643);

        let balance_converted_to_assets = client
            .call(
                &ink_e2e::bob(),
                &vault.convert_to_assets(bob_vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_assets, 10929);

        let vault_balance = balance_of2!(client, psp22_mintable, vault_id);

        let balance_converted_to_shares = client
            .call(
                &ink_e2e::bob(),
                &vault.convert_to_shares(vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_shares, 8000);

        let total_supply = client
            .call(&ink_e2e::bob(), &vault.total_supply())
            .dry_run()
            .await?
            .return_value();

        assert_eq!(total_supply, 8000);

        let total_assets = client
            .call(&ink_e2e::bob(), &vault.total_assets())
            .dry_run()
            .await?
            .return_value();

        assert_eq!(total_assets, 14573);

        // 8. Bob withdraws 2929 assets (1608 shares)
        run_if_test_debug(|| {
            println!("8. Bob withdraws 2929 assets (1608 shares)");
        });

        let withdraw_res = client
            .call(
                &ink_e2e::bob(),
                &vault.withdraw(2929, account_id(Bob), account_id(Bob)),
            )
            .submit()
            .await?;

        let contract_emitted_events = withdraw_res.contract_emitted_events()?;
        let vault_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == vault.to_account_id())
            .collect();
        let psp22_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| {
                event_with_topics.event.contract == psp22_mintable.to_account_id()
            })
            .collect();

        assert_psp22_transfer_event(
            &psp22_events[0].event,
            Some(vault.to_account_id()),
            Some(account_id(Bob)),
            2929,
            psp22_mintable.to_account_id(),
        );
        assert_psp22_transfer_event(
            &vault_events[0].event,
            Some(account_id(Bob)),
            None,
            1608,
            vault.to_account_id(),
        );

        let alice_vault_balance = balance_of2!(client, vault, account_id(Alice));
        let bob_vault_balance = balance_of2!(client, vault, account_id(Bob));
        assert_eq!(alice_vault_balance, 2000);
        assert_eq!(bob_vault_balance, 4392);

        let balance_converted_to_assets = client
            .call(
                &ink_e2e::alice(),
                &vault.convert_to_assets(alice_vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_assets, 3643);

        let balance_converted_to_assets = client
            .call(
                &ink_e2e::bob(),
                &vault.convert_to_assets(bob_vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_assets, 8000);

        let vault_balance = balance_of2!(client, psp22_mintable, vault_id);

        let balance_converted_to_shares = client
            .call(
                &ink_e2e::bob(),
                &vault.convert_to_shares(vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_shares, 6392);

        let total_supply = client
            .call(&ink_e2e::bob(), &vault.total_supply())
            .dry_run()
            .await?
            .return_value();

        let total_assets = client
            .call(&ink_e2e::bob(), &vault.total_assets())
            .dry_run()
            .await?
            .return_value();

        assert_eq!(total_supply, 6392);
        assert_eq!(total_assets, 11644);

        // 9. Alice withdraws 3643 assets (2000 shares)
        // NOTE: Bob's assets have been rounded back towards infinity
        let withdraw_res = client
            .call(
                &ink_e2e::alice(),
                &vault.withdraw(3643, account_id(Alice), account_id(Alice)),
            )
            .submit()
            .await?;

        let contract_emitted_events = withdraw_res.contract_emitted_events()?;
        let vault_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == vault.to_account_id())
            .collect();
        let psp22_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| {
                event_with_topics.event.contract == psp22_mintable.to_account_id()
            })
            .collect();

        assert_psp22_transfer_event(
            &psp22_events[0].event,
            Some(vault.to_account_id()),
            Some(account_id(Alice)),
            3643,
            psp22_mintable.to_account_id(),
        );
        assert_psp22_transfer_event(
            &vault_events[0].event,
            Some(account_id(Alice)),
            None,
            2000,
            vault.to_account_id(),
        );

        let alice_vault_balance = balance_of2!(client, vault, account_id(Alice));
        let bob_vault_balance = balance_of2!(client, vault, account_id(Bob));
        assert_eq!(alice_vault_balance, 0);
        assert_eq!(bob_vault_balance, 4392);

        let balance_converted_to_assets = client
            .call(
                &ink_e2e::alice(),
                &vault.convert_to_assets(alice_vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_assets, 0);

        let balance_converted_to_assets = client
            .call(
                &ink_e2e::bob(),
                &vault.convert_to_assets(bob_vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_assets, 8000);

        let vault_balance = balance_of2!(client, psp22_mintable, vault_id);

        let balance_converted_to_shares = client
            .call(
                &ink_e2e::bob(),
                &vault.convert_to_shares(vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_shares, 4392);

        let total_supply = client
            .call(&ink_e2e::bob(), &vault.total_supply())
            .dry_run()
            .await?
            .return_value();

        let total_assets = client
            .call(&ink_e2e::bob(), &vault.total_assets())
            .dry_run()
            .await?
            .return_value();

        assert_eq!(total_supply, 4392);
        assert_eq!(total_assets, 8001);

        // 10. Bob redeem 4392 shares (8000 tokens)

        let redeem_res = client
            .call(
                &ink_e2e::bob(),
                &vault.redeem(4392, account_id(Bob), account_id(Bob)),
            )
            .submit()
            .await?;

        let contract_emitted_events = redeem_res.contract_emitted_events()?;
        let vault_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == vault.to_account_id())
            .collect();
        let psp22_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| {
                event_with_topics.event.contract == psp22_mintable.to_account_id()
            })
            .collect();
        assert_eq!(redeem_res.return_value(), Ok(8000));

        assert_psp22_transfer_event(
            &psp22_events[0].event,
            Some(vault.to_account_id()),
            Some(account_id(Bob)),
            8000,
            psp22_mintable.to_account_id(),
        );
        assert_psp22_transfer_event(
            &vault_events[0].event,
            Some(account_id(Bob)),
            None,
            4392,
            vault.to_account_id(),
        );

        let alice_vault_balance = balance_of2!(client, vault, account_id(Alice));
        let bob_vault_balance = balance_of2!(client, vault, account_id(Bob));
        assert_eq!(alice_vault_balance, 0);
        assert_eq!(bob_vault_balance, 0);

        let balance_converted_to_assets = client
            .call(
                &ink_e2e::alice(),
                &vault.convert_to_assets(alice_vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_assets, 0);

        let balance_converted_to_assets = client
            .call(
                &ink_e2e::bob(),
                &vault.convert_to_assets(bob_vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_assets, 0);

        let vault_balance = balance_of2!(client, psp22_mintable, vault_id);

        let balance_converted_to_shares = client
            .call(
                &ink_e2e::bob(),
                &vault.convert_to_shares(vault_balance, Rounding::Down),
            )
            .dry_run()
            .await?
            .return_value()
            .unwrap();
        assert_eq!(balance_converted_to_shares, 0);

        let total_supply = client
            .call(&ink_e2e::bob(), &vault.total_supply())
            .dry_run()
            .await?
            .return_value();
        let total_assets = client
            .call(&ink_e2e::bob(), &vault.total_assets())
            .dry_run()
            .await?
            .return_value();

        assert_eq!(total_supply, 0);
        assert_eq!(total_assets, 1);

        Ok(())
    }
}
