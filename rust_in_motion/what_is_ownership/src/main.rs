#![feature(test)]

fn main() {
    let s = String::from("books");
    let plural = pluralize(s.clone());

    println!("I have {}, you have many {}", s, plural);
}

fn pluralize(s: String) -> String {
    if s.is_empty() {
        s
    } else if s.ends_with('s') {
        s
    } else {
        s + "s"
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[test]
    fn pluralize_test() {
        assert_eq!(pluralize("book".to_string()), String::from("books"));
        assert_eq!(pluralize("books".to_string()), String::from("books"));
        assert_eq!(pluralize("cat".to_string()), String::from("cats"));
        assert_eq!(pluralize("".to_string()), String::from(""));
    }

    #[bench]
    fn pluralize_bench(b: &mut Bencher) {
        let words = [
            (String::from("book"), String::from("books")),
            (String::from("books"), String::from("books")),
            (String::from("cat"), String::from("cats")),
            (String::from("cats"), String::from("cats")),
            (String::from(""), String::from("")),
        ];

        for word in words.iter() {
            assert_eq!(pluralize(word.0.clone()), word.1);
        }

        b.iter(|| {
            for word in words.iter() {
                pluralize(word.0.clone());
            }
        })
    }
}
