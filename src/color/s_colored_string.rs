use color::*;
use style::*;
use std::fmt;


#[derive(Debug, Clone)]
pub struct ColoredString {
    input: String,
    fgcolor: Option<Color>,
    bgcolor: Option<Color>,
    style: Style,
}

impl ColoredString {
    fn reder_string(&self) -> String {
        if self.input.is_empty() {
            return String::new();
        }
        let mut st = String::from("\x1B[");
        
        st.push_str(self.style.to_str().as_str());

        let has_write = if self.fgcolor.is_some() {
            st.push_str(self.fgcolor.unwrap().to_fg_str());
            true
        } else {
            false
        };
        
        
        if self.bgcolor.is_some() {
            if has_write { st.push(';'); };
            st.push_str(self.bgcolor.unwrap().to_bg_str());
            true
        } else {
            false
        };

        st.push('m');

        // println!("debug:{}", st.split_at(0).1); // Debug
        // println!("debug:{}", st.split_at(1).1); // Debug

        st.push_str(self.input.as_str());
        
        st.push_str("\x1B[0m");
        st
    }
}

impl Default for ColoredString {
    fn default() -> Self {
        ColoredString {
            input: String::default(),
            fgcolor: None,
            bgcolor: None,
            style: Style::clear(),
        }
    }
}

impl fmt::Display for ColoredString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reder_string())
    }
}

macro_rules! def_str_color {
    ($side:ident: $name: ident => $color: path) => {
        fn $name(self) -> ColoredString {
            ColoredString {
                input: String::from(self),
                $side: Some($color),
                .. ColoredString::default()
            }
        }
    }
}

macro_rules! def_str_style {
    ($name:ident, $style:path) => {
        fn $name(self) -> ColoredString {
            ColoredString {
                input: String::from(self),
                style: Style::new($style),
                .. ColoredString::default()
            }
        }
    }
}

// we should put operator of ColoredString here, since Struct private field cant be modified in a different mod
impl<'a> Colorize for &'a str {

    fn red(self) -> ColoredString {
        ColoredString {
            input: String::from(self),
            fgcolor: Some(Color::Red),
            .. ColoredString::default()
        }
    }
    
    def_str_color!(fgcolor: black => Color::Black);
    def_str_color!(fgcolor: yellow => Color::Yellow);
    def_str_color!(fgcolor: blue => Color::Blue);
    def_str_color!(fgcolor: green => Color::Green);
    
    fn on_red(self) -> ColoredString {
        ColoredString {
            input: String::from(self),
            bgcolor: Some(Color::Red),
            .. ColoredString::default()
        }
    }

    def_str_color!(bgcolor: on_black => Color::Black);
    def_str_color!(bgcolor: on_yellow => Color::Yellow);
    def_str_color!(bgcolor: on_blue => Color::Blue);
    def_str_color!(bgcolor: on_green => Color::Green);

    fn bold(self) -> ColoredString {
        ColoredString {
            input: String::from(self),
            style: Style::new(Styles::Bold),
            .. ColoredString::default()
        }
    }

    def_str_style!(clear, Styles::Clear);
    def_str_style!(dimmed, Styles::Dimmed);
    def_str_style!(underline, Styles::Underline);
    def_str_style!(reversed, Styles::Reversed);
    def_str_style!(italic, Styles::Italic);
    def_str_style!(blink, Styles::Blink);
    def_str_style!(hidden, Styles::Hidden);
    def_str_style!(strikethrough, Styles::Strikethrough);

}


macro_rules! def_color {
    ($side:ident: $name: ident => $color: path) => {
        fn $name(self) -> Self {
            ColoredString {
                $side: Some($color),
                .. self
            }
        }
    }
}

macro_rules! def_style {
    ($name:ident, $style:path) => {
        fn $name(self) -> Self {
            ColoredString {
                style: Style::glue(self.style, $style),
                .. self
            }
        }
    }
}

impl<'a> Colorize for ColoredString {

    fn red(self) -> Self {
        ColoredString {
            fgcolor: Some(Color::Red),
            .. self
        }
    }

    def_color!(fgcolor: black => Color::Black);
    def_color!(fgcolor: yellow => Color::Yellow);
    def_color!(fgcolor: blue => Color::Blue);
    def_color!(fgcolor: green => Color::Green);

    fn on_red(self) -> Self {
        ColoredString {
            bgcolor: Some(Color::Red),
            .. self
        }
    }

    def_color!(bgcolor: on_black => Color::Black);
    def_color!(bgcolor: on_yellow => Color::Yellow);
    def_color!(bgcolor: on_blue => Color::Blue);
    def_color!(bgcolor: on_green => Color::Green);

    fn bold(self) -> Self {
        ColoredString {
            style: Style::glue(self.style, Styles::Bold),
            .. self
        }
    }

    def_style!(clear, Styles::Clear);
    def_style!(dimmed, Styles::Dimmed);
    def_style!(underline, Styles::Underline);
    def_style!(reversed, Styles::Reversed);
    def_style!(italic, Styles::Italic);
    def_style!(blink, Styles::Blink);
    def_style!(hidden, Styles::Hidden);
    def_style!(strikethrough, Styles::Strikethrough);

}