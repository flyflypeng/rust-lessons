use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

#[derive(Debug, Clone, Default)]
struct Developer {
    name: String,
    age: u8,
    language: Language,
}

impl Default for Language {
    fn default() -> Self {
        Language::Rust
    }
}

impl Developer {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

impl fmt::Display for Developer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({} years old): {:?} Developer",
            self.name, self.age, self.language
        )
    }
}

fn main() {
    let dev1 = Developer::new("JPF");
    let dev2 = Developer::default();
    let dev3: Developer = Default::default();

    println!("dev1: {}\ndev2: {}\ndev3: {}", dev1, dev2, dev3)
}
