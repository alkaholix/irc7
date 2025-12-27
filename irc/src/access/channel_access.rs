//! Channel access control

use crate::access::AccessListImpl;
use crate::enums::AccessLevel;
use crate::traits::AccessList;
use std::sync::Arc;

/// Channel access wrapper
pub struct ChannelAccess {
    access_list: AccessListImpl,
}

impl Default for ChannelAccess {
    fn default() -> Self {
        Self::new()
    }
}

impl ChannelAccess {
    /// Create new channel access control
    pub fn new() -> Self {
        Self {
            access_list: AccessListImpl::new(),
        }
    }

    /// Get the access list
    pub fn access_list(&self) -> &AccessListImpl {
        &self.access_list
    }

    /// Get mutable access list
    pub fn access_list_mut(&mut self) -> &mut AccessListImpl {
        &mut self.access_list
    }

    /// Check access level for an address
    pub fn check(&self, address: &str) -> AccessLevel {
        self.access_list.check(address)
    }
}
