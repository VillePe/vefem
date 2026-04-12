mod common;

#[cfg(test)]
mod tests {
    use vputilslib::equation_handler::EquationHandler;
    use vefem::printing;
    use vefem::settings::CalculationSettings;
    use vefem::structure::StructureModel;
    use crate::common::{get_fem_matriisi_loads, get_structure_fem_matriisit_releases};

    #[test]
    fn test() {
        let structure = get_structure_fem_matriisit_releases();
        let loads = get_fem_matriisi_loads();
        let struct_model = StructureModel {
            elements: structure.0,
            nodes: structure.1,
            loads,
            calc_settings: CalculationSettings::default(),
            load_combinations: vec![],
        };
        let results = vefem::fem::fem_handler::calculate(&struct_model, &EquationHandler::new());
        printing::print_results(&results, &struct_model, true);
    }
}