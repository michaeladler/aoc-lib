/// Search for `needle` in `haystack`.
pub fn search_haystack<T: PartialEq>(needle: &[T], haystack: &[T]) -> Option<usize> {
    if needle.is_empty() {
        // special case: `haystack.windows(0)` will panic, so this case
        // needs to be handled separately in whatever way you feel is
        // appropriate
        return Some(0);
    }

    haystack
        .windows(needle.len())
        .position(|subslice| subslice == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_haystack_empty() {
        let needle = b"world";
        let haystack = b"";
        let pos = search_haystack(&needle[..], &haystack[..]);
        assert_eq!(None, pos);
    }

    #[test]
    fn test_search_haystack_needle_empty() {
        let needle = b"";
        let haystack = b"hello world";
        let pos = search_haystack(&needle[..], &haystack[..]);
        assert_eq!(Some(0), pos);
    }

    #[test]
    fn test_search_haystack() {
        let needle = b"world";
        let haystack = b"hello world!";
        let pos = search_haystack(&needle[..], &haystack[..]);
        assert_eq!(Some(6), pos);
    }
}
