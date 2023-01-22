//! E-mail definition
//! WANT: no_std RFC 5322, and RFC 6532 compliant email address parser.
//! Candidates:
//! A: ^[\w!#$%&'*+/=?`{|}~^-]+(?:\.[\w!#$%&'*+/=?`{|}~^-]+)*@(?:[A-Z0-9-]+\.)+[A-Z]{2,6}$
//! B: ^[A-Z0-9_!#$%&'*+/=?`{|}~^.-]+@[A-Z0-9.-]+$ - KISS
//!
//! Probably going to choose B.

/// E-Mail address as specified in RFC 5322, and RFC 6532
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EmailAddress<'a> {
    user: &'a str,
    host: &'a str,
}

impl<'a> EmailAddress<'a> {}

/// Email Address Errors
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone)]
pub enum EmailAddressError {
    /// TODO: This is not implemented
    NotImplemented,
    /// E-Mail address is Invalid
    Invalid,
}

impl<'a> TryFrom<&str> for EmailAddress<'a> {
    type Error = EmailAddressError;

    fn try_from(_email_str: &str) -> Result<Self, Self::Error> {
        Err(EmailAddressError::NotImplemented)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("security@rust-lang.org", None)]
    #[case("abc", Some(EmailAddressError::Invalid))]
    fn str_email(#[case] email_str: &str, #[case] err: Option<EmailAddressError>) {
        let email: Result<EmailAddress<'_>, EmailAddressError> = email_str.try_into();

        match email {
            Err(e) => {
                assert_eq!(Some(e), err);
            }
            Ok(_) => {
                assert_eq!(err, None);
            }
        }
    }
}
