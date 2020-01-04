//! Endpoints for push notifications.

use serde::{Deserialize, Serialize};

pub mod get_notifications;
pub mod get_pushers;
pub mod set_pusher;

/// Defines a pusher
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pusher {
    /// This is a unique identifier for this pusher. Max length, 512 bytes.
    pub pushkey: String,

    /// The kind of the pusher. If set to None in a call to set_pusher, this
    /// will delete the pusher
    pub kind: Option<PusherKind>,

    /// This is a reverse-DNS style identifier for the application. Max length, 64 chars.
    pub app_id: String,

    /// A string that will allow the user to identify what application owns this pusher.
    pub app_display_name: String,

    /// A string that will allow the user to identify what device owns this pusher.
    pub device_display_name: String,

    /// This string determines which set of device specific rules this pusher executes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_tag: Option<String>,

    /// The preferred language for receiving notifications (e.g. 'en' or 'en-US')
    pub lang: String,

    /// Information for the pusher implementation itself.
    pub data: PusherData,
}

/// Which kind a pusher is
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum PusherKind {
    /// A pusher that sends HTTP pokes.
    #[serde(rename = "http")]
    Http,

    /// A pusher that emails the user with unread notifications.
    #[serde(rename = "email")]
    Email,
}

/// Information for the pusher implementation itself.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherData {
    /// Required if the pusher's kind is http. The URL to use to send notifications to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// The format to use when sending notifications to the Push Gateway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<PushFormat>,
}

/// A special format that the homeserver should use when sending notifications to a Push Gateway.
/// Currently, only "event_id_only" is supported as of [Push Gateway API r0.1.1](https://matrix.org/docs/spec/push_gateway/r0.1.1#homeserver-behaviour)
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum PushFormat {
    /// Require the homeserver to only send a reduced set of fields in the push.
    #[serde(rename = "event_id_only")]
    EventIdOnly,
}
