use style::*;
#[derive(Debug, Clone)]
pub struct Style(Vec<u8>);


impl Style {

    pub fn clear() -> Style {
        Style(vec![0u8])
    }

    pub fn new(s: Styles) -> Style {
        Style(vec![s.to_u8()])
    }

    pub fn glue(st: Style,s: Styles) -> Style {
        let mut v = st.0;
        v.push(s.to_u8());
        Style(v)
    }

    pub fn to_str(&self) -> String {
        let s = self.0
            .iter()
            .fold(String::new(), |acc, num| format!("{}{};", acc, num));
        // print!("{:?}", s);
        s
    }
}