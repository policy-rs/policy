#![no_main]

use libfuzzer_sys::fuzz_target;

use policy::{EmailAddress, EmailAddressError};

// TODO this needs proper
fuzz_target!(|data: &str| {
    let _email: Result<EmailAddress<'_>, EmailAddressError> = data.try_into();
});
