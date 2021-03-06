//! [GET /_matrix/client/r0/login](https://matrix.org/docs/spec/client_server/r0.6.0#get-matrix-client-r0-login)

use ruma_api::ruma_api;
use serde::{Deserialize, Serialize};

use super::login::LoginType;

ruma_api! {
    metadata {
        description: "Gets the homeserver's supported login types to authenticate users. Clients should pick one of these and supply it as the type when logging in.",
        method: GET,
        name: "get_login_types",
        path: "/_matrix/client/r0/login",
        rate_limited: true,
        requires_authentication: false,
    }

    request {}

    response {
        /// The homeserver's supported login types.
        pub flows: Vec<LoginFlow>
    }
}

/// A supported login type in a homeserver
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct LoginFlow {
    /// The login type.
    #[serde(rename = "type")]
    pub login_type: LoginType,
}
