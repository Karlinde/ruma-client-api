//! Endpoints for key management

use ruma_events::Algorithm;
use ruma_identifiers::{DeviceId, UserId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Error as FmtError, Formatter};

pub mod claim_keys;
pub mod get_key_changes;
pub mod get_keys;
pub mod upload_keys;

/// The basic key algorithms in the specification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyAlgorithm {
    /// The Ed25519 signature algorithm.
    #[serde(rename = "ed25519")]
    Ed25519,

    /// The Curve25519 ECDH algorithm.
    #[serde(rename = "curve25519")]
    Curve25519,

    /// The Curve25519 ECDH algorithm, but the key also contains signatures
    #[serde(rename = "signed_curve25519")]
    SignedCurve25519,
}

impl Display for KeyAlgorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        let algorithm_str = match *self {
            KeyAlgorithm::Ed25519 => "ed25519",
            KeyAlgorithm::Curve25519 => "curve25519",
            KeyAlgorithm::SignedCurve25519 => "signed_curve25519",
        };
        write!(f, "{}", algorithm_str)?;
        Ok(())
    }
}

impl<'a> TryFrom<&'a str> for KeyAlgorithm {
    type Error = &'static str;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        match s {
            "ed25519" => Ok(KeyAlgorithm::Ed25519),
            "curve25519" => Ok(KeyAlgorithm::Curve25519),
            "signed_curve25519" => Ok(KeyAlgorithm::SignedCurve25519),
            _ => Err("Unknown algorithm"),
        }
    }
}

/// A key algorithm and a device id, combined with a ':'
pub type AlgorithmAndDeviceId = String;

/// Combine a KeyAlgorithm and a DeviceId together with a ':'.
/// For use with the key management endpoints.
pub fn combine_algo_and_device_id(
    algorithm: KeyAlgorithm,
    device_id: &DeviceId,
) -> AlgorithmAndDeviceId {
    format!("{}:{}", algorithm, device_id)
}

/// Parse a KeyAlgorithm and a DeviceId from a string where they are combined with a ':'.
/// For use with the key management endpoints.
pub fn parse_algo_and_device_id(s: &str) -> Result<(KeyAlgorithm, DeviceId), &'static str> {
    let parts = s.split(':').collect::<Vec<_>>();

    if parts.len() != 2 {
        return Err("Invalid format");
    }

    let algorithm = KeyAlgorithm::try_from(parts[0])?;
    Ok((algorithm, parts[1].to_string()))
}

/// Identity keys for a device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceKeys {
    /// The ID of the user the device belongs to. Must match the user ID used when logging in.
    pub user_id: UserId,
    /// The ID of the device these keys belong to. Must match the device ID used when logging in.
    pub device_id: DeviceId,
    /// The encryption algorithms supported by this device.
    pub algorithms: Vec<Algorithm>,
    /// Public identity keys.
    pub keys: HashMap<AlgorithmAndDeviceId, String>,
    /// Signatures for the device key object.
    pub signatures: HashMap<UserId, HashMap<AlgorithmAndDeviceId, String>>,
    /// Additional data added to the device key information by intermediate servers, and
    /// not covered by the signatures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsigned: Option<UnsignedDeviceInfo>,
}

/// Additional data added to device key information by intermediate servers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsignedDeviceInfo {
    /// The display name which the user set on the device.
    device_display_name: String,
}

/// A key for the SignedCurve25519 algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedKey {
    /// Base64-encoded 32-byte Curve25519 public key.
    pub key: String,
    /// Signatures for the key object.
    pub signatures: HashMap<UserId, HashMap<AlgorithmAndDeviceId, String>>,
}

/// A one-time public key for "pre-key" messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneTimeKey {
    /// A key containing signatures, for the SignedCurve25519 algorithm.
    SignedKey(SignedKey),
    /// A string-valued key, for the Ed25519 and Curve25519 algorithms.
    Key(String),
}
