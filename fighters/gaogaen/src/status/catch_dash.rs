use super::*;

unsafe extern "C" fn gaogaen_catch_dash_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    fighter.sub_status_CatchDash();
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_CAN_ANGLE_CATCH);
    fighter.sub_shift_status_main(L2CValue::Ptr(gaogaen_catch_dash_main_loop as *const () as _))
}

unsafe extern "C" fn gaogaen_catch_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let turn_run_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("turn_run_stick_x"));
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() && fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) {
        if stick_x*lr <= turn_run_stick_x {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_CAN_ANGLE_CATCH)
    && MotionModule::motion_kind(fighter.module_accessor) == hash40("catch_dash") {
        if stick_y >= 0.7 {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("catch_dash_hi"), -1.0, 1.0, 0.0, false, false);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_CAN_ANGLE_CATCH);
        }
        else if stick_y <= -0.7 {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("catch_dash_lw"), -1.0, 1.0, 0.0, false, false);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_CAN_ANGLE_CATCH);
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
        if MotionModule::is_end(fighter.module_accessor) {
            if situation_kind != *SITUATION_KIND_GROUND {
                return 0.into();
            }
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("gaogaen")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_CATCH_DASH, gaogaen_catch_dash_main_status)
    .install()
    ;
}