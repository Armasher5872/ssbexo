use super::*;

unsafe extern "C" fn lucas_jump_aerial_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_0 = fighter.status_pre_JumpAerial_sub().get_i32() == 0;
    let should_end = is_0 as i32 & 1 == 0;
    if !should_end {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, true, false, true, 0, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, 0, 0);
    }
    return (should_end as i32).into();
}

unsafe extern "C" fn lucas_attack_s4_start_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackS4Start_Common();
    fighter.sub_shift_status_main(L2CValue::Ptr(lucas_attack_s4_start_main_loop as *const () as _))
}

unsafe extern "C" fn lucas_attack_s4_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let smash_hold = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if 0 < jump_attack_frame {
        if !StopModule::is_stop(fighter.module_accessor)
        && fighter.sub_check_button_jump().get_bool() {
            let log = fighter.status_attack();
            let info = log[0x10f40d7b92u64].get_i64();
            let mot = MotionModule::motion_kind(fighter.module_accessor);
            MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(mot), -1);
            WorkModule::set_int64(fighter.module_accessor, info, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            fighter.change_status_jump_mini_attack(true.into());
            return 1.into();
        }
    }
    if jump_attack_frame == 1 {
        if !fighter.global_table[IS_STOP].get_bool()
        && 0 < attack_kind {
            FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
            WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_HOLD) {
        if smash_hold {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4_HOLD.into(), true.into());
                return 1.into();
            }
        }
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4) {
        if smash_hold {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4.into(), false.into());
            return 1.into();
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_ATTACK_S4_FLAG_REFLECT_START) {
        shield!(fighter, *MA_MSC_SHIELD_SET_STATUS, *COLLISION_KIND_REFLECTOR, *FIGHTER_LUCAS_REFLECTOR_KIND_BAT, *SHIELD_STATUS_NORMAL, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_ATTACK_S4_FLAG_REFLECT_START);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_ATTACK_S4_HIT_REFLECT) {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_S4.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn lucas_attack_s4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_ATTACK_S4_FLAG_REFLECT_START);
    fighter.sub_AttackS4(true.into());
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucas_attack_s4_main_loop as *const () as _))
}

unsafe extern "C" fn lucas_attack_s4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let combo = ComboModule::count(fighter.module_accessor) as i32;
    let s4_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("s4_combo_max"), 0);
    if !StatusModule::is_changing(fighter.module_accessor) {
        if combo < s4_combo_max
        && fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
            fighter.attack_s4_mtrans();
        }
    }
    else {
        fighter.attack_s4_mtrans();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_ATTACK_S4_FLAG_REFLECT_START) {
        shield!(fighter, *MA_MSC_SHIELD_SET_STATUS, *COLLISION_KIND_REFLECTOR, *FIGHTER_LUCAS_REFLECTOR_KIND_BAT, *SHIELD_STATUS_NORMAL, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_ATTACK_S4_FLAG_REFLECT_START);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_ATTACK_S4_FLAG_REFLECT_END) {
        shield!(fighter, *MA_MSC_SHIELD_SET_STATUS, *COLLISION_KIND_REFLECTOR, *FIGHTER_LUCAS_REFLECTOR_KIND_BAT, *SHIELD_STATUS_NONE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_ATTACK_S4_FLAG_REFLECT_END);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_ATTACK_S4_HIT_REFLECT)
    && fighter.global_table[CURRENT_FRAME].get_f32() < 14.0 {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_ATTACK_S4_HIT_REFLECT);
        MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 14.0, true, false, false);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn lucas_attack_s4_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
                macros::EFFECT(fighter, Hash40::new("starman_smash"), Hash40::new("top"), 0, 5, 12, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn lucas_special_guard_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn lucas_special_guard_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucas_special_guard_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_guard"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucas_special_guard_main_loop as *const () as _))
}

unsafe extern "C" fn lucas_special_guard_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
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
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if frame == 120.0 {
        fighter.gimmick_flash();
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

unsafe extern "C" fn lucas_special_guard_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucas_special_guard_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP);
    WorkModule::set_int(fighter.module_accessor, 720, FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_TIMER);
    0.into()
}

