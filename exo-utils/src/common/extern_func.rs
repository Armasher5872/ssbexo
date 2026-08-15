#![allow(improper_ctypes)] //Addresses Building warning: `extern` block uses type `rect::Rect`, which is not FFI-safe
use super::*;

unsafe extern "C" {
    #[link_name = "_ZN3app6camera13get_dead_areaEv"]
    pub fn get_dead_area() -> Rect;

    #[link_name = "_ZN3app8lua_bind32HitModule__reset_status_all_implEPNS_26BattleObjectModuleAccessorEi"]
    pub fn hit_module_reset_status_all(boma: *mut BattleObjectModuleAccessor, param_2: i32);

    #[link_name = "_ZN3app8lua_bind38FighterManager__get_fighter_entry_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
    pub fn get_fighter_entry(manager: *mut smash::app::FighterManager, entry_id: u32) -> *mut u8;

    #[link_name = "_ZN3app8lua_bind39ArticleModule__get_article_from_no_implEPNS_26BattleObjectModuleAccessorEii"]
    pub fn get_article_from_no(boma: *mut BattleObjectModuleAccessor, article_kind: i32, article_id: i32) -> *mut smash::app::Article;

    #[link_name = "_ZN3app8lua_bind53FighterControlModuleImpl__set_command_life_count_implEPNS_26BattleObjectModuleAccessorEiih"]
    pub fn fighter_control_module_set_command_life_count(boma: *mut BattleObjectModuleAccessor, command_category: i32, command: i32, life: i32);
    
    #[link_name = "_ZN3app16kiiladarzmanager15set_visible_hudEb"]
    pub fn set_vis_hud(param_1: bool);
    
    #[link_name = "_ZN3app17sv_camera_manager10dead_rangeEP9lua_State"]
	pub fn dead_range(lua_state: u64) -> Vector4f;
    
    #[link_name = "_ZN3app19sv_fighter_audience20notify_event_msc_cmdEP9lua_State"]
    pub fn sv_fighter_audience_notify_event_msc_cmd(lua_state: u64);

    #[link_name = "_ZN3app23FighterSpecializer_Edge32set_one_winged_light_weight_dataERNS_7FighterEb"]
    pub fn set_one_winged_light_weight_data(fighter: &mut smash::app::Fighter, bool_check: bool);

    #[link_name = "_ZN3app24FighterSpecializer_Cloud20display_final_windowEb"]
	pub fn display_final_window(param_1: bool);

    #[link_name = "_ZN3app24FighterSpecializer_Luigi14delete_plungerERNS_7FighterEb"]
	pub fn delete_plunger(fighter: *mut smash::app::Fighter, param: bool) -> u64;

    #[link_name = "_ZN3app24FighterSpecializer_Demon15sub_rage_systemERNS_7FighterEb"]
    pub fn sub_rage_system(fighter: *mut smash::app::Fighter, param: bool);

    #[link_name = "_ZN3app26WeaponSpecializer_EdgeFire14request_effectERNS_26BattleObjectModuleAccessorE"]
    pub fn weapon_specializer_edge_fire_request_effect(boma: *mut BattleObjectModuleAccessor);

    #[link_name = "_ZN3app28FighterInklingLinkEventPaint13new_l2c_tableEv"]
    pub fn FighterInklingLinkEventPaint__new_l2c_table() -> smash::lib::L2CValue;

    #[link_name = "_ZN7lua2cpp16L2CFighterCommon28status_LandingLightSub_paramEN3lib8L2CValueE"]
    pub fn status_LandingLightSub_param(fighter: &mut L2CFighterCommon, param_2: L2CValue);

    pub fn change_version_string(arg: u64, string: *const std::os::raw::c_char);
}

pub fn is_on_ryujinx() -> bool {
    unsafe {
        //Ryujinx skip based on text addr
        let text_addr = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        if text_addr == 0x8504000 || text_addr == 0x80004000 {
            return true;
        } 
        else {
            return false;
        }
    }
}