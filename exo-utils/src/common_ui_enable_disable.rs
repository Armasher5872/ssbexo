#[derive(Default, Copy, Clone)]
pub struct CommonUiEnableDisable {
    pub is_enabled: bool
}

impl CommonUiEnableDisable {
    pub fn new(_layout_data: u64) -> Self {
        return Self {
            is_enabled: true
        };
    }
}