#[derive(Debug)]
pub enum SubcategoryAttributeType {
  Input,
  Select,
  Boolean,
  Unknown,
}

pub fn subcategory_attribute_type(typ: &str) -> SubcategoryAttributeType {
  match typ {
    "input" => SubcategoryAttributeType::Input,
    "select" => SubcategoryAttributeType::Select,
    "boolean" => SubcategoryAttributeType::Boolean,
    _ => SubcategoryAttributeType::Unknown,
  }
}
