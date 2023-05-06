use derive::Greet;

#[derive(Greet)]
struct PerSon {
    name: String,
    age: u32,
}

fn main() {
    let hieu = PerSon {
        name: "Hieu".to_string(),
        age: 24,
    };

    hieu.greet();
}
