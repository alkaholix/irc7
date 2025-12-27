//! Security trait definitions

use crate::enums::SupportPackageSequence;
use async_trait::async_trait;
use std::sync::Arc;

use super::User;

/// A support package for authentication
#[async_trait]
pub trait SupportPackage: Send + Sync {
    /// Get the package name
    fn name(&self) -> &str;

    /// Initialize the package for a user
    fn initialize(&mut self, user: &dyn User);

    /// Process authentication data
    async fn authenticate(&mut self, data: &str) -> SupportPackageSequence;

    /// Get the current sequence state
    fn sequence(&self) -> SupportPackageSequence;

    /// Get credentials after successful authentication
    fn get_credentials(&self) -> Option<Arc<dyn Credentials>>;

    /// Get the response to send to the client
    fn get_response(&self) -> Option<String>;

    /// Check if authentication is complete
    fn is_complete(&self) -> bool;

    /// Check if authentication failed
    fn is_failed(&self) -> bool;
}

/// User credentials
pub trait Credentials: Send + Sync {
    /// Get the username
    fn username(&self) -> &str;

    /// Get the domain
    fn domain(&self) -> &str;

    /// Get the user ID
    fn user_id(&self) -> Option<&str>;

    /// Get the PUID (Passport User ID)
    fn puid(&self) -> Option<&str>;

    /// Check if the credentials are valid
    fn is_valid(&self) -> bool;
}

/// A credential stored in the credential provider
pub trait Credential: Send + Sync {
    /// Get the username
    fn username(&self) -> &str;

    /// Get the domain
    fn domain(&self) -> &str;

    /// Get the password hash
    fn password_hash(&self) -> &str;

    /// Get the user level
    fn level(&self) -> crate::enums::UserAccessLevel;

    /// Verify a password
    fn verify_password(&self, password: &str) -> bool;
}

/// A credential provider for authentication
#[async_trait]
pub trait CredentialProvider: Send + Sync {
    /// Get credentials by username
    async fn get_credentials(&self, username: &str) -> Option<Arc<dyn Credential>>;

    /// Validate username and password
    async fn validate(&self, username: &str, password: &str) -> Option<Arc<dyn Credential>>;

    /// Add credentials
    async fn add(&mut self, username: String, credential: Arc<dyn Credential>);

    /// Remove credentials
    async fn remove(&mut self, username: &str) -> Option<Arc<dyn Credential>>;
}

/// Security manager for managing authentication packages
pub trait SecurityManager: Send + Sync {
    /// Get available packages
    fn get_packages(&self) -> Vec<String>;

    /// Get a package by name
    fn get_package(&self, name: &str) -> Option<Box<dyn SupportPackage>>;

    /// Add a package
    fn add_package(&mut self, name: String, factory: Box<dyn Fn() -> Box<dyn SupportPackage> + Send + Sync>);

    /// Remove a package
    fn remove_package(&mut self, name: &str);

    /// Get the default package
    fn default_package(&self) -> Box<dyn SupportPackage>;
}
