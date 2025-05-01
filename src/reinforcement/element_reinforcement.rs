use serde::{Deserialize, Serialize};

use super::{RebarCollection, ShearRebarGroup};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementReinforcement {
    pub main_rebars: Vec<RebarCollection>,
    pub shear_rebars: Vec<ShearRebarGroup>,
}