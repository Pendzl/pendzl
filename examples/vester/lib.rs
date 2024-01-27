// SPDX-License-Identifier: MIT
#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[pendzl::implementation(GeneralVest)]
#[ink::contract]
pub mod vester {
    #[ink(storage)]
    #[derive(Default, StorageFieldGetter)]
    pub struct Vester {
        #[storage_field]
        general_vest: GeneralVestData,
    }

    impl Vester {
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }
    }
}

#[cfg(all(test, feature = "e2e-tests"))]
pub mod tests {
    use crate::vester::{VesterRef, VestingError, VestingSchedule, *};
    use ink::{
        codegen::Env,
        env::{test::EmittedEvent, DefaultEnvironment},
        scale::Decode as _,
        ToAccountId,
    };
    use ink_e2e::{events::ContractEmitted, ChainBackend, ContractsBackend};
    use my_psp22_mintable::my_psp22_mintable::{ContractRef as PSP22Ref, *};
    use pendzl::{
        contracts::token::psp22::{PSP22Error, Transfer, PSP22},
        test_utils::{
            accounts, change_caller, get_account_balance, set_account_balance, set_block_timestamp,
            set_value_transferred,
        },
        traits::{AccountId, Balance, Timestamp},
    };
    use test_helpers::{assert_eq_msg, assert_lt, keypair_to_account};

    pub const ONE_HOUR: u64 = 60 * 60 * 1000;
    pub const ONE_DAY: u64 = 24 * ONE_HOUR;

    struct CreateVestingScheduleArgs {
        vest_to: AccountId,
        asset: Option<AccountId>,
        amount: Balance,
        schedule: VestingSchedule,
    }

    fn create_duration_as_amount_schedule_args(
        vest_to: AccountId,
        asset: Option<AccountId>,
        waiting_duration: Timestamp,
        duration: Timestamp,
    ) -> CreateVestingScheduleArgs {
        CreateVestingScheduleArgs {
            vest_to,
            asset,
            amount: duration.into(),
            schedule: VestingSchedule::Constant(waiting_duration, duration),
        }
    }

    fn assert_token_released_event(
        event: &EmittedEvent,
        expected_caller: AccountId,
        expected_to: AccountId,
        expected_asset: Option<AccountId>,
        expected_amount: Balance,
    ) {
        let TokenReleased {
            caller,
            asset,
            to,
            amount,
        } = <TokenReleased>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");

        assert_eq_msg!("caller", caller, expected_caller);
        assert_eq_msg!("Assets", asset, expected_asset);
        assert_eq_msg!("To", to, expected_to);
        assert_eq_msg!("Amounts", amount, expected_amount);
    }

    fn assert_vesting_scheduled_event(
        event: &ContractEmitted<DefaultEnvironment>,
        expected_creator: AccountId,
        expected_receiver: AccountId,
        expected_asset: Option<AccountId>,
        expected_amount: Balance,
        expected_schedule: VestingSchedule,
    ) {
        let VestingScheduled {
            creator,
            asset,
            receiver,
            amount,
            schedule,
        } = <VestingScheduled>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");
        assert_eq_msg!("Asset", asset, expected_asset);
        assert_eq_msg!("creator", creator, expected_creator);
        assert_eq_msg!("receiver", receiver, expected_receiver);
        assert_eq_msg!("Amounts", amount, expected_amount);
        assert_eq_msg!("schedule", schedule, expected_schedule);
    }

    fn assert_psp22_transfer_event<E: ink::env::Environment<AccountId = AccountId>>(
        event: &ContractEmitted<E>,
        expected_from: AccountId,
        expected_to: AccountId,
        expected_value: Balance,
        expected_asset: AccountId,
    ) {
        let decoded_event = <Transfer>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");

        let Transfer { from, to, value } = decoded_event;

        assert_eq_msg!("Transfer.from", from, Some(expected_from));
        assert_eq_msg!("Transfer.to", to, Some(expected_to));
        assert_eq_msg!("Transfer.value", value, expected_value);
        assert_eq_msg!("Transfer.asset", event.contract, expected_asset);
    }

    fn assert_token_released_event_e2e<E: ink::env::Environment<AccountId = AccountId>>(
        event: &ContractEmitted<E>,
        expected_caller: AccountId,
        expected_to: AccountId,
        expected_asset: Option<AccountId>,
        expected_amount: Balance,
    ) {
        let TokenReleased {
            caller,
            asset,
            to,
            amount,
        } = <TokenReleased>::decode(&mut &event.data[..])
            .expect("encountered invalid contract event data buffer");

        assert_eq_msg!("caller", caller, expected_caller);
        assert_eq_msg!("Assets", asset, expected_asset);
        assert_eq_msg!("To", to, expected_to);
        assert_eq_msg!("Amounts", amount, expected_amount);
    }

