// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(PSP22, PSP22Vault, PSP22Metadata)]
#[ink::contract]
pub mod my_psp22_vault {
    use pendzl::traits::String;
    #[ink(storage)]
    #[derive(Default, Storage)]
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

    use pendzl::contracts::token::psp22::extensions::vault::implementation::PSP22VaultInternalDefaultImpl;

    #[overrider(PSP22VaultInternal)]
    fn _max_deposit(&self, to: &AccountId) -> Balance {
        if let Some(v) = self.max_deposit_and_mint {
            v
        } else {
            self._max_deposit_default_impl(to)
        }
    }
    #[overrider(PSP22VaultInternal)]
    fn _max_mint(&self, to: &AccountId) -> Balance {
        if let Some(v) = self.max_deposit_and_mint {
            v
        } else {
            self._max_mint_default_impl(to)
        }
    }
}

#[cfg(all(test, feature = "e2e-tests"))]
pub mod tests {
    use super::my_psp22_vault::{Contract as VaultContract, ContractRef as VaultRef, *};
    use my_psp22_metadata::my_psp22_metadata::{
        Contract as PSP22MetadataContract, ContractRef as PSP22MetadataRef, *,
    };
    use my_psp22_mintable::my_psp22_mintable::{
        Contract as PSP22MintableContract, ContractRef as PSP22MintableRef, *,
    };

    use ink::ToAccountId;
    use ink_e2e::account_id;
    use ink_e2e::AccountKeyring::{Alice, Bob};
    use ink_e2e::ContractsBackend;
    use test_helpers::balance_of;
    use test_helpers::mint;

    use my_psp22_metadata::my_psp22_metadata::PSP22Metadata;
    use pendzl::contracts::token::psp22::PSP22;

    type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

    const NAME: &str = "My Token";
    const SYMBAOL: &str = "MTKN";
    const DECIMALS: u8 = 12;

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

        assert!(
            matches!(vault_decimals, Err(CallDryRun)),
            "should panic with \"overflow\""
        );
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
                &psp22_mintable.mint(ink_e2e::account_id(Alice), deposit_amount),
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
            format!("{:?}", deposit),
            "Err(Custom(\"Vault: Max\"))",
            "should panic with \"Vault: Max\""
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
                &psp22_mintable.mint(ink_e2e::account_id(Alice), mint_amount),
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
            format!("{:?}", mint),
            "Err(Custom(\"Vault: Max\"))",
            "should panic with \"Vault: Max\""
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
                &psp22_mintable.mint(ink_e2e::account_id(Alice), balance),
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

        println!("{:?}", redeem);

        assert_eq!(
            format!("{:?}", redeem),
            "Err(Custom(\"Vault: Max\"))",
            "should panic with \"Vault: Max\""
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
                &psp22_mintable.mint(ink_e2e::account_id(Alice), max_withdraw),
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
            format!("{:?}", withdraw),
            "Err(Custom(\"Vault: Max\"))",
            "should panic with \"Vault: Max\""
        );
        Ok(())
    }
}
