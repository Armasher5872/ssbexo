use super::*;

unsafe extern "C" fn ganon_catch_pull_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if [*FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_CATCH_CUT].contains(&status_kind) {
        EFFECT_OFF_KIND(fighter, Hash40::new("ganon_catch_holdstart"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("ganon_catch_hold"), true, true);
    }
    fighter.status_end_CatchPull()
}

unsafe extern "C" fn ganon_catch_pull_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if [*FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_CATCH_CUT].contains(&status_kind) {
        if fighter.sub_is_not_throw_status_kind(status_kind.into()).get_bool() {
            CatchModule::catch_cut(fighter.module_accessor, false, false);
        }
        EFFECT_OFF_KIND(fighter, Hash40::new("ganon_catch_holdstart"), true, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("ganon_catch_hold"), true, true);
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_PULL_FLAG_UNNECESSARY_CLEAR_TRANS) {
        ModelModule::clear_joint_srt(fighter.module_accessor, Hash40::new("trans"));
    }
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_CATCH_PULL, ganon_catch_pull_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_CATCH_PULL, ganon_catch_pull_exit_status)
    .install()
    ;
}