use e2e_rust::{fizzbuzz, word_counts};

fn main() {
    println!("FizzBuzz 1..=15:");
    for n in 1..=15 {
        println!("  {n:>2} -> {}", fizzbuzz(n));
    }

    let text = "Rust is fast, Rust is safe, and Rust is fun.";
    let counts = word_counts(text);
    println!("\nWord counts for {text:?}:");
    let mut entries: Vec<_> = counts.into_iter().collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    for (word, count) in entries {
        println!("  {word}: {count}");
    }
}
