use super::*;

unsafe extern "C" fn wario_kamikaze_burst_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn wario_kamikaze_burst_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_kamikaze"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn wario_kamikaze_burst_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_pos_x = PostureModule::pos_x(owner_boma);
    let owner_pos_y = PostureModule::pos_y(owner_boma);
    let owner_pos_z = PostureModule::pos_z(owner_boma);
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x, y: owner_pos_y+3.0, z: owner_pos_z});
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("kamikaze"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(wario_kamikaze_burst_main_loop as *const () as _))
}

unsafe extern "C" fn wario_kamikaze_burst_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

unsafe extern "C" fn wario_kamikaze_burst_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("wario_fireballcloned")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_WARIO_KAMIKAZE_STATUS_KIND_KAMIKAZE, wario_kamikaze_burst_pre_status)
    .status(Init, *WEAPON_WARIO_KAMIKAZE_STATUS_KIND_KAMIKAZE, wario_kamikaze_burst_init_status)
    .status(Main, *WEAPON_WARIO_KAMIKAZE_STATUS_KIND_KAMIKAZE, wario_kamikaze_burst_main_status)
    .status(End, *WEAPON_WARIO_KAMIKAZE_STATUS_KIND_KAMIKAZE, wario_kamikaze_burst_end_status)
    .install()
    ;
}