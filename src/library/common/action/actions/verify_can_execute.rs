#[macro_use]
build_action! { VerifyCanExecute (src_prog, hook, arg_id, args, do_return, return_value) {
        // TODO: Depending on LogVerbosity, log all use of this action
        // TODO: Use OsString?
        let library: &str = &hook.library;
        let symbol: &str = &hook.symbol;
        let any = String::from("ANY");
        let class = match (library, symbol) {
            ("/lib/x86_64-linux-gnu/libdl.so.2", "dlopen") |
            ("/lib/x86_64-linux-gnu/libdl.so.2", "dlmopen") => {
                String::from("Filesystem/Path/Library")
            },
            _ => String::from("Filesystem/Path/Executable")
        };
        let all_allowed_executables: Vec<String> = {
            let whitelist_cache_lock = crate::common::db::WL_CACHE.lock().expect("WhiteBeam: Failed to lock mutex");
            whitelist_cache_lock.iter().filter(|whitelist| (whitelist.class == class) && ((whitelist.path == src_prog) || (whitelist.path == any))).map(|whitelist| whitelist.value.clone()).collect()
        };
        // Permit ANY
        if all_allowed_executables.iter().any(|executable| executable == &any) {
            return (hook, args, do_return, return_value);
        }
        let argument: crate::common::db::ArgumentRow = args.iter().find(|arg| arg.id == arg_id).expect("WhiteBeam: Lost track of environment").clone();
        let target_executable: String = match (library, symbol) {
            ("/lib/x86_64-linux-gnu/libdl.so.2", "dlopen") |
            ("/lib/x86_64-linux-gnu/libdl.so.2", "dlmopen") => {
                if argument.real == 0 {
                    return (hook, args, do_return, return_value);
                }
                unsafe { String::from(std::ffi::CStr::from_ptr(argument.real as *const libc::c_char).to_str().expect("WhiteBeam: Unexpected null reference")) }
            },
            ("/lib/x86_64-linux-gnu/libc.so.6", "fexecve") => {
                let canonical_path = platform::canonicalize_fd(argument.real as i32).expect("WhiteBeam: Lost track of environment");
                canonical_path.into_os_string().into_string().expect("WhiteBeam: Unexpected null reference")
            },
            _ => unsafe { String::from(std::ffi::CStr::from_ptr(argument.real as *const libc::c_char).to_str().expect("WhiteBeam: Unexpected null reference")) }
        };
        // Permit whitelisted executables
        if all_allowed_executables.iter().any(|executable| executable == &target_executable) {
            return (hook, args, do_return, return_value);
        }
        // Permit execution if not running in prevention mode
        if !(crate::common::db::get_prevention()) {
            event::send_log_event(event::LogClass::Info as i64, format!("Detection: {} executed {} (VerifyCanExecute)", &src_prog, &target_executable));
            return (hook, args, do_return, return_value);
        }
        // Permit authorized execution
        if crate::common::db::get_valid_auth_env() {
            return (hook, args, do_return, return_value);
        }
        // Deny by default
        event::send_log_event(event::LogClass::Warn as i64, format!("Prevention: Blocked {} from executing {} (VerifyCanExecute)", &src_prog, &target_executable));
        eprintln!("WhiteBeam: {}: Permission denied", &target_executable);
        if (&hook.symbol).contains("exec") && (&hook.library).contains("libc.so") {
            // Terminate the child process
            unsafe { libc::exit(126) };
        }
        do_return = true;
        match (library, symbol) {
            ("/lib/x86_64-linux-gnu/libdl.so.2", "dlopen") |
            ("/lib/x86_64-linux-gnu/libdl.so.2", "dlmopen") => {
                // TODO: dlerror?
                return_value = 0;
            },
            _ => {
                return_value = -1;
            }
        };
}}
