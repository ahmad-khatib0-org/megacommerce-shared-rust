use std::io::Cursor;

use base64::{engine::general_purpose, Engine as _};
use image::{ImageFormat, ImageReader};

#[derive(Debug, Clone)]
pub struct ImageValidationConfig {
  pub max_size_bytes: usize,
  pub allowed_formats: Vec<ImageFormat>,
  pub max_width: u32,
  pub max_height: u32,
  pub min_width: u32,
  pub min_height: u32,
}

#[derive(Debug, Clone)]
pub struct ImageValidationResult {
  pub format: ImageFormat,
  pub dimensions: (u32, u32),
  pub size_bytes: usize,
  pub is_valid: bool,
  pub decoded_data: Vec<u8>,
}

impl Default for ImageValidationConfig {
  fn default() -> Self {
    Self {
      max_size_bytes: 1024 * 1024, // 1MB
      allowed_formats: vec![ImageFormat::Png, ImageFormat::Jpeg, ImageFormat::WebP],
      max_width: 4000,
      max_height: 4000,
      min_width: 100,
      min_height: 100,
    }
  }
}

#[derive(Debug)]
pub enum ImageValidationError {
  LargeImage(String),
  InvalidBase64(String),
  UnknownFormat(String),
  NotAllowedFormat(String),
  SmallDimensions(String),
  LargeDimensions(String),
  UnknownDimensions(String),
}

pub fn validate_base64_image(
  data: &str,
  config: &ImageValidationConfig,
) -> Result<ImageValidationResult, ImageValidationError> {
  let clean_data = data.split(',').last().unwrap_or(data);

  // Quick size check before decoding
  // Base64 encoding uses 4 ASCII characters to represent 3 bytes of binary data:
  // Input: 3 bytes of binary data (3 × 8 = 24 bits)
  // Output: 4 base64 characters (4 × 6 = 24 bits)
  // So the relationship is: 4 chars (base64) = 3 bytes (binary)
  let estimated_size = (clean_data.len() * 3) / 4;
  if estimated_size > config.max_size_bytes {
    return Err(ImageValidationError::LargeImage(format!(
      "Estimated size: {} is bigger than allowed size: {}",
      estimated_size, config.max_size_bytes
    )));
  }

  let decoded_data = general_purpose::STANDARD
    .decode(clean_data)
    .map_err(|err| ImageValidationError::InvalidBase64(format!("Invalid base64 data: {}", err)))?;

  // Exact size validation
  if decoded_data.len() > config.max_size_bytes {
    return Err(ImageValidationError::LargeImage(format!(
      "Actual image size: {} is bigger than allowed size: {}",
      decoded_data.len(),
      config.max_size_bytes
    )));
  }

  let cursor = Cursor::new(&decoded_data);
  let reader = ImageReader::new(cursor).with_guessed_format().map_err(|err| {
    ImageValidationError::UnknownFormat(format!("Unable to determine image format: {}", err))
  })?;

  let format = reader.format().ok_or_else(|| {
    ImageValidationError::UnknownFormat("Unable to determine image format".to_string())
  })?;

  if !config.allowed_formats.contains(&format) {
    return Err(ImageValidationError::NotAllowedFormat(format!(
      "Image format {} is not allowed",
      format.extensions_str().first().unwrap_or(&"unknown")
    )));
  }

  let dimensions = reader.into_dimensions().map_err(|err| {
    ImageValidationError::UnknownDimensions(format!("Unable to get dimensions: {}", err))
  })?;

  if dimensions.0 < config.min_width || dimensions.1 < config.min_height {
    return Err(ImageValidationError::SmallDimensions(format!(
      "Image dimensions {}x{} are too small, minimum is {}x{}",
      dimensions.0, dimensions.1, config.min_width, config.min_height
    )));
  }

  if dimensions.0 > config.max_width || dimensions.1 > config.max_height {
    return Err(ImageValidationError::LargeDimensions(format!(
      "Image dimensions {}x{} are too large, maximum is {}x{}",
      dimensions.0, dimensions.1, config.max_width, config.max_height
    )));
  }

  Ok(ImageValidationResult {
    format,
    dimensions,
    size_bytes: decoded_data.len(),
    is_valid: true,
    decoded_data,
  })
}
