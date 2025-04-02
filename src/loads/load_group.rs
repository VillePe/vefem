use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadGroup {
    id: u8,
    name: Cow<'static, str>,
    pub uls_factor: f64,
    pub psii0: f64,
    pub psii1: f64,
    pub psii2: f64,
    pub group_type: GroupType,
}

#[allow(dead_code)]
impl LoadGroup {
    pub const PERMANENT: Self = Self{id: 0, name: Cow::Borrowed("Pysyvät"), uls_factor: 1.15, psii0: 1.0, psii1: 1.0, psii2: 1.0, group_type: GroupType::Permanent};
    pub const PERMANENT_FAV: Self = Self{id: 1, name: Cow::Borrowed("Pysyvät, edu."), uls_factor: 1.15, psii0: 1.0, psii1: 1.0, psii2: 1.0, group_type: GroupType::Permanent};
    pub const CLASS_A: Self = Self{id: 2, name: Cow::Borrowed("KL A"), uls_factor: 1.0, psii0: 1.0, psii1: 1.0, psii2: 1.0, group_type: GroupType::LiveLoad};
    pub const CLASS_B: Self = Self{id: 3, name: Cow::Borrowed("KL B"), uls_factor: 1.0, psii0: 1.0, psii1: 1.0, psii2: 1.0, group_type: GroupType::LiveLoad};
    pub const CLASS_C: Self = Self{id: 4, name: Cow::Borrowed("KL C"), uls_factor: 1.0, psii0: 1.0, psii1: 1.0, psii2: 1.0, group_type: GroupType::LiveLoad};
    pub const CLASS_D: Self = Self{id: 5, name: Cow::Borrowed("KL D"), uls_factor: 1.0, psii0: 1.0, psii1: 1.0, psii2: 1.0, group_type: GroupType::LiveLoad};
    pub const CLASS_E: Self = Self{id: 6, name: Cow::Borrowed("KL E"), uls_factor: 1.0, psii0: 1.0, psii1: 1.0, psii2: 1.0, group_type: GroupType::LiveLoad};
    pub const CLASS_F: Self = Self{id: 7, name: Cow::Borrowed("KL F"), uls_factor: 1.0, psii0: 1.0, psii1: 1.0, psii2: 1.0, group_type: GroupType::LiveLoad};
    pub const CLASS_G: Self = Self{id: 8, name: Cow::Borrowed("KL G"), uls_factor: 1.0, psii0: 1.0, psii1: 1.0, psii2: 1.0, group_type: GroupType::LiveLoad};
    pub const CLASS_H: Self = Self{id: 9, name: Cow::Borrowed("KL H"), uls_factor: 1.0, psii0: 1.0, psii1: 1.0, psii2: 1.0, group_type: GroupType::LiveLoad};
    pub const SNOW: Self = Self{id: 10, name: Cow::Borrowed("Lumi"), uls_factor: 1.0, psii0: 1.0, psii1: 1.0, psii2: 1.0, group_type: GroupType::LiveLoad};
    pub const WIND_POS: Self = Self{id: 11, name: Cow::Borrowed("Tuuli+"), uls_factor: 1.0, psii0: 1.0, psii1: 1.0, psii2: 1.0, group_type: GroupType::LiveLoad};
    pub const WIND_NEG: Self = Self{id: 12, name: Cow::Borrowed("Tuuli-"), uls_factor: 1.0, psii0: 1.0, psii1: 1.0, psii2: 1.0, group_type: GroupType::LiveLoad};
    pub const THERMAL: Self = Self{id: 13, name: Cow::Borrowed("Lämpö"), uls_factor: 1.0, psii0: 1.0, psii1: 1.0, psii2: 1.0, group_type: GroupType::LiveLoad};    

    pub fn new_user_1(name: String, uls_factor: f64, psii0: f64, psii1: f64, psii2: f64, group_type: GroupType) -> Self {
        LoadGroup::new_user(21, name, uls_factor, psii0, psii1, psii2, group_type)
    }

    pub fn new_user_2(name: String, uls_factor: f64, psii0: f64, psii1: f64, psii2: f64, group_type: GroupType) -> Self {
        LoadGroup::new_user(22, name, uls_factor, psii0, psii1, psii2, group_type)
    }

    pub fn new_user_3(name: String, uls_factor: f64, psii0: f64, psii1: f64, psii2: f64, group_type: GroupType) -> Self {
        LoadGroup::new_user(23, name, uls_factor, psii0, psii1, psii2, group_type)
    }

    fn new_user(id: u8, name: String, uls_factor: f64, psii0: f64, psii1: f64, psii2: f64, group_type: GroupType) -> Self {
        Self {
            id: id, 
            name: Cow::Owned(name), 
            uls_factor, 
            psii0, 
            psii1, 
            psii2,
            group_type
        }
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GroupType {
    Permanent,
    LiveLoad,
}

#[cfg(test)]
mod test {
    use super::LoadGroup;


    #[test]
    fn add_user_load_group() {
        let load_group = LoadGroup::new_user_1(
            "testi".to_string(), 1.0, 1.0, 1.0, 1.0, super::GroupType::Permanent);
        
        println!("{0}", load_group.get_name());
        assert!(load_group.get_name().eq("testi"));
    }
}