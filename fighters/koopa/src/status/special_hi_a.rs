use super::*;

unsafe extern "C" fn koopa_special_hi_a_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut log_mask_flag = (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64;
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G {
        log_mask_flag = (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64;
    }
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_SOUND);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, log_mask_flag, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn koopa_special_hi_a_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_hi_use_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KOOPA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USE_COUNT);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG_MOT_RESTART) {
        if special_hi_use_count == 1 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_fall"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG_MOT_RESTART);
    }
    else {
        if special_hi_use_count == 1 {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi_fall"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi"), -1.0, 1.0, 0.0, false, false);
        }
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_RESTART_FRAME);
    fighter.sub_shift_status_main(L2CValue::Ptr(koopa_special_hi_a_main_loop as *const () as _))
}

unsafe extern "C" fn koopa_special_hi_a_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let special_hi_use_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KOOPA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USE_COUNT);
    let jump_restart_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_RESTART_FRAME);
    let special_hi_jump_restart_start_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_jump_restart_start_frame"));
    let special_hi_jump_restart_end_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_jump_restart_end_frame"));
    let special_hi_jump_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_jump_speed_y"));
    let special_hi_jump_restart_prohibition_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("special_hi_jump_restart_prohibition_frame"));
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if situation_kind != *SITUATION_KIND_GROUND {
        if current_frame >= special_hi_jump_restart_start_frame && current_frame <= special_hi_jump_restart_end_frame {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && special_hi_jump_restart_prohibition_frame < current_frame-jump_restart_frame {
                WorkModule::set_float(fighter.module_accessor, current_frame, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_WORK_FLOAT_JUMP_RESTART_FRAME);
                sv_kinetic_energy!(add_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_hi_jump_speed_y);
                sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_hi_jump_speed_y);
            }
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if special_hi_use_count == 1 {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn koopa_special_hi_a_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_hi_use_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KOOPA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USE_COUNT);
    if special_hi_use_count < 1 {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_KOOPA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USE_COUNT);
    }
    original_status(End, fighter, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A)(fighter)
}

pub fn install() {
    Agent::new("koopa")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, koopa_special_hi_a_pre_status)
    .status(Main, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, koopa_special_hi_a_main_status)
    .status(End, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, koopa_special_hi_a_end_status)
    .install()
    ;
}