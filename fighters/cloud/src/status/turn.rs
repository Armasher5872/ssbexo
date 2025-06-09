use super::*;

unsafe extern "C" fn cloud_turn_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    cloud_status_pre_turncommon(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Turn_Main as *const () as _))
}

unsafe extern "C" fn cloud_status_pre_turncommon(fighter: &mut L2CFighterCommon) {
    let group_terms = [
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND
    ];
    let group_terms_ex = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B
    ];
    let motion = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_PUNISHER_MODE) {"punish_turn"} else {"turn"};
    for x in 0..group_terms.len() {
        WorkModule::enable_transition_term_group(fighter.module_accessor, group_terms[x]);
    }
    for y in 0..group_terms_ex.len() {
        WorkModule::unable_transition_term_group_ex(fighter.module_accessor, group_terms_ex[y]);
    }
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_TURN_ATTACK_S4_REV_PAD);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new(motion), 0.0, 1.0, false, 0.0, false, false);
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_TURN, cloud_turn_main_status)
    .install()
    ;
}