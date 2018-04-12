pub trait Colorize {
  fn red(self) -> super::ColoredString;
  fn black(self) -> super::ColoredString;
  fn green(self) -> super::ColoredString;
  fn yellow(self) -> super::ColoredString;
  fn blue(self) -> super::ColoredString;

  fn on_red(self) -> super::ColoredString;
  fn on_black(self) -> super::ColoredString;
  fn on_green(self) -> super::ColoredString;
  fn on_yellow(self) -> super::ColoredString;
  fn on_blue(self) -> super::ColoredString;

  fn bold(self) -> super::ColoredString;
  fn clear(self) -> super::ColoredString;
  fn dimmed(self) -> super::ColoredString;
  fn underline(self) -> super::ColoredString;
  fn reversed(self) -> super::ColoredString;
  fn italic(self) -> super::ColoredString;
  fn blink(self) -> super::ColoredString;
  fn hidden(self) -> super::ColoredString;
  fn strikethrough(self) -> super::ColoredString;
}