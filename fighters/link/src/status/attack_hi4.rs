use super::*;

unsafe extern "C" fn link_attack_hi4_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
    WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    0.into()
}

unsafe extern "C" fn link_attack_hi4_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
    fighter.sub_attack_xx4_common_uniq_process_exit()
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_HI4, link_attack_hi4_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_ATTACK_HI4, link_attack_hi4_exit_status)
    .install()
    ;
}