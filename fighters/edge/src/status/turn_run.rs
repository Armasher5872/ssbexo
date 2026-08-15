use super::*;

unsafe extern "C" fn edge_turn_run_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let motion = if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {"turn_run_wing"} else {"turn_run"};
    fighter.status_TurnRun_Sub(hash40(motion).into(), true.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_TurnRun_Main as *const () as _))
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_TURN_RUN, edge_turn_run_main_status)
    .install()
    ;
}