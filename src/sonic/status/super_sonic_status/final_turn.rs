use super::*;

unsafe extern "C" fn supersonic_final_turn_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn supersonic_final_turn_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 4.0, 4.0);
    0.into()
}

unsafe extern "C" fn supersonic_final_turn_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("final_turn"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(supersonic_final_turn_main_loop as *const () as _))
}

unsafe extern "C" fn supersonic_final_turn_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let timer = WorkModule::get_int(owner_boma, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_FINAL_SMASH_TIMER);
    WorkModule::inc_int(owner_boma, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_FINAL_SMASH_TIMER);
    if timer >= 900 {
        weapon.change_status(WEAPON_SONIC_SUPERSONIC_STATUS_KIND_PRE_END.into(), false.into());
        StatusModule::change_status_request_from_script(owner_boma, *FIGHTER_SONIC_STATUS_KIND_FINAL_END, false);
    }
    if MotionModule::is_end(weapon.module_accessor) {
        weapon.change_status(WEAPON_SONIC_SUPERSONIC_STATUS_KIND_IDLE.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn supersonic_final_turn_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let status_kind = weapon.global_table[STATUS_KIND].get_i32();
    if ![*WEAPON_SONIC_SUPERSONIC_STATUS_KIND_IDLE, WEAPON_SONIC_SUPERSONIC_STATUS_KIND_TURN, WEAPON_SONIC_SUPERSONIC_STATUS_KIND_MOVE_BALL_START, WEAPON_SONIC_SUPERSONIC_STATUS_KIND_MOVE_BALL, WEAPON_SONIC_SUPERSONIC_STATUS_KIND_MOVE_BALL_END].contains(&status_kind) {
        WorkModule::set_int(owner_boma, 0, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_FINAL_SMASH_TIMER);
    }
    0.into()
}

pub fn install() {
    Agent::new("sonic_supersonic")
    .status(Pre, WEAPON_SONIC_SUPERSONIC_STATUS_KIND_TURN, supersonic_final_turn_pre_status)
    .status(Init, WEAPON_SONIC_SUPERSONIC_STATUS_KIND_TURN, supersonic_final_turn_init_status)
    .status(Main, WEAPON_SONIC_SUPERSONIC_STATUS_KIND_TURN, supersonic_final_turn_main_status)
    .status(End, WEAPON_SONIC_SUPERSONIC_STATUS_KIND_TURN, supersonic_final_turn_end_status)
    .install()
    ;
}