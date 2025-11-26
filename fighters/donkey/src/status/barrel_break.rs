use super::*;

unsafe extern "C" fn donkey_barrel_break_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn donkey_barrel_break_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::set_int(weapon.module_accessor, 40, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn donkey_barrel_break_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    ModelModule::set_joint_translate(weapon.module_accessor, Hash40::new("rotx"), &Vector3f{x: 0.0, y: 3.5, z: 0.0}, false, false);
    ReflectorModule::set_status_all(weapon.module_accessor, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("break"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(donkey_barrel_break_main_loop as *const () as _))
}

unsafe extern "C" fn donkey_barrel_break_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    WorkModule::on_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    ModelModule::set_joint_translate(weapon.module_accessor, Hash40::new("rotx"), &Vector3f{x: 0.0, y: 3.5, z: 0.0}, false, false);
    if should_remove_barrel(weapon) {
        remove_barrel(weapon);
    }
    0.into()
}

unsafe extern "C" fn donkey_barrel_break_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn donkey_barrel_break_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn donkey_barrel_break_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("donkey_barrel")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_DONKEY_BARREL_STATUS_KIND_BREAK, donkey_barrel_break_pre_status)
    .status(Init, *WEAPON_DONKEY_BARREL_STATUS_KIND_BREAK, donkey_barrel_break_init_status)
    .status(Main, *WEAPON_DONKEY_BARREL_STATUS_KIND_BREAK, donkey_barrel_break_main_status)
    .status(Exec, *WEAPON_DONKEY_BARREL_STATUS_KIND_BREAK, donkey_barrel_break_exec_status)
    .status(End, *WEAPON_DONKEY_BARREL_STATUS_KIND_BREAK, donkey_barrel_break_end_status)
    .status(Exit, *WEAPON_DONKEY_BARREL_STATUS_KIND_BREAK, donkey_barrel_break_exit_status)
    .install()
    ;
}