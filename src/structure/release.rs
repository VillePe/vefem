#![allow(dead_code)]

#[derive(Debug)]
pub struct Release {
    /// Start release in elements local X-axis
    pub s_tx: bool,
    /// Start release in elements local Z-axis
    pub s_tz: bool,
    /// Start rotation release about elements local Y-axis
    pub s_ry: bool,
    /// End release in elements local X-axis
    pub e_tx: bool,
    /// End release in elements local Z-axis
    pub e_tz: bool,
    /// End rotation release about elements local Y-axis
    pub e_ry: bool,
}
impl Release {
    /// Creates new Release object that has no releases set (translations and rotation are all locked)
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Gets the release value from given index (from range 0..=5, 0: s_tx, 1: s_tz, 2: s_ry, 3: e_tx, 4: e_tz, 5: e_ry)
    pub fn get_release_value(&self, i: usize) -> Option<bool> {
        match i {
            0 => Some(self.s_tx),
            1 => Some(self.s_tz),
            2 => Some(self.s_ry),
            3 => Some(self.e_tx),
            4 => Some(self.e_tz),
            5 => Some(self.e_ry),
            _ => None,
        }
    }

    pub fn start_release_any(&self) -> bool {
        self.s_tx || self.s_tz || self.s_ry
    }

    pub fn end_release_any(&self) -> bool { self.e_tx || self.e_tz || self.e_ry }

    pub fn start_release_count(&self) -> usize {
        let tx = if self.s_tx { 1 } else { 0 };
        let tz = if self.s_tz { 1 } else { 0 };
        let ry = if self.s_ry { 1 } else { 0 };
        tx + tz + ry
    }

    pub fn end_release_count(&self) -> usize {
        let tx = if self.e_tx { 1 } else { 0 };
        let tz = if self.e_tz { 1 } else { 0 };
        let ry = if self.e_ry { 1 } else { 0 };
        tx + tz + ry
    }
}
impl Default for Release {
    fn default() -> Self {
        Self {
            s_tx: false,
            s_tz: false,
            s_ry: false,
            e_tx: false,
            e_tz: false,
            e_ry: false,
        }
    }
}
