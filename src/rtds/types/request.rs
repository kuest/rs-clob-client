use bon::Builder;
use secrecy::ExposeSecret as _;
use serde::Serialize;
use serde_json::Value;

use super::response::CommentType;
use crate::auth::Credentials;

/// RTDS subscription request message.
#[non_exhaustive]
#[derive(Clone, Debug, Serialize, Builder)]
pub struct SubscriptionRequest {
    /// Action type ("subscribe" or "unsubscribe")
    pub action: SubscriptionAction,
    /// List of subscriptions
    pub subscriptions: Vec<Subscription>,
}

impl SubscriptionRequest {
    /// Create a subscribe request.
    #[must_use]
    pub fn subscribe(subscriptions: Vec<Subscription>) -> Self {
        Self {
            action: SubscriptionAction::Subscribe,
            subscriptions,
        }
    }

    /// Create an unsubscribe request.
    #[must_use]
    pub fn unsubscribe(subscriptions: Vec<Subscription>) -> Self {
        Self {
            action: SubscriptionAction::Unsubscribe,
            subscriptions,
        }
    }
}

/// Subscription action type.
#[non_exhaustive]
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionAction {
    /// Subscribe to topics
    Subscribe,
    /// Unsubscribe from topics
    Unsubscribe,
}

/// Individual subscription configuration.
///
/// # Security
///
/// When serialized, this struct exposes sensitive credentials (`clob_auth`) in plaintext.
/// Ensure subscription requests are only sent over secure WebSocket connections (`wss://`)
/// and never logged or exposed in error messages.
#[non_exhaustive]
#[derive(Clone, Debug, Builder)]
pub struct Subscription {
    /// Topic name (e.g., `comments`, `activity`)
    pub topic: String,
    /// Message type filter (e.g., `update`, `comment_created`, or `*` for all)
    pub msg_type: String,
    /// Optional filters (string or JSON object)
    pub filters: Option<String>,
    /// CLOB authentication (key, secret, passphrase)
    pub clob_auth: Option<Credentials>,
}

impl Subscription {
    /// Create a subscription for comments.
    #[must_use]
    pub fn comments(msg_type: Option<CommentType>) -> Self {
        let type_str = msg_type.map_or("*".to_owned(), |t| {
            serde_json::to_string(&t)
                .ok()
                .and_then(|s| s.trim_matches('"').to_owned().into())
                .unwrap_or_else(|| "*".to_owned())
        });
        Self {
            topic: "comments".to_owned(),
            msg_type: type_str,
            filters: None,
            clob_auth: None,
        }
    }

    /// Set CLOB authentication for this subscription.
    #[must_use]
    pub fn with_clob_auth(mut self, credentials: Credentials) -> Self {
        self.clob_auth = Some(credentials);
        self
    }

    /// Set custom filters for this subscription.
    #[must_use]
    pub fn with_filters(mut self, filters: String) -> Self {
        self.filters = Some(filters);
        self
    }
}

// Custom Serialize implementation for Subscription to handle auth fields
impl Serialize for Subscription {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap as _;

        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("topic", &self.topic)?;
        map.serialize_entry("type", &self.msg_type)?;

        if let Some(filters) = &self.filters {
            if let Ok(json_value) = serde_json::from_str::<Value>(filters) {
                // Other topics: parse and emit as raw JSON, e.g. ["btcusdt","ethusdt"]
                map.serialize_entry("filters", &json_value)?;
            } else {
                // Fallback: emit as string if not valid JSON
                map.serialize_entry("filters", filters)?;
            }
        }

        // SECURITY: Credentials are intentionally revealed here for the WebSocket auth protocol.
        // This data is only sent over wss:// connections to the RTDS server.
        if let Some(creds) = &self.clob_auth {
            let auth = serde_json::json!({
                "key": creds.key.to_string(),
                "secret": creds.secret.expose_secret(),
                "passphrase": creds.passphrase.expose_secret(),
            });
            map.serialize_entry("clob_auth", &auth)?;
        }

        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn comments_keep_community_protocol() {
        let request = SubscriptionRequest::subscribe(vec![Subscription::comments(None)]);
        assert_eq!(
            serde_json::to_value(request).unwrap(),
            serde_json::json!({"action":"subscribe","subscriptions":[{"topic":"comments","type":"*"}]})
        );
    }
}
