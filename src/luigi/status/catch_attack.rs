use super::*;

unsafe extern "C" fn luigi_catch_attack_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchAttack()
}

pub fn install() {
    Agent::new("luigi")
    .status(End, *FIGHTER_STATUS_KIND_CATCH_ATTACK, luigi_catch_attack_end_status)
    .install()
    ;
}