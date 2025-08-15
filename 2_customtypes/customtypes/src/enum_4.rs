enum List {
    Cons(u32, Box<List>), // eleman ve sonraki node, enumlarda bellek alanı sabit olması gerektiği için box ile heape alınıyor
    Nil, // listenin sonunu belirtiyor
}

impl List {
    fn new() -> List {
        // boş bir liste oluşturma
        List::Nil
    }

    fn prepend(self, elem: u32) -> List {
        // baştan eleman eklemek için
        List::Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            // ref burada match ile dönülen içeriği belirtiyor ona owernship yapmadan ulaşmamızı sağlıyor
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            // head listenin başını belirtiyor
            List::Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            List::Nil => "Nil".to_string(),
        }
    }
}

pub fn fourthenum() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
