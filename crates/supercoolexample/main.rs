use derivesupercoolgetter::SuperCoolGetter;

#[derive(SuperCoolGetter)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let karasawa = Person {
        name: String::from("Karasawa"),
        age: 47,
    };

    let Some(PersonRef::Name(name)) = karasawa.get("name") else {
        unreachable!()
    };
    let age = *karasawa.get_as::<u32>("age").unwrap();

    println!("name: {}, age: {}", name, age);
}
