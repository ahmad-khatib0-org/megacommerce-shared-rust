use std::{collections::HashMap, sync::Arc};

use tonic::{Request, Status};

use crate::models::{
  context::{Context, Session},
  network::Header,
};

pub fn middleware_context(mut req: Request<()>) -> Result<Request<()>, Status> {
  let m = req.metadata_mut();

  let get_string = |key: &str| m.get(key).and_then(|v| v.to_str().ok()).unwrap_or("").to_string();

  let get_int = |key: &str| {
    m.get(key).and_then(|v| v.to_str().ok()).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0)
  };

  let get_bool =
    |key: &str| m.get(key).and_then(|v| v.to_str().ok()).map(|s| s == "true").unwrap_or(false);

  let get_props = |key: &str| {
    m.get(key)
      .and_then(|v| v.to_str().ok())
      .map(|s| {
        s.split(',')
          .filter_map(|pair| {
            let mut parts = pair.trim().splitn(2, ':');
            if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
              Some((k.trim().to_string(), v.trim().to_string()))
            } else {
              None
            }
          })
          .collect::<HashMap<_, _>>()
      })
      .unwrap_or_default()
  };

  let context = {
    let session = Session {
      id: get_string(Header::XSessionID.as_str()),
      token: get_string(Header::Authorization.as_str()),
      created_at: get_int(Header::XSessionCreatedAt.as_str()),
      expires_at: get_int(Header::XSessionExpiresAt.as_str()),
      last_activity_at: get_int(Header::XLastActivityAt.as_str()),
      user_id: get_string(Header::XUserID.as_str()),
      device_id: get_string(Header::XDeviceID.as_str()),
      roles: get_string(Header::XRoles.as_str()),
      is_oauth: get_bool(Header::XIsOAuth.as_str()),
      props: get_props(Header::XProps.as_str()),
    };

    Context::new(
      session,
      get_string(Header::XRequestID.as_str()),
      get_string(Header::XIPAddress.as_str()),
      get_string(Header::XForwardedFor.as_str()),
      get_string(":path"),
      get_string(Header::UserAgent.as_str()),
      get_string(Header::AcceptLanguage.as_str()),
      get_string(Header::XTimezone.as_str()),
    )
  };

  req.extensions_mut().insert(Arc::new(context));

  Ok(req)
}
