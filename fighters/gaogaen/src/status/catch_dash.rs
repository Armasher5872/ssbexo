use super::*;

unsafe extern "C" fn gaogaen_catch_dash_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    ItemModule::set_have_item_visibility(boma, false, 0);
    gaogaen_sub_status_catch_dash(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(gaogaen_catch_dash_main_loop as *const () as _))
}

unsafe extern "C" fn gaogaen_sub_status_catch_dash(fighter: &mut L2CFighterCommon) {
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let boma = fighter.module_accessor;
    let catch_turn_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("catch_turn_frame"));
    if stick_y >= 0.7 {
        MotionModule::change_motion(boma, Hash40::new("catch_dash_hi"), 0.0, 1.0, false, 0.0, false, false);
    }
    else if stick_y <= -0.7 {
        MotionModule::change_motion(boma, Hash40::new("catch_dash_lw"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(boma, Hash40::new("catch_dash"), 0.0, 1.0, false, 0.0, false, false);
    }
    WorkModule::set_int(boma, catch_turn_frame, *FIGHTER_STATUS_CATCH_DASH_WORK_INT_CATCH_TURN_FRAME);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    if !StopModule::is_stop(boma) {
        fighter.CatchDashUniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_CatchDashUniq as *const () as _));
}

unsafe extern "C" fn gaogaen_catch_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let boma = fighter.module_accessor;
    let lr = PostureModule::lr(boma);
    let turn_run_stick_x = WorkModule::get_param_float(boma, hash40("common"), hash40("turn_run_stick_x"));
    if CancelModule::is_enable_cancel(boma) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN) {
        if stick_x*lr <= turn_run_stick_x {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_TURN.into(), true.into());
        }
    }
    if WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
        if MotionModule::is_end(boma) {
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
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