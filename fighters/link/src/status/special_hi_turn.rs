use super::*;

unsafe extern "C" fn link_special_hi_glide_turn_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {    
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_FALL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_turn_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    let max_jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    let lr = PostureModule::lr(fighter.module_accessor);
    if jump_count < max_jump_count {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
    }
    PostureModule::reverse_lr(fighter.module_accessor);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.3);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.3);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.07);
    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 1.0);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.0*lr, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.0, 0.0);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.009, 0.0);
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_turn_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_glide_turn"), 0.0, 1.0, false, 0.0, false, false);
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, -1);
    }
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, Hash40::new("glide_turn"), false, -1.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(link_special_hi_glide_turn_main_loop as *const () as _))
}

unsafe extern "C" fn link_special_hi_glide_turn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let get_stick_prev_y = ControlModule::get_stick_prev_y(fighter.module_accessor);
    let squat_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"));
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if get_stick_prev_y < squat_stick_y {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_DROP.into(), false.into());
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAND.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_turn_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_TURN, link_special_hi_glide_turn_pre_status)
    .status(Init, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_TURN, link_special_hi_glide_turn_init_status)
    .status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_TURN, link_special_hi_glide_turn_main_status)
    .status(End, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_TURN, link_special_hi_glide_turn_end_status)
    .install()
    ;
}