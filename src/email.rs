//! E-mail definition
//! WANT: no_std RFC 5322, and RFC 6532 compliant email address parser.
//! Candidates:
//! A: ^[\w!#$%&'*+/=?`{|}~^-]+(?:\.[\w!#$%&'*+/=?`{|}~^-]+)*@(?:[A-Z0-9-]+\.)+[A-Z]{2,6}$
//! B: ^[A-Z0-9_!#$%&'*+/=?`{|}~^.-]+@[A-Z0-9.-]+$ - KISS
//!
//! Probably going to choose B.

/// E-Mail address as specified in RFC 5322, and RFC 6532
#[derive(Clone, Eq, PartialEq)]
pub struct EmailAddress {
}

impl EmailAddress {
}

#[cfg(test)]
mod test {
    use super::*;

    
}
