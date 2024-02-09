// Copyright (C), 2024 Hibiscus Software and contributors. Some rights
// reserved. This work is licensed under the terms of the MIT license
// which can be found in the root directory of this project.

use serde::{Deserialize, Serialize};

/// Subscribe message
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Subscribe {
    /// Array of topic names or prefixes
    pub topics: Vec<String>,
    /// A client-generated unique identifier for this subscription. Use the same
    /// UID later to unsubscribe.
    pub sub_uid: u32,
    /// Subscription options
    pub options: Option<SubscriptionOptions>,
}

/// Unsubscribe message
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Unsubscribe {
    /// The same unique identifier passed to the subscribe message
    pub sub_uid: u32,
}

/// Subscription options
#[derive(Serialize, Deserialize, Default, Debug, PartialEq)]
#[serde(default)]
pub struct SubscriptionOptions {
    /// Periodic sweep time (in seconds)
    #[serde(default = "default_periodic")]
    pub periodic: f64,
    /// All changes flag
    pub all: bool,
    /// No value changes flag
    pub topics_only: bool,
    /// Prefix flag
    pub prefix: bool,
}

/// Default update rate in Network Tables v3
fn default_periodic() -> f64 {
    return 0.1;
}
