#[derive(Debug)]
pub enum SubcategoryAttributeType {
  Input,
  Select,
  Boolean,
  Unknown,
}

impl SubcategoryAttributeType {
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::Input => "input",
      Self::Select => "select",
      Self::Boolean => "boolean",
      Self::Unknown => "unknown",
    }
  }

  pub fn from_str(typ: &str) -> Self {
    match typ {
      "input" => SubcategoryAttributeType::Input,
      "select" => SubcategoryAttributeType::Select,
      "boolean" => SubcategoryAttributeType::Boolean,
      _ => SubcategoryAttributeType::Unknown,
    }
  }
}
