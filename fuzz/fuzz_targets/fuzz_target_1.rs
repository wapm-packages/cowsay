#![no_main]
use libfuzzer_sys::fuzz_target;
use cowsay::make_bubble;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        make_bubble(s.to_string(), 40, false, true);
    }
});
