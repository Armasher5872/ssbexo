use super::*;

//Special N Pre Status
unsafe extern "C" fn donkey_special_n_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, 0, 0, 0, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_DEFAULT_POWER_0);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_DEFAULT_POWER_1);
    0.into()
}

pub fn install() {
    Agent::new("donkey")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, donkey_special_n_pre_status)
    .install()
    ;
}