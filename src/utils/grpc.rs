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
