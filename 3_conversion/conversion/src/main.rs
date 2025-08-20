use std::convert::From;
use std::convert::Into;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number{
    //number structi implement edilerek içerisine from fonksiyonu eklendi
    fn from(item:i32)-> Self{
        Number {value:item}
    }
}


#[derive(Debug,PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber{
    type Error = ();

    fn try_from(value:i32) -> Result<Self, Self::Error>{
        if value % 2 == 0{
            Ok(EvenNumber(value))
        }else {
            Err(())
        }
    }
}


fn main() {
   //easy convert str into a string
   // let my_str = "hello";
   // let my_string = String::from(my_str);
   // println!("{}",my_string);

   //let num = Number::from(30);
   // From tanımlandığında into otomatik olarak oluşur
   let int = 5;
   let num: Number = int.into();
  // println!("My number is {:?}",num);

   //TryFrom
   assert_eq!(EvenNumber::try_from(8),Ok(EvenNumber(8)));
   assert_eq!(EvenNumber::try_from(5),Err(()));

   //TryInto
   let result: Result<EvenNumber, ()> = 8i32.try_into();
   assert_eq!(result,Ok(EvenNumber(8)));
   println!("8.try_into() = {:?}", result);

   let result: Result<EvenNumber, ()> = 5i32.try_into();
   assert_eq!(result, Err(()));
   println!("5.try_into() = {:?}", result);
   
   //parsing string
   let parsed : i32 = "5".parse().unwrap(); //Ok(5) -> 5 eğer sayı dışında bir string varsa Err->panic! hatası
   let turbo_parsed = "10".parse::<i32>().unwrap();
   let sum = parsed + turbo_parsed;
   println!("Sum: {:?}",sum);


}
