// Load OS-specific modules
#[macro_use]
mod hook;
mod system;

use libc::{c_char, c_int};
use std::ffi::CStr;
use crate::library::common::hash;
use crate::library::common::event;

hook! {
    unsafe fn hooked_execve(filename: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int => execve {
        // TODO: Check /opt/whitebeam/cache.json for whitelist status
        let mut allow_exec = false;
        // TODO: Garbage collection here
        let c_str: &CStr = CStr::from_ptr(filename);
        let str_slice: &str = c_str.to_str().unwrap();
        let program: String = str_slice.to_owned(); // If necessary
        // Hardcoded paths for testing purposes
        let hardcoded_whitelist = [
                    // Shells
                    "/bin/bash".to_string(),
                    "/bin/sh".to_string(),
                    // Utilities
                    "/bin/cat".to_string(),
                    "/bin/rm".to_string(),
                    "/bin/ls".to_string(),
                    "/bin/ps".to_string(),
                    "/usr/bin/whoami".to_string(),
                    "/usr/bin/id".to_string(),
                    // Whitebeam
                    "/opt/whitebeam/whitebeam".to_string()
                    ];
        if hardcoded_whitelist.contains(&program) {
            allow_exec = true;
        }
        let hexdigest = hash::common_hash_file(&program);
        let uid = system::get_current_uid();
        if allow_exec {
            event::send_exec_event(uid, &program, &hexdigest, allow_exec);
            real!(hooked_execve)(filename, argv, envp)
        } else {
            event::send_exec_event(uid, &program, &hexdigest, allow_exec);
            *system::errno_location() = system::EACCES;
            return -1
        }
    }
}