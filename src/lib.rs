#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg, doc_cfg_hide))]
#![cfg_attr(docsrs, doc(cfg_hide(docsrs)))]
#![deny(clippy::unwrap_used, missing_docs)]
#![warn(
    clippy::unwrap_used,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![doc = include_str!("../README.md")]

#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

/* Like who needs heap ?
#[cfg(feature = "alloc")]
#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

#[cfg(feature = "heapless")]
#[allow(unused_imports)]
#[macro_use]
extern crate heapless;

use core::fmt;
impl fmt::Debug for X
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("X")
            .field("x", self.r_bytes())
            .finish()
    }
impl fmt::Display for X {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}", self)
    }
}
*/

mod email;
/// RFC 5322, and RFC 6532
pub use email::EmailAddress;

/// Re-Export SemVer reflecting Source of Truth
mod semver;
pub use crate::semver::SemVer;

mod error;

/// TODO: SecurityPolicy
#[allow(non_snake_case)]
#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityPolicy {
    /// v - Policy SemVer version
    pub(crate) v: SemVer,
    /// E - Email-address - RFC 5322, and RFC 6532
    pub(crate) E: EmailAddress,
}

#[allow(non_snake_case)]
impl SecurityPolicy {
    /// v - version - from [`crate::SemVer`]
    pub fn v(&self) -> SemVer {
        self.v.clone()
    }
    /// E - email - from [`crate::EmailAddress`]
    pub fn E(&self) -> EmailAddress {
        self.E.clone()
    }
}

/// Error for Security Policy
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone)]
pub enum SecurityPolicyError {
    /// Regex fails to match the policy - in future nom used
    InvalidVersion,
    /// TODO: Get rid of this thing.
    Unknown,
}

impl TryFrom<&str> for SecurityPolicy {
    type Error = SecurityPolicyError;

    fn try_from(policy_str: &str) -> Result<Self, Self::Error> {
        Err(SecurityPolicyError::Unknown)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("v=0.1.0", None)]
    #[case("v=0.1000.0", Some(SecurityPolicyError::InvalidVersion))]
    #[case("v=0.-1.0", Some(SecurityPolicyError::InvalidVersion))]
    fn version(#[case] v_str: &str, #[case] err: Option<SecurityPolicyError>) {
        let policy: Result<SecurityPolicy, SecurityPolicyError> = v_str.try_into();

        match policy {
            Err(e) => {
                assert_eq!(Some(e), err);
            }
            Ok(p) => {
                assert_eq!(err, None);
            }
        }
    }
}
