use super::*;

unsafe extern "C" fn lucariom_mega_evolve_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_NONE), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn lucariom_mega_evolve_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let parent_situation_kind = LinkModule::get_parent_situation_kind(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT);
    weapon.set_situation(parent_situation_kind.into());
    0.into()
}

unsafe extern "C" fn lucariom_mega_evolve_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.fastshift(L2CValue::Ptr(lucariom_mega_evolve_main_loop as *const () as _))
}

unsafe extern "C" fn lucariom_mega_evolve_main_loop(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucariom_mega_evolve_exec_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucariom_mega_evolve_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::clear_screen(weapon.module_accessor, 0, 0);
    EffectModule::remove_screen(weapon.module_accessor, Hash40::new("bg_lucario_final"), -1);
    0.into()
}

unsafe extern "C" fn lucariom_mega_evolve_exit_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::clear_screen(weapon.module_accessor, 0, 0);
    EffectModule::remove_screen(weapon.module_accessor, Hash40::new("bg_lucario_final"), -1);
    0.into()
}

pub fn install() {
    Agent::new("lucario_lucariom")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_LUCARIO_LUCARIOM_STATUS_KIND_MEGA_EVOLVE, lucariom_mega_evolve_pre_status)
    .status(Init, *WEAPON_LUCARIO_LUCARIOM_STATUS_KIND_MEGA_EVOLVE, lucariom_mega_evolve_init_status)
    .status(Main, *WEAPON_LUCARIO_LUCARIOM_STATUS_KIND_MEGA_EVOLVE, lucariom_mega_evolve_main_status)
    .status(Exec, *WEAPON_LUCARIO_LUCARIOM_STATUS_KIND_MEGA_EVOLVE, lucariom_mega_evolve_exec_status)
    .status(End, *WEAPON_LUCARIO_LUCARIOM_STATUS_KIND_MEGA_EVOLVE, lucariom_mega_evolve_end_status)
    .status(Exit, *WEAPON_LUCARIO_LUCARIOM_STATUS_KIND_MEGA_EVOLVE, lucariom_mega_evolve_exit_status)
    .install()
    ;
}