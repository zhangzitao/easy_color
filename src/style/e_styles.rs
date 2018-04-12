#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum Styles {
    Clear,
    Bold,
    Dimmed,
    Underline,
    Reversed,
    Italic,
    Blink,
    Hidden,
    Strikethrough,
}


impl Styles {

    pub fn to_u8(&self) -> u8 {
        match *self {
            Styles::Clear => 0,
            Styles::Bold => 1,
            Styles::Dimmed => 2,
            Styles::Italic => 3,
            Styles::Underline => 4,
            Styles::Blink => 5,
            Styles::Reversed => 7,
            Styles::Hidden => 8,
            Styles::Strikethrough => 9,
        }
    }

}