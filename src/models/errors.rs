use std::{collections::HashMap, error::Error, fmt, sync::Arc};

use derive_more::Display;
use megacommerce_proto::{AppError as AppErrorProto, NestedStringMap, StringMap};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tonic::Code;

use super::{
  context::Context,
  translate::{tr, TranslateFunc},
};

pub type BoxedErr = Box<dyn Error + Sync + Send>;
pub type OptionalErr = Option<BoxedErr>;
pub type OptionalParams = Option<HashMap<String, Value>>;

pub const MSG_ID_ERR_INTERNAL: &str = "server.internal.error";
pub const MSG_ERR_INTERNAL: &str =
  "Sorry, Unexpected internal server error. Our team has been notified. Please try again";

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorType {
  NoRows,
  UniqueViolation,
  ForeignKeyViolation,
  NotNullViolation,
  JsonMarshal,
  JsonUnmarshal,
  Connection,
  Privileges,
  Internal,
  DBConnectionError,
  DBInsertError,
  DBUpdateError,
  DBDeleteError,
  ConfigError,
  HttpRequestError,
  HttpResponseError,
  HttpEmptyResponse,
  MissingField,
  InvalidData,
  TimedOut,
  TaskFailed,
  Base64Invalid,
  RegexInvalid,
  InvalidNumber,
}

impl fmt::Display for ErrorType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ErrorType::DBConnectionError => write!(f, "db_connection_error"),
      ErrorType::DBInsertError => write!(f, "db_insert_error"),
      ErrorType::DBUpdateError => write!(f, "db_update_error"),
      ErrorType::DBDeleteError => write!(f, "db_delete_error"),
      ErrorType::NoRows => write!(f, "no_rows"),
      ErrorType::UniqueViolation => write!(f, "unique_violation"),
      ErrorType::ForeignKeyViolation => write!(f, "foreign_key_violation"),
      ErrorType::NotNullViolation => write!(f, "not_null_violation"),
      ErrorType::JsonMarshal => write!(f, "json_marshal"),
      ErrorType::JsonUnmarshal => write!(f, "json_unmarshal"),
      ErrorType::Connection => write!(f, "connection_exception"),
      ErrorType::Privileges => write!(f, "insufficient_privilege"),
      ErrorType::ConfigError => write!(f, "config_error"),
      ErrorType::Internal => write!(f, "internal_error"),
      ErrorType::HttpRequestError => write!(f, "http_request_error"),
      ErrorType::HttpResponseError => write!(f, "http_response_error"),
      ErrorType::HttpEmptyResponse => write!(f, "http_empty_response"),
      ErrorType::MissingField => write!(f, "missing_field"),
      ErrorType::InvalidData => write!(f, "invalid_data"),
      ErrorType::TimedOut => write!(f, "timed_out"),
      ErrorType::TaskFailed => write!(f, "task_failed"),
      ErrorType::Base64Invalid => write!(f, "base64_invalid"),
      ErrorType::RegexInvalid => write!(f, "regex_invalid"),
      ErrorType::InvalidNumber => write!(f, "invalid_number"),
    }
  }
}

#[derive(Debug)]
pub struct SimpleError {
  pub message: String,
  pub _type: ErrorType,
  pub err: BoxedErr,
}

#[derive(Debug, Display)]
#[display("InternalError: {path}: {msg}, temp: {temp}, err: {err_type} {err}")]
pub struct InternalError {
  pub err: Box<dyn Error + Send + Sync>,
  pub err_type: ErrorType,
  pub temp: bool,
  pub msg: String,
  pub path: String,
}

impl InternalError {
  pub fn new(path: String, err: BoxedErr, err_type: ErrorType, temp: bool, msg: String) -> Self {
    Self { err, err_type, temp, msg, path }
  }
}

impl Error for InternalError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    Some(self.err.as_ref())
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppErrorError {
  pub id: String,
  pub params: Option<HashMap<String, Value>>,
}

#[derive(Debug, Default)]
pub struct AppErrorErrors {
  pub err: OptionalErr,
  pub errors_internal: Option<HashMap<String, AppErrorError>>,
  pub errors_nested_internal: Option<HashMap<String, HashMap<String, AppErrorError>>>,
}

#[derive(Debug)]
pub struct AppError {
  pub ctx: Arc<Context>,
  pub id: String,
  pub path: String,
  pub message: String,
  pub detailes: String,
  pub request_id: Option<String>,
  pub status_code: i32,
  pub tr_params: OptionalParams,
  pub skip_translation: bool,
  pub error: OptionalErr,
  pub errors: Option<HashMap<String, String>>,
  pub errors_nested: Option<HashMap<String, HashMap<String, String>>>,
  pub errors_internal: Option<HashMap<String, AppErrorError>>,
  pub errors_nested_internal: Option<HashMap<String, HashMap<String, AppErrorError>>>,
}

