use vputilslib::equation_handler::EquationHandler;

use crate::{
    fem::{self},
    structure::CalculationModel,
};
use std::ffi::{c_char, CStr, CString};

#[no_mangle]
pub extern "C" fn vefem_calculate(structure_json: *const c_char) -> *mut c_char {
    let calc_model_json = unsafe { CStr::from_ptr(structure_json).to_str().unwrap() };
    let calc_model = match serde_json::from_str::<CalculationModel>(&calc_model_json) {
        Ok(calc_model) => calc_model,
        Err(e) => {
            return CString::new(format!("Invalid JSON. Error: {}", e)).unwrap().into_raw();
        }
    };

    let results = fem::calculate(&calc_model, &mut EquationHandler::new());

    let results_json = serde_json::to_string_pretty(&results).unwrap();
    CString::new(results_json).unwrap().into_raw()
}

// expected value at line 95 column 20