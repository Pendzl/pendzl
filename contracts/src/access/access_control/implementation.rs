// Copyright (c) 2024 C Forge. All Rights Reserved.
// SPDX-License-Identifier: MIT

pub use super::{
    AccessControl, AccessControlError, AccessControlInternal,
    AccessControlStorage, RoleAdminChanged, RoleGranted, RoleRevoked, RoleType,
    DEFAULT_ADMIN_ROLE,
};
use ink::{env::DefaultEnvironment, storage::Mapping};
use pendzl::traits::{AccountId, DefaultEnv, StorageFieldGetter};

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct AccessControlData {
    pub admin_roles: Mapping<RoleType, RoleType>,
    pub members: Mapping<(RoleType, Option<AccountId>), ()>,
}

impl AccessControlData {
    pub fn new(admin: Option<AccountId>) -> Self {
        let mut instance: AccessControlData = Default::default();
        if let Some(admin) = admin {
            instance.add(DEFAULT_ADMIN_ROLE, &Some(admin));
            ink::env::emit_event::<DefaultEnvironment, RoleGranted>(
                RoleGranted {
                    role: 0,
                    grantee: None,
                    grantor: Some(admin),
                },
            );
        }
        instance
    }
}

impl AccessControlStorage for AccessControlData {
    fn has_role(&self, role: RoleType, address: &Option<AccountId>) -> bool {
        self.members.contains(&(role, *address))
            || self.members.contains(&(role, None))
    }

    fn add(&mut self, role: RoleType, member: &Option<AccountId>) {
        self.members.insert(&(role, *member), &());
    }

    fn remove(&mut self, role: RoleType, member: &Option<AccountId>) {
        self.members.remove(&(role, *member));
    }

    fn get_role_admin(&self, role: RoleType) -> Option<RoleType> {
        self.admin_roles.get(role)
    }

    fn set_role_admin(&mut self, role: RoleType, new_admin: RoleType) {
        self.admin_roles.insert(role, &new_admin);
    }
}

pub trait AccessControlDefaultImpl: AccessControlInternal + Sized {
    fn has_role_default_impl(
        &self,
        role: RoleType,
        address: Option<AccountId>,
    ) -> bool {
        self._has_role(role, address)
    }

    fn get_role_admin_default_impl(&self, role: RoleType) -> RoleType {
        self._get_role_admin(role)
    }

    fn grant_role_default_impl(
        &mut self,
        role: RoleType,
        account: Option<AccountId>,
    ) -> Result<(), AccessControlError> {
        self._ensure_has_role(
            self._get_role_admin(role),
            Some(Self::env().caller()),
        )?;

        self._grant_role(role, account)?;

        Ok(())
    }

    fn revoke_role_default_impl(
        &mut self,
        role: RoleType,
        account: Option<AccountId>,
    ) -> Result<(), AccessControlError> {
        self._ensure_has_role(
            self._get_role_admin(role),
            Some(Self::env().caller()),
        )?;
        self._do_revoke_role(role, account)?;
        Ok(())
    }

    fn renounce_role_default_impl(
        &mut self,
        role: RoleType,
        account: Option<AccountId>,
    ) -> Result<(), AccessControlError> {
        if account != Some(Self::env().caller()) {
            return Err(AccessControlError::InvalidCaller);
        }
        self._do_revoke_role(role, account)?;
        Ok(())
    }

    fn set_role_admin_default_impl(
        &mut self,
        role: RoleType,
        new_admin: RoleType,
    ) -> Result<(), AccessControlError> {
        self._ensure_has_role(
            self._get_role_admin(role),
            Some(Self::env().caller()),
        )?;
        self._set_role_admin(role, new_admin);
        Ok(())
    }
}

pub trait AccessControlInternalDefaultImpl:
    StorageFieldGetter<AccessControlData>
where
    AccessControlData: AccessControlStorage,
{
    fn _default_admin_default_impl() -> RoleType {
        DEFAULT_ADMIN_ROLE
    }

    fn _has_role_default_impl(
        &self,
        role: RoleType,
        account: Option<AccountId>,
    ) -> bool {
        self.data().has_role(role, &account)
    }

    fn _grant_role_default_impl(
        &mut self,
        role: RoleType,
        grantee: Option<AccountId>,
    ) -> Result<(), AccessControlError> {
        if self.data().has_role(role, &grantee) {
            return Err(AccessControlError::RoleRedundant);
        }
        self.data().add(role, &grantee);
        let grantor = Self::env().caller();
        Self::env().emit_event(RoleGranted {
            role,
            grantee,
            grantor: Some(grantor),
        });
        Ok(())
    }

    fn _do_revoke_role_default_impl(
        &mut self,
        role: RoleType,
        account: Option<AccountId>,
    ) -> Result<(), AccessControlError> {
        self._ensure_has_role_default_impl(role, account)?;
        self.data().remove(role, &account);
        let sender = Self::env().caller();
        Self::env().emit_event(RoleRevoked {
            role,
            account,
            sender,
        });
        Ok(())
    }

    fn _set_role_admin_default_impl(&mut self, role: RoleType, new: RoleType) {
        let previous = self._get_role_admin_default_impl(role);
        if new != previous {
            self.data().set_role_admin(role, new);
            Self::env().emit_event(RoleAdminChanged {
                role,
                previous,
                new,
            })
        }
    }

    fn _ensure_has_role_default_impl(
        &self,
        role: RoleType,
        account: Option<AccountId>,
    ) -> Result<(), AccessControlError> {
        if !self.data().has_role(role, &account)
            && !self.data().has_role(role, &None)
        {
            return Err(AccessControlError::MissingRole);
        }
        Ok(())
    }

    fn _get_role_admin_default_impl(&self, role: RoleType) -> RoleType {
        self.data()
            .get_role_admin(role)
            .unwrap_or(Self::_default_admin_default_impl())
    }
}
