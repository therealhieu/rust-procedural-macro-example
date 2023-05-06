mod use_proc_macro {
    use derive::add_greet;

    add_greet!(
        struct Person {
            name: String,
            age: u32,
        }
    );

    pub fn main() {
        let hieu = Person {
            name: "Hieu".to_string(),
            age: 30,
        };

        hieu.greet();
    }
}

mod use_derive_macro {
    use derive::Greet;

    #[derive(Greet)]
    struct PerSon {
        name: String,
        age: u32,
    }

    pub fn main() {
        let hieu = PerSon {
            name: "Hieu".to_string(),
            age: 30,
        };

        hieu.greet();
    }
}

mod use_attribute_macro {
    use derive::greet;

    #[greet(content = "Hello, my name is {name}  and I a {age} years old.")]
    struct Person {
        name: String,
        age: u32,
    }

    pub fn main() {
        let hieu = Person {
            name: "Hieu".to_string(),
            age: 30,
        };

        hieu.greet();
    }
}

mod use_derive_macro_with_attributes {
    use derive::Greet2;

    #[derive(Greet2)]
    #[greet2(content = "Hello, my name is {name}  and I a {age} years old.")]
    struct Person {
        name: String,
        age: u32,
    }

    pub fn main() {
        let hieu = Person {
            name: "Hieu".to_string(),
            age: 30,
        };

        hieu.greet();
    }
}

fn main() {
    use_proc_macro::main();
    use_derive_macro::main();
    use_attribute_macro::main();
    use_derive_macro_with_attributes::main();
}
