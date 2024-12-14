
use nix::libc::c_char;
use nix::pty::{forkpty, ForkptyResult, Winsize};
use nix::unistd::{execvp, fork, ForkResult};
use std::ffi::{CStr, CString, OsString};
use std::os::fd::OwnedFd;
use std::os::unix::ffi::OsStringExt;
use std::process;

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
use bindings::*;

fn create_subprocess_with_pty(
    rows: u16,
    cols: u16,
    term: &str,
) -> Option<(i32, OwnedFd)> {
    let win = Winsize {
        ws_row: rows,
        ws_col: cols,
        ws_xpixel: 0,  // These are typically unused; leaving as zero.
        ws_ypixel: 0,
    };

    let fork_res = unsafe { forkpty(Some(&win), None).ok()? };

    match fork_res {
        ForkptyResult::Parent { child, master } => Some((child.into(), master)),
        ForkptyResult::Child => {
            // setenv("TERM", term, 1)?;

            let prog_cstr = CString::new("/bin/bash").unwrap();
            let mut argv: Vec<CString> = vec![prog_cstr.clone()];
            argv.push(CString::new("-").unwrap());

            // Convert Vec<CString> into Vec<*const i8> for execvp.
            let argv_cstrs: Vec<&CStr> = argv.iter().map(|arg| arg.as_c_str()).collect();
            execvp(&prog_cstr, &argv_cstrs).unwrap_or_else(|_| process::exit(-1));
            None
        }
    }
}