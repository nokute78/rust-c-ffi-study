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

#[link(name = "string")]
extern {
    fn C_print_str(str: *const c_char);
}

pub fn main() {
    let str = CString::new("Hello").expect("CString::new failed");
    unsafe {
        C_print_str(str.as_ptr());
        
    }
}
