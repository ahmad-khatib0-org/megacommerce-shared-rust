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

impl AnyValue {
  pub fn from_string(s: String) -> Self {
    AnyValue::String(s)
  }

  pub fn from_str(s: &str) -> Self {
    AnyValue::String(s.to_string())
  }

  pub fn from_bool(b: bool) -> Self {
    AnyValue::Bool(b)
  }

  pub fn from_int32(i: i32) -> Self {
    AnyValue::Int32(i)
  }

  pub fn from_int64(i: i64) -> Self {
    AnyValue::Int64(i)
  }

  pub fn from_float(f: f32) -> Self {
    AnyValue::Float(f)
  }

  pub fn from_double(d: f64) -> Self {
    AnyValue::Double(d)
  }

  pub fn from_bytes(bytes: Vec<u8>) -> Self {
    AnyValue::Bytes(bytes)
  }

  pub fn from_slice(slice: &[u8]) -> Self {
    AnyValue::Bytes(slice.to_vec())
  }

  pub fn from_unknown(bytes: Vec<u8>) -> Self {
    AnyValue::Unknown(bytes)
  }

  // Convenience method to create from any type that implements Into<Vec<u8>>
  pub fn from_unknown_slice(slice: &[u8]) -> Self {
    AnyValue::Unknown(slice.to_vec())
  }

  // Try to convert to specific types (useful for extracting values)
  pub fn as_string(&self) -> Option<&String> {
    match self {
      AnyValue::String(s) => Some(s),
      _ => None,
    }
  }

  pub fn as_bool(&self) -> Option<bool> {
    match self {
      AnyValue::Bool(b) => Some(*b),
      _ => None,
    }
  }

  pub fn as_int32(&self) -> Option<i32> {
    match self {
      AnyValue::Int32(i) => Some(*i),
      _ => None,
    }
  }

  pub fn as_int64(&self) -> Option<i64> {
    match self {
      AnyValue::Int64(i) => Some(*i),
      _ => None,
    }
  }

  pub fn as_float(&self) -> Option<f32> {
    match self {
      AnyValue::Float(f) => Some(*f),
      _ => None,
    }
  }

  pub fn as_double(&self) -> Option<f64> {
    match self {
      AnyValue::Double(d) => Some(*d),
      _ => None,
    }
  }

  pub fn as_bytes(&self) -> Option<&Vec<u8>> {
    match self {
      AnyValue::Bytes(bytes) => Some(bytes),
      _ => None,
    }
  }

  pub fn as_unknown(&self) -> Option<&Vec<u8>> {
    match self {
      AnyValue::Unknown(bytes) => Some(bytes),
      _ => None,
    }
  }

  // Check the type of the value
  pub fn is_string(&self) -> bool {
    matches!(self, AnyValue::String(_))
  }

  pub fn is_bool(&self) -> bool {
    matches!(self, AnyValue::Bool(_))
  }

  pub fn is_int32(&self) -> bool {
    matches!(self, AnyValue::Int32(_))
  }

  pub fn is_int64(&self) -> bool {
    matches!(self, AnyValue::Int64(_))
  }

  pub fn is_float(&self) -> bool {
    matches!(self, AnyValue::Float(_))
  }

  pub fn is_double(&self) -> bool {
    matches!(self, AnyValue::Double(_))
  }

  pub fn is_bytes(&self) -> bool {
    matches!(self, AnyValue::Bytes(_))
  }

  pub fn is_unknown(&self) -> bool {
    matches!(self, AnyValue::Unknown(_))
  }
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
