use super::*;

unsafe extern "C" fn snake_attack_s4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let restart_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("attack_s4_s"), restart_frame, 1.0, 0.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_attack_s4_main_loop as *const () as _))
}

unsafe extern "C" fn snake_attack_s4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SNAKE_INSTANCE_WORK_ID_INT_ATTACK_S4_COUNT);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_SNAKE_INSTANCE_WORK_ID_INT_ATTACK_S4_COUNT);
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return true.into();
    }
    if count == 1 && motion_kind == hash40("attack_s4_s") {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s4_s2"), 0.0, 1.0, false, 0.0, false, false);
    }
    if count == 2 && motion_kind == hash40("attack_s4_s2") {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s4_s3"), 0.0, 1.0, false, 0.0, false, false);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into();
    }
    0.into()
}

unsafe extern "C" fn snake_attack_s4_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SNAKE_INSTANCE_WORK_ID_INT_ATTACK_S4_COUNT);
    0.into()
}

unsafe extern "C" fn snake_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let itemmanager = smash2::app::ItemManager::instance().unwrap();
    let grenade_count = smash2::app::ItemManager::get_num_of_ownered_item(itemmanager, fighter.battle_object_id, smash2::app::ItemKind::Snakegrenade);
    if ItemModule::is_have_item(fighter.module_accessor, 0) {
        ItemModule::drop_item(fighter.module_accessor, 90.0, 0.0, 0);
    }
    if !ArticleModule::is_generatable(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE)
    || grenade_count >= 2 {
        fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_THROW.into(), false.into());
        return 1.into();
    }
    else {
        WorkModule::set_float(fighter.module_accessor, -1.0, *FIGHTER_SNAKE_STATUS_SPECIAL_N_WORK_FLOAT_THROW_RATE);
        if !StopModule::is_stop(fighter.module_accessor) {
            fun_710001c850(fighter, false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_710001c850 as *const () as _));
        WorkModule::set_int64(fighter.module_accessor, hash40("special_n_start") as i64, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_start") as i64, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n_start"), L2CValue::Hash40s("special_air_n_start"), false.into());
        if situation_kind == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(snake_special_n_main_loop as *const () as _));
    }
    0.into()
}

unsafe extern "C" fn fun_710001c850(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    if !bool_check.get_bool() {
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_SPECIAL_N_FLAG_BUTTON_SPECIAL_OFF);
            fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::I32(0));
            fighter.global_table[SUB_STATUS].assign(&L2CValue::I32(0));
        }
    }
    0.into()
}

unsafe extern "C" fn snake_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let mot_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
    let mot_air_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind == *SITUATION_KIND_GROUND 
        && situation_kind == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_air_kind), -1.0, 1.0, 0.0, false, false);
        }
        if prev_situation_kind == *SITUATION_KIND_AIR
        && situation_kind == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_kind), -1.0, 1.0, 0.0, false, false);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn snake_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn snake_special_hi_hang_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("snake")
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4, snake_attack_s4_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_S4, snake_attack_s4_end_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, snake_special_n_main_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, snake_special_hi_pre_status)
    .status(Pre, *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_HI_HANG, snake_special_hi_hang_pre_status)
    .install()
    ;
}