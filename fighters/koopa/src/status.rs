use super::*;

//Neutral Special

unsafe extern "C" fn koopa_special_n_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, 0, 0, 0, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn koopa_special_n_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_STEP);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_PREV_STEP);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_PREV_GENERATE_KIND);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_SE1_HANDLE);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_SE2_HANDLE);
    if kind != *FIGHTER_KIND_KIRBY {
        if kind != *FIGHTER_KIND_KOOPAG {
            WorkModule::set_int64(fighter.module_accessor, hash40("se_koopa_special_n02") as i64, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_HEAD_NODE);
            WorkModule::set_int64(fighter.module_accessor, hash40("head") as i64, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_HEAD_NODE);
        }
        else {
            WorkModule::set_int64(fighter.module_accessor, hash40("se_koopag_special_n02") as i64, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_HEAD_NODE);
            WorkModule::set_int64(fighter.module_accessor, hash40("head") as i64, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_HEAD_NODE);
        }
    }
    else {
        WorkModule::set_int64(fighter.module_accessor, hash40("se_koopa_special_n02") as i64, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_HEAD_NODE);
        WorkModule::set_int64(fighter.module_accessor, hash40("head") as i64, *FIGHTER_KOOPA_STATUS_BREATH_WORK_INT_HEAD_NODE);
    }
    if situation_kind != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
    }
    0.into()
}

unsafe extern "C" fn koopa_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n"), L2CValue::Hash40s("special_air_n"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(koopa_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn koopa_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n"), -1.0, 1.0, 0.0, false, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn koopa_special_n_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn koopa_special_n_exec_stop_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn koopa_special_n_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn koopa_special_n_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Fireball

unsafe extern "C" fn koopa_firebreath_move_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, 0, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn koopa_firebreath_move_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_float(weapon.module_accessor, hash40("param_breath"), hash40("life")) as i32;
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_breath"), hash40("max_speed"));
    let lr = PostureModule::lr(weapon.module_accessor);
    let owner_boma = get_owner_boma(weapon);
    WorkModule::on_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    PostureModule::set_scale(weapon.module_accessor, 1.0, false);
    if !StopModule::is_stop(weapon.module_accessor) {
        koopa_firebreath_move_substatus(weapon, false.into());
    }
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_max*lr);
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(koopa_firebreath_move_substatus as *const () as _));
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(koopa_firebreath_move_main_loop as *const () as _))
}

unsafe extern "C" fn koopa_firebreath_move_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

unsafe extern "C" fn koopa_firebreath_move_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let pos = PostureModule::pos(weapon.module_accessor);
    let owner_boma = get_owner_boma(weapon);
    if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_ALL) 
    || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32) || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32) 
    || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_UP as u32) || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        EffectModule::req(weapon.module_accessor, Hash40::new("sys_bomb_b"), pos, &Vector3f::zero(), 1.0, 0, -1, false, 0);
        SoundModule::play_se(weapon.module_accessor, Hash40::new("se_common_bomb_m"), true, false, false, false, enSEType(0));
        WorkModule::off_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    if life <= 0 {
        WorkModule::off_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

unsafe extern "C" fn koopa_firebreath_move_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    EffectModule::detach_all(weapon.module_accessor, 5);
    WorkModule::off_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    0.into()
}

//Up Special

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

//Down Special

unsafe extern "C" fn koopa_special_lw_a_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut log_mask_flag = (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG_FROM_GR) {
        log_mask_flag = (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64;
    }
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_SOUND);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, log_mask_flag, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn koopa_special_lw_a_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG_FROM_GR) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 30.0, 1.0, false, 0.0, false, false);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG_FROM_GR);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    GroundModule::set_passable_check(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(koopa_special_lw_a_main_loop as *const () as _))
}

unsafe extern "C" fn koopa_special_lw_a_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
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
    if situation_kind != *SITUATION_KIND_AIR {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_LW_FLAG1) {
            fun_7100006150(fighter);
            fighter.change_status(FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_L.into(), false.into());
            return 1.into();
        }
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    0.into()
}

unsafe extern "C" fn fun_7100006150(fighter: &mut L2CFighterCommon) {
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
}

pub fn install() {
    Agent::new("koopa")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, koopa_special_n_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, koopa_special_n_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, koopa_special_n_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, koopa_special_n_exec_status)
    .status(ExecStop, *FIGHTER_STATUS_KIND_SPECIAL_N, koopa_special_n_exec_stop_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, koopa_special_n_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_N, koopa_special_n_exit_status)
    .status(Pre, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, koopa_special_hi_a_pre_status)
    .status(Main, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, koopa_special_hi_a_main_status)
    .status(End, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, koopa_special_hi_a_end_status)
    .status(Pre, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_A, koopa_special_lw_a_pre_status)
    .status(Main, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_A, koopa_special_lw_a_main_status)
    .install()
    ;
    Agent::new("koopa_breath")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_KOOPA_BREATH_STATUS_KIND_MOVE, koopa_firebreath_move_pre_status)
    .status(Main, *WEAPON_KOOPA_BREATH_STATUS_KIND_MOVE, koopa_firebreath_move_main_status)
    .status(End, *WEAPON_KOOPA_BREATH_STATUS_KIND_MOVE, koopa_firebreath_move_end_status)
    .install()
    ;
}