use chrono::Utc;

pub fn time_get_millis() -> u64 {
  Utc::now().timestamp_millis().try_into().unwrap()
}

pub fn time_get_seconds() -> u64 {
  Utc::now().timestamp().try_into().unwrap()
}
