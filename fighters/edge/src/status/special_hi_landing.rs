use super::*;

unsafe extern "C" fn edge_special_hi_landing_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_RUSH_CANCEL);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("edge_octaslash_line"), true, true);
    0.into()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING, edge_special_hi_landing_end_status)
    .install()
    ;
}