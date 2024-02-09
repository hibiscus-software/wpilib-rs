// Copyright (C), 2024 Hibiscus Software and contributors. Some rights
// reserved. This work is licensed under the terms of the MIT license
// which can be found in the root directory of this project.

use serde::{Deserialize, Serialize};

use super::text::text::DataType;

macro_rules! impl_conversion {
    ($self:ident, $($inst:ident), +) => {
        match $self {
            $(NTValue::$inst(_) => DataType::$inst,)+
        }
    };
}

/// A Network Tables value
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum NTValue {
    Boolean(bool),
    Double(f64),
    Int(i64),
    Float(f32),
    String(String),
    Raw(Vec<u8>),
    RPC(Vec<u8>),
    BooleanArray(Vec<bool>),
    DoubleArray(Vec<f64>),
    IntArray(Vec<i64>),
    FloatArray(Vec<f32>),
    StringArray(Vec<String>),
}

impl NTValue {
    pub fn data_type(&self) -> DataType {
        impl_conversion!(
            self,
            Boolean,
            Double,
            Int,
            Float,
            String,
            Raw,
            RPC,
            BooleanArray,
            DoubleArray,
            IntArray,
            FloatArray,
            StringArray
        )
    }
}