unsafe extern "C" fn lucas_special_guard_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucas_special_guard_burst_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn lucas_special_guard_burst_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucas_special_guard_burst_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_guard_burst"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucas_special_guard_burst_main_loop as *const () as _))
}

unsafe extern "C" fn lucas_special_guard_burst_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
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

unsafe extern "C" fn lucas_special_guard_burst_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucas_special_guard_burst_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP);
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_TIMER);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("lucas_pkfr_hold"), false, false);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_status_defense_up"), false, false);
    0.into()
}

unsafe extern "C" fn lucas_special_guard_burst_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP);
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_DEFENSE_UP_TIMER);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("lucas_pkfr_hold"), false, false);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_status_defense_up"), false, false);
    0.into()
}

unsafe extern "C" fn lucas_special_n_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_ACTIVE_PK_FREEZE) {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_ACTIVE_PK_FREEZE);
        fighter.change_status(FIGHTER_LUCAS_STATUS_KIND_SPECIAL_N_FIRE.into(), false.into());
    }
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn lucas_pkfreeze_move_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let get_energy_gravity = KineticModule::get_energy(weapon.module_accessor, *WEAPON_LUCAS_PK_FREEZE_KINETIC_ENERGY_ID_GRAVITY) as *mut smash::app::KineticEnergy;
    KineticModule::change_kinetic(weapon.module_accessor, *WEAPON_KINETIC_TYPE_RESET);
    smash::app::lua_bind::KineticEnergy::clear_speed(get_energy_gravity);
    smash::app::lua_bind::KineticEnergy::unable(get_energy_gravity);
    EffectModule::req_follow(weapon.module_accessor, Hash40::new("lucas_pkfr_bullet_ed"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, false, 0, 0, -1, 0, 0, false, false);
    WorkModule::set_flag(owner_boma, true, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_ACTIVE_PK_FREEZE);
    0.into()
}

unsafe extern "C" fn lucas_pkfreeze_tame_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let bang_time = WorkModule::get_param_int(weapon.module_accessor, hash40("param_pkfreeze"), hash40("bang_time"));
    WorkModule::set_int(weapon.module_accessor, bang_time, *WEAPON_LUCAS_PK_FREEZE_INSTANCE_WORK_ID_INT_FRAME);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("tame"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(lucas_pkfreeze_tame_main_loop as *const () as _))
}

unsafe extern "C" fn lucas_pkfreeze_tame_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_pos_x = PostureModule::pos_x(owner_boma);
    let owner_pos_y = PostureModule::pos_y(owner_boma);
    let owner_pos_z = PostureModule::pos_z(owner_boma);
    if StatusModule::status_kind(owner_boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW && WorkModule::is_flag(owner_boma, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_ACTIVE_PK_FREEZE) {
        PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x, y: owner_pos_y+6.0, z: owner_pos_z+11.5});
        WorkModule::set_int(weapon.module_accessor, 7, *WEAPON_LUCAS_PK_FREEZE_INSTANCE_WORK_ID_INT_FRAME);
    }
    if WorkModule::count_down_int(weapon.module_accessor, *WEAPON_LUCAS_PK_FREEZE_INSTANCE_WORK_ID_INT_FRAME, 0) || !WorkModule::is_flag(owner_boma, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_ACTIVE_PK_FREEZE) {
        WorkModule::set_flag(owner_boma, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_ACTIVE_PK_FREEZE);
        weapon.change_status(WEAPON_LUCAS_PK_FREEZE_STATUS_KIND_BANG.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn lucas_special_s_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    0.into()
}

unsafe extern "C" fn lucas_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s"), L2CValue::Hash40s("special_air_s"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(lucas_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn lucas_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP)
    && fighter.global_table[CURRENT_FRAME].get_f32() > 42.0 {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn lucas_special_s_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL {
        let mut landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("landing_frame"));
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
            landing_frame = 8.0;
        }
        if 0.0 < landing_frame {
            WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        }
    }
    0.into()
}

