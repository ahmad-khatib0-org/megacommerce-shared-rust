use std::fmt;

use megacommerce_proto::Any;

#[derive(Debug)]
pub enum AnyValue {
  String(String),
  Bool(bool),
  Int32(i32),
  Int64(i64),
  Float(f32),
  Double(f64),
  Bytes(Vec<u8>),
  Unknown(Vec<u8>),
}

pub fn grpc_deserialize_any(any: &Any) -> AnyValue {
  match any.type_url.as_str() {
    "type.googleapis.com/google.protobuf.StringValue" => String::from_utf8(any.value.clone())
      .map(AnyValue::String)
      .unwrap_or_else(|_| AnyValue::Unknown(any.value.clone())),
    "type.googleapis.com/google.protobuf.BoolValue" => {
      AnyValue::Bool(any.value.first().map(|&b| b != 0).unwrap_or(false))
    }
    "type.googleapis.com/google.protobuf.Int32Value" => any
      .value
      .as_slice()
      .try_into()
      .map(|bytes| AnyValue::Int32(i32::from_le_bytes(bytes)))
      .unwrap_or(AnyValue::Unknown(any.value.clone())),
    "type.googleapis.com/google.protobuf.Int64Value" => any
      .value
      .as_slice()
      .try_into()
      .map(|bytes| AnyValue::Int64(i64::from_le_bytes(bytes)))
      .unwrap_or(AnyValue::Unknown(any.value.clone())),
    "type.googleapis.com/google.protobuf.FloatValue" => any
      .value
      .as_slice()
      .try_into()
      .map(|bytes| AnyValue::Float(f32::from_le_bytes(bytes)))
      .unwrap_or(AnyValue::Unknown(any.value.clone())),
    "type.googleapis.com/google.protobuf.DoubleValue" => any
      .value
      .as_slice()
      .try_into()
      .map(|bytes| AnyValue::Double(f64::from_le_bytes(bytes)))
      .unwrap_or(AnyValue::Unknown(any.value.clone())),
    "type.googleapis.com/google.protobuf.BytesValue" => AnyValue::Bytes(any.value.clone()),
    _ => AnyValue::Unknown(any.value.clone()),
  }
}

impl fmt::Display for AnyValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      AnyValue::String(s) => write!(f, "string:\"{}\"", s),
      AnyValue::Bool(b) => write!(f, "bool:{}", b),
      AnyValue::Int32(i) => write!(f, "i32:{}", i),
      AnyValue::Int64(i) => write!(f, "i64:{}", i),
      AnyValue::Float(n) => write!(f, "float:{}", n),
      AnyValue::Double(n) => write!(f, "double:{}", n),
      AnyValue::Bytes(bytes) => {
        if bytes.len() <= 8 {
          write!(f, "bytes:{}", hex::encode(bytes))
        } else {
          write!(f, "bytes:{}...", hex::encode(&bytes[..8]))
        }
      }
      AnyValue::Unknown(bytes) => {
        if bytes.len() <= 8 {
          write!(f, "unknown:{}", hex::encode(bytes))
        } else {
          write!(f, "unknown:{}...", hex::encode(&bytes[..8]))
        }
      }
    }
  }
}
