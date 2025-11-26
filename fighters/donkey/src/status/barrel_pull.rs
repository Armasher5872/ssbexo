use super::*;

unsafe extern "C" fn donkey_barrel_pull_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn donkey_barrel_pull_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_barrel"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_float(weapon.module_accessor, 4.0, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_FLOAT_HP);
    0.into()
}

unsafe extern "C" fn donkey_barrel_pull_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("pull"), 0.0, 2.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(donkey_barrel_pull_main_loop as *const () as _))
}

unsafe extern "C" fn donkey_barrel_pull_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_lr = PostureModule::lr(owner_boma);
    PostureModule::set_lr(weapon.module_accessor, owner_lr);
    PostureModule::update_rot_y_lr(weapon.module_accessor);
    if !LinkModule::is_link(weapon.module_accessor, *LINK_NO_CONSTRAINT) {
        weapon.change_status(WEAPON_DONKEY_BARREL_STATUS_KIND_BREAK.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn donkey_barrel_pull_exec_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn donkey_barrel_pull_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn donkey_barrel_pull_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("donkey_barrel")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_DONKEY_BARREL_STATUS_KIND_PULL, donkey_barrel_pull_pre_status)
    .status(Init, *WEAPON_DONKEY_BARREL_STATUS_KIND_PULL, donkey_barrel_pull_init_status)
    .status(Main, *WEAPON_DONKEY_BARREL_STATUS_KIND_PULL, donkey_barrel_pull_main_status)
    .status(Exec, *WEAPON_DONKEY_BARREL_STATUS_KIND_PULL, donkey_barrel_pull_exec_status)
    .status(End, *WEAPON_DONKEY_BARREL_STATUS_KIND_PULL, donkey_barrel_pull_end_status)
    .status(Exit, *WEAPON_DONKEY_BARREL_STATUS_KIND_PULL, donkey_barrel_pull_exit_status)
    .install()
    ;
}