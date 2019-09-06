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

#[link(name = "int")]
extern {
    fn return_int()-> libc::c_int;
    fn return_int64_t() -> i64; // libc::int64_t is deprecated.
    fn print_int(val: libc::c_int);
    fn print_int64_t(val: i64);

}
fn main() {
    unsafe {
        println!("return_int() = {}",return_int());
        print_int(11);
        println!("return_int64_t() = {}",return_int64_t());
        print_int64_t(2147483648 * 2);
    }
}
