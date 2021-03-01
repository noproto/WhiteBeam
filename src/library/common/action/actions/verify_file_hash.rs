use std::io::prelude::*;

fn fail(library: &str, symbol: &str, argument_path: &str) {
    if symbol.contains("exec") && library.contains("libc.so") {
        // Terminate the child process
        eprintln!("WhiteBeam: {}: Permission denied", argument_path);
        unsafe { libc::exit(126) };
    } else {
        unimplemented!("WhiteBeam: The '{}' symbol (from {}) is not supported by the ConsumeVariadic action", symbol, library);
    }
}

#[macro_use]
build_action! { VerifyFileHash (_src_prog, hook, arg_id, args, do_return, return_value) {
        // TODO: Depending on LogVerbosity, log all use of this action
        // NB: For Execution hooks, system executables that aren't read world may be whitelisted as ANY
        if !(crate::common::db::get_prevention()) {
            return (hook, args, do_return, return_value);
        }
        // Permit authorized execution
        if crate::common::db::get_valid_auth_env() {
            return (hook, args, do_return, return_value);
        }
        let library: &str = &hook.library;
        let symbol: &str = &hook.symbol;
        let any = String::from("ANY");
        let class = String::from("Hash/");
        let argument_path = {
            let argument: crate::common::db::ArgumentRow = args.iter().find(|arg| arg.id == arg_id).expect("WhiteBeam: Lost track of environment").clone();
            String::from(unsafe { std::ffi::CStr::from_ptr(argument.real as *const i8) }.to_str().expect("WhiteBeam: Unexpected null reference"))
        };
        let all_allowed_hashes: Vec<(String, String)> = {
            let whitelist_cache_lock = crate::common::db::WL_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
            whitelist_cache_lock.iter().filter(|whitelist| (whitelist.class.starts_with(&class)) && ((whitelist.path == argument_path) || (whitelist.path == any))).map(|whitelist| (whitelist.class.clone(), whitelist.value.clone())).collect()
        };
        // Permit ANY
        if all_allowed_hashes.iter().any(|hash_tuple| hash_tuple.1 == any) {
            return (hook, args, do_return, return_value);
        }
        // Permit whitelisted file hashes (consecutively). This allows hybrid hashing schemes for additional security (e.g. SHA3 and BLAKE3).
        let hash_count = all_allowed_hashes.len();
        let mut argument_file: std::fs::File = match std::fs::File::open(&argument_path) {
            Ok(f) => f,
            Err(_e) => {
                fail(library, symbol, &argument_path);
                unreachable!("WhiteBeam: Lost track of environment");
            }
        };
        let passed_all: bool = all_allowed_hashes.iter().all(|hash_tuple| {
            argument_file.seek(std::io::SeekFrom::Start(0)).expect("WhiteBeam: VerifyFileHash failed to seek in target file");
            hash_tuple.1 == crate::common::hash::process_hash(&mut argument_file, &(hash_tuple.0), None)
        });
        if (hash_count > 0) && passed_all {
            return (hook, args, do_return, return_value);
        }
        // Deny by default
        fail(library, symbol, &argument_path);
}}
