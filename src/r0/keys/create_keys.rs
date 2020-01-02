//! [POST /_matrix/client/r0/keys/upload](https://matrix.org/docs/spec/client_server/r0.6.0#post-matrix-client-r0-keys-upload)

use super::{AlgorithmAndDeviceId, DeviceKeys, KeyAlgorithm, OneTimeKey};
use js_int::UInt;
use ruma_api::ruma_api;
use std::collections::HashMap;

ruma_api! {
    metadata {
        description: "Publishes end-to-end encryption keys for the device.",
        method: POST,
        name: "create_keys",
        path: "/_matrix/client/r0/keys/upload",
        rate_limited: false,
        requires_authentication: true,
    }

    request {
        /// Identity keys for the device. May be absent if no new identity keys are required.
        #[serde(skip_serializing_if = "Option::is_none")]
        device_keys: Option<DeviceKeys>,

        /// One-time public keys for "pre-key" messages.
        #[serde(skip_serializing_if = "Option::is_none")]
        one_time_keys: Option<HashMap<AlgorithmAndDeviceId, OneTimeKey>>,

    }

    response {
        /// For each key algorithm, the number of unclaimed one-time keys of that
        /// type currently held on the server for this device.
        one_time_key_counts: HashMap<KeyAlgorithm, UInt>
    }
}
