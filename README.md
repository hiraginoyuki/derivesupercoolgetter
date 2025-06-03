```rs
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
```
becomes
```rs
struct Person {
    name: String,
    age: u32,
}
pub enum PersonRef<'a> {
    Name(&'a String),
    Age(&'a u32),
}
pub enum PersonMut<'a> {
    Name(&'a mut String),
    Age(&'a mut u32),
}
impl<'a> SuperCoolGetter<'a> for Person {
    type FieldRef = PersonRef<'a>;
    type FieldMut = PersonMut<'a>;
    fn get(&'a self, field_name: &str) -> Option<Self::FieldRef> {
        match field_name {
            "name" => ::core::option::Option::Some(PersonRef::Name(&self.name)),
            "age" => ::core::option::Option::Some(PersonRef::Age(&self.age)),
            _ => None,
        }
    }
    fn get_mut(&'a mut self, field_name: &str) -> Option<Self::FieldMut> {
        match field_name {
            "name" => ::core::option::Option::Some(PersonMut::Name(&mut self.name)),
            "age" => ::core::option::Option::Some(PersonMut::Age(&mut self.age)),
            _ => None,
        }
    }
    fn get_as<T: 'static>(&'a self, field_name: &str) -> Option<&'a T> {
        match field_name {
            "name" => (&self.name as &dyn ::core::any::Any).downcast_ref(),
            "age" => (&self.age as &dyn ::core::any::Any).downcast_ref(),
            _ => None,
        }
    }
    fn get_mut_as<T: 'static>(&'a mut self, field_name: &str) -> Option<&'a mut T> {
        match field_name {
            "name" => (&mut self.name as &mut dyn ::core::any::Any).downcast_mut(),
            "age" => (&mut self.age as &mut dyn ::core::any::Any).downcast_mut(),
            _ => None,
        }
    }
}
```

```sh
cargo expand -p supercoolexample
cargo run
```
