//* # UTF8 Slice
//* A lightweight heapless way to do slicing on unicode strings in Rust.
//*
//* # What does the library provide
//* This library provides 4 utility functions to deal with unicode slices.
//*
//* ## `utf8_slice::slice(s: &str, begin: usize, end: usize) -> &str`
//* This will do the same as `&s[begin..end]`, but now taking into account utf8 characters.
//*
//* ## `utf8_slice::from(s: &str, begin: usize) -> &str`
//* This will do the same as `&s[begin..]`, but now taking into account utf8 characters.
//*
//* ## `utf8_slice::till(s: &str, end: usize) -> &str`
//* This will do the same as `&s[..end]`, but now taking into account utf8 characters.
//*
//* ## `utf8_slice::len(s: &str) -> usize`
//* This will do the same as `s.len()`, but now taking into account utf8 characters.
//* # License
//* MIT
//*
//* # Examples
//*
//* ```
//* let s = "The 🚀 goes to the 🌑!";
//*
//* let rocket = utf8_slice::slice(s, 4, 5);
//* # assert_eq!(utf8_slice::slice(s, 4, 5), "🚀");
//* // Will equal "🚀"
//* ```

/// Fetches a slice of a string from a begin to an end index
/// taking into account utf8/unicode character indices.
///
/// # Arguments
///
/// * `s` - An input string to take the slice from
/// * `begin` - Where the slice begins
/// * `end` - Where the slice ends
///
/// # Examples
///
/// ```
/// let s = "The 🚀 goes to the 🌑!";
///
/// let rocket = utf8_slice::slice(s, 4, 5);
/// # assert_eq!(utf8_slice::slice(s, 4, 5), "🚀");
/// // Will equal "🚀"
/// ```
///
/// # Note
/// * Will return an empty string for invalid indices *
pub fn slice(s: &str, begin: usize, end: usize) -> &str {
    if end < begin {
        return "";
    }

    s.char_indices()
        .nth(begin)
        .and_then(|(start_pos, _)| {
            if end >= len(s) {
                return Some(&s[start_pos..]);
            }

            s[start_pos..]
                .char_indices()
                .nth(end - begin)
                .map(|(end_pos, _)| &s[start_pos..start_pos + end_pos])
        })
        .unwrap_or("")
}

/// Fetches a slice of a string from a starting index
/// taking into account utf8/unicode character indices.
///
/// # Arguments
///
/// * `s` - An input string to take the slice from
/// * `begin` - Where the slice begins
///
/// # Examples
///
/// ```
/// let s = "The 🚀 goes to the 🌑!";
///
/// let rocket_goes_to_the_moon = utf8_slice::from(s, 4);
/// # assert_eq!(utf8_slice::from(s, 4), "🚀 goes to the 🌑!");
/// // Will equal "🚀 goes to the 🌑!"
/// ```
///
/// # Note
/// * Will return an empty string for invalid indices *
pub fn from(s: &str, begin: usize) -> &str {
    slice(s, begin, len(s))
}

/// Fetches a slice of a string until an ending index
/// taking into account utf8/unicode character indices.
///
/// # Arguments
///
/// * `s` - An input string to take the slice from
/// * `end` - Where the slice ends
///
/// # Examples
///
/// ```
/// let s = "The 🚀 goes to the 🌑!";
///
/// let the_rocket = utf8_slice::till(s, 5);
/// # assert_eq!(utf8_slice::till(s, 4), "The 🚀");
/// // Will equal "The 🚀"
/// ```
///
/// # Note
/// * Will return an empty string for invalid indices *
pub fn till(s: &str, end: usize) -> &str {
    slice(s, 0, end)
}

/// Fetches the length in characters of an utf8/unicode string
///
/// # Arguments
///
/// * `s` - The string of which to fetch the length
pub fn len(s: &str) -> usize {
    s.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_as_std_slice() {
        let s = "xjfdlskfaj sdfjlkj";
        for i in 0..s.len() {
            for j in i..s.len() + 1 {
                assert_eq!(&s[i..j], slice(s, i, j));
            }
        }
    }

    #[test]
    fn test_slice() {
        assert_eq!(slice("\u{345}ab\u{898}xyz", 1, 4), "ab\u{898}");
        assert_eq!(slice("\u{345}ab\u{898}xyz", 0, 4), "\u{345}ab\u{898}");
        assert_eq!(slice("\u{345}ab\u{898}xyz", 5, 4), "");
        assert_eq!(slice("\u{345}ab   \u{898}xyz", 0, 1), "\u{345}");
        assert_eq!(slice("abcdef", 0, 6), "abcdef");
        assert_eq!(slice("\u{345}ab\u{898}xyz", 1, 7), "ab\u{898}xyz");
    }

    #[test]
    fn test_from() {
        assert_eq!(from("\u{345}ab\u{898}xyz", 1), "ab\u{898}xyz");
        assert_eq!(from("\u{345}ab\u{898}xyz", 3), "\u{898}xyz");
        assert_eq!(from("\u{345}ab\u{898}xyz", 10), "");
        assert_eq!(from("\u{345}ab   \u{898}xyz", 0), "\u{345}ab   \u{898}xyz");
    }

    #[test]
    fn test_till() {
        assert_eq!(till("\u{345}ab\u{898}xyz", 1), "\u{345}");
        assert_eq!(till("\u{345}ab\u{898}xyz", 3), "\u{345}ab");
        assert_eq!(till("\u{345}ab\u{898}xyz", 0), "");
    }

    #[test]
    fn test_len() {
        assert_eq!(len(""), 0);
        assert_eq!(len("👨‍🚀"), 3);
        assert_eq!(len("abc"), 3);
        assert_eq!(len("abd👨‍🚀"), 6);
    }
}
