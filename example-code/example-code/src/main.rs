use serde::{Deserialize, Serialize};

fn to_letter_grade(num:u8)->String{
    match num{
        0..=59 => "F".to_string(),
        60..=69 => "D".to_string(),
        70..=79 => "C".to_string(),
        80..=89 => "B".to_string(),
        90..=100 => "A".to_string(),
        _ => "Invalid grade".to_string(),
    }
}

#[derive(Serialize, Deserialize, Debug)] 
struct Person{
    name:String,
    age:u8,
    gender: Gender,
    grade: u8,
}

#[derive(Debug, Serialize, Deserialize)]
enum Gender {
    Male,
    Female,
    Other,
}

trait PersonInfo {
    fn get_info(&self) -> String;
}


impl PersonInfo for Person {
    fn get_info(&self) -> String {
        format!("Name: {}, Age: {}, Gender: {}", self.name, self.age, match self.gender {
            Gender::Male => "Male",
            Gender::Female => "Female",
            Gender::Other => "Other",
        })
    }
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        gender: Gender::Female,
        grade:85,
    };
    println!("{}", person.get_info());
    println!("Letter Grade: {}", to_letter_grade(person.grade));

    let serialized = serde_json::to_string(&person).unwrap();
    println!("Serialized: {}", serialized);

    // save json write fonksiyonu yol ve içerik alır
    std::fs::write("person.json", serialized).unwrap();

    // read json read_to_string fonksiyonu yol alır
    let json_data = std::fs::read_to_string("person.json").unwrap();
    let deserialized: Person = serde_json::from_str(&json_data).unwrap();
    println!("Deserialized from file: {}", deserialized.get_info());
}
