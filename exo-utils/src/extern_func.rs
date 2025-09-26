#![allow(improper_ctypes_definitions, improper_ctypes)]
use super::*;

//A variety of extern C functions mainly regarding custom game modes and other offsets in Main
extern "C" {
    #[link_name = "\u{1}_ZN3app4item12disable_areaEP9lua_Statei"]
    pub fn disable_area(lua_state: u64, area_kind: i32);

    #[link_name = "_ZN3app6camera13get_dead_areaEv"]
    pub fn get_dead_area() -> Rect;

    /*
    #[link_name = "\u{1}_ZN3app8lua_bind28PostureModule__rot_y_lr_implEPNS_26BattleObjectModuleAccessorE"]
	pub fn imported_rot_y_lr(boma: &mut BattleObjectModuleAccessor) -> f32;

    #[link_name = "\u{1}_ZN3app9curryshot15is_preview_modeEv"]
	pub fn is_preview_mode() -> bool;
    */

    #[link_name = "\u{1}_ZN3app8lua_bind38FighterManager__get_fighter_entry_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
    pub fn get_fighter_entry(manager: *mut smash::app::FighterManager, entry_id: u32) -> *mut u8;

    #[link_name = "\u{1}_ZN3app8lua_bind39ArticleModule__get_article_from_no_implEPNS_26BattleObjectModuleAccessorEii"]
    pub fn get_article_from_no(boma: *mut smash::app::BattleObjectModuleAccessor, article_kind: i32, article_id: i32) -> *mut smash::app::Article;

    #[link_name = "\u{1}_ZN3app8lua_bind53FighterControlModuleImpl__set_command_life_count_implEPNS_26BattleObjectModuleAccessorEiih"]
    pub fn set_command_life_count(boma: *mut smash::app::BattleObjectModuleAccessor, command_category: i32, command_pad: i32, life: u32);

    #[link_name = "\u{1}_ZN3app9holywater35HOLYWATER_FIRE_PILLAR_GRAVITY_ACCELENS_11FighterKindE"]
    pub fn fire_pillar_gravity_accel(kind: FighterKind) -> f32;

    #[link_name = "\u{1}_ZN3app9holywater39HOLYWATER_FIRE_PILLAR_GRAVITY_ACCEL_MAXENS_11FighterKindE"]
    pub fn fire_pillar_gravity_accel_max(kind: FighterKind) -> f32;

    #[link_name = "\u{1}_ZN3app9holywater29HOLYWATER_FIRE_PILLAR_SPEED_YENS_11FighterKindE"]
    pub fn fire_pillar_speed_y(kind: FighterKind) -> f32;
    
    #[link_name = "_ZN3app10sv_animcmd25EFFECT_GLOBAL_BACK_GROUNDEP9lua_State"]
    pub fn effect_global_back_ground(lua_state: u64);

    #[link_name = "\u{1}_ZN3app11FighterUtil33get_ground_correct_kind_air_transERNS_26BattleObjectModuleAccessorEi"]
    pub fn get_ground_correct_kind_air_trans(boma: &mut smash::app::BattleObjectModuleAccessor, something: i32) -> i32;

    #[link_name = "\u{1}_ZN3app16kiiladarzmanager15set_visible_hudEb"]
    pub fn set_vis_hud(param_1: bool);

    #[link_name = "\u{1}_ZN3app17sv_camera_manager10dead_rangeEP9lua_State"]
	pub fn dead_range(lua_state: u64) -> Vector4f;

    #[link_name = "\u{1}_ZN3app17sv_camera_manager12camera_rangeEv"]
	pub fn camera_range() -> Vector4f;

    #[link_name = "\u{1}_ZN3app18kinetic_energy_rot12set_rotationEP9lua_StateRKN3phx8Vector3fE"]
    pub fn kinetic_energy_rot_set_rotation(lua_state: u64, rotation: *const Vector3f);

    #[link_name = "\u{1}_ZN3app19FighterCutInManager13is_one_on_oneEv"]
    pub fn FighterCutInManager__is_one_on_one() -> bool;
    
    #[link_name = "\u{1}_ZN3app19sv_fighter_audience20notify_event_msc_cmdEP9lua_State"]
    pub fn sv_fighter_audience_notify_event_msc_cmd(lua_state: u64);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control6enableEP9lua_State"]
    pub fn kinetic_energy_control_enable(lua_state: u64);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control9set_accelEP9lua_StateRKN3phx8Vector2fE"]
    pub fn kinetic_energy_control_set_accel(lua_state: u64, accel: *const Vector2f);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control9set_brakeEP9lua_StateRKN3phx8Vector2fE"]
    pub fn kinetic_energy_control_set_brake(lua_state: u64, accel: *const Vector2f);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control16set_stable_speedEP9lua_StateRKN3phx8Vector2fE"]
    pub fn kinetic_energy_control_set_stable_speed(lua_state: u64, accel: *const Vector2f);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control15set_limit_speedEP9lua_StateRKN3phx8Vector2fE"]
    pub fn kinetic_energy_control_set_limit_speed(lua_state: u64, accel: *const Vector2f);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control9set_speedEP9lua_StateRKN3phx8Vector2fE"]
    pub fn kinetic_energy_control_set_speed(lua_state: u64, accel: *const Vector2f);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_gravity9set_accelEP9lua_Statef"]
    pub fn kinetic_energy_gravity_set_accel(lua_state: u64, accel: f32);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_gravity15set_limit_speedEP9lua_Statef"]
    pub fn kinetic_energy_gravity_set_limit_speed(lua_state: u64, accel: f32);

    #[link_name = "\u{1}_ZN3app24FighterSpecializer_Cloud20display_final_windowEb"]
	pub fn display_final_window(param_1: bool);

    #[link_name = "\u{1}_ZN3app24FighterSpecializer_Luigi14delete_plungerERNS_7FighterEb"]
	pub fn delete_plunger(fighter: *mut smash::app::Fighter, param: bool) -> u64;

    #[link_name = "\u{1}_ZN3app24FighterSpecializer_Sonic17start_final_sonicERNS_7FighterE"]
	pub fn start_final_sonic(instance: *mut smash::app::Fighter) -> u64;

    #[link_name = "\u{1}_ZN3app24FighterSpecializer_Sonic24start_final_sonic_cameraERNS_7FighterE"]
	pub fn start_final_sonic_camera(instance: *mut smash::app::Fighter) -> u64;

    #[link_name = "\u{1}_ZN3app25FighterSpecializer_Dedede29end_special_n_shot_object_hitERNS_7FighterE"]
	pub fn end_special_n_shot_object_hit(fighter: *mut smash::app::Fighter) -> u64;

    #[link_name = "\u{1}_ZN3app26kinetic_energy_control_rot12set_rotationEP9lua_StateRKN3phx8Vector3fE"]
    pub fn kinetic_energy_control_rot_set_rotation(lua_state: u64, rotation: *const Vector3f);

    #[link_name = "\u{1}_ZN3app27FighterSpecializer_Gekkouga42set_special_s_transition_term_forbid_groupERNS_21FighterModuleAccessorEb"]
    pub fn FighterSpecializer_Gekkouga__set_special_s_transition_term_forbid_group(boma: *mut smash::app::FighterModuleAccessor, bool_check: bool);

    #[link_name = "\u{1}_ZN3app28FighterInklingLinkEventPaint13new_l2c_tableEv"]
    pub fn FighterInklingLinkEventPaint__new_l2c_table() -> smash::lib::L2CValue;

    #[link_name = "\u{1}_ZN3app30WeaponSpecializer_LinkBowarrow7to_itemERNS_26BattleObjectModuleAccessorE"]
    pub fn to_item(boma: *mut smash::app::BattleObjectModuleAccessor);

    pub fn change_version_string(arg: u64, string: *const c_char);
}

pub fn is_on_ryujinx() -> bool {
    unsafe {
        //Ryujinx skip based on text addr
        let text_addr = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        if text_addr == 0x8504000 || text_addr == 0x80004000 {
            println!("On Ryujinx");
            return true;
        } 
        else {
            println!("Not on Ryujinx");
            return false;
        }
    }
}