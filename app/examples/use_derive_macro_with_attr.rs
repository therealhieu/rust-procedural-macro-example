use derive::Greet2;

#[derive(Greet2)]
#[greet2(content = "Hello, my name is {name}  and I a {age} years old.")]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let hieu = Person {
        name: "Hieu".to_string(),
        age: 24,
    };

    hieu.greet();
}
