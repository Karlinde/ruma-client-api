//! [GET /_matrix/client/r0/pushrules/](https://matrix.org/docs/spec/client_server/r0.6.0#get-matrix-client-r0-pushrules)

use ruma_api::ruma_api;
use serde::{Deserialize, Serialize};

use super::Action;

ruma_api! {
    metadata {
        description: "Retrieve all push rulesets for this user.",
        method: GET,
        name: "get_pushrules_all",
        path: "/_matrix/client/r0/pushrules/",
        rate_limited: false,
        requires_authentication: true,
    }

    request {}

    response {
        /// The global ruleset
        pub global: Ruleset
    }
}

/// A set of push rules
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ruleset {
    /// Content-specific rules
    #[serde(rename = "content")]
    pub content_rules: Vec<PushRule>,

    /// User-configured rules that override all other kinds
    #[serde(rename = "override")]
    pub override_rules: Vec<PushRule>,

    /// Room-specific rules
    #[serde(rename = "room")]
    pub room_rules: Vec<PushRule>,

    /// Sender-specific rules
    #[serde(rename = "sender")]
    pub sender_rules: Vec<PushRule>,

    /// Lowest priority user-defined rules
    #[serde(rename = "underride")]
    pub underride_rules: Vec<PushRule>,
}

/// A push rule
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PushRule {
    /// The actions to perform when this rule is matched.
    pub actions: Vec<Action>,

    /// Whether this is a default rule, or has been set explicitly.
    pub default: bool,

    /// Whether the push rule is enabled or not.
    pub enabled: bool,

    /// The ID of this rule.
    pub rule_id: String,

    /// The conditions that must hold true for an event in order for a rule to be applied to an event. A rule with no conditions always matches.
    /// Only applicable to underride and override rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<PushCondition>>,

    /// The glob-style pattern to match against. Only applicable to content rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

/// A condition for a push rule
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind")] // Using internally tagged enum representation to match the spec
pub enum PushCondition {
    /// This is a glob pattern match on a field of the event.
    #[serde(rename = "event_match")]
    EventMatch {
        /// The dot-separated field of the event to match, e.g. `content.body`
        key: String,
        /// The glob-style pattern to match against.
        pattern: String,
    },

    /// This matches unencrypted messages where `content.body` contains
    /// the owner's display name in that room.
    #[serde(rename = "contains_display_name")]
    ContainsDisplayName,

    /// This matches the current number of members in the room.
    #[serde(rename = "room_member_count")]
    RoomMemberCount {
        /// A decimal integer optionally prefixed by one of, ==, <, >, >= or <=.
        /// Default prefix is ==.
        is: String,
    },

    /// This takes into account the current power levels in the room, ensuring the
    /// sender of the event has high enough power to trigger the notification.
    #[serde(rename = "sender_notification_permission")]
    SenderNotificationPermission {
        /// A string that determines the power level the sender must have to
        /// trigger notifications of a given type, such as `room`.
        key: String,
    },
}
