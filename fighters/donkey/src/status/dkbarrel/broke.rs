use super::*;

//Barrel Break Pre Status
unsafe extern "C" fn donkey_barrel_break_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

//Barrel Break Init Status
unsafe extern "C" fn donkey_barrel_break_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::set_int(weapon.module_accessor, 45, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

//Barrel Break Main Status
unsafe extern "C" fn donkey_barrel_break_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    WorkModule::off_flag(owner_boma, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_LINKED);
    WorkModule::off_flag(owner_boma, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE);
    ReflectorModule::set_status_all(weapon.module_accessor, ShieldStatus(*SHIELD_STATUS_NONE), 0);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("break"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(donkey_barrel_break_main_loop as *const () as _))
}

unsafe extern "C" fn donkey_barrel_break_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let pos_x = PostureModule::pos_x(weapon.module_accessor);
    let pos_y = PostureModule::pos_y(weapon.module_accessor);
    let pos_z = PostureModule::pos_z(weapon.module_accessor);
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: pos_x, y: pos_y-4.0, z: pos_z});
    if should_remove_barrel(weapon) {
        barrel_removal(weapon);
    }
    0.into()
}

//Barrel Break Exec Status
unsafe extern "C" fn donkey_barrel_break_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

//Barrel Break End Status
unsafe extern "C" fn donkey_barrel_break_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

//Barrel Break Exit Status
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