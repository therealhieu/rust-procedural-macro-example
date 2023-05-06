use derive::add_greet;

add_greet!(
    struct Person {
        name: String,
        age: u32,
    }
);

fn main() {
    let hieu = Person {
        name: "Hieu".to_string(),
        age: 24,
    };

    hieu.greet();
}
