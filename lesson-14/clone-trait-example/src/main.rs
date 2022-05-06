use std::os::unix::raw::dev_t;

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

#[derive(Clone, Debug)]
struct Developer {
    name: String,
    age: u8,
    language: Language,
}

fn main() {
    let dev = Developer {
        name: "Tyr".to_string(),
        age: 18,
        language: Language::Rust,
    };
    let dev1 = dev.clone();

    println!(
        "dev: {:?}, addr of dev: {:p}, addr of age: {:p}, dev addr of dev name: {:p}",
        dev,
        &dev,
        &(dev.age),
        dev.name.as_str()
    );
    println!(
        "dev1: {:?}, addr of dev1: {:p}, addr of dev name: {:p}",
        dev1,
        &dev1,
        dev1.name.as_str()
    );
}