unsafe extern "C" fn lucas_pkfire_shoot_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_pkfire"), hash40("life"));
    let speed;
    let lr = PostureModule::lr(weapon.module_accessor);
    if WorkModule::is_flag(owner_boma, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_DEFENSE_UP) {
        speed = 4.0;
    }
    else {
        speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_pkfire"), hash40("speed_air"));
    }
    HitModule::set_status(weapon.module_accessor, 0, HitStatus(*HIT_STATUS_NORMAL), 0);
    HitModule::set_status(weapon.module_accessor, 1, HitStatus(*HIT_STATUS_OFF), 0);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    KineticModule::unable_energy(weapon.module_accessor, *WEAPON_LUCAS_PK_FIRE_KINETIC_ENERGY_ID_GRAVITY);
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed*lr, 0.0);
    0.into()
}

unsafe extern "C" fn lucas_pkfire_shoot_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("shoot"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(weapon.module_accessor) {
        fun_7100023f80(weapon, false.into());
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_7100023f80 as *const () as _));
    weapon.fastshift(L2CValue::Ptr(lucas_pkfire_shoot_main_loop as *const () as _))
}

unsafe extern "C" fn fun_7100023f80(weapon: &mut L2CWeaponCommon, bool_check: L2CValue) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if bool_check.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        if life > 0 {
            if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
                return 0.into();
            }
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        }
        else {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        }
    }
    0.into()
}

unsafe extern "C" fn lucas_pkfire_shoot_main_loop(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucas_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn lucas_special_lw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LUCAS_AIR_STOP_X_NORMAL_MAX_SPECIAL_LW);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    0.into()
}

unsafe extern "C" fn lucas_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stop_y_time = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40("stop_y_time"));
    let start_xmul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("start_xmul"));
    let start_ymul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("start_ymul"));
    WorkModule::set_int(fighter.module_accessor, stop_y_time, *FIGHTER_LUCAS_STATUS_SPECIAL_LW_WORK_INT_STOP_Y_TIME);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: start_xmul, y: start_ymul, z: 1.0}, -1);
    }
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw"), L2CValue::Hash40s("special_air_lw"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(lucas_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn lucas_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let stop_y_time = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_LW_WORK_INT_STOP_Y_TIME);
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
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LUCAS_AIR_STOP_X_NORMAL_MAX_SPECIAL_LW);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, false, false);
    }
    if 0 < stop_y_time {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_LUCAS_STATUS_SPECIAL_LW_WORK_INT_STOP_Y_TIME);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    if stop_y_time <= 0
    && situation_kind == *SITUATION_KIND_AIR {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn lucas_special_lw_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn lucas_special_lw_catch_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_CATCH as u64, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn lucas_special_lw_catch_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    0.into()
}

unsafe extern "C" fn lucas_special_lw_catch_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_catch"), L2CValue::Hash40s("special_air_lw_catch"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(lucas_special_lw_catch_main_loop as *const () as _))
}

unsafe extern "C" fn lucas_special_lw_catch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_catch"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_catch"), -1.0, 1.0, 0.0, false, false);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn lucas_special_lw_catch_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_PSI_COUNTER_ATTACKED);
    0.into()
}

unsafe extern "C" fn lucas_special_lw_catch_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_PSI_COUNTER_ATTACKED);
    0.into()
}

unsafe extern "C" fn lucas_special_lw_throw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, true, false, 0, (*FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE) as u32, 0, 0);
    0.into()
}

unsafe extern "C" fn lucas_special_lw_throw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    0.into()
}

unsafe extern "C" fn lucas_special_lw_throw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    grabbed_anim_selector(fighter, "thrown_hi", 1.0);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw_throw"), L2CValue::Hash40s("special_air_lw_throw"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(lucas_special_lw_throw_main_loop as *const () as _))
}

unsafe extern "C" fn lucas_special_lw_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
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
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_throw"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_throw"), -1.0, 1.0, 0.0, false, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_PSI_COUNTER_THROWN) {
        if capture_id != 0x50000000 {
            AttackModule::hit_absolute_joint(fighter.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, capture_id as u32, Hash40::new("throw"), 0, 0);
        }
        CameraModule::reset_all(fighter.module_accessor);  
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_LUCAS_INSTANCE_WORK_ID_FLAG_PSI_COUNTER_THROWN);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn lucas_special_lw_throw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_throw_uniq_process_exit()
}

