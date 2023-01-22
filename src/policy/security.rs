//! SecurityPolicy

use crate::EmailAddress;

/// TODO: SecurityPolicy
#[allow(non_snake_case)]
#[non_exhaustive]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityPolicy {
    /// v - Policy SemVer version
    pub(crate) v: semver::Version,
    /// E - Email-address - RFC 5322, and RFC 6532
    pub(crate) E: EmailAddress,
}

#[allow(non_snake_case)]
impl SecurityPolicy {
    /// v - version - from [`crate::SemVer`]
    pub fn v(&self) -> semver::Version {
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
