extern crate easy_color;
use easy_color::*;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
    println!("{}", "sssss66666666".green().red().on_blue().bold().underline().on_yellow().italic());
}