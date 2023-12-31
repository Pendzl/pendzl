#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(PSP37, PSP37Batch)]
#[ink::contract]
pub mod my_psp37 {
    use pendzl::traits::Storage;

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), PSP37Error> {
            psp37::Internal::_mint_to(self, to, ids_amounts)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::{
            address_of,
            balance_of_37,
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn batch_transfer_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp37_batch", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 20;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.mint(
                        address_of!(Alice),
                        vec![(token_1.clone(), amount_1.clone()), (token_2.clone(), amount_2.clone())],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            let batch_transfer_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.batch_transfer(
                        address_of!(Bob),
                        vec![(token_1.clone(), amount_1.clone()), (token_2.clone(), amount_2.clone())],
                        vec![],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(batch_transfer_tx, Ok(()));

            assert_eq!(balance_of_37!(client, address, Bob, Some(token_1.clone())), amount_1);
            assert_eq!(balance_of_37!(client, address, Bob, Some(token_2.clone())), amount_2);
            assert_eq!(balance_of_37!(client, address, Alice, Some(token_1.clone())), 0);
            assert_eq!(balance_of_37!(client, address, Alice, Some(token_2.clone())), 0);

            Ok(())
        }

        #[ink_e2e::test]
        async fn batch_transfer_from_should_work(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp37_batch", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 20;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.mint(
                        address_of!(Alice),
                        vec![(token_1.clone(), amount_1.clone()), (token_2.clone(), amount_2.clone())],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            assert_eq!(balance_of_37!(client, address, Alice, Some(token_1.clone())), amount_1);
            assert_eq!(balance_of_37!(client, address, Alice, Some(token_2.clone())), amount_2);
            assert_eq!(balance_of_37!(client, address, Bob, Some(token_1.clone())), 0);
            assert_eq!(balance_of_37!(client, address, Bob, Some(token_2.clone())), 0);

            let batch_transfer_from_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.batch_transfer_from(
                        address_of!(Alice),
                        address_of!(Bob),
                        vec![(token_1.clone(), amount_1.clone()), (token_2.clone(), amount_2.clone())],
                        vec![],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(batch_transfer_from_tx, Ok(()));

            assert_eq!(balance_of_37!(client, address, Bob, Some(token_1.clone())), amount_1);
            assert_eq!(balance_of_37!(client, address, Bob, Some(token_2.clone())), amount_2);
            assert_eq!(balance_of_37!(client, address, Alice, Some(token_1.clone())), 0);
            assert_eq!(balance_of_37!(client, address, Alice, Some(token_2.clone())), 0);

            Ok(())
        }

        #[ink_e2e::test]
        async fn batch_transfer_from_with_no_approve_should_fail(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp37_batch", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 20;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.mint(
                        address_of!(Alice),
                        vec![(token_1.clone(), amount_1.clone()), (token_2.clone(), amount_2.clone())],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            let batch_transfer_from_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.batch_transfer_from(
                        address_of!(Bob),
                        address_of!(Alice),
                        vec![(token_1.clone(), amount_1.clone()), (token_2.clone(), amount_2.clone())],
                        vec![],
                    )
                });
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(batch_transfer_from_tx, Err(_)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn batch_transfer_from_with_approve_should_work(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_psp37_batch", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 20;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.mint(
                        address_of!(Alice),
                        vec![(token_1.clone(), amount_1.clone()), (token_2.clone(), amount_2.clone())],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            let approve_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.approve(address_of!(Bob), None, 1));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("approve failed")
            }
            .return_value();

            assert_eq!(approve_tx, Ok(()));

            let batch_transfer_from_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.batch_transfer_from(
                        address_of!(Alice),
                        address_of!(Bob),
                        vec![(token_1.clone(), amount_1.clone()), (token_2.clone(), amount_2.clone())],
                        vec![],
                    )
                });
                client
                    .call(&ink_e2e::bob(), _msg, 0, None)
                    .await
                    .expect("transfer failed")
            }
            .return_value();

            assert_eq!(batch_transfer_from_tx, Ok(()));

            assert_eq!(balance_of_37!(client, address, Bob, Some(token_1.clone())), amount_1);
            assert_eq!(balance_of_37!(client, address, Bob, Some(token_2.clone())), amount_2);
            assert_eq!(balance_of_37!(client, address, Alice, Some(token_1.clone())), 0);
            assert_eq!(balance_of_37!(client, address, Alice, Some(token_2.clone())), 0);

            Ok(())
        }
    }
}
