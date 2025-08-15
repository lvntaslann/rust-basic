enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x: i64, y: i64},
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn inspect(event: WebEvent){
    match event {
        WebEvent:: PageLoad => println!("page loaded"),
        WebEvent:: PageUnload => println!("page unloaded"),
        WebEvent:: KeyPress(c) => println!("pressed '{}'.",c),
        WebEvent:: Paste(s) => println!("pressed '{}'.",s),

        WebEvent:: Click {x,y} => {
            println!("clicked at x={}, y = {}",x,y)
        }
    }

}

pub fn firstenum(){
    let pressed = WebEvent:: KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click{x:20 , y:80};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let x = Operations::Add;
    let result = x.run(20,30);
    println!("Result : {}",result);
}
