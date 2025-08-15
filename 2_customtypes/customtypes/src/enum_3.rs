#![allow(dead_code)]

enum Number{
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0xff00ff00,
    Blue = 0x0000ff,
}

pub fn thirdenum(){
    let number1 = Number:: Zero as i32;
    let number2 = Number:: One as i32;

    let color1 = Color:: Red as i32;
    let color2 = Color:: Blue as i32;
    println!("Zero is {}",number1);
    println!("Zero is {}",number2);
    println!("roses are #{:06x}",color1);
    println!("violets are #{:06x}",color2);
}