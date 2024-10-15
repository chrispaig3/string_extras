use core::ops::Range;

#[derive(Debug, PartialEq, Eq)]
pub struct Snippet {
    text: String,
    length: usize,
}

/// snippet: Is a handy function for parsing snippets of text out of Strings.
/// It consumes a String reference and returns a `Snippet` object containing the specified text and it's length.
/// Specify the length of the snippet using the range syntax `0..whatever`.
pub fn snippet<'t>(text: &'t String, range: Range<usize>) -> Snippet {
    let mut s = String::from(text);
    let result = s.drain(range).collect::<String>();
    let len = result.len();
    Snippet {
        text: result,
        length: len,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snippet() {
        let s = String::from("Hello, World!");
        let len = s.len();
        let result = snippet(&s, 0..len);
        assert_eq!(result.text, "Hello, World!".to_string());
        assert_eq!(result.length, 13);
    }
}
