// SPDX-License-Identifier: MIT

pub use subxt_signer::sr25519::Keypair;

#[macro_export]
macro_rules! balance_of {
    ($client:ident, $contract:ident, $account:ident) => {{
        $client
            .call(
                &ink_e2e::alice(),
                &$contract.balance_of(ink_e2e::account_id($account)),
            )
            .dry_run()
            .await?
            .return_value()
    }};
}

#[macro_export]
macro_rules! balance_of2 {
    ($client:ident, $contract:ident, $account:expr) => {{
        $client
            .call(&ink_e2e::alice(), &$contract.balance_of($account))
            .dry_run()
            .await?
            .return_value()
    }};
}

#[macro_export]
macro_rules! owner_of {
    ($client:ident, $contract:ident, $id:expr) => {{
        $client
            .call(
                &ink_e2e::alice(),
                &$contract.owner_of(pendzl::contracts::token::psp34::Id::U8($id)),
            )
            .dry_run()
            .await
            .expect("owner of dry failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! balance_of_37 {
    ($client:ident, $contract:ident, $account:ident, $token:expr) => {{
        let _msg = build_message::<ContractRef>($contract.clone())
            .call(|contract| contract.balance_of(ink_e2e::account_id($account), $token));
        $client
            .call_dry_run(&ink_e2e::alice(), &_msg)
            .await
            .return_value()
    }};
}

#[macro_export]
macro_rules! has_role {
    ($client:ident, $contract:ident, $role:expr, $account:ident) => {{
        $client
            .call(
                &ink_e2e::alice(),
                &$contract.has_role($role, Some(ink_e2e::account_id($account))),
            )
            .dry_run()
            .await
            .expect("has_role dry failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! grant_role {
    ($client:ident, $contract:ident, $role:expr, $account:ident) => {{
        $client
            .call(
                &ink_e2e::alice(),
                &mut $contract.grant_role($role, Some(ink_e2e::account_id($account))),
            )
            .submit()
            .await
            .expect("grant_role failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! revoke_role {
    ($client:ident, $contract:ident, $role:expr, $account:ident) => {{
        $client
            .call(
                &ink_e2e::alice(),
                &$contract.revoke_role($role, Some(ink_e2e::account_id($account))),
            )
            .submit()
            .await
            .expect("revoke_role_failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! mint_dry_run {
    ($client:ident, $contract:ident, $signer:ident, $account:ident, $amount:ident) => {{
        $client
            .call(
                &ink_e2e::$signer(),
                contract.mint(ink_e2e::account_id($account), $amount),
            )
            .dry_run()
            .await
            .expect("mint_dry_run failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! mint {
    ($client:ident, $contract:ident, $signer:ident, $account:ident, $amount:expr) => {{
        $client
            .call(
                &ink_e2e::$signer(),
                &$contract.mint(ink_e2e::account_id($account), $amount),
            )
            .submit()
            .await
            .expect("mint failed")
            .return_value()
    }};

    ($client:ident, $contract:ident, $account:ident, $amount:expr) => {{
        $client
            .call(
                &ink_e2e::alice(),
                &$contract.mint(ink_e2e::account_id($account), $amount),
            )
            .submit()
            .await
            .expect("mint failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! mint2 {
    ($client:ident, $contract:ident, $signer:ident, $account:ident, $amount:ident) => {{
        $client
            .call(&ink_e2e::$signer(), &$contract.mint($account, $amount))
            .submit()
            .await
            .expect("mint failed")
            .return_value()
    }};

    ($client:ident, $contract:ident, $account:ident, $amount:ident) => {{
        $client
            .call(&ink_e2e::alice(), &$contract.mint($account, $amount))
            .submit()
            .await
            .expect("mint failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! approve {
    ($client:ident, $contract:ident, $signer:ident, $account:ident, $amount:expr) => {{
        $client
            .call(&ink_e2e::$signer(), &$contract.approve($account, $amount))
            .submit()
            .await
            .expect("approve failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! get_role_member_count {
    ($client:ident, $contract:ident, $role:expr) => {{
        $client
            .call(&ink_e2e::alice(), &contract.get_role_member_count($role))
            .dry_run()
            .await
            .expect("get_role_member_count failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! get_role_member {
    ($client:ident, $contract:ident, $role:expr, $index:expr) => {{
        $client
            .call(&ink_e2e::alice(), &contract.get_role_member($role, $index))
            .dry_run()
            .await
            .expect("get_role_member failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! get_shares {
    ($client:ident, $contract:ident, $user:ident) => {{
        $client
            .call(
                &ink_e2e::alice(),
                &contract.shares(ink_e2e::account_id($user)),
            )
            .dry_run()
            .await
            .expect("get_shares failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! method_call {
    ($client:ident, $contract:ident, $method:ident) => {{
        $client
            .call(&ink_e2e::alice(), &$contract.$method() )
            .submit()
            .await
            .expect("method_call failed")
            .return_value()
    }};
    ($client:ident, $contract:ident, $signer:ident, $method:ident) => {{
        $client
            .call(&ink_e2e::$signer(), &$contract.$method() )
            .submit()
            .await
            .expect("method_call failed")
            .return_value()
    }};
    ($client:ident, $contract:ident, $method:ident($($args:expr),*)) => {{
        $client
            .call(&ink_e2e::alice(), &$contract.$method($($args),*))
            .submit()
            .await
            .expect("method_call failed")
            .return_value()
    }};
    ($client:ident, $contract:ident, $signer:ident, $method:ident($($args:expr),*)) => {{
        $client
            .call(&ink_e2e::$signer(), &$contract.$method($($args),*))
            .submit()
            .await
            .expect("method_call failed")
            .return_value()
    }};
}

#[macro_export]
macro_rules! method_call_dry_run {
    ($client:ident, $contract:ident, $method:ident) => {{
        $client
            .call(&ink_e2e::alice(), &$contract.$method())
            .dry_run()
            .await?
            .return_value()
    }};
    ($client:ident, $contract:ident, $method:ident($($args:expr),*)) => {{
        $client
            .call(&ink_e2e::alice(), &$contract.$method($($args),*))
            .dry_run()
            .await?
            .return_value()
    }};
    ($client:ident, $contract:ident, $signer:ident, $method:ident) => {{
        $client
            .call(&ink_e2e::$signer(), &$contract.$method() )
            .dry_run()
            .await?
            .return_value()
    }};
    ($client:ident, $contract:ident, $signer:ident, $method:ident($($args:expr),*)) => {{
        $client
            .call(&ink_e2e::$signer(), &$contract.$method($($args),*) )
            .dry_run()
            .await
            .return_value()
    }};
}

#[macro_export]
macro_rules! assert_eq_msg {
    ($msg:expr, $encountered:expr, $expected:expr) => {
        assert_eq!(
            $encountered, $expected,
            "{} were not equal: encountered {:?}, expected {:?}",
            $msg, $encountered, $expected
        );
    };
}

pub fn keypair_to_account<AccountId: From<[u8; 32]>>(keypair: &Keypair) -> AccountId {
    AccountId::from(keypair.public_key().0)
}

pub fn assert_gt<T: PartialOrd>(a: T, b: T) {
    assert!(a > b);
}

pub fn assert_gte<T: PartialOrd>(a: T, b: T) {
    assert!(a >= b);
}

pub fn assert_lt<T: PartialOrd>(a: T, b: T) {
    assert!(a < b);
}

pub fn assert_lte<T: PartialOrd>(a: T, b: T) {
    assert!(a <= b);
}

pub fn run_if_test_debug<F: FnOnce()>(func: F) {
    match std::env::var("TEST_DEBUG").is_ok() {
        true => func(),
        false => (),
    }
}
