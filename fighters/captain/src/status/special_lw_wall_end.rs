use super::*;

//Down Special Wall Bounce
unsafe extern "C" fn captain_special_lw_wall_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_JUMP) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_throw"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(captain_special_lw_wall_end_main_loop as *const () as _))
}

unsafe extern "C" fn captain_special_lw_wall_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let get_sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if !fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        if get_sum_speed_y <= 0.0 {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_LIGHT.into(), false.into());
        }
    }
    if situation_kind != prev_situation_kind {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        else {
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn captain_special_lw_wall_end_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HIT_JUMP);
    0.into()
}

pub fn install() {
    Agent::new("captain")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_WALL_END, captain_special_lw_wall_end_main_status)
    .status(End, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_WALL_END, captain_special_lw_wall_end_end_status)
    .install()
    ;
}