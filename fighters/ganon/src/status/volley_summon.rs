use super::*;

unsafe extern "C" fn ganon_volley_summon_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn ganon_volley_summon_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    ModelModule::set_scale(boma, 0.85);
    KineticModule::unable_energy(boma, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    0.into()
}

unsafe extern "C" fn ganon_volley_summon_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    MotionModule::change_motion(boma, Hash40::new("summon_boar"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(ganon_volley_summon_main_loop as *const () as _))
}

unsafe extern "C" fn ganon_volley_summon_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    let owner_boma = get_owner_boma(weapon);
    let owner_lr = PostureModule::lr(owner_boma);
    let owner_pos_x = PostureModule::pos_x(owner_boma);
    let owner_pos_y = PostureModule::pos_y(owner_boma);
    let owner_pos_z = PostureModule::pos_z(owner_boma);
    PostureModule::set_pos(boma, &Vector3f{x: owner_pos_x+(10.0*owner_lr), y: owner_pos_y+12.0, z: owner_pos_z});
    0.into()
}

unsafe extern "C" fn ganon_volley_summon_exec_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn ganon_volley_summon_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn ganon_volley_summon_exit_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("ganon_volley")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_GANON_VOLLEY_STATUS_KIND_SUMMON, ganon_volley_summon_pre_status)
    .status(Init, *WEAPON_GANON_VOLLEY_STATUS_KIND_SUMMON, ganon_volley_summon_init_status)
    .status(Main, *WEAPON_GANON_VOLLEY_STATUS_KIND_SUMMON, ganon_volley_summon_main_status)
    .status(Exec, *WEAPON_GANON_VOLLEY_STATUS_KIND_SUMMON, ganon_volley_summon_exec_status)
    .status(End, *WEAPON_GANON_VOLLEY_STATUS_KIND_SUMMON, ganon_volley_summon_end_status)
    .status(End, *WEAPON_GANON_VOLLEY_STATUS_KIND_SUMMON, ganon_volley_summon_exit_status)
    .install()
    ;
}