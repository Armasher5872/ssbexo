use super::*;

unsafe extern "C" fn yoshi_jump_aerial_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_0 = fighter.status_pre_JumpAerial_sub().get_i32() == 0;
    let should_end = is_0 as i32 & 1 == 0;
    if !should_end {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, true, false, true, 0, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, 0, 0);
    }
    return (should_end as i32).into();
}

unsafe extern "C" fn yoshi_jump_aerial_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let global_stick_x = fighter.global_table[STICK_X].get_f32();
    let turn_count = WorkModule::get_param_int(fighter.module_accessor, hash40("jump_aerial"), hash40("turn_count"));
    let turn_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("jump_aerial"), hash40("turn_cont_value"));
    let aerial_damage_reaction = WorkModule::get_float(fighter.module_accessor, *FIGHTER_YOSHI_INSTANCE_WORK_ID_FLOAT_AERIAL_DAMAGE_REACTION);
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = global_stick_x*lr;
    fighter.status_JumpAerialSub(false.into(), false.into());
    DamageModule::set_no_reaction_mode_status(fighter.module_accessor, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_REACTION_VALUE as u8}, aerial_damage_reaction, -1.0, -1);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_YOSHI_INSTANCE_WORK_ID_FLAG_JUMP_AERIAL_ARMOR);
    if stick_x*-1.0 > turn_cont_value {
        WorkModule::set_int(fighter.module_accessor, turn_count, *FIGHTER_YOSHI_INSTANCE_WORK_ID_INT_AERIAL_TURN_COUNT);
    }
    else {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_YOSHI_INSTANCE_WORK_ID_INT_AERIAL_TURN_COUNT);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(yoshi_jump_aerial_main_loop as *const () as _))
}

unsafe extern "C" fn yoshi_jump_aerial_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    fighter.status_JumpAerial_Main();
    if current_frame > 20.0 {
        DamageModule::set_no_reaction_mode_status(fighter.module_accessor, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_NORMAL as u8}, -1.0, -1.0, -1);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_YOSHI_INSTANCE_WORK_ID_FLAG_JUMP_AERIAL_ARMOR);
    }
    0.into()
}

unsafe extern "C" fn yoshi_attack_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_common(true.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(yoshi_attack_air_main_loop as *const () as _))
}

unsafe extern "C" fn yoshi_attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut BattleObjectModuleAccessor;
    if !yoshi_attack_air_main_common(fighter).get_bool() {
        fighter.sub_air_check_superleaf_fall_slowly();
        if !fighter.global_table[IS_STOP].get_bool() {
            smash::app::FighterUtil::check_cloud_through_out(module_accessor);
        }
    }
    0.into()
}

unsafe extern "C" fn yoshi_attack_air_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if !fighter.attack_air_common_strans().get_bool() {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                if fighter.sub_air_check_fall_common().get_bool() {
                    return true.into();
                }
            }
        }
        if prev_status_kind == *FIGHTER_STATUS_KIND_PASS {
            if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                GroundModule::set_passable_check(fighter.module_accessor, true);
            }
        }
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            if !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_AIR_FLIP) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_AIR_FLIP);
                PostureModule::reverse_lr(fighter.module_accessor);
                PostureModule::update_rot_y_lr(fighter.module_accessor);
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return true.into();
            }
        }
        if prev_status_kind != *FIGHTER_STATUS_KIND_TREAD_JUMP
        && motion_kind == hash40("attack_air_lw")
        && (12.0..=36.0).contains(&frame)
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.015);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.05);
        }
        if MotionModule::is_end(fighter.module_accessor) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return false.into();
    }
    true.into()
}

unsafe extern "C" fn yoshi_special_n_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_GROUND as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, (*FIGHTER_STATUS_ATTR_START_TURN as u32 | *FIGHTER_STATUS_ATTR_DISABLE_CURRY_FACE as u32), *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn yoshi_special_n_1_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_GROUND as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_DISABLE_CURRY_FACE as u32, 0, 0);
    0.into()
}

unsafe extern "C" fn yoshi_special_lw_landing_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("yoshi")
    .status(Pre, *FIGHTER_STATUS_KIND_JUMP_AERIAL, yoshi_jump_aerial_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_JUMP_AERIAL, yoshi_jump_aerial_main_status)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, yoshi_attack_air_main_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, yoshi_special_n_pre_status)
    .status(Pre, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_N_1, yoshi_special_n_1_pre_status)
    .status(Pre, *FIGHTER_YOSHI_STATUS_KIND_SPECIAL_LW_LANDING, yoshi_special_lw_landing_pre_status)
    .install()
    ;
}