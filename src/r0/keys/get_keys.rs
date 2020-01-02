//! [POST /_matrix/client/r0/keys/query](https://matrix.org/docs/spec/client_server/r0.6.0#post-matrix-client-r0-keys-query)

use super::DeviceKeys;
use js_int::UInt;
use ruma_api::ruma_api;
use ruma_identifiers::{DeviceId, UserId};
use serde_json::Value;
use std::collections::HashMap;

ruma_api! {
    metadata {
        description: "Returns the current devices and identity keys for the given users.",
        method: POST,
        name: "get_keys",
        path: "/_matrix/client/r0/keys/query",
        rate_limited: false,
        requires_authentication: true,
    }

    request {
        /// The time (in milliseconds) to wait when downloading keys from remote servers.
        /// 10 seconds is the recommended default.
        #[serde(skip_serializing_if = "Option::is_none")]
        timeout: Option<UInt>,

        /// The keys to be downloaded. An empty list indicates all devices for the corresponding user.
        device_keys: HashMap<UserId, Vec<DeviceId>>,

        /// If the client is fetching keys as a result of a device update received in a sync request,
        /// this should be the 'since' token of that sync request, or any later sync token.
        /// This allows the server to ensure its response contains the keys advertised by the notification in that sync.
        #[serde(skip_serializing_if = "Option::is_none")]
        token: Option<String>
    }

    response {
        /// If any remote homeservers could not be reached, they are recorded here.
        /// The names of the properties are the names of the unreachable servers.
        failures: HashMap<String, Value>,

        /// Information on the queried devices.
        device_keys: HashMap<UserId, HashMap<DeviceId, DeviceKeys>>,
    }
}
