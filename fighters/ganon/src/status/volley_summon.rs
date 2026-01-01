use super::*;

unsafe extern "C" fn ganon_volley_summon_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn ganon_volley_summon_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    ModelModule::set_scale(weapon.module_accessor, 1.0);
    KineticModule::unable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    WorkModule::set_int(weapon.module_accessor, 20, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, 20, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn ganon_volley_summon_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("summon_boar"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(ganon_volley_summon_main_loop as *const () as _))
}

unsafe extern "C" fn ganon_volley_summon_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_lr = PostureModule::lr(owner_boma);
    let owner_pos_x = PostureModule::pos_x(owner_boma);
    let owner_pos_y = PostureModule::pos_y(owner_boma);
    let owner_pos_z = PostureModule::pos_z(owner_boma);
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x+(10.0*owner_lr), y: owner_pos_y+12.0, z: owner_pos_z});
    if should_remove_projectile(weapon) {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

unsafe extern "C" fn ganon_volley_summon_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
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