
use jsonrpsee::{proc_macros::rpc};
use jsonrpsee::core::async_trait;

#[rpc(server, namespace = "supplier")]

pub trait Rpc{

    #[method(name="say_hello")]
    async fn hello(&self, name: String) -> String;

    #[method(name="name")]
    async fn name(&self) -> String;

    #[method(name="project")]
    async fn project(&self) -> String;
}

struct  RpcIml;

#[async_trait]
impl RpcServer for RpcIml {
    async fn hello(&self, name: String) -> String {
        todo!()
    }

    async fn name(&self) -> String {
        todo!()
    }

    async fn project(&self) -> String {
        todo!()
    }
}


// Given a struct representing a person, implement a function `increase_age` that takes a mutable reference
// to a person and increases their age by 1. 
// The function should return a Result<(), String> where the String
// contains an error message in case the age is already at the maximum allowed value (let's say 100).
// Call the function

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn increase_age(mut person: Person) -> Result<(), String> {
    let message = "error";
        if person.age >= 100 {
            return Err("Age is already at or above 100".to_string());
        }

        let mut age: u32 = person.age;
        age += 1;
        person.age = age;
        Ok(())

    }
    //Complete code
}

fn main() {
    let mut person = Person {
        name: String::from("John"),
        age: 30,
    };

    println!("Before: {} is {} years old.", person.name, person.age);
    let results = match Person::increase_age(person) {
        Ok(()) => {
            println!("Ok");
        },
        Err(s) => {
            println!("{}", s);
        }

    };


    //Call function here and handle the message and error
}
