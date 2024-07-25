use super::*;

unsafe extern "C" fn supersonic_final_start_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_NONE), *WEAPON_KINETIC_TYPE_GENERIC_RESET, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn supersonic_final_start_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let situation_kind = weapon.global_table[SITUATION_KIND].get_i32();
    let motion_kind = MotionModule::motion_kind(weapon.module_accessor);
    let final_charge = WorkModule::is_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE);
    if situation_kind != *SITUATION_KIND_GROUND {
        if final_charge {
            if motion_kind == hash40("final_start") {
                MotionModule::change_motion_inherit_frame(weapon.module_accessor, Hash40::new("final_air_start"), -1.0, 1.0, 0.0, false, false);
            }
            MotionModule::change_motion(weapon.module_accessor, Hash40::new("final_air_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            if motion_kind == hash40("final_start_ball") {
                MotionModule::change_motion_inherit_frame(weapon.module_accessor, Hash40::new("final_air_start_ball"), -1.0, 1.0, 0.0, false, false);
            }
            MotionModule::change_motion(weapon.module_accessor, Hash40::new("final_air_start_ball"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        if final_charge {
            if motion_kind == hash40("final_air_start") {
                MotionModule::change_motion_inherit_frame(weapon.module_accessor, Hash40::new("final_start"), -1.0, 1.0, 0.0, false, false);
            }
            MotionModule::change_motion(weapon.module_accessor, Hash40::new("final_start"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            if motion_kind == hash40("final_air_start_ball") {
                MotionModule::change_motion_inherit_frame(weapon.module_accessor, Hash40::new("final_start_ball"), -1.0, 1.0, 0.0, false, false);
            }
            MotionModule::change_motion(weapon.module_accessor, Hash40::new("final_start_ball"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    weapon.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    weapon.fastshift(L2CValue::Ptr(supersonic_final_start_main_loop as *const () as _))
}

unsafe extern "C" fn supersonic_final_start_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let final_charge = WorkModule::is_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE);
    if MotionModule::is_end(weapon.module_accessor) {
        if final_charge {
            weapon.change_status(WEAPON_SONIC_SUPERSONIC_STATUS_KIND_RISE.into(), false.into());
        }
        else {
            weapon.change_status(WEAPON_SONIC_SUPERSONIC_STATUS_KIND_IDLE.into(), false.into());
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("sonic_supersonic")
    .status(Pre, *WEAPON_SONIC_SUPERSONIC_STATUS_KIND_START, supersonic_final_start_pre_status)
    .status(Main, *WEAPON_SONIC_SUPERSONIC_STATUS_KIND_START, supersonic_final_start_main_status)
    .install()
    ;
}