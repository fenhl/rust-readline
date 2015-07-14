extern crate readline as rl;
extern crate libc;

use std::io::{BufRead,BufReader};
use std::fs::File;
use std::path::Path;
use std::ffi::CStr;
use std::str;

fn complete(text: String) -> Vec<String> {
    let path = Path::new("/usr/share/dict/words");
    let file = BufReader::new(File::open(&path).unwrap());
    let mut entries: Vec<String> = Vec::new();
    for line in file.lines() {
        let word = line.unwrap();
        if (&word).starts_with(&text) {
            entries.push(word);
        }
    }
    return entries;
}

extern fn rl_compentry_func(text: *const i8, state: i32) -> *const i8 {
    if state == 0 {
        let txt = unsafe { CStr::from_ptr(text).to_bytes() };
        let entries = complete(str::from_utf8(txt).unwrap().to_string());
        rl::set_compentries(entries);
    }
    rl::get_compentry(state as usize)
}

extern fn my_attempted_completion_function(text: *const i8, _start: i32, _end: i32) -> *mut *const i8 {
    return rl::rl_completion_matches(text, rl_compentry_func)
}

// cargo run --example simple
pub fn main() {
    rl::rl_initialize().unwrap();
    //println!("{}", rl::rl_readline_version())
    println!("{}", rl::rl_library_version());

    rl::set_rl_attempted_completion_function(Some(my_attempted_completion_function));

    loop {
        match rl::readline("> ") {
            Some(line) => {
                let l = line.as_ref();
                rl::add_history(l);
                println!("{}", l);
                //println!("{}", rl::history_get(-2));
            },
            _ => {
                println!("");
                break
            }
        }
    }
}