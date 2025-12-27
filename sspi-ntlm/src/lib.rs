//! SSPI NTLM Authentication
//!
//! This crate provides NTLM authentication support for IRC7.

use thiserror::Error;

/// NTLM errors
#[derive(Error, Debug)]
pub enum NtlmError {
    #[error("Invalid message type")]
    InvalidMessageType,
    #[error("Invalid message format")]
    InvalidFormat,
    #[error("Authentication failed")]
    AuthenticationFailed,
    #[error("Unsupported feature")]
    Unsupported,
}

/// NTLM flags
pub mod flags {
    pub const NEGOTIATE_UNICODE: u32 = 0x00000001;
    pub const NEGOTIATE_OEM: u32 = 0x00000002;
    pub const REQUEST_TARGET: u32 = 0x00000004;
    pub const NEGOTIATE_SIGN: u32 = 0x00000010;
    pub const NEGOTIATE_SEAL: u32 = 0x00000020;
    pub const NEGOTIATE_NTLM: u32 = 0x00000200;
    pub const NEGOTIATE_ALWAYS_SIGN: u32 = 0x00008000;
    pub const TARGET_TYPE_DOMAIN: u32 = 0x00010000;
    pub const TARGET_TYPE_SERVER: u32 = 0x00020000;
    pub const NEGOTIATE_NTLM2: u32 = 0x00080000;
    pub const NEGOTIATE_TARGET_INFO: u32 = 0x00800000;
    pub const NEGOTIATE_128: u32 = 0x20000000;
    pub const NEGOTIATE_56: u32 = 0x80000000;
}

/// NTLM Type 1 message (Negotiate)
#[derive(Debug, Clone)]
pub struct NtlmType1Message {
    pub flags: u32,
    pub domain: Option<String>,
    pub workstation: Option<String>,
}

impl NtlmType1Message {
    /// Parse a Type 1 message from bytes
    pub fn parse(data: &[u8]) -> Result<Self, NtlmError> {
        if data.len() < 16 {
            return Err(NtlmError::InvalidFormat);
        }

        // Verify signature
        if &data[0..8] != b"NTLMSSP\0" {
            return Err(NtlmError::InvalidFormat);
        }

        // Verify message type
        let msg_type = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);
        if msg_type != 1 {
            return Err(NtlmError::InvalidMessageType);
        }

        let flags = u32::from_le_bytes([data[12], data[13], data[14], data[15]]);

        Ok(Self {
            flags,
            domain: None,
            workstation: None,
        })
    }
}

/// NTLM Type 2 message (Challenge)
#[derive(Debug, Clone)]
pub struct NtlmType2Message {
    pub flags: u32,
    pub challenge: [u8; 8],
    pub target_name: Option<String>,
}

impl NtlmType2Message {
    /// Create a new Type 2 message
    pub fn new(flags: u32, challenge: [u8; 8]) -> Self {
        Self {
            flags,
            challenge,
            target_name: None,
        }
    }

    /// Encode the message to bytes
    pub fn encode(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(56);

        // Signature
        data.extend_from_slice(b"NTLMSSP\0");

        // Message type (2)
        data.extend_from_slice(&2u32.to_le_bytes());

        // Target name fields (empty for now)
        data.extend_from_slice(&0u16.to_le_bytes()); // Length
        data.extend_from_slice(&0u16.to_le_bytes()); // Max length
        data.extend_from_slice(&56u32.to_le_bytes()); // Offset

        // Flags
        data.extend_from_slice(&self.flags.to_le_bytes());

        // Challenge
        data.extend_from_slice(&self.challenge);

        // Reserved (8 bytes)
        data.extend_from_slice(&[0u8; 8]);

        // Target info fields (empty for now)
        data.extend_from_slice(&0u16.to_le_bytes());
        data.extend_from_slice(&0u16.to_le_bytes());
        data.extend_from_slice(&56u32.to_le_bytes());

        data
    }
}

/// NTLM Type 3 message (Authenticate)
#[derive(Debug, Clone)]
pub struct NtlmType3Message {
    pub flags: u32,
    pub lm_response: Vec<u8>,
    pub nt_response: Vec<u8>,
    pub domain: String,
    pub username: String,
    pub workstation: String,
}

impl NtlmType3Message {
    /// Parse a Type 3 message from bytes
    pub fn parse(data: &[u8]) -> Result<Self, NtlmError> {
        if data.len() < 64 {
            return Err(NtlmError::InvalidFormat);
        }

        // Verify signature
        if &data[0..8] != b"NTLMSSP\0" {
            return Err(NtlmError::InvalidFormat);
        }

        // Verify message type
        let msg_type = u32::from_le_bytes([data[8], data[9], data[10], data[11]]);
        if msg_type != 3 {
            return Err(NtlmError::InvalidMessageType);
        }

        let flags = u32::from_le_bytes([data[60], data[61], data[62], data[63]]);

        Ok(Self {
            flags,
            lm_response: Vec::new(),
            nt_response: Vec::new(),
            domain: String::new(),
            username: String::new(),
            workstation: String::new(),
        })
    }
}

/// NTLM authentication handler
pub struct NtlmAuth {
    challenge: [u8; 8],
}

impl NtlmAuth {
    /// Create a new NTLM authenticator
    pub fn new() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let mut challenge = [0u8; 8];
        for (i, byte) in challenge.iter_mut().enumerate() {
            *byte = ((seed >> (i * 8)) & 0xFF) as u8;
        }

        Self { challenge }
    }

    /// Process a Type 1 message and generate a Type 2 challenge
    pub fn process_type1(&self, _msg: &NtlmType1Message) -> NtlmType2Message {
        let flags = flags::NEGOTIATE_UNICODE
            | flags::NEGOTIATE_NTLM
            | flags::NEGOTIATE_ALWAYS_SIGN
            | flags::TARGET_TYPE_SERVER;

        NtlmType2Message::new(flags, self.challenge)
    }

    /// Process a Type 3 message and validate authentication
    pub fn process_type3(&self, _msg: &NtlmType3Message) -> Result<(), NtlmError> {
        // In a real implementation, would validate the response
        Err(NtlmError::Unsupported)
    }
}

impl Default for NtlmAuth {
    fn default() -> Self {
        Self::new()
    }
}
