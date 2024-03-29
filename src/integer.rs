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
extern "C" {
    fn C_return_int()-> libc::c_int;
    fn C_return_int64_t() -> i64; // libc::int64_t is deprecated.
    fn C_print_int(val: libc::c_int);
    fn C_print_int64_t(val: i64);

    fn C_ret_even_odd(num: libc::c_int) -> EVEN_ODD;

    static C_INT_MAX: libc::c_int;
}

#[repr(C)]
enum EVEN_ODD{
    EVEN = 10,
    ODD,
}

fn print_even_odd(val: EVEN_ODD) {
    match val {
        EVEN_ODD::ODD => {println!("ODD")}
        EVEN_ODD::EVEN => {println!("EVEN")}
    }
}

pub fn main() {
    unsafe {
        println!("C_return_int() = {}",C_return_int());
        C_print_int(11);
        println!("C_return_int64_t() = {}",C_return_int64_t());
        C_print_int64_t(2147483648 * 2);

        let val = 10;
        print!("{} is ", val);
        print_even_odd( C_ret_even_odd(val) );

        let val = 11;
        print!("{} is ", val);
        print_even_odd( C_ret_even_odd(val) );

        println!("Global variable C_INT_MAX is {}", C_INT_MAX);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_c_return_int() {
        unsafe {
            assert_eq!(10, super::C_return_int());
        }
    }
    #[test]
    fn test_c_return_int64_t() {
        unsafe {
            assert_eq!(2147483648, super::C_return_int64_t());
        }
    }

    #[test]
    fn test_c_ret_even_odd() {
        unsafe {
            match super::C_ret_even_odd(10) {
                super::EVEN_ODD::ODD => {panic!("10 is EVEN")}
                _ => {}
            }
            match super::C_ret_even_odd(11) {
                super::EVEN_ODD::EVEN => {panic!("10 is ODD")}
                _ => {}
            }
        }
    }

    #[test]
    fn test_c_int_max() {
        unsafe{
            assert_eq!(libc::INT_MAX, super::C_INT_MAX);
        }
    }
}
