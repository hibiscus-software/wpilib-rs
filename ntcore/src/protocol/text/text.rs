// Copyright (C), 2024 Hibiscus Software and contributors. Some rights
// reserved. This work is licensed under the terms of the MIT license
// which can be found in the root directory of this project.

use serde::{Deserialize, Serialize};

use crate::protocol::binary::NTValue;

/// The following data types are supported
#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub enum DataType {
    /// Represents a boolean, true or false
    Boolean,
    /// Represents an IEEE 754 double-precision floating-point number
    Double,
    /// Represents a signed 64-bit integer
    Int,
    /// Represents an IEEE 754 single-precision floating-point number
    Float,
    /// Represents a sequence of bytes
    String,
    /// Represents a sequence of raw bytes
    Raw,
    /// Represents a remote procedure call declaration
    RPC,
    /// Represents an array of booleans
    BooleanArray,
    /// Represents an array of doubles
    DoubleArray,
    /// Represents an array of integers
    IntArray,
    /// Represents an array of floats
    FloatArray,
    /// Represents an array of strings
    StringArray,
}

impl DataType {
    pub fn default_value(&self) -> NTValue {
        match self {
            DataType::Boolean => NTValue::Int(0),
            DataType::Double => todo!(),
            DataType::Int => todo!(),
            DataType::Float => todo!(),
            DataType::String => todo!(),
            DataType::Raw => todo!(),
            DataType::RPC => todo!(),
            DataType::BooleanArray => todo!(),
            DataType::DoubleArray => todo!(),
            DataType::IntArray => todo!(),
            DataType::FloatArray => todo!(),
            DataType::StringArray => todo!(),
        }
    }
}
