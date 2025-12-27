//! Authentication support packages

use std::sync::Arc;

use async_trait::async_trait;
use parking_lot::RwLock;

use crate::enums::SupportPackageSequence;
use crate::security::UserCredentials;
use crate::traits::{Credentials, SupportPackage as SupportPackageTrait, User};

/// Anonymous authentication package
pub struct AnonPackage {
    sequence: RwLock<SupportPackageSequence>,
    credentials: RwLock<Option<Arc<dyn Credentials>>>,
}

impl Default for AnonPackage {
    fn default() -> Self {
        Self::new()
    }
}

impl AnonPackage {
    /// Create a new ANON package
    pub fn new() -> Self {
        Self {
            sequence: RwLock::new(SupportPackageSequence::Initial),
            credentials: RwLock::new(None),
        }
    }
}

#[async_trait]
impl SupportPackageTrait for AnonPackage {
    fn name(&self) -> &str {
        "ANON"
    }

    fn initialize(&mut self, _user: &dyn User) {
        *self.sequence.write() = SupportPackageSequence::InProgress;
    }

    async fn authenticate(&mut self, _data: &str) -> SupportPackageSequence {
        // Anonymous auth always succeeds
        *self.credentials.write() = Some(Arc::new(UserCredentials::anonymous()));
        *self.sequence.write() = SupportPackageSequence::Complete;
        SupportPackageSequence::Complete
    }

    fn sequence(&self) -> SupportPackageSequence {
        *self.sequence.read()
    }

    fn get_credentials(&self) -> Option<Arc<dyn Credentials>> {
        self.credentials.read().clone()
    }

    fn get_response(&self) -> Option<String> {
        None
    }

    fn is_complete(&self) -> bool {
        matches!(*self.sequence.read(), SupportPackageSequence::Complete)
    }

    fn is_failed(&self) -> bool {
        matches!(*self.sequence.read(), SupportPackageSequence::Failed)
    }
}

/// NTLM authentication package (stub)
pub struct NtlmPackage {
    sequence: RwLock<SupportPackageSequence>,
    credentials: RwLock<Option<Arc<dyn Credentials>>>,
    response: RwLock<Option<String>>,
}

impl Default for NtlmPackage {
    fn default() -> Self {
        Self::new()
    }
}

impl NtlmPackage {
    /// Create a new NTLM package
    pub fn new() -> Self {
        Self {
            sequence: RwLock::new(SupportPackageSequence::Initial),
            credentials: RwLock::new(None),
            response: RwLock::new(None),
        }
    }
}

#[async_trait]
impl SupportPackageTrait for NtlmPackage {
    fn name(&self) -> &str {
        "NTLM"
    }

    fn initialize(&mut self, _user: &dyn User) {
        *self.sequence.write() = SupportPackageSequence::InProgress;
    }

    async fn authenticate(&mut self, data: &str) -> SupportPackageSequence {
        // NTLM authentication would process Type1/Type2/Type3 messages
        // This is a stub implementation
        let sequence = self.sequence.read().clone();
        match sequence {
            SupportPackageSequence::Initial | SupportPackageSequence::InProgress => {
                // Would process NTLM messages here
                *self.response.write() = Some("NTLM challenge".to_string());
                SupportPackageSequence::InProgress
            }
            _ => sequence,
        }
    }

    fn sequence(&self) -> SupportPackageSequence {
        *self.sequence.read()
    }

    fn get_credentials(&self) -> Option<Arc<dyn Credentials>> {
        self.credentials.read().clone()
    }

    fn get_response(&self) -> Option<String> {
        self.response.read().clone()
    }

    fn is_complete(&self) -> bool {
        matches!(*self.sequence.read(), SupportPackageSequence::Complete)
    }

    fn is_failed(&self) -> bool {
        matches!(*self.sequence.read(), SupportPackageSequence::Failed)
    }
}

/// GateKeeper authentication package (stub)
pub struct GateKeeperPackage {
    sequence: RwLock<SupportPackageSequence>,
    credentials: RwLock<Option<Arc<dyn Credentials>>>,
}

impl Default for GateKeeperPackage {
    fn default() -> Self {
        Self::new()
    }
}

impl GateKeeperPackage {
    /// Create a new GateKeeper package
    pub fn new() -> Self {
        Self {
            sequence: RwLock::new(SupportPackageSequence::Initial),
            credentials: RwLock::new(None),
        }
    }
}

#[async_trait]
impl SupportPackageTrait for GateKeeperPackage {
    fn name(&self) -> &str {
        "GateKeeper"
    }

    fn initialize(&mut self, _user: &dyn User) {
        *self.sequence.write() = SupportPackageSequence::InProgress;
    }

    async fn authenticate(&mut self, _data: &str) -> SupportPackageSequence {
        // GateKeeper/Passport authentication would process tokens here
        *self.sequence.write() = SupportPackageSequence::Failed;
        SupportPackageSequence::Failed
    }

    fn sequence(&self) -> SupportPackageSequence {
        *self.sequence.read()
    }

    fn get_credentials(&self) -> Option<Arc<dyn Credentials>> {
        self.credentials.read().clone()
    }

    fn get_response(&self) -> Option<String> {
        None
    }

    fn is_complete(&self) -> bool {
        matches!(*self.sequence.read(), SupportPackageSequence::Complete)
    }

    fn is_failed(&self) -> bool {
        matches!(*self.sequence.read(), SupportPackageSequence::Failed)
    }
}
