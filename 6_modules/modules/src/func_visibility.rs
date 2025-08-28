// pub -> public
// belirtilmezde default olarak privatetır
// pub(crate) aynı crate içerisinden erişilebilir mainde kullanılabilir
// pub(self) sadece aynı modül içerisindekiler erişebilir mainde kullanılamaz
// pub(super) aynı modülde üst modülden erişilebilir mainde kullanılabilir

pub mod my_mod {
    fn private_function() {
        println!("called 'my_mod::private_function()'");
    }


    pub fn function() {
        println!("called 'my_mod::function()'");
    }

    pub fn indirect_access() {
        print!("called 'my_mod::indirect_access()', that\n> ");
        private_function();
    }

    pub mod nested{
        pub fn function(){
            println!("called 'my_mod::nested::function()'");
        }
        #[allow(dead_code)]
        fn private_function() {
            println!("called 'my_mod::nested::private_function()'");
        }

        pub(in crate::func_visibility::my_mod) fn public_function_in_my_mod() {
            println!("called 'my_mod::nested::public_function_in_my_mod()'");
        }

        pub(super) fn public_function_in_super_mod() {
            println!("called 'my_mod::nested::public_function_in_super_mod()'");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called 'my_mod::call_public_function_in_my_mod()', that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    pub(crate) fn public_function_in_crate(){
        println!("called 'my_mod::public_function_in_crate()'");
    }

    mod private_nested{
        #[allow(dead_code)]
        pub fn function(){
            println!("called 'my_mod::private_nested::function()'");
        }
        #[allow(dead_code)]
        pub(crate) fn restricted_function(){
            println!("called 'my_mod::private_nested::restricted_function()'");
        }
    }
}
