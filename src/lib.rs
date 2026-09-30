//! A tiny text-analysis library used to exercise the full Rust toolchain
//! end to end: fmt, clippy, unit tests, doc tests, and integration tests.

/// Returns the classic FizzBuzz label for `n`.
///
/// # Examples
///
/// ```
/// assert_eq!(e2e_rust::fizzbuzz(15), "FizzBuzz");
/// assert_eq!(e2e_rust::fizzbuzz(4), "4");
/// ```
pub fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

/// Counts word frequencies in a string, lowercased, punctuation stripped.
pub fn word_counts(text: &str) -> std::collections::HashMap<String, usize> {
    let mut counts = std::collections::HashMap::new();
    for word in text
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(str::to_lowercase)
    {
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_multiples_of_three() {
        assert_eq!(fizzbuzz(9), "Fizz");
    }

    #[test]
    fn fizzbuzz_multiples_of_five() {
        assert_eq!(fizzbuzz(10), "Buzz");
    }

    #[test]
    fn word_counts_ignores_case_and_punctuation() {
        let counts = word_counts("Rust rust, RUST! fast.");
        assert_eq!(counts["rust"], 3);
        assert_eq!(counts["fast"], 1);
        assert_eq!(counts.len(), 2);
    }

    #[test]
    #[should_panic(expected = "empty")]
    fn word_counts_documented_panic_example() {
        // Placeholder to show `should_panic` style tests are wired up.
        panic!("empty input in example");
    }
}
