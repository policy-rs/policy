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

//------------------------------------------------------------------------
// Errors
//------------------------------------------------------------------------

mod error;

//------------------------------------------------------------------------
// Types / Re-Exports
//------------------------------------------------------------------------

#[cfg(feature = "email")]
mod email;

/// RFC 5322, and RFC 6532
#[cfg(feature = "email")]
pub use email::{EmailAddress, EmailAddressError};

/// Re-Export SemVer reflecting Source of Truth
#[cfg(feature = "semver")]
pub use semver;

//------------------------------------------------------------------------
// Policies
//------------------------------------------------------------------------

mod policy;

/// SecurityPolicy
#[cfg(feature = "security")]
pub use policy::security::{SecurityPolicy, SecurityPolicyError};
