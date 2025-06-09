use super::*;

unsafe extern "C" fn luigi_catch_turn_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchTurn()
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_CATCH_TURN, luigi_catch_turn_end_status)
    .install()
    ;
}