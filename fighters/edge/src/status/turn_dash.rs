use super::*;

unsafe extern "C" fn edge_turn_dash_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    edge_status_turn_dash_sub(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_TurnDash_Main as *const () as _))
}

unsafe extern "C" fn edge_status_turn_dash_sub(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let motion = if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {"turn_dash_wing"} else {"turn_dash"};
    MotionModule::change_motion(boma, Hash40::new(motion), 0.0, 1.0, false, 0.0, false, false);
    WorkModule::unable_transition_term_group_ex(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    fighter.status_DashCommon();
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_TURN_DASH, edge_turn_dash_main_status)
    .install()
    ;
}