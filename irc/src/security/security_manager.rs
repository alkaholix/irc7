//! Security manager implementation

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::security::{AnonPackage, GateKeeperPackage, NtlmPackage};
use crate::traits::{SecurityManager as SecurityManagerTrait, SupportPackage};

/// Factory function type for creating support packages
type PackageFactory = Box<dyn Fn() -> Box<dyn SupportPackage> + Send + Sync>;

/// Security manager for managing authentication packages
pub struct SecurityManager {
    packages: RwLock<HashMap<String, PackageFactory>>,
}

impl Default for SecurityManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityManager {
    /// Create a new security manager with default packages
    pub fn new() -> Self {
        let manager = Self {
            packages: RwLock::new(HashMap::new()),
        };

        // Add default packages
        manager.packages.write().insert(
            "ANON".to_string(),
            Box::new(|| Box::new(AnonPackage::new())),
        );
        manager.packages.write().insert(
            "NTLM".to_string(),
            Box::new(|| Box::new(NtlmPackage::new())),
        );
        manager.packages.write().insert(
            "GateKeeper".to_string(),
            Box::new(|| Box::new(GateKeeperPackage::new())),
        );

        manager
    }

    /// Create an empty security manager
    pub fn empty() -> Self {
        Self {
            packages: RwLock::new(HashMap::new()),
        }
    }
}

impl SecurityManagerTrait for SecurityManager {
    fn get_packages(&self) -> Vec<String> {
        self.packages.read().keys().cloned().collect()
    }

    fn get_package(&self, name: &str) -> Option<Box<dyn SupportPackage>> {
        self.packages
            .read()
            .get(&name.to_uppercase())
            .map(|factory| factory())
    }

    fn add_package(
        &mut self,
        name: String,
        factory: Box<dyn Fn() -> Box<dyn SupportPackage> + Send + Sync>,
    ) {
        self.packages.write().insert(name.to_uppercase(), factory);
    }

    fn remove_package(&mut self, name: &str) {
        self.packages.write().remove(&name.to_uppercase());
    }

    fn default_package(&self) -> Box<dyn SupportPackage> {
        Box::new(AnonPackage::new())
    }
}
