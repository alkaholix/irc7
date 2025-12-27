//! User access control

use crate::access::AccessListImpl;

/// User access wrapper
pub struct UserAccess {
    access_list: AccessListImpl,
}

impl Default for UserAccess {
    fn default() -> Self {
        Self::new()
    }
}

impl UserAccess {
    /// Create new user access control
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
}
