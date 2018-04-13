# Easy Color for Rust

当你在控制台需要输出时，可以方便的调整字体样式和颜色

## 使用方法


    extern crate easy_color;
    use easy_color::*;
    
    fn main() {
        println!("{}", "Hello World!".green().red().on_blue().bold().underline().on_yellow().italic());
    }

颜色，用```red()```：

    println!("{}", "Hello World!".red())

背景用```on_red()```：

    println!("{}", "Hello World!".on_red())

样式用```bold()```：

    println!("{}", "Hello World!".bold())

## 参考crate
- [Colored](https://github.com/mackwic/colored)