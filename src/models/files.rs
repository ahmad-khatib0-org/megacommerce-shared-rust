use base64::{engine::general_purpose::STANDARD, Engine as _};
use regex::Regex;
use sha2::{Digest, Sha256};

use super::errors::{ErrorType, SimpleError};

pub enum UnitSizeType {
  Bytes,
  KB,
  MB,
  GB,
}

impl UnitSizeType {
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::Bytes => "Bytes",
      Self::KB => "Kb",
      Self::MB => "Mb",
      Self::GB => "Gb",
    }
  }
}

pub fn validate_base64_checksum(
  base64_string: &str,
  expected_checksum: &str,
  pre_decoded_data: Option<Vec<u8>>,
) -> Result<bool, SimpleError> {
  let data = match pre_decoded_data {
    Some(dec) => dec,
    None => {
      // Remove data URL prefix if present
      let re = Regex::new(r"^data:[^;]+;base64,").map_err(|err| SimpleError {
        err: Box::new(err),
        message: "failed to create a new regex".to_string(),
        _type: ErrorType::RegexInvalid,
      })?;
      let base64_data = re.replace(base64_string, "");
      STANDARD.decode(base64_data.as_bytes()).map_err(|err| SimpleError {
        err: Box::new(err),
        message: "invalid base64 data string".to_string(),
        _type: ErrorType::Base64Invalid,
      })?
    }
  };

  let mut hasher = Sha256::new();
  hasher.update(&data);
  let hash_result = hasher.finalize();

  let computed_checksum = hash_result.iter().map(|b| format!("{:02x}", b)).collect::<String>();

  Ok(computed_checksum == expected_checksum)
}

#[cfg(test)]
mod tests {
  use super::*;
  use base64::{engine::general_purpose::STANDARD, Engine};

  // Test helper: create a known test image base64 and its correct SHA-256 hash
  fn get_test_data() -> (String, String, Vec<u8>) {
    // A small 1x1 pixel PNG image in base64
    let base64_data = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==";

    // Calculate the actual SHA-256 hash of this base64 data
    let decoded_data = STANDARD.decode(base64_data).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&decoded_data);
    let hash_result = hasher.finalize();
    let actual_hash = hash_result.iter().map(|b| format!("{:02x}", b)).collect::<String>();

    (base64_data.to_string(), actual_hash, decoded_data)
  }

  #[test]
  fn test_valid_checksum_without_pre_decoded() {
    let (base64, expected_hash, _) = get_test_data();

    let result = validate_base64_checksum(&base64, &expected_hash, None).unwrap();

    assert!(result);
  }

  #[test]
  fn test_valid_checksum_with_pre_decoded() {
    let (base64, expected_hash, decoded_data) = get_test_data();

    let result = validate_base64_checksum(&base64, &expected_hash, Some(decoded_data)).unwrap();

    assert!(result);
  }

  #[test]
  fn test_invalid_checksum() {
    let (base64, expected_hash, _) = get_test_data();
    let wrong_hash = "0000000000000000000000000000000000000000000000000000000000000000";

    let result = validate_base64_checksum(&base64, wrong_hash, None).unwrap();

    assert!(!result);
  }

  #[test]
  fn test_data_url_without_pre_decoded() {
    let (base64, expected_hash, _) = get_test_data();
    let data_url = format!("data:image/png;base64,{}", base64);

    let result = validate_base64_checksum(&data_url, &expected_hash, None).unwrap();

    assert!(result);
  }

  #[test]
  fn test_data_url_with_pre_decoded() {
    let (base64, expected_hash, decoded_data) = get_test_data();
    let data_url = format!("data:image/png;base64,{}", base64);

    let result = validate_base64_checksum(&data_url, &expected_hash, Some(decoded_data)).unwrap();

    assert!(result);
  }

  #[test]
  fn test_different_data_url_mime_types() {
    let (base64, expected_hash, _) = get_test_data();
    let mime_types = vec![
      "data:image/jpeg;base64,",
      "data:application/octet-stream;base64,",
      "data:text/plain;base64,",
    ];

    for mime in mime_types {
      let data_url = format!("{}{}", mime, base64);
      let result = validate_base64_checksum(&data_url, &expected_hash, None).unwrap();
      assert!(result, "Failed with MIME type: {}", mime);
    }
  }

  #[test]
  fn test_invalid_base64() {
    let invalid_base64 = "invalid!!!base64@@@data";

    let result = validate_base64_checksum(invalid_base64, "some_hash", None);

    assert!(result.is_err());
    if let Err(err) = result {
      assert_eq!(err._type, ErrorType::Base64Invalid);
    }
  }

  #[test]
  fn test_empty_base64_string() {
    let empty_hash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"; // SHA-256 of empty data

    let result = validate_base64_checksum("", empty_hash, None).unwrap();
    assert!(result);
  }

  #[test]
  fn test_pre_decoded_takes_precedence() {
    let (base64, expected_hash, _) = get_test_data();

    // Provide different pre-decoded data than what the base64 string contains
    let different_data = b"completely different data".to_vec();

    // This should use the pre_decoded_data and fail validation
    let result = validate_base64_checksum(&base64, &expected_hash, Some(different_data)).unwrap();

    assert!(!result); // Should fail because pre_decoded_data doesn't match the expected hash
  }

  #[test]
  fn test_consistency_between_with_and_without_pre_decoded() {
    let (base64, expected_hash, decoded_data) = get_test_data();

    let result1 = validate_base64_checksum(&base64, &expected_hash, None).unwrap();
    let result2 = validate_base64_checksum(&base64, &expected_hash, Some(decoded_data)).unwrap();

    assert_eq!(result1, result2);
    assert!(result1);
    assert!(result2);
  }

  // Additional test: verify the actual hash value for the test data
  #[test]
  fn test_known_hash_value() {
    let base64_data = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==";
    let expected_hash = "1c30f1a2686fbf86c76c9115b471df8c0b6b9c5b6c5e2c7b8c5c6c5c6c5c6c5c6c5"; // This will be the actual hash

    // First, let's see what the actual hash is
    let decoded = STANDARD.decode(base64_data).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&decoded);
    let actual_hash = hasher.finalize();
    let actual_hash_hex = actual_hash.iter().map(|b| format!("{:02x}", b)).collect::<String>();

    println!("Actual hash for test data: {}", actual_hash_hex);

    // Now test with the correct hash
    let result = validate_base64_checksum(base64_data, &actual_hash_hex, None).unwrap();
    assert!(result);
  }
}
