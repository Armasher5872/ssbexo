#![allow(improper_ctypes)] //Addresses Building warning: `extern` block uses type `rect::Rect`, which is not FFI-safe
use super::*;

extern "C" {
    #[link_name = "_ZN3app6camera13get_dead_areaEv"]
    pub fn get_dead_area() -> Rect;
    
    #[link_name = "\u{1}_ZN3app8lua_bind38FighterManager__get_fighter_entry_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
    pub fn get_fighter_entry(manager: *mut smash::app::FighterManager, entry_id: u32) -> *mut u8;

    #[link_name = "\u{1}_ZN3app8lua_bind39ArticleModule__get_article_from_no_implEPNS_26BattleObjectModuleAccessorEii"]
    pub fn get_article_from_no(boma: *mut smash::app::BattleObjectModuleAccessor, article_kind: i32, article_id: i32) -> *mut smash::app::Article;

    #[link_name = "_ZN3app10sv_animcmd25EFFECT_GLOBAL_BACK_GROUNDEP9lua_State"]
    pub fn effect_global_back_ground(lua_state: u64);

    #[link_name = "\u{1}_ZN3app16kiiladarzmanager15set_visible_hudEb"]
    pub fn set_vis_hud(param_1: bool);

    #[link_name = "\u{1}_ZN3app17sv_camera_manager10dead_rangeEP9lua_State"]
	pub fn dead_range(lua_state: u64) -> Vector4f;

    #[link_name = "\u{1}_ZN3app19FighterCutInManager13is_one_on_oneEv"]
    pub fn FighterCutInManager__is_one_on_one() -> bool;
    
    #[link_name = "\u{1}_ZN3app19sv_fighter_audience20notify_event_msc_cmdEP9lua_State"]
    pub fn sv_fighter_audience_notify_event_msc_cmd(lua_state: u64);

    #[link_name = "\u{1}_ZN3app24FighterSpecializer_Cloud20display_final_windowEb"]
	pub fn display_final_window(param_1: bool);

    #[link_name = "\u{1}_ZN3app24FighterSpecializer_Luigi14delete_plungerERNS_7FighterEb"]
	pub fn delete_plunger(fighter: *mut smash::app::Fighter, param: bool) -> u64;

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