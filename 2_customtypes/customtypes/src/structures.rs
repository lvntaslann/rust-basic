#![allow(dead_code)] // kullanılmayan fonkların uyarılarını kapatmak için

#[derive(Debug)] // struct ve enum veri tipindeki içerikleri {:?} veya {:#?} formatında terminalde yazdırmak için
struct Person{
    name: String,
    age:u8,
}

struct Unit;

struct Pair(i32,f32);

#[derive(Debug)]
struct Point{
    x:f32,
    y:f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right:Point,
}


pub fn my_structures(){
    let name = String::from("Peter");
    let age = 27;
    let peter = Person{name,age};

    println!{"{:?}",peter};

    let point: Point = Point{x:5.2, y:0.4};
    let another_point: Point = Point{x:10.3,y:0.2};

    println!("point cordinates: ({},{})",point.x,point.y);

    let bottom_right: Point = Point { x: 10.3, ..another_point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point {x:left_edge , y:top_edge} = point;

    let _rectangle = Rectangle{
        top_left : Point{x:left_edge,y:top_edge},
        bottom_right: bottom_right,
    };
    println!("{:#?}",_rectangle);
    let _unit = Unit;

    let pair = Pair(1,0.1);

    println!("pair contains {:?} and {:?}",pair.0,pair.1);

    let Pair(integer,decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

}