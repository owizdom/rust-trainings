#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    // =========================
    // ASSOCIATED FUNCTION
    // =========================
    // Called on the type itself: Person::new(...)
    fn new(name: &str, age: u8) -> Person {
        Person {
            name: name.to_string(),
            age,
        }
    }

    // =========================
    // METHOD (borrows self)
    // =========================
    // Called on an instance: person.greet()
    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }

    // =========================
    // METHOD (mutable self)
    // =========================
    fn birthday(&mut self) {
        self.age += 1;
    }
}

fn main() {
    // Associated function (like a constructor)
    let mut person = Person::new("James", 22);

    // Method using &self
    person.greet();

    // Method using &mut self
    person.birthday();

    println!("{:?}", person);
}
