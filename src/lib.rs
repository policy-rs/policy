#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg, doc_cfg_hide))]
#![cfg_attr(docsrs, doc(cfg_hide(docsrs)))]
#![deny(
    clippy::unwrap_used,
    missing_docs
)]
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
pub use semver;

mod error;

/// TODO: SecurityPolicy
#[allow(non_snake_case)]
#[non_exhaustive]
#[derive(Clone, Eq, PartialEq)]
pub struct SecurityPolicy {
    /// v - SemVer version
    pub(crate) v: semver::Version,
    /// E - Email-address - RFC 5322, and RFC 6532
    pub(crate) E: EmailAddress,
}

#[allow(non_snake_case)]
impl SecurityPolicy {
    /// v - version - from [`semver::Version`]
    pub fn v(&self) -> semver::Version {
        self.v.clone()
    }
    /// E - email - from [`EmailAddress`]
    pub fn E(&self) -> EmailAddress {
        self.E.clone()
    }    
}

/// Error for Security Policy
#[non_exhaustive]
pub enum SecurityPolicyError {
    /// Regex fails to match the policy - in future nom used
    NoMatch,
}

impl TryFrom<&str> for SecurityPolicy {
    type Error = SecurityPolicyError;
    
    fn try_from(policy_str: &str) -> Result<Self, Self::Error> {
        Err(SecurityPolicyError::NoMatch)
    }
}
