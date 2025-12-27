//! Credential implementations

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use parking_lot::RwLock;

use crate::enums::UserAccessLevel;
use crate::traits::{Credential as CredentialTrait, CredentialProvider as CredentialProviderTrait, Credentials};

/// Simple credential implementation
#[derive(Debug, Clone)]
pub struct SimpleCredential {
    username: String,
    domain: String,
    password_hash: String,
    level: UserAccessLevel,
}

impl SimpleCredential {
    /// Create a new credential
    pub fn new(username: String, domain: String, password_hash: String, level: UserAccessLevel) -> Self {
        Self {
            username,
            domain,
            password_hash,
            level,
        }
    }
}

impl CredentialTrait for SimpleCredential {
    fn username(&self) -> &str {
        &self.username
    }

    fn domain(&self) -> &str {
        &self.domain
    }

    fn password_hash(&self) -> &str {
        &self.password_hash
    }

    fn level(&self) -> UserAccessLevel {
        self.level
    }

    fn verify_password(&self, password: &str) -> bool {
        // Simple comparison - in production would use proper hashing
        self.password_hash == password
    }
}

/// User credentials after authentication
#[derive(Debug, Clone)]
pub struct UserCredentials {
    username: String,
    domain: String,
    user_id: Option<String>,
    puid: Option<String>,
    valid: bool,
}

impl UserCredentials {
    /// Create new credentials
    pub fn new(username: String, domain: String) -> Self {
        Self {
            username,
            domain,
            user_id: None,
            puid: None,
            valid: true,
        }
    }

    /// Create anonymous credentials
    pub fn anonymous() -> Self {
        Self {
            username: String::new(),
            domain: String::new(),
            user_id: None,
            puid: None,
            valid: true,
        }
    }
}

impl Credentials for UserCredentials {
    fn username(&self) -> &str {
        &self.username
    }

    fn domain(&self) -> &str {
        &self.domain
    }

    fn user_id(&self) -> Option<&str> {
        self.user_id.as_deref()
    }

    fn puid(&self) -> Option<&str> {
        self.puid.as_deref()
    }

    fn is_valid(&self) -> bool {
        self.valid
    }
}

/// In-memory credential provider
pub struct InMemoryCredentialProvider {
    credentials: RwLock<HashMap<String, Arc<dyn CredentialTrait>>>,
}

impl Default for InMemoryCredentialProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryCredentialProvider {
    /// Create a new provider
    pub fn new() -> Self {
        Self {
            credentials: RwLock::new(HashMap::new()),
        }
    }

    /// Add a credential synchronously
    pub fn add_sync(&self, username: &str, credential: Arc<dyn CredentialTrait>) {
        self.credentials.write().insert(username.to_lowercase(), credential);
    }
}

#[async_trait]
impl CredentialProviderTrait for InMemoryCredentialProvider {
    async fn get_credentials(&self, username: &str) -> Option<Arc<dyn CredentialTrait>> {
        self.credentials.read().get(&username.to_lowercase()).cloned()
    }

    async fn validate(&self, username: &str, password: &str) -> Option<Arc<dyn CredentialTrait>> {
        let cred = self.credentials.read().get(&username.to_lowercase()).cloned()?;
        if cred.verify_password(password) {
            Some(cred)
        } else {
            None
        }
    }

    async fn add(&mut self, username: String, credential: Arc<dyn CredentialTrait>) {
        self.credentials.write().insert(username.to_lowercase(), credential);
    }

    async fn remove(&mut self, username: &str) -> Option<Arc<dyn CredentialTrait>> {
        self.credentials.write().remove(&username.to_lowercase())
    }
}
