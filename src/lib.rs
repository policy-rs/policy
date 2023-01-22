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
*/

// SemVer reflecting Source of Truth
pub use semver;

/// TODO: SecurityPolicy
pub struct SecurityPolicy {
    /// SemVer version
    pub version: semver::Version,
}
