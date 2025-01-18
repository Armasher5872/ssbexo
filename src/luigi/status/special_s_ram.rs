use super::*;

unsafe extern "C" fn luigi_special_s_ram_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_SPECIAL_S_RAM_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_SPECIAL_S_RAM_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_SPECIAL_S_RAM_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn luigi_special_s_ram_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_LAST_STRANS) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_HIT);
    }
    WorkModule::set_int(fighter.module_accessor, 9, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_S_DISCHARGE_CHANCE);
    0.into()
}

pub fn install() {
    Agent::new("luigi")
    .status(Pre, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_special_s_ram_pre_status)
    .status(End, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, luigi_special_s_ram_end_status)
    .install()
    ;
}