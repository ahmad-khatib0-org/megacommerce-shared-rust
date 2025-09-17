/// returns redis key
///
/// * `jti`: is jwt token id
pub fn auth_token_status_key(jti: &str) -> String {
  return format!("auth:token#{}", jti);
}

/// returns redis key
///
/// * `email`: is the user email
pub fn auth_user_data_key(email: &str) -> String {
  return format!("auth:user#{}", email);
}
