#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,

    RGB(u32,u32,u32),
    HSV(u32,u32,u32),
    HSL(u32,u32,u32),
    CMY(u32,u32,u32),
    CMYK(u32,u32,u32,u32),
}
pub fn match_ex(){
    let number = 13;

    println!("Tell me about {}",number);

    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        13..=19 => println!("Ain't teen"),
        _=> println!("Ain't special"),
    }

    let boolean = true;

    let binary = match boolean{
        false =>0,
        true => 1,
    };

    println!("{} -> {}",boolean,binary);
}

pub fn match_tupple(){
    let triple = (0,-2,3);
    let triple2 = (3,2,4);
    println!("Tell me about {:?}",triple2);
    match triple2 {
        (0,y,z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1,..)  => println!("First is `1` and the rest doesn't matter"),
        (..,2)  => println!("last is `2` and the rest doesn't matter"),
        (3,..,4)=> println!("First is `3`, last is `4`, and the rest doesn't matter"),
        _       => println!("It doesn't matter what they are"),
    }
}

pub fn match_array_slices(){
    let array = [1,-2,6];

    match array {
        [0,second,third] => println!("array[0] = -1, array[1] = {} and all the other ones were ignored",second),
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}

pub fn match_enum(){
    let color = Color::RGB(122,17,40);
    let color2 = Color::Red;

    println!("What color is it ?");

    match color2 {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
    }
}
