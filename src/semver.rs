//! MVP Naive SemVer representation

/// SemVer
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SemVer {
    major: u64,
    minor: u64,
    patch: u64,
}
