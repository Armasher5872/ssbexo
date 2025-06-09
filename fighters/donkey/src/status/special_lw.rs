use super::*;

//Special Lw Pre Status
unsafe extern "C" fn donkey_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

//Special Lw Init Status
unsafe extern "C" fn donkey_special_lw_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Special Lw Main Status
unsafe extern "C" fn donkey_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let throw_hi_status_kind = fighter.global_table[THROW_HI_STATUS_KIND].get_i32();
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE) || !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_BARREL) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_BARREL, false, -1);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_BARREL, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_LINKED);
    fighter.change_status(throw_hi_status_kind.into(), false.into());
    1.into()
}

//Special Lw Exec Status
unsafe extern "C" fn donkey_special_lw_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Special Lw End Status
unsafe extern "C" fn donkey_special_lw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_LINKED);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE);
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_BARREL) {
            let barrel_boma = get_article_boma(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_BARREL);
            StatusModule::change_status_request_from_script(barrel_boma, *WEAPON_DONKEY_BARREL_STATUS_KIND_BREAK, false);
        }
    }
    0.into()
}

//Special Lw Exit Status
unsafe extern "C" fn donkey_special_lw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_LINKED);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE);
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_BARREL) {
            let barrel_boma = get_article_boma(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_BARREL);
            StatusModule::change_status_request_from_script(barrel_boma, *WEAPON_DONKEY_BARREL_STATUS_KIND_BREAK, false);
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("donkey")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, donkey_special_lw_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, donkey_special_lw_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, donkey_special_lw_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, donkey_special_lw_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, donkey_special_lw_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_LW, donkey_special_lw_exit_status)
    .install()
    ;
}