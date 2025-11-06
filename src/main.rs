// src/main.rs

// ---------- Notes ----------
// rustc = rust compiler (pronounced "rustic")
// cargo = rust project manager

// cargo build <- compiles project with rustc


mod basics; // module declaration (~= namespace declaration)

fn main() {
    println!("Hello, Rust!\n");

    basics::formatted_printing();
    basics::primitives();
}
