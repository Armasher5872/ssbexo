use super::*;

unsafe extern "C" fn kamui_attack_lw4_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_LW4)(fighter);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_mask"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_rspearfoot"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_lspearfoot"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_wing"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_fronthair"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_hair"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_rfoot"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_lfoot"), true);
    ret
}

unsafe extern "C" fn kamui_attack_lw4_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original_status(Exit, fighter, *FIGHTER_STATUS_KIND_ATTACK_LW4)(fighter);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_mask"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_rspearfoot"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_lspearfoot"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_wing"), false);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_fronthair"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_hair"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_rfoot"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kamui_lfoot"), true);
    ret
}

unsafe extern "C" fn kamui_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_START);
        StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_VISIBILITY);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_START);
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK);
    }
    0.into()
}

unsafe extern "C" fn kamui_special_s_attack_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_START) {
        StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_VISIBILITY);
    }
    else {
        StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_VISIBILITY);
    }
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("kamui")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_LW4, kamui_attack_lw4_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_ATTACK_LW4, kamui_attack_lw4_exit_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, kamui_special_s_pre_status)
    .status(Pre, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK, kamui_special_s_attack_pre_status)
    .install()
    ;
}