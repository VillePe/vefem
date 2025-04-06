use vputilslib::equation_handler::EquationHandler;

use crate::{
    fem::{self},
    structure::StructureModel,
};
use std::ffi::{c_char, CStr, CString};

#[no_mangle]
pub extern "C" fn vefem_calculate(structure_json: *const c_char) -> *mut c_char {
    let calc_model_json = unsafe { CStr::from_ptr(structure_json).to_str().unwrap() };
    let calc_model = match serde_json::from_str::<StructureModel>(&calc_model_json) {
        Ok(calc_model) => calc_model,
        Err(e) => {
            return CString::new(format!("Invalid JSON. Error: {}", e)).unwrap().into_raw();
        }
    };

    let results = fem::fem_handler::calculate(&calc_model, &mut EquationHandler::new());

    let results_json = serde_json::to_string_pretty(&results).unwrap();
    CString::new(results_json).unwrap().into_raw()
}


#[no_mangle]
pub extern "C" fn version() -> *mut c_char {
    let version = CARGO_VERSION.unwrap_or("Could not get the version of the library!");
    CString::new(version).unwrap().into_raw()
}

const CARGO_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");