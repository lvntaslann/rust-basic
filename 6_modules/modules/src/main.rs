mod func_visibility;
mod struct_visibility;
mod super_and_self;
mod my;
fn function() {
    println!("called 'function()'");
}

fn main() {
    // func visibility
    println!("\n-- function visibility example --");
    func_visibility::my_mod::function();
    func_visibility::my_mod::indirect_access();
    func_visibility::my_mod::nested::function(); 
    func_visibility::my_mod::call_public_function_in_my_mod();
    func_visibility::my_mod::public_function_in_crate();
    function();

    // struct visbility
    println!("\n-- struct visibility example --");
    let open_box = struct_visibility::my_struct::OpenBox{contents: "public information"};

    println!("The open box contains : {}",open_box.contents);

    // contents private olduğu için hatalı kullanım
    //let closed_box = my::ClosedBox { contents: "classified information" };

    // struct public ondan referans oluşturabiliriz ama contents private olduğu için print kısmı hatalaı olacaktır
    let _closed_box = struct_visibility::my_struct::ClosedBox::new("Classified information");
    //println!("The closed box contains: {}", _closed_box.contents);

    super_and_self::my::indirect_call();
    println!("\n-- split mod example --");

    my::function();
    my::indirect_access();
    my::nested::function();
}
