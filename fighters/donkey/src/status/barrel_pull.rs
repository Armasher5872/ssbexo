use super::*;

unsafe extern "C" fn donkey_barrel_pull_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn donkey_barrel_pull_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let life = WorkModule::get_param_int(boma, hash40("param_barrel"), hash40("life"));
    WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(boma, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_float(boma, 4.0, *WEAPON_DONKEY_BARREL_INSTANCE_WORK_ID_FLOAT_HP);
    GroundModule::set_rhombus_offset(boma, &Vector2f{x: 0.0, y: -3.5});
    0.into()
}

unsafe extern "C" fn donkey_barrel_pull_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    MotionModule::change_motion(boma, Hash40::new("pull"), 0.0, 2.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(donkey_barrel_pull_main_loop as *const () as _))
}

unsafe extern "C" fn donkey_barrel_pull_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    let owner_lr = PostureModule::lr(owner_boma);
    PostureModule::set_lr(boma, owner_lr);
    PostureModule::update_rot_y_lr(boma);
    if !LinkModule::is_link(boma, *LINK_NO_CONSTRAINT) {
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