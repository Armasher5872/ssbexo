use super::*;

pub static UI_MANAGER: Lazy<RwLock<UiManager>> = Lazy::new(|| {
    RwLock::new(
        UiManager {
            palutena_meter: [PalutenaMeter::default(); 8],
            robot_meter: [RobotMeter::default(); 8]
        }
    )}
);

#[repr(C)]
pub struct UiManager {
    pub palutena_meter: [PalutenaMeter; 8],
    pub robot_meter: [RobotMeter; 8]
}

impl UiManager {
    fn get_ui_index_from_entry_id(entry_id: u32) -> u32 {
        let mut index = 0;
        for n in 0..entry_id {
            if get_battle_object_from_entry_id(n).is_some() {
                index += 1;
            }
        }
        return index;
    }
    //Palutena
    #[export_name = "UiManager__set_palutena_meter_enable"]
    pub extern "C" fn set_palutena_meter_enable(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        manager.palutena_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].set_enable(enable);
    }
    #[export_name = "UiManager__set_palutena_meter_info"]
    pub extern "C" fn set_palutena_meter_info(entry_id: u32, current: f32, max: f32, per_level: f32) {
        let mut manager = UI_MANAGER.write();
        manager.palutena_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].set_meter_info(current, max, per_level);
    }
    #[export_name = "UiManager__reset_palutena_meter"]
    pub extern "C" fn reset_palutena_meter(entry_id: u32) {
        let mut manager = UI_MANAGER.write();
        manager.palutena_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].reset();
    }
    #[export_name = "UiManager__change_palutena_meter_color_green"]
    pub extern "C" fn change_palutena_meter_color_green(entry_id: u32) {
        let mut manager = UI_MANAGER.write();
        manager.palutena_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].change_color_green();
    }
    #[export_name = "UiManager__change_palutena_meter_color_purple"]
    pub extern "C" fn change_palutena_meter_color_purple(entry_id: u32) {
        let mut manager = UI_MANAGER.write();
        manager.palutena_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].change_color_purple();
    }
    //R.O.B.
    #[export_name = "UiManager__set_robot_meter_enable"]
    pub extern "C" fn set_robot_meter_enable(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        manager.robot_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].set_enable(enable);
    }
    #[export_name = "UiManager__set_robot_meter_info"]
    pub extern "C" fn set_robot_meter_info(entry_id: u32, current: f32, max: f32, per_level: f32) {
        let mut manager = UI_MANAGER.write();
        manager.robot_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].set_meter_info(current, max, per_level);
    }
    #[export_name = "UiManager__reset_robot_meter"]
    pub extern "C" fn reset_robot_meter(entry_id: u32) {
        let mut manager = UI_MANAGER.write();
        manager.robot_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].reset();
    }
    #[export_name = "UiManager__change_robot_meter_color_blue"]
    pub extern "C" fn change_robot_meter_color_blue(entry_id: u32) {
        let mut manager = UI_MANAGER.write();
        manager.robot_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].change_color_blue();
    }
    #[export_name = "UiManager__change_robot_meter_color_yellow"]
    pub extern "C" fn change_robot_meter_color_yellow(entry_id: u32) {
        let mut manager = UI_MANAGER.write();
        manager.robot_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].change_color_yellow();
    }
    #[export_name = "UiManager__change_robot_meter_color_red"]
    pub extern "C" fn change_robot_meter_color_red(entry_id: u32) {
        let mut manager = UI_MANAGER.write();
        manager.robot_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].change_color_red();
    }
}