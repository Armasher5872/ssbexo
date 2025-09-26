use super::*;

unsafe extern "C" fn ganon_catch_wait_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if [*FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_CATCH_CUT].contains(&status_kind) {
        EFFECT_OFF_KIND(fighter, Hash40::new("ganon_catch_holdstart"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("ganon_catch_hold"), true, true);
    }
    fighter.status_end_CatchWait()
}

unsafe extern "C" fn ganon_catch_wait_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if [*FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_CATCH_CUT].contains(&status_kind) {
        EFFECT_OFF_KIND(fighter, Hash40::new("ganon_catch_holdstart"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("ganon_catch_hold"), true, true);
    }
    fighter.sub_catch_wait_uniq_process_exit()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_CATCH_PULL, ganon_catch_wait_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_CATCH_PULL, ganon_catch_wait_exit_status)
    .install()
    ;
}