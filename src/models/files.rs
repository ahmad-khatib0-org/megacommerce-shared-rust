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
