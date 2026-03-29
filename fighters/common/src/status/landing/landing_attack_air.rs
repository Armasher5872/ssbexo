use super::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_LandingAttackAir)]
unsafe extern "C" fn status_pre_landing_attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&prev_status_kind) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP); //Makes wavedashes edge cancelable
    }
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE);
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_F);
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_B);
    }
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_LANDING_ATTACK_AIR_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_LANDING_ATTACK_AIR_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_LANDING_ATTACK_AIR_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, true, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32, 0);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(status_pre_landing_attack_air);
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}