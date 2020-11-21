# UTF8 Slice
A lightweight heapless way to do slicing on unicode strings in Rust.

# What does the library provide
This library provides 4 utility functions to deal with unicode slices.

### `utf8_slice::slice(s: &str, begin: usize, end: usize) -> &str`
This will do the same as `&s[begin..end]`, but now taking into account utf8 characters.

### `utf8_slice::from(s: &str, begin: usize) -> &str`
This will do the same as `&s[begin..]`, but now taking into account utf8 characters.

### `utf8_slice::till(s: &str, end: usize) -> &str`
This will do the same as `&s[..end]`, but now taking into account utf8 characters.

### `utf8_slice::len(s: &str) -> usize`
This will do the same as `s.len()`, but now taking into account utf8 characters.

# Documentation
[Link to Documentation](https://docs.rs/utf8_slice/1.0.0/utf8_slice/)

# License
MIT
