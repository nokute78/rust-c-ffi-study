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
extern crate libc;

#[link(name = "callback")]
extern "C" {
    fn C_callback_print(val: libc::c_int);
    fn C_callback_plus_one(val: libc::c_int) -> libc::c_int;
}

fn exec_c_func(input: libc::c_int, callback:unsafe extern "C" fn(libc::c_int)) {
    unsafe {
        callback(input);
    }
}

fn exec_c_func_return_int(input: libc::c_int, callback: unsafe extern "C" fn(libc::c_int) -> libc::c_int) -> libc::c_int {
    unsafe {
        callback(input)
    }
}

pub fn main() {
    let func = C_callback_print;
    exec_c_func(10, func);

    let func = C_callback_plus_one;
    println!("input:{} return:{}", 10, exec_c_func_return_int(10, func));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_c_callback_plus_one() {
        let func = super::C_callback_plus_one;
        assert_eq!(11, super::exec_c_func_return_int(10,func));
    }
}
