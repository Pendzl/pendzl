// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

pub use crate::{
    access_control,
    access_control::extensions::enumerable,
    traits::access_control::{
        extensions::enumerable::*,
        *,
    },
};
pub use access_control::{
    AccessControlImpl,
    Internal as _,
    InternalImpl as _,
};
use pendzl::{
    storage::{
        Mapping,
        MultiMapping,
        ValueGuard,
    },
    traits::{
        AccountId,
        Storage,
    },
};

#[derive(Default, Debug)]
#[pendzl::storage_item]
pub struct Data {
    pub admin_roles: Mapping<RoleType, RoleType, ValueGuard<RoleType>>,
    pub role_members: MultiMapping<RoleType, Option<AccountId>, ValueGuard<RoleType>>,
}

pub trait MembersManagerImpl: Storage<Data> {
    fn _has_role_impl(&self, role: RoleType, address: &Option<AccountId>) -> bool {
        self.data().role_members.contains_value(role, address)
    }

    fn _add_impl(&mut self, role: RoleType, member: &Option<AccountId>) {
        self.data().role_members.insert(role, member);
    }

    fn _remove_impl(&mut self, role: RoleType, member: &Option<AccountId>) {
        self.data().role_members.remove_value(role, member);
    }

    fn _get_role_admin_impl(&self, role: RoleType) -> Option<RoleType> {
        self.data().admin_roles.get(role)
    }

    fn _set_role_admin_impl(&mut self, role: RoleType, new_admin: RoleType) {
        self.data().admin_roles.insert(role, &new_admin);
    }
}

pub trait AccessControlEnumerableImpl: Storage<Data> {
    fn get_role_member_impl(&self, role: RoleType, index: u32) -> Option<AccountId> {
        self.data()
            .role_members
            .get_value(role, &(index as u128))
            .unwrap_or(None)
    }

    fn get_role_member_count_impl(&self, role: RoleType) -> u32 {
        self.data().role_members.count(role) as u32
    }
}
