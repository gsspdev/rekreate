use serde::{Serialize, Deserialize};
use serde_json;

fn main() {
    #[derive(Serialize, Deserialize)]
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
        weight: f32,
    }

    let john = Person {
        name: String::from("John"),
        age: 30,
        weight: 165.4,
    };
    // let person_json = serde_json::to_string(&john).unwrap();
    // serde_json::to_writer(&std::io::stdout(), &john).unwrap();
    // println!("{:?}", serde::Deserialize(&john).unwrap());
    println!("{:?}", &john);
}