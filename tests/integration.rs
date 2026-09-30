//! End-to-end integration tests: exercise the public API exactly as an
//! external consumer would, through the compiled crate.

use e2e_rust::{fizzbuzz, word_counts};

#[test]
fn fizzbuzz_full_sequence() {
    let expected = [
        "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14",
        "FizzBuzz",
    ];
    let actual: Vec<String> = (1..=15).map(fizzbuzz).collect();
    assert_eq!(actual, expected);
}

#[test]
fn word_counts_empty_input() {
    assert!(word_counts("").is_empty());
    assert!(word_counts("  ,, ;; ").is_empty());
}

#[test]
fn binary_runs_end_to_end() {
    let exe = env!("CARGO_BIN_EXE_e2e_rust");
    let output = std::process::Command::new(exe)
        .output()
        .expect("binary should run");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("15 -> FizzBuzz"));
    assert!(stdout.contains("rust: 3"));
}
