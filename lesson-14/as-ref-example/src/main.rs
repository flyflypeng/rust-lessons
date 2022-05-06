#[allow(dead_code)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Language::Rust => "Rust",
            Language::TypeScript => "TypeScript",
            Language::Elixir => "Elixir",
            Language::Haskell => "Haskell",
        }
    }
}

fn print_ref(v: impl AsRef<str>) {
    println!("{}", v.as_ref())
}

fn main() {
    let lang = Language::Rust;

    print_ref("Hello, world!");
    print_ref("Hello, world!".to_string());
    print_ref(lang);
}
