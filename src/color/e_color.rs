#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
}


impl Color {
  
  pub fn to_fg_str(&self) -> &str {
    match *self {
      Color::Black => "30",
      Color::Red => "31",
      Color::Green => "32",
      Color::Yellow => "33",
      Color::Blue => "34",
    }
  }

  pub fn to_bg_str(&self) -> &str {
    match *self {
      Color::Black => "40",
      Color::Red => "41",
      Color::Green => "42",
      Color::Yellow => "43",
      Color::Blue => "44",
    }
  }

}