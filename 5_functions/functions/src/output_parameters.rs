fn create_fn() -> impl Fn(){
    let text = "Fn".to_owned();
    move || println!("{}", text)
}

fn create_fnmut() -> impl FnMut(){
    let mut text = "FnMut".to_owned();
    move || {
        text.push_str("!");
        println!("{}", text)
    }
}

fn create_fnonce() -> impl FnOnce(){
    let text = "FnOnce".to_owned();
    move || println!("{}", text)
}

pub fn output_parameters(){
    let fn_once = create_fnonce();
    let mut fn_mut = create_fnmut();
    let fn_plain = create_fn();
    fn_once();
    fn_mut();
    fn_plain();
}