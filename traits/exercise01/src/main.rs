trait Greet {
    fn greet(&self) -> String {
        String::from("Hi there")
    }
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, my name is {}!", self.name)
    }
}

struct Dog {
    name: String,
}

impl Greet for Dog {
    fn greet(&self) -> String{
        format!("{} is a good dog!", self.name)
    }
}

struct Thing {
}

impl Greet for Thing{}

fn main() {
    let p = Person { name: String::from("Peter") };
    let d = Dog { name: String::from("Fido") };
    let t = Thing {};
    println!("{}", p.greet());
    println!("{}", d.greet());
    println!("{}", t.greet());
}
