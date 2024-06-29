use super::*;

//A variety of extern C functions mainly regarding custom game modes and other offsets in Main
extern "C" {
	#[link_name = "\u{1}_ZN3app17sv_camera_manager10dead_rangeEP9lua_State"]
	pub fn dead_range(lua_state: u64) -> Vector4f;

    #[link_name = "\u{1}_ZN3app17sv_camera_manager12camera_rangeEv"]
	pub fn camera_range() -> Vector4f;

    #[link_name = "\u{1}_ZN3app9curryshot15is_preview_modeEv"]
	pub fn is_preview_mode() -> bool;

    #[link_name = "\u{1}_ZN3app8lua_bind28PostureModule__rot_y_lr_implEPNS_26BattleObjectModuleAccessorE"]
	pub fn imported_rot_y_lr(boma: &mut BattleObjectModuleAccessor) -> f32;

    #[link_name = "\u{1}_ZN3app24FighterSpecializer_Luigi14delete_plungerERNS_7FighterEb"]
	pub fn delete_plunger(fighter: *mut smash::app::Fighter, param: bool) -> u64;

    #[link_name = "\u{1}_ZN3app25FighterSpecializer_Dedede29end_special_n_shot_object_hitERNS_7FighterE"]
	pub fn end_special_n_shot_object_hit(fighter: *mut smash::app::Fighter) -> u64;

    pub fn change_version_string(arg: u64, string: *const c_char);
}

pub unsafe extern "C" fn should_remove_projectile(weapon: &mut L2CWeaponCommon) -> bool {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if GroundModule::is_wall_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32)
    || WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HIT_WALL)
    || life <= 0 {
        return true;
    }
    return false;
}

pub unsafe extern "C" fn should_use_special_n_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE) {
        return 0.into();
    }
    1.into()
}

pub unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE) {
        return 0.into();
    }
    1.into()
}

pub unsafe extern "C" fn should_use_special_hi_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE) {
        return 0.into();
    }
    1.into()
}

pub unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE) {
        return 0.into();
    }
    1.into()
}

//Related to CheckAttack
pub unsafe fn get_table_value(table: *mut smash2::lib::L2CTable, key: &str) -> smash2::lib::L2CValue {
    let hash = if key.starts_with("0x") {
        smash2::phx::Hash40::from_hex_str(key).unwrap()
    } 
    else {
        smash2::phx::hash40(key)
    };
    (*table).get_map(hash).unwrap().clone()
}

//Shield Specials
pub unsafe extern "C" fn if_shield_special(fighter: &mut L2CFighterCommon) -> L2CValue {
    let kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
        if kind == *FIGHTER_KIND_NESS {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_N.into(),true.into());
            return true.into();
        }
        if [*FIGHTER_KIND_PICHU, *FIGHTER_KIND_PZENIGAME].contains(&kind) {
            fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(),true.into());
            return true.into();
        }
    }
    return false.into();
}

pub unsafe extern "C" fn empty_waza_customize() -> L2CValue {
    0.into()
}

//Gets the original Waza Customize
pub unsafe fn get_original_customizer(fighter: &mut L2CFighterCommon) -> Option<unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue> {
    let ptr = fighter.global_table["move_customizer_original"].get_ptr();
    if !ptr.is_null() {
        Some(std::mem::transmute(ptr))
    } 
    else {
        None
    }
}

//Related to Waza Customize
pub unsafe fn set_move_customizer(fighter: &mut L2CFighterCommon, customizer: unsafe extern "C" fn(&mut L2CFighterCommon) -> L2CValue) {
    if fighter.global_table["move_customizer_set"].get_bool() {
        return;
    }
    let waza_customize_control = fighter.global_table[WAZA_CUSTOMIZE_CONTROL].clone();
    fighter.global_table["move_customizer_set"].assign(&L2CValue::Bool(true));
    fighter.global_table["move_customizer_original"].assign(&waza_customize_control);
    fighter.global_table[WAZA_CUSTOMIZE_CONTROL].assign(&L2CValue::Ptr(customizer as *const () as _));
}