//! [POST /_matrix/client/r0/keys/claim](https://matrix.org/docs/spec/client_server/r0.6.0#post-matrix-client-r0-keys-claim)

use super::{AlgorithmAndDeviceId, KeyAlgorithm, OneTimeKey};
use js_int::UInt;
use ruma_api::ruma_api;
use ruma_identifiers::{DeviceId, UserId};
use serde_json::Value;
use std::collections::HashMap;

ruma_api! {
    metadata {
        description: "Claims one-time keys for use in pre-key messages.",
        method: POST,
        name: "claim_keys",
        path: "/_matrix/client/r0/keys/claim",
        rate_limited: false,
        requires_authentication: true,
    }

    request {
        /// The time (in milliseconds) to wait when downloading keys from remote servers.
        /// 10 seconds is the recommended default.
        #[serde(skip_serializing_if = "Option::is_none")]
        timeout: Option<UInt>,

        /// The keys to be claimed.
        one_time_keys: HashMap<UserId, HashMap<DeviceId, KeyAlgorithm>>,
    }

    response {
        /// If any remote homeservers could not be reached, they are recorded here.
        /// The names of the properties are the names of the unreachable servers.
        failures: HashMap<String, Value>,

        /// One-time keys for the queried devices.
        one_time_keys: HashMap<UserId, HashMap<DeviceId, HashMap<AlgorithmAndDeviceId, OneTimeKey>>>,
    }
}