impl AppError {
  pub fn new(
    ctx: Arc<Context>,
    path: impl Into<String>,
    id: impl Into<String>,
    id_params: OptionalParams,
    details: impl Into<String>,
    status_code: i32,
    mut errors: Option<AppErrorErrors>,
  ) -> Self {
    if errors.is_none() {
      errors =
        Some(AppErrorErrors { err: None, errors_internal: None, errors_nested_internal: None });
    };

    let unwraped = {
      let e = errors.unwrap();
      (e.err, e.errors_internal, e.errors_nested_internal)
    };

    let mut err = Self {
      ctx,
      id: id.into(),
      path: path.into(),
      message: "".to_string(),
      detailes: details.into(),
      request_id: None,
      status_code,
      tr_params: id_params,
      skip_translation: false,
      error: unwraped.0,
      errors: None,
      errors_nested: None,
      errors_internal: unwraped.1,
      errors_nested_internal: unwraped.2,
    };

    let boxed_tr = Box::new(|lang: &str, id: &str, params: &HashMap<String, serde_json::Value>| {
      let params_option = if params.is_empty() { None } else { Some(params.clone()) };
      tr(lang, id, params_option).map_err(|e| Box::new(e) as Box<dyn Error>)
    });

    err.translate(Some(boxed_tr));
    err
  }

  pub fn error_string(&self) -> String {
    let mut s = String::new();

    if !self.path.is_empty() {
      s.push_str(&self.path);
      s.push_str(": ");
    }

    if !self.message.is_empty() {
      s.push_str(&self.message);
    }

    if !self.detailes.is_empty() {
      s.push_str(&format!(", {}", self.detailes));
    }

    if let Some(ref err) = self.error {
      s.push_str(&format!(", {}", err.to_string()));
    }

    s
  }

  pub fn translate(&mut self, tf: Option<TranslateFunc>) {
    if self.skip_translation {
      return;
    }

    if let Some(tf) = tf {
      let empty = HashMap::new();
      let params = self.tr_params.as_ref().unwrap_or(&empty);
      if let Ok(translated) = tf(&self.ctx.accept_language, &self.id, params) {
        self.message = translated;
        return;
      }
    }
    self.message = self.id.clone();
  }

  pub fn unwrap(&self) -> Option<&(dyn Error + Send + Sync)> {
    self.error.as_deref()
  }

  pub fn wrap(mut self, err: Box<dyn Error + Send + Sync>) -> Self {
    self.error = Some(err);
    self
  }

  pub fn wipe_detailed(&mut self) {
    self.error = None;
    self.detailes.clear();
  }

  pub fn default() -> Self {
    Self {
      ctx: Arc::new(Context::default()),
      path: String::new(),
      id: String::new(),
      message: String::new(),
      detailes: String::new(),
      request_id: None,
      status_code: Code::Ok as i32,
      tr_params: None,
      skip_translation: false,
      error: None,
      errors: None,
      errors_nested: None,
      errors_internal: None,
      errors_nested_internal: None,
    }
  }

  /// Convert to proto-generated struct
  pub fn to_proto(&self) -> AppErrorProto {
    let mut nested = HashMap::new();
    if let Some(errors) = &self.errors_nested {
      for (k, v) in errors {
        nested.insert(k.clone(), StringMap { values: v.clone() });
      }
    }

    let mut errors: HashMap<String, String> = HashMap::new();
    for (key, value) in self.errors_internal.clone().unwrap_or_default().iter() {
      let result =
        tr(&self.ctx.accept_language(), &value.id, value.params.clone()).unwrap_or_default();
      errors.insert(key.to_string(), result);
    }

    AppErrorProto {
      id: self.id.clone(),
      r#where: self.path.clone(),
      message: self.message.clone(),
      detailed_error: self.detailes.clone(),
      status_code: self.status_code as i32,
      skip_translation: self.skip_translation,
      request_id: self.request_id.clone().unwrap_or_default(),
      errors: Some(StringMap { values: errors }),
      errors_nested: Some(NestedStringMap { data: nested }),
    }
  }

  pub fn to_internal(self, ctx: Arc<Context>, path: String) -> Self {
    let errors = AppErrorErrors { err: self.error, ..Default::default() };
    Self::new(
      ctx,
      path,
      MSG_ID_ERR_INTERNAL,
      None,
      self.detailes,
      Code::Internal.into(),
      Some(errors),
    )
  }
}

/// Convert from proto-generated struct
pub fn app_error_from_proto_app_error(ctx: Arc<Context>, ae: &AppErrorProto) -> AppError {
  let (errors, nested) = convert_proto_params(ae);

  AppError {
    ctx,
    id: ae.id.clone(),
    path: ae.r#where.clone(),
    message: ae.message.clone(),
    detailes: ae.detailed_error.clone(),
    request_id: Some(ae.request_id.clone()).filter(|s| !s.is_empty()),
    status_code: ae.status_code as i32,
    tr_params: None,
    skip_translation: ae.skip_translation,
    error: None,
    errors,
    errors_nested: nested,
    errors_internal: None,
    errors_nested_internal: None,
  }
}

/// Convert proto params to HashMaps
pub fn convert_proto_params(
  ae: &AppErrorProto,
) -> (Option<HashMap<String, String>>, Option<HashMap<String, HashMap<String, String>>>) {
  let mut shallow = HashMap::new();
  let mut nested = HashMap::new();

  if let Some(ref p) = ae.errors {
    shallow.extend(p.values.clone());
  }
  if let Some(ref n) = ae.errors_nested {
    for (k, v) in &n.data {
      nested.insert(k.clone(), v.values.clone());
    }
  }

  (Some(shallow), Some(nested))
}

// Implement std::fmt::Display for error formatting
impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.error_string())
  }
}

impl Error for AppError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    self.error.as_ref().map(|e| e.as_ref() as &(dyn Error + 'static))
  }
}
