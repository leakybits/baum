use crate::{retry, Res};

/// Parses a single element from the source.
///
/// # Errors
/// Returns a non-fatal error if the source is empty.
pub fn any<T>(src: &[T]) -> Res<&[T], &T> {
    src.split_first()
        .ok_or_else(|| retry!(src, "unexpected end of source"))
        .map(|(t, src)| (src, t))
}
