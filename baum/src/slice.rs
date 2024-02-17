use super::*;

/// Parses a single element from the source.
pub fn any<T>(src: &[T]) -> Res<&[T], &T> {
    src.split_first()
        .ok_or_else(|| retry!(src, "unexpected end of source"))
        .map(|(t, src)| (src, t))
}
