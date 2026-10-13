// Assignment 1 — Ownership, Borrowing, Strings
// Solved: (b), (f), (g), (h), (i) + the 5 string functions = 10 tasks.
// Task (a) was skipped on purpose.
//
// Run:   rustc assignment01.rs && ./assignment01
// Test:  rustc --test assignment01.rs && ./assignment01

// Part 1 — Fix the borrow checker
// (b) E0502: cannot borrow as mutable because it is also borrowed as immutable
//
// FIX: `first` is a reference into the vector's heap buffer. `push` may
// reallocate that buffer, which would leave `first` dangling, so the compiler
// forbids mutating `v` while `first` is still alive (used later).
// We simply mutate first and only then take the reference, so the shared
// borrow no longer overlaps the mutable one. No clone needed.
#![allow(dead_code)]
fn fix_b() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    let first = &v[0];
    println!("(b) first = {first}");
}
// (f) E0505: cannot move out of `name` because it is borrowed
//
// FIX: `shout` took ownership of the String, but `r` still borrowed `name`
// and was used afterwards. The function only reads the text, so it should
// borrow (`&str`) instead of consuming its argument. Now `name` is never
// moved and `r` stays valid.
fn shout(s: &str) -> String {
    s.to_uppercase()
}
fn fix_f() {
    let name = String::from("ada");
    let r = &name;
    let loud = shout(&name);
    println!("(f) {r} {loud}");
}
// (g) E0502: cannot borrow `scores` as mutable because it is also borrowed
//     as immutable
// FIX: the `for s in &scores` loop holds a shared borrow of `scores` for its
// whole duration, so pushing inside it is not allowed (push could reallocate
// the buffer the iterator is walking). Instead, first collect the values to
// add (the shared borrow ends after `collect`), then mutate afterwards.
fn fix_g() {
    let mut scores = vec![10, 20, 30];
    let extra: Vec<i32> = scores.iter().copied().filter(|&s| s > 15).collect();
    scores.extend(extra);
    println!("(g) {scores:?}");
}
// (h) E0499: cannot borrow `c` as mutable more than once at a time
//
// FIX: `a` and `b` were two live `&mut` borrows of the same value, which
// would allow aliasing + mutation. `bump` returns an i32 (Copy), so we don't
// need to keep the references alive: call the method one after another, each
// with a short-lived mutable borrow that ends before the next begins.
struct Counter {
    value: i32,
}
impl Counter {
    fn bump(&mut self) -> i32 {
        self.value += 1;
        self.value
    }
}
fn fix_h() {
    let mut c = Counter { value: 0 };
    let a = c.bump();
    let b = c.bump();
    println!("(h) {a} {b}");
}
// (i) E0515: cannot return value referencing local variable
//
// FIX: `w` is dropped at the end of the function, so a `&'static str`
// pointing into it would dangle. A reference can't outlive the data it
// points to, so we return an OWNED value instead: the String is moved out of
// `w` (no copy, no clone) and ownership goes to the caller.
struct Wrapper {
    value: String,
}
#[allow(dead_code)]
fn unwrap_text(w: &Wrapper) -> &str {
    &w.value
}
fn fixed_unwrap() -> String {
    let w = Wrapper {
        value: String::from("hi"),
    };
    w.value
}
fn fix_i() {
    println!("(i) {}", fixed_unwrap());
}
// Part 2 — Strings and slices
/// Text up to the first space, or the whole string.
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

/// Count of a/e/i/o/u, ignoring case.
fn count_vowels(s: &str) -> usize {
    s.chars()
        .filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
}

/// "the quick brown fox" -> "fox brown quick the"
fn reverse_words(s: &str) -> String {
    s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

/// Upper-case the first character; empty string stays empty.
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// Palindrome check ignoring case and non-alphanumeric characters.
fn is_palindrome(s: &str) -> bool {
    let cleaned: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect();
    cleaned.iter().eq(cleaned.iter().rev())
}

fn main() {
    fix_b();
    fix_f();
    fix_g();
    fix_h();
    fix_i();

    println!("first_word: {}", first_word("hello world"));
    println!("count_vowels: {}", count_vowels("Hello World"));
    println!("reverse_words: {}", reverse_words("the quick brown fox"));
    println!("capitalize: {}", capitalize("rust"));
    println!(
        "is_palindrome: {}",
        is_palindrome("A man, a plan, a canal: Panama")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
        assert_eq!(first_word(""), "");
        let owned = String::from("a b");
        assert_eq!(first_word(&owned), "a");
    }

    #[test]
    fn t_count_vowels() {
        assert_eq!(count_vowels("Hello World"), 3);
        assert_eq!(count_vowels("AEIOU aeiou"), 10);
        assert_eq!(count_vowels("xyz"), 0);
    }

    #[test]
    fn t_reverse_words() {
        assert_eq!(reverse_words("the quick brown fox"), "fox brown quick the");
        assert_eq!(reverse_words("  a   b  "), "b a");
        assert_eq!(reverse_words(""), "");
    }

    #[test]
    fn t_capitalize() {
        assert_eq!(capitalize("rust"), "Rust");
        assert_eq!(capitalize(""), "");
        assert_eq!(capitalize("ärger"), "Ärger");
    }

    #[test]
    fn t_is_palindrome() {
        assert!(is_palindrome("A man, a plan, a canal: Panama"));
        assert!(!is_palindrome("hello"));
        assert!(is_palindrome(""));
    }

    #[test]
    fn t_fixes() {
        assert_eq!(fixed_unwrap(), "hi");
        assert_eq!(shout("ada"), "ADA");
    }
}


