use super::*;

unsafe extern "C" fn link_special_hi_glide_land_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {    
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_land_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_glide_land"), 0.0, 1.0, false, 0.0, false, false);
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, -1);
    }
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, Hash40::new("glide_land"), false, -1.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(link_special_hi_glide_land_main_loop as *const () as _))
}

unsafe extern "C" fn link_special_hi_glide_land_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_land_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAND, link_special_hi_glide_land_pre_status)
    .status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAND, link_special_hi_glide_land_main_status)
    .status(End, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAND, link_special_hi_glide_land_end_status)
    .install()
    ;
}