use super::*;

unsafe extern "C" fn link_special_hi_glide_start_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_FALL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_start_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let get_sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if get_sum_speed_y > 0.0 {
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, get_sum_speed_y*1.3);
    }
    else {
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, get_sum_speed_y*0.5);
    }
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.5);
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_start_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::sub_int(fighter.module_accessor, 30, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_STAMINA);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_glide_start"), 0.0, 1.0, false, 0.0, false, false);
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, -1);
    }
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, Hash40::new("special_hi_glide_start"), false, -1.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(link_special_hi_glide_start_main_loop as *const () as _))
}

unsafe extern "C" fn link_special_hi_glide_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAND.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_start_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_start_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_CHARGE_FRAME);
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_start_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_SPECIAL_HI_CHARGE_FRAME);
    0.into()
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE_START, link_special_hi_glide_start_pre_status)
    .status(Init, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE_START, link_special_hi_glide_start_init_status)
    .status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE_START, link_special_hi_glide_start_main_status)
    .status(Exec, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE_START, link_special_hi_glide_start_exec_status)
    .status(End, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE_START, link_special_hi_glide_start_end_status)
    .status(Exit, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE_START, link_special_hi_glide_start_exit_status)
    .install()
    ;
}