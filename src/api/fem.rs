use crate::fem::*;
use std::ffi::{c_char, CStr};

pub extern "C" fn vefem_calculate(structure_json: *const c_char) -> *const c_char {
   "".as_ptr() as *const c_char
}