mod api;

pub use api::ApplyOutcome;
pub use api::ApplyStatus;
pub use api::AttemptStatus;
pub use api::CloudBackend;
pub use api::CloudBackendFuture;
pub use api::CloudTaskError;
pub use api::CreatedTask;
pub use api::DiffSummary;
pub use api::Result;
pub use api::TaskId;
pub use api::TaskListPage;
pub use api::TaskStatus;
pub use api::TaskSummary;
pub use api::TaskText;
pub use api::TurnAttempt;

mod http;
pub use http::HttpClient;

/// Which end of a string to keep when capping its byte length on a UTF-8 char boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SliceEnd {
    /// Keep at most `max_bytes` from the start of the string.
    Prefix,
    /// Keep at most `max_bytes` from the end of the string.
    Suffix,
}

/// Return a UTF-8 char-boundary-safe slice of `s` containing at most `max_bytes` bytes
/// from either the start (`SliceEnd::Prefix`) or end (`SliceEnd::Suffix`) of `s`.
pub fn slice_to_char_boundary(s: &str, max_bytes: usize, end: SliceEnd) -> &str {
    if s.len() <= max_bytes {
        return s;
    }
    match end {
        SliceEnd::Prefix => &s[..s.floor_char_boundary(max_bytes)],
        SliceEnd::Suffix => &s[s.ceil_char_boundary(s.len() - max_bytes)..],
    }
}

#[cfg(test)]
mod tests {
    use super::SliceEnd;
    use super::slice_to_char_boundary;

    #[test]
    fn slice_to_char_boundary_prefix_and_suffix_never_split_chars() {
        assert_eq!(
            slice_to_char_boundary("éa", /*max_bytes*/ 1, SliceEnd::Prefix),
            ""
        );
        assert_eq!(
            slice_to_char_boundary("éa", /*max_bytes*/ 2, SliceEnd::Prefix),
            "é"
        );
        assert_eq!(
            slice_to_char_boundary("aé", /*max_bytes*/ 1, SliceEnd::Suffix),
            ""
        );
        assert_eq!(
            slice_to_char_boundary("aé", /*max_bytes*/ 2, SliceEnd::Suffix),
            "é"
        );
        assert_eq!(
            slice_to_char_boundary("🦀", /*max_bytes*/ 3, SliceEnd::Prefix),
            ""
        );
        assert_eq!(
            slice_to_char_boundary("🦀", /*max_bytes*/ 3, SliceEnd::Suffix),
            ""
        );
        assert_eq!(
            slice_to_char_boundary("🦀", /*max_bytes*/ 4, SliceEnd::Prefix),
            "🦀"
        );
        assert_eq!(
            slice_to_char_boundary("🦀", /*max_bytes*/ 4, SliceEnd::Suffix),
            "🦀"
        );
    }
}
