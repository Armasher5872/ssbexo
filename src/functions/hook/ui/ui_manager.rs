use super::*;

pub static UI_MANAGER: Lazy<RwLock<UiManager>> = Lazy::new(|| {
    RwLock::new(
        UiManager {
            palutena_meter: [PalutenaMeter::default(); 8],
            robot_meter: [RobotMeter::default(); 8],
            ice_climber_meter: [IceClimberMeter::default(); 8]
        }
    )}
);

#[repr(C)]
pub struct UiManager {
    pub palutena_meter: [PalutenaMeter; 8],
    pub robot_meter: [RobotMeter; 8],
    pub ice_climber_meter: [IceClimberMeter; 8]
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
    //Ice Climbers
    #[export_name = "UiManager__set_iceclimber_meter_enable_1"]
    pub extern "C" fn set_iceclimber_meter_enable_1(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        manager.ice_climber_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].set_enable_1(enable);
    }
    #[export_name = "UiManager__set_iceclimber_meter_enable_2"]
    pub extern "C" fn set_iceclimber_meter_enable_2(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        manager.ice_climber_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].set_enable_2(enable);
    }
    #[export_name = "UiManager__set_iceclimber_meter_enable_3"]
    pub extern "C" fn set_iceclimber_meter_enable_3(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        manager.ice_climber_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].set_enable_3(enable);
    }
    #[export_name = "UiManager__set_iceclimber_meter_info"]
    pub extern "C" fn set_iceclimber_meter_info(entry_id: u32, element_1: i32, element_2: i32, element_3: i32) {
        let mut manager = UI_MANAGER.write();
        manager.ice_climber_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].set_meter_info(element_1, element_2, element_3);
    }
    #[export_name = "UiManager__set_iceclimber_meter_color"]
    pub unsafe extern "C" fn set_iceclimber_meter_color(entry_id: u32, percent: f32) {
        let mut manager = UI_MANAGER.write();
        manager.ice_climber_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].update_color(percent);
    }
}