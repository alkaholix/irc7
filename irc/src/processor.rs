//! Server processing loop
//!
//! The processor handles the main game loop for the IRC server:
//! - Processing incoming commands from users
//! - Dispatching commands to handlers
//! - Managing user registration
//! - Handling timeouts and keep-alive pings

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::RwLock;
use tokio::time::interval;

use crate::constants::raws;
use crate::io::DataRegulator;
use crate::message::ChatFrameImpl;
use crate::traits::{
    ChatFrame, ChatMessage, Command, DataRegulator as DataRegulatorTrait,
    Server, User,
};

/// Server processor configuration
#[derive(Debug, Clone)]
pub struct ProcessorConfig {
    /// Tick rate in milliseconds
    pub tick_rate_ms: u64,
    /// Maximum backoff time in milliseconds
    pub max_backoff_ms: u64,
    /// Backoff increment in milliseconds
    pub backoff_increment_ms: u64,
}

impl Default for ProcessorConfig {
    fn default() -> Self {
        Self {
            tick_rate_ms: 10,
            max_backoff_ms: 1000,
            backoff_increment_ms: 10,
        }
    }
}

/// Server processor
pub struct ServerProcessor {
    config: ProcessorConfig,
    running: Arc<RwLock<bool>>,
}

impl ServerProcessor {
    /// Create a new server processor
    pub fn new(config: ProcessorConfig) -> Self {
        Self {
            config,
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Create with default config
    pub fn with_defaults() -> Self {
        Self::new(ProcessorConfig::default())
    }

    /// Start the processing loop
    pub async fn start(
        &self,
        server: Arc<dyn Server>,
        shutdown_signal: tokio::sync::broadcast::Receiver<()>,
    ) {
        *self.running.write().await = true;
        self.process_loop(server, shutdown_signal).await;
    }

    /// Stop the processing loop
    pub async fn stop(&self) {
        *self.running.write().await = false;
    }

    /// Main processing loop
    async fn process_loop(
        &self,
        server: Arc<dyn Server>,
        mut shutdown_signal: tokio::sync::broadcast::Receiver<()>,
    ) {
        let mut backoff_ms = 0u64;
        let mut tick = interval(Duration::from_millis(self.config.tick_rate_ms));

        loop {
            tokio::select! {
                _ = tick.tick() => {
                    if !*self.running.read().await {
                        break;
                    }

                    let mut has_work = false;

                    // Process all users
                    for user in server.get_users() {
                        // Check if incoming threshold exceeded
                        if let Some(regulator) = self.get_user_regulator(&*user) {
                            if regulator.is_incoming_threshold_exceeded() {
                                // Disconnect user
                                continue;
                            }

                            // Process incoming messages
                            if regulator.incoming_queue_length() > 0 {
                                has_work = true;
                                backoff_ms = 0;

                                self.process_next_command(&server, &user, &regulator).await;
                            }

                            // Flush outgoing messages
                            self.flush_user(&user, &regulator).await;
                        }
                    }

                    // Backoff if no work
                    if !has_work {
                        if backoff_ms < self.config.max_backoff_ms {
                            backoff_ms += self.config.backoff_increment_ms;
                        }
                        tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                    }
                }
                _ = shutdown_signal.recv() => {
                    break;
                }
            }
        }
    }

    /// Get the data regulator for a user
    fn get_user_regulator(&self, _user: &dyn User) -> Option<Arc<DataRegulator>> {
        // This would normally come from the user's connection
        // For now, return None since we need to integrate with the User trait
        None
    }

    /// Process the next command for a user
    async fn process_next_command(
        &self,
        server: &Arc<dyn Server>,
        user: &Arc<dyn User>,
        regulator: &Arc<DataRegulator>,
    ) {
        // Peek at the message
        let message = match regulator.peek_incoming_message() {
            Some(msg) => msg,
            None => return,
        };

        // Get the user's protocol
        let protocol = user.get_protocol();

        // Look up the command
        let command = match protocol.get_command(message.command()) {
            Some(cmd) => cmd,
            None => {
                // Unknown command
                regulator.pop_incoming_message();
                user.send(&raws::err_unknowncommand(
                    &**server,
                    &**user,
                    message.command_name(),
                ));
                return;
            }
        };

        // Pop the message now that we're processing it
        regulator.pop_incoming_message();

        // Create chat frame
        let frame = ChatFrameImpl::new(
            0, // sequence_id would be tracked per user
            message,
            server.clone(),
            user.clone(),
        );

        // Check if command requires registration
        if command.registration_needed(&frame) && !user.is_registered() {
            user.send(&raws::err_notregistered(&**server, &**user));
            return;
        }

        // Validate parameters
        if !command.parameters_are_valid(&frame) {
            user.send(&raws::err_needmoreparams(
                &**server,
                &**user,
                frame.chat_message().command_name(),
            ));
            return;
        }

        // Execute the command
        command.execute(&frame).await;

        // Try registration if not registered
        if !user.is_registered() {
            self.try_register(server, user).await;
        }
    }

    /// Try to register a user
    async fn try_register(&self, _server: &Arc<dyn Server>, user: &Arc<dyn User>) {
        // User can register when they have:
        // - A nickname
        // - A username (from USER command)
        // - Passed authentication (if required)

        let addr = user.get_address();
        if !addr.nickname.is_empty() && !addr.user.is_empty() {
            // Basic registration requirements met
            // In a full implementation, would check authentication status too
        }
    }

    /// Flush outgoing messages for a user
    async fn flush_user(&self, user: &Arc<dyn User>, regulator: &Arc<DataRegulator>) {
        let mut messages = Vec::new();

        // Collect all outgoing messages
        while let Some(msg) = regulator.pop_outgoing() {
            messages.push(format!("{}\r\n", msg));
        }

        // Send them all
        if !messages.is_empty() {
            let combined = messages.join("");
            user.send(&combined);
        }
    }
}

/// Process a single command directly (for testing or simple cases)
pub async fn process_command(
    command: &dyn Command,
    frame: &dyn ChatFrame,
) {
    command.execute(frame).await;
}
