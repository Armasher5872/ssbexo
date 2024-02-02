use super::*;

/*   AIR LASSO STATUS SCRIPTS   */

unsafe extern "C" fn donkey_air_lasso_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_CATCH as u64, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn donkey_air_lasso_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("air_catch"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(donkey_air_lasso_loop as *const () as _))
}

unsafe extern "C" fn donkey_air_lasso_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        WorkModule::set_float(fighter.module_accessor, 15.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn donkey_air_lasso_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

/*   CATCH STATUS SCRIPTS   */

unsafe extern "C" fn donkey_catch_pull_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation = if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_AIR_LASSO {
        *SITUATION_KIND_AIR
    }
    else {
        *SITUATION_KIND_GROUND
    };
    StatusModule::init_settings(fighter.module_accessor, SituationKind(situation), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, true, false, 0, (*FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE) as u32, 0, 0);
    0.into()
}

unsafe extern "C" fn donkey_catch_pull_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_AIR_LASSO {
        sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
    }
    fighter.status_CatchPull_common(hash40("catch_wait").into());
    fighter.sub_shift_status_main(L2CValue::Ptr(donkey_catch_pull_main_loop as *const () as _))
}

unsafe extern "C" fn donkey_catch_pull_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_AIR_LASSO {
        fighter.change_status(FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START.into(), false.into());
        1.into()
    }
    else {
        fighter.status_CatchPull_Main()
    }
}

unsafe extern "C" fn donkey_catch_wait_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, true, false, 0, (*FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE) as u32, 0, 0);
    0.into()
}

/*   CARGO THROW STATUS SCRIPTS   */

//Status Pre Shouldered Donkey Start
#[skyline::hook(replace = L2CFighterCommon_status_pre_ShoulderedDonkeyStart)]
unsafe extern "C" fn status_pre_shouldered_donkey_start(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, true, false, 0, (*FIGHTER_STATUS_ATTR_NO_DROP_ITEM | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE) as u32, 0, 0);
    0.into()
}

//Status Pre Shouldered Donkey
#[skyline::hook(replace = L2CFighterCommon_status_pre_ShoulderedDonkey)]
unsafe extern "C" fn status_pre_shouldered_donkey(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, true, false, 0, (*FIGHTER_STATUS_ATTR_NO_DROP_ITEM | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE) as u32, 0, 0);
    0.into()
}

unsafe extern "C" fn donkey_shoulder_start_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, 0, 0);
    0.into()
}

/*   NEUTRAL B STATUS SCRIPTS   */

unsafe extern "C" fn donkey_special_n_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, 0, 0, 0, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_DEFAULT_POWER_0);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_DEFAULT_POWER_1);
    0.into()
}

/*   SIDE B (Now Spinning Kong) STATUS SCRIPTS   */

unsafe extern "C" fn donkey_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, 0, 0, 0, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_DISABLE, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn donkey_special_s_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Removes the momentum canceling on Spinning Kong
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let sum_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if situation_kind != *SITUATION_KIND_GROUND {
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.05/*Maximum Horizontal Air Acceleration*/);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.7/*Maximum Horizontal Air Speed*/, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.7/*Maximum Horizontal Air Speed*/, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.9/*Maximum Vertical Air Speed*/);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.0085);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.034/*Maximum Horizontal Ground Acceleration*/);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.5/*Maximum Horizontal Ground Speed*/, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.5/*Maximum Horizontal Ground Speed*/, 0.0);
    }
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, sum_speed, 0.0);
    sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    0.into()
}

unsafe extern "C" fn donkey_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s"), L2CValue::Hash40s("special_air_s"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(donkey_special_s_loop as *const () as _))
}

unsafe extern "C" fn donkey_special_s_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("sys_spin_wind"), false, false);
        sv_module_access::effect(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
    if frame > 38.0 {
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -frame/750.0);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn donkey_special_s_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn donkey_special_s_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn donkey_special_s_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

/*   UP B (Barrel Cannon) STATUS SCRIPTS   */

unsafe extern "C" fn donkey_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_DISABLE, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,  0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn donkey_special_hi_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn donkey_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, false, -1);
    ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    let lr = PostureModule::lr(fighter.module_accessor);
    if lr == -1.0 {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, Hash40::new("entry_l"), true, -1.0);
    }
    else {
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, Hash40::new("entry_r"), true, -1.0);
    }
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi"), L2CValue::Hash40s("special_air_hi"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(donkey_special_hi_loop as *const () as _))
}

unsafe extern "C" fn donkey_special_hi_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    if frame < 21.0 {
        sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    if frame == 21.0 {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let speed = smash::phx::Vector3f{ x: 0.0, y: 3.5, z: 0.0 };
        KineticModule::add_speed(fighter.module_accessor, &speed);
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn donkey_special_hi_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn donkey_special_hi_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    VisibilityModule::set_model_visible(fighter.module_accessor, true);
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    0.into()
}

unsafe extern "C" fn donkey_special_hi_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

/*   DOWN B (Barrel Toss) STATUS SCRIPTS   */

unsafe extern "C" fn donkey_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn donkey_special_lw_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn donkey_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion;
    if donkey_barrel_bool(fighter.module_accessor) {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE);
        WorkModule::set_int(fighter.module_accessor, 900, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_TIMER);
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_BARREL), 0, 0, false, false);
        if ItemModule::get_have_item_kind(fighter.module_accessor, 0) == *ITEM_KIND_BARREL {
            fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY.into(), false.into());
            return 1.into();
        }
    }
    else {
        if PostureModule::lr(fighter.module_accessor) == 1.0 {
            motion = Hash40::new("appeal_lw_r");
        }
        else {
            motion = Hash40::new("appeal_lw_l");
        }
        MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_shift_status_main(L2CValue::Ptr(donkey_special_lw_main_loop as *const () as _));
    }
    0.into()
}

unsafe extern "C" fn donkey_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    1.into()
}

unsafe extern "C" fn donkey_special_lw_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn donkey_special_lw_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn donkey_special_lw_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_shouldered_donkey_start,
            status_pre_shouldered_donkey
        );
    }
}

pub fn install() {
    Agent::new("donkey")
    .status(Pre, *FIGHTER_STATUS_KIND_AIR_LASSO, donkey_air_lasso_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_AIR_LASSO, donkey_air_lasso_main_status)
    .status(End, *FIGHTER_STATUS_KIND_AIR_LASSO, donkey_air_lasso_end_status)
    .status(Pre, *FIGHTER_STATUS_KIND_CATCH_PULL, donkey_catch_pull_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_CATCH_PULL, donkey_catch_pull_main_status)
    .status(Pre, *FIGHTER_STATUS_KIND_CATCH_WAIT, donkey_catch_wait_pre_status)
    .status(Pre, *FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START, donkey_shoulder_start_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, donkey_special_n_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, donkey_special_s_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, donkey_special_s_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, donkey_special_s_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, donkey_special_s_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, donkey_special_s_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, donkey_special_s_exit_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, donkey_special_hi_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, donkey_special_hi_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, donkey_special_hi_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, donkey_special_hi_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, donkey_special_hi_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_HI, donkey_special_hi_exit_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, donkey_special_lw_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, donkey_special_lw_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, donkey_special_lw_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, donkey_special_lw_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, donkey_special_lw_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_LW, donkey_special_lw_exit_status)
    .install()
    ;
    skyline::nro::add_hook(nro_hook);
}