    type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;
    #[ink_e2e::test]
    async fn create_vesting_schedule_psp22(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        let vester_creator = ink_e2e::alice();
        let vester_submitter = client
            .create_and_fund_account(&ink_e2e::alice(), 10_000_000_000_000)
            .await;
        let psp22_mintable_creator = ink_e2e::bob();
        let mut psp22_constructor = PSP22Ref::new(1_000_000);
        let mut psp22 = client
            .instantiate(
                "my_psp22_mintable",
                &psp22_mintable_creator,
                &mut psp22_constructor,
            )
            .submit()
            .await
            .expect("instantiate psp22 failed")
            .call::<Contract>();

        // create_vest args
        let create_vest_args = CreateVestingScheduleArgs {
            vest_to: keypair_to_account(&ink_e2e::charlie()),
            asset: Some(psp22.to_account_id()),
            amount: 100,
            schedule: VestingSchedule::Constant(1, 2),
        };

        let mut vester_constructor = VesterRef::new();
        let mut vester = client
            .instantiate("vester", &vester_creator, &mut vester_constructor)
            .submit()
            .await
            .expect("instantiate vester failed")
            .call::<Vester>();

        let create_vest_res = client
            .call(
                &vester_submitter,
                &vester.create_vest(
                    create_vest_args.vest_to,
                    create_vest_args.asset,
                    create_vest_args.amount,
                    create_vest_args.schedule.clone(),
                    vec![],
                ),
            )
            .dry_run()
            .await
            .expect("create vest failed")
            .return_value();
        assert_eq!(
            create_vest_res,
            Err(VestingError::PSP22Error(PSP22Error::InsufficientAllowance))
        );

        let _ = client
            .call(
                &vester_submitter,
                &psp22.increase_allowance(vester.to_account_id(), create_vest_args.amount),
            )
            .submit()
            .await
            .expect("give allowance failed")
            .return_value();

        let create_vest_res = client
            .call(
                &vester_submitter,
                &vester.create_vest(
                    create_vest_args.vest_to,
                    create_vest_args.asset,
                    create_vest_args.amount,
                    create_vest_args.schedule.clone(),
                    vec![],
                ),
            )
            .dry_run()
            .await
            .expect("create vest failed")
            .return_value();
        assert_eq!(
            create_vest_res,
            Err(VestingError::PSP22Error(PSP22Error::InsufficientBalance))
        );

        let _ = client
            .call(
                &vester_creator,
                &psp22.mint(
                    keypair_to_account(&vester_submitter),
                    create_vest_args.amount,
                ),
            )
            .submit()
            .await
            .expect("mint failed");

        let balance_of_vester = client
            .call(&ink_e2e::alice(), &psp22.balance_of(vester.to_account_id()))
            .dry_run()
            .await?
            .return_value();

        let balance_of_vester_submitter = client
            .call(
                &ink_e2e::alice(),
                &psp22.balance_of(keypair_to_account(&vester_submitter)),
            )
            .dry_run()
            .await?
            .return_value();

        assert_eq!(balance_of_vester, 0);
        assert_eq!(balance_of_vester_submitter, create_vest_args.amount);

        let create_vest_res = client
            .call(
                &vester_submitter,
                &vester.create_vest(
                    create_vest_args.vest_to,
                    create_vest_args.asset,
                    create_vest_args.amount,
                    create_vest_args.schedule.clone(),
                    vec![],
                ),
            )
            .submit()
            .await
            .expect("create vest failed");

        let contract_emitted_events = create_vest_res.contract_emitted_events()?;
        let vester_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == vester.to_account_id())
            .collect();
        let psp22_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == psp22.to_account_id())
            .collect();
        assert!(matches!(create_vest_res.return_value(), Ok(())));

        assert_psp22_transfer_event(
            &psp22_events[1].event,
            keypair_to_account(&vester_submitter),
            vester.to_account_id(),
            create_vest_args.amount,
            psp22.to_account_id(),
        );
        assert_vesting_scheduled_event(
            &vester_events[0].event,
            keypair_to_account(&vester_submitter),
            create_vest_args.vest_to,
            Some(psp22.to_account_id()),
            create_vest_args.amount,
            create_vest_args.schedule,
        );

        let balance_of_vester = client
            .call(&ink_e2e::alice(), &psp22.balance_of(vester.to_account_id()))
            .dry_run()
            .await?
            .return_value();
        let balance_of_vester_submitter = client
            .call(
                &ink_e2e::alice(),
                &psp22.balance_of(keypair_to_account(&vester_submitter)),
            )
            .dry_run()
            .await?
            .return_value();

        assert_eq!(balance_of_vester, create_vest_args.amount);
        assert_eq!(balance_of_vester_submitter, 0);

        Ok(())
    }
    #[ink_e2e::test]
    async fn release_psp22(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        let vester_creator = ink_e2e::alice();
        let vester_submitter = client
            .create_and_fund_account(&ink_e2e::alice(), 10_000_000_000_000)
            .await;
        let psp22_mintable_creator = ink_e2e::bob();
        let mut psp22_constructor = PSP22Ref::new(1_000_000);
        let mut psp22 = client
            .instantiate(
                "my_psp22_mintable",
                &psp22_mintable_creator,
                &mut psp22_constructor,
            )
            .submit()
            .await
            .expect("instantiate psp22 failed")
            .call::<Contract>();
        let vest_to = ink_e2e::charlie();

        // create_vest args
        let create_vest_args = CreateVestingScheduleArgs {
            vest_to: keypair_to_account(&ink_e2e::charlie()),
            asset: Some(psp22.to_account_id()),
            amount: 100,
            schedule: VestingSchedule::Constant(1, 2),
        };

        let mut vester_constructor = VesterRef::new();
        let mut vester = client
            .instantiate("vester", &vester_creator, &mut vester_constructor)
            .submit()
            .await
            .expect("instantiate vester failed")
            .call::<Vester>();

        let _ = client
            .call(
                &vester_submitter,
                &psp22.increase_allowance(vester.to_account_id(), create_vest_args.amount),
            )
            .submit()
            .await
            .expect("give allowance failed")
            .return_value();

        let _ = client
            .call(
                &vester_creator,
                &psp22.mint(
                    keypair_to_account(&vester_submitter),
                    create_vest_args.amount,
                ),
            )
            .submit()
            .await
            .expect("mint failed");

        let balance_of_vester = client
            .call(&ink_e2e::alice(), &psp22.balance_of(vester.to_account_id()))
            .dry_run()
            .await?
            .return_value();

        let create_vest_res = client
            .call(
                &vester_submitter,
                &vester.create_vest(
                    create_vest_args.vest_to,
                    create_vest_args.asset,
                    create_vest_args.amount,
                    create_vest_args.schedule.clone(),
                    vec![],
                ),
            )
            .submit()
            .await
            .expect("create vest failed");

        let contract_emitted_events = create_vest_res.contract_emitted_events()?;
        let vester_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == vester.to_account_id())
            .collect();
        let psp22_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == psp22.to_account_id())
            .collect();
        assert!(matches!(create_vest_res.return_value(), Ok(())));

        assert_psp22_transfer_event(
            &psp22_events[1].event, //psp22 transfer emits 2 events, here we check for the actual Transfer event
            keypair_to_account(&vester_submitter),
            vester.to_account_id(),
            create_vest_args.amount,
            psp22.to_account_id(),
        );
        assert_vesting_scheduled_event(
            &vester_events[0].event,
            keypair_to_account(&vester_submitter),
            create_vest_args.vest_to,
            Some(psp22.to_account_id()),
            create_vest_args.amount,
            create_vest_args.schedule,
        );

        let balance_of_vester_pre = client
            .call(&ink_e2e::alice(), &psp22.balance_of(vester.to_account_id()))
            .dry_run()
            .await?
            .return_value();
        let balance_of_vest_receiver_pre = client
            .call(
                &ink_e2e::alice(),
                &psp22.balance_of(create_vest_args.vest_to),
            )
            .dry_run()
            .await?
            .return_value();

        let dry_run = client
            .call(
                &vest_to,
                &vester.release(
                    Some(create_vest_args.vest_to),
                    create_vest_args.asset,
                    vec![],
                ),
            )
            .dry_run()
            .await
            .expect("release failed");

        let release_res = client
            .call(
                &vest_to,
                &vester.release(
                    Some(create_vest_args.vest_to),
                    create_vest_args.asset,
                    vec![],
                ),
            )
            .gas_limit(dry_run.exec_result.gas_required * 2)
            .submit()
            .await
            .expect("release failed");

        let balance_of_vest_receiver = client
            .call(
                &ink_e2e::alice(),
                &psp22.balance_of(create_vest_args.vest_to),
            )
            .dry_run()
            .await?
            .return_value();

        let contract_emitted_events = release_res.contract_emitted_events()?;
        let vester_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == vester.to_account_id())
            .collect();
        let psp22_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == psp22.to_account_id())
            .collect();

        let return_value = release_res.return_value();
        assert!(
            return_value.is_ok(),
            "release failed. res: {:?}",
            return_value
        );

        assert_psp22_transfer_event(
            &psp22_events[0].event,
            vester.to_account_id(),
            keypair_to_account(&vest_to),
            create_vest_args.amount,
            psp22.to_account_id(),
        );
        assert_token_released_event_e2e(
            &vester_events[0].event,
            create_vest_args.vest_to,
            create_vest_args.vest_to,
            Some(psp22.to_account_id()),
            create_vest_args.amount,
        );

        assert_eq!(
            balance_of_vester,
            balance_of_vester_pre - create_vest_args.amount
        );
        assert_eq!(
            balance_of_vest_receiver,
            balance_of_vest_receiver_pre + create_vest_args.amount
        );

        Ok(())
    }

    #[ink_e2e::test]
    async fn release_psp22_different_caller(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        let vester_creator = ink_e2e::alice();
        let vester_submitter = client
            .create_and_fund_account(&ink_e2e::alice(), 10_000_000_000_000)
            .await;
        let release_caller = client
            .create_and_fund_account(&ink_e2e::alice(), 10_000_000_000_000)
            .await;
        let psp22_mintable_creator = ink_e2e::bob();
        let mut psp22_constructor = PSP22Ref::new(1_000_000);
        let mut psp22 = client
            .instantiate(
                "my_psp22_mintable",
                &psp22_mintable_creator,
                &mut psp22_constructor,
            )
            .submit()
            .await
            .expect("instantiate psp22 failed")
            .call::<Contract>();
        let vest_to = ink_e2e::charlie();

        // create_vest args
        let create_vest_args = CreateVestingScheduleArgs {
            vest_to: keypair_to_account(&ink_e2e::charlie()),
            asset: Some(psp22.to_account_id()),
            amount: 100,
            schedule: VestingSchedule::Constant(1, 2),
        };

        let mut vester_constructor = VesterRef::new();
        let mut vester = client
            .instantiate("vester", &vester_creator, &mut vester_constructor)
            .submit()
            .await
            .expect("instantiate vester failed")
            .call::<Vester>();

        let _ = client
            .call(
                &vester_submitter,
                &psp22.increase_allowance(vester.to_account_id(), create_vest_args.amount),
            )
            .submit()
            .await
            .expect("give allowance failed")
            .return_value();

        let _ = client
            .call(
                &vester_creator,
                &psp22.mint(
                    keypair_to_account(&vester_submitter),
                    create_vest_args.amount,
                ),
            )
            .submit()
            .await
            .expect("mint failed");

        let create_vest_res = client
            .call(
                &vester_submitter,
                &vester.create_vest(
                    create_vest_args.vest_to,
                    create_vest_args.asset,
                    create_vest_args.amount,
                    create_vest_args.schedule.clone(),
                    vec![],
                ),
            )
            .submit()
            .await
            .expect("create vest failed");

        let contract_emitted_events = create_vest_res.contract_emitted_events()?;
        let vester_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == vester.to_account_id())
            .collect();
        let psp22_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == psp22.to_account_id())
            .collect();
        assert!(matches!(create_vest_res.return_value(), Ok(())));

        /*
        psp22 transfer_from emits 2 events (Approval - decrease allowance of vester_submitter & Transfer)
        here we check for the actual Transfer event, hence pulling event of index 1
        */
        assert_psp22_transfer_event(
            &psp22_events[1].event,
            keypair_to_account(&vester_submitter),
            vester.to_account_id(),
            create_vest_args.amount,
            psp22.to_account_id(),
        );
        assert_vesting_scheduled_event(
            &vester_events[0].event,
            keypair_to_account(&vester_submitter),
            create_vest_args.vest_to,
            Some(psp22.to_account_id()),
            create_vest_args.amount,
            create_vest_args.schedule,
        );

        let balance_of_vester_pre = client
            .call(&ink_e2e::alice(), &psp22.balance_of(vester.to_account_id()))
            .dry_run()
            .await?
            .return_value();
        let balance_of_vest_receiver_pre = client
            .call(
                &ink_e2e::alice(),
                &psp22.balance_of(create_vest_args.vest_to),
            )
            .dry_run()
            .await?
            .return_value();

        let release_res = client
            .call(
                &release_caller,
                &vester.release(
                    Some(create_vest_args.vest_to),
                    create_vest_args.asset,
                    vec![],
                ),
            )
            .dry_run()
            .await
            .expect("release failed");

        let release_res = client
            .call(
                &release_caller,
                &vester.release(
                    Some(create_vest_args.vest_to),
                    create_vest_args.asset,
                    vec![],
                ),
            )
            .gas_limit(release_res.exec_result.gas_required * 2)
            .submit()
            .await
            .expect("release failed");

        let balance_of_vester = client
            .call(&ink_e2e::alice(), &psp22.balance_of(vester.to_account_id()))
            .dry_run()
            .await?
            .return_value();
        let balance_of_vest_receiver = client
            .call(
                &ink_e2e::alice(),
                &psp22.balance_of(create_vest_args.vest_to),
            )
            .dry_run()
            .await?
            .return_value();

        let contract_emitted_events = release_res.contract_emitted_events()?;
        let vester_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == vester.to_account_id())
            .collect();
        let psp22_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == psp22.to_account_id())
            .collect();

        let return_value = release_res.return_value();
        assert!(
            return_value.is_ok(),
            "release failed. res: {:?}",
            return_value
        );

        assert_psp22_transfer_event(
            &psp22_events[0].event,
            vester.to_account_id(),
            keypair_to_account(&vest_to),
            create_vest_args.amount,
            psp22.to_account_id(),
        );
        assert_token_released_event_e2e(
            &vester_events[0].event,
            keypair_to_account(&release_caller),
            create_vest_args.vest_to,
            Some(psp22.to_account_id()),
            create_vest_args.amount,
        );

        assert_eq!(
            balance_of_vester,
            balance_of_vester_pre - create_vest_args.amount
        );
        assert_eq!(
            balance_of_vest_receiver,
            balance_of_vest_receiver_pre + create_vest_args.amount
        );

        Ok(())
    }

    #[ink_e2e::test]
    async fn create_vesting_schedule_native(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        let vester_creator = ink_e2e::alice();
        let vester_submitter = client
            .create_and_fund_account(&ink_e2e::alice(), 10_000_000_000_000)
            .await;

        // create_vest args
        let create_vest_args = CreateVestingScheduleArgs {
            vest_to: keypair_to_account(&ink_e2e::charlie()),
            asset: None,
            amount: 10_000_000,
            schedule: VestingSchedule::Constant(1, 2),
        };

        let mut vester_constructor = VesterRef::new();
        let mut vester = client
            .instantiate("vester", &vester_creator, &mut vester_constructor)
            .submit()
            .await
            .expect("instantiate vester failed")
            .call::<Vester>();

        let balance_of_vester_before = client
            .free_balance(vester.to_account_id())
            .await
            .expect("free balance failed");
        let balance_of_vester_submitter_before = client
            .free_balance(keypair_to_account(&vester_submitter))
            .await
            .expect("free balance failed");

        let create_vest_res = client
            .call(
                &vester_submitter,
                &vester.create_vest(
                    create_vest_args.vest_to,
                    create_vest_args.asset,
                    create_vest_args.amount,
                    create_vest_args.schedule.clone(),
                    vec![],
                ),
            )
            .value(create_vest_args.amount - 1)
            .dry_run()
            .await
            .expect("create vest failed")
            .return_value();
        assert_eq!(create_vest_res, Err(VestingError::InvalidAmountPaid));

        let create_vest_res = client
            .call(
                &vester_submitter,
                &vester.create_vest(
                    create_vest_args.vest_to,
                    create_vest_args.asset,
                    create_vest_args.amount,
                    create_vest_args.schedule.clone(),
                    vec![],
                ),
            )
            .value(create_vest_args.amount + 1)
            .dry_run()
            .await
            .expect("create vest failed")
            .return_value();
        assert_eq!(create_vest_res, Err(VestingError::InvalidAmountPaid));

        let create_vest_res = client
            .call(
                &vester_submitter,
                &vester.create_vest(
                    create_vest_args.vest_to,
                    create_vest_args.asset,
                    create_vest_args.amount,
                    create_vest_args.schedule.clone(),
                    vec![],
                ),
            )
            .value(create_vest_args.amount)
            .submit()
            .await
            .expect("create vest failed");

        let contract_emitted_events = create_vest_res.contract_emitted_events()?;
        let vester_events: Vec<_> = contract_emitted_events
            .iter()
            .filter(|event_with_topics| event_with_topics.event.contract == vester.to_account_id())
            .collect();
        let pallet_balances_events: Vec<_> = create_vest_res
            .events
            .iter()
            .filter(|event| {
                let metadata = &event.as_ref().expect("expected event").event_metadata();
                return metadata.pallet.name() == "Balances" && metadata.variant.name == "Transfer";
            })
            .map(|e| e.unwrap())
            .collect();
        assert!(matches!(create_vest_res.return_value(), Ok(())));

        assert_eq!(pallet_balances_events.len(), 1);
        assert_vesting_scheduled_event(
            &vester_events[0].event,
            keypair_to_account(&vester_submitter),
            create_vest_args.vest_to,
            None,
            create_vest_args.amount,
            create_vest_args.schedule,
        );

        let balance_of_vester_after = client
            .free_balance(vester.to_account_id())
            .await
            .expect("free balance failed");
        let balance_of_vester_submitter_after = client
            .free_balance(keypair_to_account(&vester_submitter))
            .await
            .expect("free balance failed");

        assert_eq!(
            balance_of_vester_after,
            balance_of_vester_before + create_vest_args.amount
        );
        assert_lt(
            balance_of_vester_submitter_after,
            balance_of_vester_submitter_before - create_vest_args.amount,
        );
        Ok(())
    }

    #[ink::test]
    fn release_works() {
        let accounts = accounts();
        let vest_to = accounts.charlie;
        let vester_submitter = accounts.bob;
        let mut vester = Vester::new();

        change_caller(vester_submitter);
        let creation_timestamp = vester.env().block_timestamp();

        let waiting_duration = ONE_DAY * 3;
        let vesting_duration = ONE_DAY * 6;

        let vesting_start = creation_timestamp + waiting_duration;
        let vesting_end = vesting_start + vesting_duration;
        let create_vest_args = create_duration_as_amount_schedule_args(
            vest_to,
            None,
            waiting_duration,
            vesting_duration,
        );
        let starting_balance = 100_000;
        set_account_balance(vest_to, starting_balance);
        set_account_balance(
            vester.env().account_id(),
            create_vest_args.amount + starting_balance,
        );
        let vest_to_balance_pre = get_account_balance(vest_to);
        let vester_balance_pre = get_account_balance(vester.env().account_id());

        set_value_transferred(create_vest_args.amount);
        let res = GeneralVest::create_vest(
            &mut vester,
            create_vest_args.vest_to,
            create_vest_args.asset,
            create_vest_args.amount,
            create_vest_args.schedule.clone(),
            vec![],
        );
        set_block_timestamp(vesting_start - 1);
        assert!(res.is_ok(), "release failed. res: {:?}", res);
        //try release succeeds & does not release anything
        change_caller(create_vest_args.vest_to);
        let res = GeneralVest::release(
            &mut vester,
            Some(create_vest_args.vest_to),
            create_vest_args.asset,
            vec![],
        );
        assert!(res.is_ok(), "release failed. res: {:?}", res);
        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_token_released_event(
            &emitted_events[emitted_events.len() - 1],
            create_vest_args.vest_to,
            create_vest_args.vest_to,
            create_vest_args.asset,
            0,
        );
        // try release succeeds & does release adequate amount of tokens eq 1
        set_block_timestamp(vesting_start + 2);
        let res = GeneralVest::release(
            &mut vester,
            Some(create_vest_args.vest_to),
            create_vest_args.asset,
            vec![],
        );
        assert!(res.is_ok(), "release failed. res: {:?}", res);
        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_token_released_event(
            &emitted_events[emitted_events.len() - 1],
            create_vest_args.vest_to,
            create_vest_args.vest_to,
            create_vest_args.asset,
            1, // accounting for rounding down
        );
        //verify storage
        let vesting_data =
            vester.vesting_schedule_of(create_vest_args.vest_to, create_vest_args.asset, 0, vec![]);
        assert!(vesting_data.is_some());
        let vesting_data = vesting_data.unwrap();
        assert_eq!(vesting_data.released, 1);
        assert_eq!(vesting_data.amount, create_vest_args.amount);
        assert_eq!(
            vesting_data.schedule,
            VestingSchedule::Constant(waiting_duration, vesting_duration)
        );

        // try release succeeds & does release adequate amount of tokens
        set_block_timestamp(vesting_start + ONE_DAY);
        let res = GeneralVest::release(&mut vester, Some(vest_to), create_vest_args.asset, vec![]);
        assert!(res.is_ok(), "release failed. res: {:?}", res);
        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_token_released_event(
            &emitted_events[emitted_events.len() - 1],
            create_vest_args.vest_to,
            create_vest_args.vest_to,
            create_vest_args.asset,
            (ONE_DAY - 1 - 1).into(), //1 already released + accounting for rounding down
        );
        //verify storage
        let vesting_data =
            vester.vesting_schedule_of(create_vest_args.vest_to, create_vest_args.asset, 0, vec![]);
        assert!(vesting_data.is_some());
        let vesting_data = vesting_data.unwrap();
        assert_eq!(vesting_data.released, (ONE_DAY - 1).into());
        assert_eq!(vesting_data.amount, create_vest_args.amount);
        assert_eq!(
            vesting_data.schedule,
            VestingSchedule::Constant(waiting_duration, vesting_duration)
        );

        // try release succeeds & does release the rest of tokens
        // use django as caller
        change_caller(accounts.django);
        // get django balance
        let django_balance_pre = get_account_balance(accounts.django);
        set_block_timestamp(vesting_end + 1);
        let res = GeneralVest::release(&mut vester, Some(vest_to), create_vest_args.asset, vec![]);
        assert!(res.is_ok(), "release failed. res: {:?}", res);
        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_token_released_event(
            &emitted_events[emitted_events.len() - 1],
            accounts.django,
            create_vest_args.vest_to,
            create_vest_args.asset,
            create_vest_args.amount - u128::from(ONE_DAY - 1), // ONE_DAY + 1 already released
        );
        let next_id =
            vester.next_id_vest_of(create_vest_args.vest_to, create_vest_args.asset, vec![]);
        assert_eq!(next_id, 0);
        let vesting_data =
            vester.vesting_schedule_of(create_vest_args.vest_to, create_vest_args.asset, 0, vec![]);
        assert!(vesting_data.is_none());

        let vest_to_balance_post = get_account_balance(vest_to);
        let vester_balance_post = get_account_balance(vester.env().account_id());
        assert_eq!(
            vest_to_balance_post,
            vest_to_balance_pre + create_vest_args.amount
        );
        assert_eq!(
            vester_balance_post,
            vester_balance_pre - create_vest_args.amount
        );
        //ensure django balance has not decreased
        assert_eq!(get_account_balance(accounts.django), django_balance_pre);
    }

    #[ink::test]
    fn release_when_multiple_schedules_created() {
        let accounts = accounts();
        let vest_to = accounts.charlie;
        let vester_submitter = accounts.bob;

        let mut vester = Vester::new();

        change_caller(vester_submitter);
        let creation_timestamp = vester.env().block_timestamp();
        let first_action_timestamp = creation_timestamp + ONE_DAY * 365;
        let create_vest_args_vec: Vec<CreateVestingScheduleArgs> = vec![
            create_duration_as_amount_schedule_args(
                vest_to,
                None,
                (first_action_timestamp - ONE_DAY * 9) - creation_timestamp,
                ONE_DAY * 6,
            ), //overdue (at the first_action_timestamp)
            create_duration_as_amount_schedule_args(
                vest_to,
                None,
                (first_action_timestamp - ONE_DAY * 6) - creation_timestamp,
                ONE_DAY * 9,
            ), //started (at the first_action_timestamp)
            create_duration_as_amount_schedule_args(
                vest_to,
                None,
                (first_action_timestamp + ONE_DAY * 1) - creation_timestamp,
                ONE_DAY * 5,
            ), //not started (at the first_action_timestamp)
            create_duration_as_amount_schedule_args(
                vest_to,
                None,
                (first_action_timestamp + ONE_DAY * 3) - creation_timestamp,
                ONE_DAY * 6,
            ), //not started (at the first_action_timestamp)
            create_duration_as_amount_schedule_args(
                vest_to,
                None,
                (first_action_timestamp + ONE_DAY * 18) - creation_timestamp,
                ONE_DAY * 46,
            ), //not started (at the first_action_timestamp)
        ];
        let starting_balance = 100_000;
        set_account_balance(vest_to, starting_balance);

        for create_vest_args in create_vest_args_vec.iter() {
            let vester_balance = get_account_balance(vester.env().account_id());
            set_account_balance(
                vester.env().account_id(),
                create_vest_args.amount + vester_balance,
            );
            set_value_transferred(create_vest_args.amount);
            let res = GeneralVest::create_vest(
                &mut vester,
                create_vest_args.vest_to,
                create_vest_args.asset,
                create_vest_args.amount,
                create_vest_args.schedule.clone(),
                vec![],
            );
            assert!(res.is_ok(), "release failed. res: {:?}", res);
        }
        let vest_to_balance_pre = get_account_balance(vest_to);
        let vester_balance_pre = get_account_balance(vester.env().account_id());

        set_block_timestamp(first_action_timestamp);
        // pre action
        let event_count_before = ink::env::test::recorded_events().collect::<Vec<_>>().len();
        assert_eq!(vester.next_id_vest_of(vest_to, None, vec![]), 5);
        for i in 0..3 {
            assert!(vester
                .vesting_schedule_of(vest_to, None, i.try_into().unwrap(), vec![])
                .is_some());
        }
        change_caller(vest_to);
        let res = GeneralVest::release(&mut vester, Some(vest_to), None, vec![]);
        assert!(res.is_ok(), "release failed. res: {:?}", res);
        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_token_released_event(
            &emitted_events[emitted_events.len() - 1],
            vest_to,
            vest_to,
            None,
            (ONE_DAY * 12 - 1).into(),
        );
        assert_eq!(emitted_events.len() - event_count_before, 1);
        assert_eq!(vester.next_id_vest_of(vest_to, None, vec![]), 4);
        assert!(vester
            .vesting_schedule_of(vest_to, None, 4, vec![])
            .is_none());
        for i in 0..3 {
            assert!(vester
                .vesting_schedule_of(vest_to, None, i.try_into().unwrap(), vec![])
                .is_some());
        }

        // move time past 2nd schedule end
        if let VestingSchedule::Constant(waiting_duration, vesting_duration) =
            create_vest_args_vec[1].schedule
        {
            let vesting_end = creation_timestamp + waiting_duration + vesting_duration;
            set_block_timestamp(vesting_end + ONE_DAY);
        } else {
            panic!("variant expected to be default")
        }
        let event_count_before = ink::env::test::recorded_events().collect::<Vec<_>>().len();
        let res = GeneralVest::release(&mut vester, Some(vest_to), None, vec![]);
        assert!(res.is_ok(), "release failed. res: {:?}", res);
        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_token_released_event(
            &emitted_events[emitted_events.len() - 1],
            vest_to,
            vest_to,
            None,
            (ONE_DAY * 7 - 1).into(),
        );
        assert_eq!(emitted_events.len() - event_count_before, 1);
        assert_eq!(vester.next_id_vest_of(vest_to, None, vec![]), 3); // 1st & 2nd schedule removed
        assert!(vester
            .vesting_schedule_of(vest_to, None, 3, vec![])
            .is_none());
        for i in 0..2 {
            assert!(vester
                .vesting_schedule_of(vest_to, None, i.try_into().unwrap(), vec![])
                .is_some());
        }

        // move time past last schedule end
        if let VestingSchedule::Constant(waiting_duration, vesting_duration) =
            create_vest_args_vec[create_vest_args_vec.len() - 1].schedule
        {
            let vesting_end = creation_timestamp + waiting_duration + vesting_duration;
            set_block_timestamp(vesting_end + 1);
        } else {
            panic!("variant expected to be default")
        }
        let res = GeneralVest::release(&mut vester, Some(vest_to), None, vec![]);
        assert!(res.is_ok(), "release failed. res: {:?}", res);
        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();
        assert_token_released_event(
            &emitted_events[emitted_events.len() - 1],
            vest_to,
            vest_to,
            None,
            (ONE_DAY * 53 + 1 + 1).into(),
        );
        let next_id = vester.next_id_vest_of(vest_to, None, vec![]);
        assert_eq!(next_id, 0);
        for i in 0..5 {
            assert!(vester
                .vesting_schedule_of(vest_to, None, i.try_into().unwrap(), vec![])
                .is_none());
        }

        let vest_to_balance_post = get_account_balance(vest_to);
        let vester_balance_post = get_account_balance(vester.env().account_id());
        let total_amount = create_vest_args_vec.iter().fold(0, |acc, x| acc + x.amount);
        assert_eq!(vest_to_balance_post, vest_to_balance_pre + total_amount);
        assert_eq!(vester_balance_post, vester_balance_pre - total_amount);
    }
}
