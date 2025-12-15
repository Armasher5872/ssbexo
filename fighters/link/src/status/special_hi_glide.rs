//Credit to AParticularUser for the Link Gliding Code
use super::*;

unsafe extern "C" fn link_special_hi_glide_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {    
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_FALL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.3);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.3);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.07);
    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 1.0);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.0*lr, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.0, 0.0);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.009, 0.0);
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_glide"), 0.0, 1.0, false, 0.0, false, false);
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, true, -1);
    }
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, Hash40::new("glide"), false, -1.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(link_special_hi_glide_main_loop as *const () as _))
}

unsafe extern "C" fn link_special_hi_glide_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let get_stick_prev_y = ControlModule::get_stick_prev_y(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let slow_rate = SlowModule::rate(fighter.module_accessor);
    let special_hi_degree = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_DEGREE);
    let stamina = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_STAMINA);
    let squat_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"));
    let max_degree = 25.0;
    let change_degree_per_frame = 1.5*slow_rate;
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if get_stick_prev_y < squat_stick_y || stamina <= 0 {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_DROP.into(), false.into());
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_LAND.into(), false.into());
    }
    if stick_x*lr > 0.25 {
        if special_hi_degree < max_degree {
            WorkModule::set_float(fighter.module_accessor, special_hi_degree+change_degree_per_frame, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_DEGREE);
        }
        if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) || GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACH_WALL.into(), false.into());
        }
    }
    if stick_x*lr < -0.25 {
        if special_hi_degree > -max_degree {
            WorkModule::set_float(fighter.module_accessor, special_hi_degree-change_degree_per_frame, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_DEGREE);
        }
        if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) || GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACH_WALL.into(), false.into());
        }
    }
    change_angle(fighter, special_hi_degree, max_degree, "special_hi_glide_f", "special_hi_glide_b");
    WorkModule::sub_int(fighter.module_accessor, 1, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_STAMINA);
    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_glide"), 0.0, 1.0, false, 0.0, false, false);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL, Hash40::new("glide"), false, -1.0);
    }
    0.into()
}

unsafe extern "C" fn link_special_hi_glide_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let stamina = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_STAMINA);
    if status_kind != *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE {
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_DEGREE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_NO_GAIN);
    }
    if stamina <= 0 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    }
    0.into()
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE, link_special_hi_glide_pre_status)
    .status(Init, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE, link_special_hi_glide_init_status)
    .status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE, link_special_hi_glide_main_status)
    .status(End, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE, link_special_hi_glide_end_status)
    .install()
    ;
}