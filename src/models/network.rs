use derive_more::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
pub enum Header {
  #[display("authorization")]
  Authorization,
  #[display("x-request-id")]
  XRequestId,
  #[display("x-ip-address")]
  XIpAddress,
  #[display("x-forwarded-for")]
  XForwardedFor,
  #[display("path")]
  Path,
  #[display("user-agent")]
  UserAgent,
  #[display("accept-language")]
  AcceptLanguage,
  #[display("session-id")]
  SessionId,
  #[display("token")]
  Token,
  #[display("created-at")]
  CreatedAt,
  #[display("expires-at")]
  ExpiresAt,
  #[display("last-activity-at")]
  LastActivityAt,
  #[display("user-id")]
  UserId,
  #[display("device-id")]
  DeviceId,
  #[display("roles")]
  Roles,
  #[display("is-oauth")]
  IsOauth,
  #[display("props")]
  Props,
}

impl Header {
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::Authorization => "authorization",
      Self::XRequestId => "x-request-id",
      Self::XIpAddress => "x-ip-address",
      Self::XForwardedFor => "x-forwarded-for",
      Self::Path => "path",
      Self::UserAgent => "user-agent",
      Self::AcceptLanguage => "accept-language",
      Self::SessionId => "session-id",
      Self::Token => "token",
      Self::CreatedAt => "created-at",
      Self::ExpiresAt => "expires-at",
      Self::LastActivityAt => "last-activity-at",
      Self::UserId => "user-id",
      Self::DeviceId => "device-id",
      Self::Roles => "roles",
      Self::IsOauth => "is-oauth",
      Self::Props => "props",
    }
  }
}