pub fn install() {
    Agent::new("lucas")
    .status(Pre, *FIGHTER_STATUS_KIND_JUMP_AERIAL, lucas_jump_aerial_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4_START, lucas_attack_s4_start_main_status)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4, lucas_attack_s4_main_status)
    .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_S4, lucas_attack_s4_check_attack_status)
    .status(Pre, FIGHTER_STATUS_KIND_SPECIAL_GUARD, lucas_special_guard_pre_status)
    .status(Init, FIGHTER_STATUS_KIND_SPECIAL_GUARD, lucas_special_guard_init_status)
    .status(Main, FIGHTER_STATUS_KIND_SPECIAL_GUARD, lucas_special_guard_main_status)
    .status(Exec, FIGHTER_STATUS_KIND_SPECIAL_GUARD, lucas_special_guard_exec_status)
    .status(End, FIGHTER_STATUS_KIND_SPECIAL_GUARD, lucas_special_guard_end_status)
    .status(Exit, FIGHTER_STATUS_KIND_SPECIAL_GUARD, lucas_special_guard_exit_status)
    .status(Pre, FIGHTER_LUCAS_STATUS_KIND_SPECIAL_GUARD_BURST, lucas_special_guard_burst_pre_status)
    .status(Init, FIGHTER_LUCAS_STATUS_KIND_SPECIAL_GUARD_BURST, lucas_special_guard_burst_init_status)
    .status(Main, FIGHTER_LUCAS_STATUS_KIND_SPECIAL_GUARD_BURST, lucas_special_guard_burst_main_status)
    .status(Exec, FIGHTER_LUCAS_STATUS_KIND_SPECIAL_GUARD_BURST, lucas_special_guard_burst_exec_status)
    .status(End, FIGHTER_LUCAS_STATUS_KIND_SPECIAL_GUARD_BURST, lucas_special_guard_burst_end_status)
    .status(Exit, FIGHTER_LUCAS_STATUS_KIND_SPECIAL_GUARD_BURST, lucas_special_guard_burst_exit_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, lucas_special_n_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, lucas_special_s_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, lucas_special_s_main_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, lucas_special_s_exit_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, lucas_special_lw_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, lucas_special_lw_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, lucas_special_lw_main_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, lucas_special_lw_end_status)
    .status(Pre, FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_CATCH, lucas_special_lw_catch_pre_status)
    .status(Init, FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_CATCH, lucas_special_lw_catch_init_status)
    .status(Main, FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_CATCH, lucas_special_lw_catch_main_status)
    .status(End, FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_CATCH, lucas_special_lw_catch_end_status)
    .status(Exit, FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_CATCH, lucas_special_lw_catch_exit_status)
    .status(Pre, FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_THROW, lucas_special_lw_throw_pre_status)
    .status(Init, FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_THROW, lucas_special_lw_throw_init_status)
    .status(Main, FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_THROW, lucas_special_lw_throw_main_status)
    .status(Exit, FIGHTER_LUCAS_STATUS_KIND_SPECIAL_LW_THROW, lucas_special_lw_throw_exit_status)
    .install()
    ;
    Agent::new("lucas_pkfreeze")
    .status(End, *WEAPON_LUCAS_PK_FREEZE_STATUS_KIND_MOVE, lucas_pkfreeze_move_end_status)
    .status(Main, *WEAPON_LUCAS_PK_FREEZE_STATUS_KIND_TAME, lucas_pkfreeze_tame_main_status)
    .install()
    ;
    Agent::new("lucas_pkfire")
    .status(Init, *WEAPON_LUCAS_PK_FIRE_STATUS_KIND_SHOOT, lucas_pkfire_shoot_init_status)
    .status(Main, *WEAPON_LUCAS_PK_FIRE_STATUS_KIND_SHOOT, lucas_pkfire_shoot_main_status)
    .install()
    ;
}