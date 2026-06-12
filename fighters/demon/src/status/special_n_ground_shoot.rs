use super::*;

unsafe extern "C" fn demon_special_n_ground_shoot_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::set_int(boma, 0, *FIGHTER_DEMON_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME);
    0.into()
}

unsafe extern "C" fn demon_special_n_ground_shoot_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::set_int(boma, 0, *FIGHTER_DEMON_INSTANCE_WORK_ID_INT_SPECIAL_N_HOLD_FRAME);
    0.into()
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_N_GROUND_SHOOT, demon_special_n_ground_shoot_end_status)
    .status(Exit, *FIGHTER_DEMON_STATUS_KIND_SPECIAL_N_GROUND_SHOOT, demon_special_n_ground_shoot_exit_status)
    .install()
    ;
}