/*
   Copyright 2019 Takahiro Yamashita

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
 */

use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::copy;

#[link(name = "string")]
extern "C" {
    fn C_print_str(str: *const c_char);
    static mut kv: [C_kv; 2];
    static C_const_str: *const c_char;
}

#[repr(C)]
struct  C_kv {
    pub key:   *mut c_char,
    pub value: libc::c_int,
}

pub fn main() {
    let str = CString::new("Hello").expect("CString::new failed");
    unsafe {
        C_print_str(str.as_ptr());

        // use Rust String.
        let rust_str = "Hello Rust";
        let rust_cstring = CString::new(rust_str).expect("CString::new failed"); // convert CString.
        C_print_str(rust_cstring.as_ptr());

        C_print_str(kv[0].key);

        C_print_str(C_const_str); // print c const char*;
    }
}

