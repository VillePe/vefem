use crate::loads::Load;
use crate::structure::Element;
use crate::loads;
use std::ffi::{c_char, CStr, CString};
use crate::loads::utils::get_linked_element_numbers;

#[no_mangle]
pub extern "C" fn extract_elements_from_load(load_json: *const c_char, elements_json: *const c_char) -> *mut c_char {
    let load_json = unsafe { CStr::from_ptr(load_json).to_str().unwrap() };
    let elements_json = unsafe { CStr::from_ptr(elements_json).to_str().unwrap() };
    let load = match serde_json::from_str::<Load>(&load_json) {
        Ok(load) => load,
        Err(e) => {
            return CString::new(format!("Invalid JSON. Error: {}", e)).unwrap().into_raw();
        }
    };
    let elements = match serde_json::from_str::<Vec<Element>>(&elements_json) {
        Ok(elements) => elements,
        Err(e) => {
            return CString::new(format!("Invalid JSON. Error: {}", e)).unwrap().into_raw();
        }
    };

    let mut result_vector : Vec<i32> = Vec::new();
    let linked_elem_numbers = get_linked_element_numbers(&load);
    for element in &elements {
        if loads::utils::load_is_linked(element.number, &linked_elem_numbers) {
            result_vector.push(element.number);
        }
    }

    let results_json = serde_json::to_string_pretty(&result_vector).unwrap();
    CString::new(results_json).unwrap().into_raw()
}