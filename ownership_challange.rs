//An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    city: String,
    born: String;
}


fn main() {
    
    let name = String::from("Hasan Demirdoven");
    let age: i32 = 30;
    let city = String::from("Izmir");

    //let born = city;
    // We cannot use the "city" value because the ownership of "city" has passed to born.
    
    //To prevent this, we could set the "born" variable as city.clone(),
    //but this is not recommended as it will use additional space in the heap.
    
    //So we borrow the value of the variable "city" as follows (transfer of ownership).
    let born = &city;
    
    let person = Person { name, age, city };

    println!("{:?}", person);
}

