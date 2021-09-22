// vars immutable by default
// Rust is block-scoped

pub fn run() {
    let name = "Chad";
    let mut age = 12;
    println!("My name is {} and I am {}", name, age);

    age = 13;
    println!("My name is {} and I am {}", name, age);

    // constants
    const ID: i32 = 001;
    println!("ID is {}", ID);

    // multiple vars declared
    let (my_name, my_age) = ("Brad", 15);
    println!("My name is {} and I am {}", my_name, my_age);

}