
#![no_main]

use libfuzzer_sys::fuzz_target;
use trie_db_fuzz::fuzz_that_verify_accepts_valid_proofs;

fuzz_target!(|data: &[u8]| {
	fuzz_that_verify_accepts_valid_proofs::<reference_trie::ExtensionLayout>(data);
});
