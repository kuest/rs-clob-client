#![expect(
    clippy::module_name_repetitions,
    reason = "Re-exported names intentionally match their modules for API clarity"
)]

//! Community live-data client for comments and activity.
//! Price channels use `realtime::Client` with the `realtime` feature.

pub mod client;
pub mod error;
pub mod subscription;
pub mod types;

// Re-export commonly used types
pub use client::Client;
pub use error::RtdsError;
pub use subscription::SubscriptionInfo;
pub use types::request::{Subscription, SubscriptionAction, SubscriptionRequest};
pub use types::response::{Comment, CommentProfile, CommentType, RtdsMessage};
