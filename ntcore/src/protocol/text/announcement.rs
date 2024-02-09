// Copyright (C), 2024 Hibiscus Software and contributors. Some rights
// reserved. This work is licensed under the terms of the MIT license
// which can be found in the root directory of this project.

use super::text::DataType;
use serde::{Deserialize, Serialize};

/// Topic announcement message
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Announce {
    /// Topic name
    pub name: String,
    /// The identifier that the server will use in MessagePack messages for this topic
    pub id: i32,
    /// The data type for the topic (as a string)
    pub r#type: DataType,
    /// If this message was sent in response to a publish message, the Publisher
    /// UID provided in that message. Otherwise absent.
    pub pub_uid: Option<i32>,
    /// Topic properties
    pub properties: Vec<String>,
}

/// Topic removed message
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Unannounce {
    /// Topic name
    pub name: String,
    /// The identifier that the server was using for value updates
    pub id: i32,
}

/// Properties update message
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Properties {
    /// Topic name
    pub name: String,
    /// True if this message is in response to a setproperties message from the
    /// same client. Otherwise absent.
    pub ack: Option<bool>,
}
