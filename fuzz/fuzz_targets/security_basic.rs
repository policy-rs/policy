#![no_main]

use libfuzzer_sys::fuzz_target;

use policy::{SecurityPolicy, SecurityPolicyError};

// TODO this needs proper
fuzz_target!(|data: &str| {
    let _policy: Result<SecurityPolicy<'_>, SecurityPolicyError> = data.try_into();
});
