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
      id: get_string(&Header::SessionId.as_str()),
      token: get_string(&Header::Token.as_str()),
      created_at: get_int(&Header::CreatedAt.as_str()),
      expires_at: get_int(&Header::ExpiresAt.as_str()),
      last_activity_at: get_int(&Header::LastActivityAt.as_str()),
      user_id: get_string(&Header::UserId.as_str()),
      device_id: get_string(&Header::DeviceId.as_str()),
      roles: get_string(&Header::Roles.as_str()),
      is_oauth: get_bool(&Header::IsOauth.as_str()),
      props: get_props(&Header::Props.as_str()),
    };

    Context::new(
      session,
      get_string(Header::XRequestId.as_str()),
      get_string(Header::XIpAddress.as_str()),
      get_string(Header::XForwardedFor.as_str()),
      get_string(Header::Path.as_str()),
      get_string(Header::UserAgent.as_str()),
      get_string(Header::AcceptLanguage.as_str()),
    )
  };

  req.extensions_mut().insert(Arc::new(context));

  Ok(req)
}
