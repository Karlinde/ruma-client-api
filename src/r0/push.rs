//! Endpoints for push notifications.

use std::fmt::{Formatter, Result as FmtResult};

use serde::{
    de::{Error as SerdeError, MapAccess, Unexpected, Visitor},
    ser::SerializeStructVariant,
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_json::Value as JsonValue;

pub mod get_notifications;
pub mod get_pushers;
pub mod get_pushrules_all;
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

/// How a notification is delivered for a matching event
#[derive(Clone, Debug)]
pub enum Action {
    /// Causes matching events to generate a notification.
    Notify,

    /// Prevents matching events from generating a notification.
    DontNotify,

    /// Behaves like notify but homeservers may choose to coalesce multiple events
    /// into a single notification.
    Coalesce,

    /// Sets an entry in the 'tweaks' dictionary sent to the push gateway.
    SetTweak {
        /// The kind of this tweak
        kind: TweakKind,

        /// The value of the tweak, if any
        value: Option<JsonValue>,
    },
}

/// The different kinds of tweaks available
#[derive(Clone, Debug)]
pub enum TweakKind {
    /// The "sound" tweak.
    Sound,

    /// The "highlight" tweak.
    Highlight,

    /// A custom client-defined tweak.
    Custom(String),
}

impl Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Action::Notify => serializer.serialize_unit_variant("Action", 0, "notify"),
            Action::DontNotify => serializer.serialize_unit_variant("Action", 1, "dont_notify"),
            Action::Coalesce => serializer.serialize_unit_variant("Action", 2, "coalesce"),
            Action::SetTweak { kind, value } => {
                let kind_name = match &kind {
                    TweakKind::Sound => "sound",
                    TweakKind::Highlight => "highlight",
                    TweakKind::Custom(name) => name,
                };
                let num_fields = match value {
                    Some(_) => 2,
                    None => 1,
                };
                let mut s =
                    serializer.serialize_struct_variant("Action", 3, "SetTweak", num_fields)?;
                s.serialize_field("set_tweak", kind_name)?;

                match &value {
                    Some(value) => {
                        s.serialize_field("value", value)?;
                    }
                    None => {}
                };
                s.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ActionVisitor;
        impl<'de> Visitor<'de> for ActionVisitor {
            type Value = Action;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> FmtResult {
                write!(formatter, "a valid action object")
            }

            /// Match a simple action type
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: SerdeError,
            {
                match v {
                    "notify" => Ok(Action::Notify),
                    "dont_notify" => Ok(Action::DontNotify),
                    "coalesce" => Ok(Action::Coalesce),
                    _ => Err(E::invalid_value(
                        Unexpected::Str(v),
                        &"valid value of notify, dont_notify or coalesce",
                    )),
                }
            }

            /// Match the more complex set_tweaks action object as a key-value map
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut tweak_kind: Option<TweakKind> = None;
                let mut tweak_value: Option<JsonValue> = None;

                // We loop over all entries in the map to find one with a "set_tweak" key to find
                // which type of tweak is being set.
                // Then we also try to find one with the "value" key if it exists.
                while let Some((key, value)) = map.next_entry::<&str, JsonValue>()? {
                    match key {
                        "set_tweak" => {
                            let kind = match value.as_str() {
                                Some("sound") => TweakKind::Sound,
                                Some("highlight") => TweakKind::Highlight,
                                Some(s) => TweakKind::Custom(s.to_string()),
                                None => {
                                    return Err(A::Error::invalid_type(
                                        Unexpected::Other("non-string object"),
                                        &"string",
                                    ))
                                }
                            };
                            tweak_kind = Some(kind);
                        }
                        "value" => {
                            tweak_value = Some(value);
                        }
                        _ => {}
                    }
                }

                match tweak_kind {
                    Some(kind) => Ok(Action::SetTweak {
                        kind,
                        value: tweak_value,
                    }),
                    None => Err(A::Error::invalid_type(
                        Unexpected::Other("dict without \"set_tweak\" key"),
                        &"valid \"set_tweak\" action object",
                    )),
                }
            }
        }

        deserializer.deserialize_any(ActionVisitor)
    }
}
