#[derive(Debug)]
pub struct Release {
    pub tx: bool,
    pub tz: bool,
    pub ry: bool,
}
impl Release {
    /// Creates new Release object that has no releases set (translations and rotation are all locked)
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
impl Default for Release {
    fn default() -> Self {
        Self {
            tx: false,
            tz: false,
            ry: false,
        }
    }
}