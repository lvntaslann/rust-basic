fn call_me<F: Fn()>(f:F){
    f();
}

fn function_print(){
    println!("I am a function!");
}

pub fn input_function(){
    call_me(function_print);
    let closure = || println!("I am a closure!");
    call_me(closure);
}