//! [GET /_matrix/client/r0/pushrules/device/{profile_tag}/](https://matrix.org/docs/spec/client_server/r0.6.0#get-matrix-client-r0-pushrules)
//!
//! Note: This endpoint is not strictly defined in the spec per se, but since GET `/pushrules/` can be appended
//! by a `scope` and `scope` may be either `global` or `device/<profile_tag>`, then this endpoint should logically also be defined.

use std::collections::HashMap;

use ruma_api::ruma_api;

use super::{PushRule, RuleKind};

ruma_api! {
    metadata {
        description: "Retrieve all push rulesets in the device scope for this user.",
        method: GET,
        name: "get_pushrules_device_scope",
        path: "/_matrix/client/r0/pushrules/device/:profile_tag/",
        rate_limited: false,
        requires_authentication: true,
    }

    request {
        /// This string represents a set of device specific rules.
        #[ruma_api(path)]
        pub profile_tag: String
    }

    response {
        /// The device ruleset.
        #[ruma_api(body)]
        pub device: HashMap<RuleKind, Vec<PushRule>>
    }
}