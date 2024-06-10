use super::*;

unsafe extern "C" fn mewtwo_special_n_hold_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let max_charge_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n") as u64, hash40("max_charge_frame") as u64);
    let charge_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME) as f32;
    let end_frame = MotionModule::end_frame(fighter.module_accessor);
    let frame = end_frame*charge_frame/max_charge_frame;
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_hold") as i64, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_hold") as i64, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_AIR_MOT);
    if fighter.global_table[STATUS_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fun_7100008c60(fighter); //Air Motion Change Function
        fun_7100008bc0(fighter); //Air Situation Set Function
        fun_710000ed50(fighter); //Air Transition Term Function
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        fun_7100008920(fighter); //Ground Motion Change Function
        fun_7100008880(fighter); //Ground Situation Set Function
        fun_710000ec30(fighter); //Ground Transition Term Function
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, frame, true, false, false);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.sub_shift_status_main(L2CValue::Ptr(mewtwo_special_n_hold_main_loop as *const () as _))
}

unsafe extern "C" fn fun_7100008c60(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_AIR_MOT);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(air_mot), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    }
    else {
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new_raw(air_mot), -1.0, 1.0, 0.0);
    }
    0.into()
}

unsafe extern "C" fn fun_7100008920(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ground_mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(ground_mot), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    }
    else {
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new_raw(ground_mot), -1.0, 1.0, 0.0);
    }
    0.into()
}

unsafe extern "C" fn fun_7100008bc0(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    0.into()
}

unsafe extern "C" fn fun_7100008880(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_situation(SITUATION_KIND_GROUND.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    0.into()
}

unsafe extern "C" fn fun_710000ed50(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
    0.into()
}

unsafe extern "C" fn fun_710000ec30(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
    0.into()
}

unsafe extern "C" fn mewtwo_special_n_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fun_7100008c60(fighter); //Air Motion Change Function
            fun_7100008bc0(fighter); //Air Situation Set Function
            fun_710000ed50(fighter); //Air Transition Term Function
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        }
        else {
            fun_7100008920(fighter); //Ground Motion Change Function
            fun_7100008880(fighter); //Ground Situation Set Function
            fun_710000ec30(fighter); //Ground Transition Term Function
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
    }
    WorkModule::set_float(fighter.module_accessor, (current_frame/70.588)+1.0, FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLOAT_PSYCHIC_GLARE_POWER);
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        fighter.change_status(FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_SHOOT.into(), true.into());
    }
    else {
        if fun_710000e390(fighter).get_bool() {
            fighter.change_status(FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
        }
        else {
            if !MotionModule::is_end(fighter.module_accessor) {
                return 0.into();
            }
            fighter.gimmick_flash();
            fighter.change_status(FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn fun_710000e390(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            return true.into();
        }
        if fighter.sub_check_jump_in_charging_for_cancel_status((*FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS).into()).get_bool() {
            return true.into();
        }
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if fighter.is_cat_flag(Cat2::StickEscape) {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            } 
            else {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            }
            return true.into();
        }
        if fighter.is_cat_flag(Cat2::StickEscapeF) {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F) {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            } 
            else {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            }
            return true.into();
        }
        if fighter.is_cat_flag(Cat2::StickEscapeB) {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B) {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            } 
            else {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            }
            return true.into();
        }
        if fighter.sub_check_command_guard().get_bool() {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON) {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            } 
            else {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            }
            return true.into();
        }
        if fighter.sub_check_jump_in_charging_for_cancel_status((*FIGHTER_MEWTWO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS).into()).get_bool() {
            return true.into();
        }
    }
    return false.into();
}

unsafe extern "C" fn mewtwo_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_FALL, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, (*FIGHTER_STATUS_ATTR_START_TURN as u32 | *FIGHTER_STATUS_ATTR_INTO_DOOR as u32), *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn mewtwo_special_hi_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn mewtwo_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let pos_z = PostureModule::pos_z(fighter.module_accessor);
    let result = &mut Vector2f{x: 0.0, y: 0.0};
    let ray_check_hit_pos = GroundModule::ray_check_hit_pos(fighter.module_accessor, &Vector2f{x: pos_x, y: pos_y}, &Vector2f{x: 0.0, y: -6.0}, result, true);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi_start"), L2CValue::Hash40s("special_air_hi_start"), false.into());
    if ray_check_hit_pos {
        let ray_check_y = result.y;
        let uniq_float_start_y = 1.5;
        PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: pos_x, y: ray_check_y + uniq_float_start_y, z: pos_z});
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(mewtwo_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn mewtwo_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.global_table[CURRENT_FRAME].get_f32() == 3.0 {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x, 0.0);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn mewtwo_special_hi_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn mewtwo_special_hi_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("mewtwo_final_aura"), false, true);
    0.into()
}

unsafe extern "C" fn mewtwo_special_hi_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn mewtwo_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_SHOOT | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn mewtwo_futuresight_ball_shoot_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn mewtwo_futuresight_ball_shoot_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_bindball"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("shoot"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(mewtwo_futuresight_ball_shoot_main_loop as *const () as _))
}

unsafe extern "C" fn mewtwo_futuresight_ball_shoot_main_loop(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn mewtwo_futuresight_ball_shoot_end_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    macros::STOP_SE(weapon, Hash40::new("se_mewtwo_special_n01"));
    EffectModule::detach_all(weapon.module_accessor, 5);
    0.into()
}

pub fn install() {
    Agent::new("mewtwo")
    .status(Main, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_HOLD, mewtwo_special_n_hold_main_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, mewtwo_special_hi_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, mewtwo_special_hi_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, mewtwo_special_hi_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, mewtwo_special_hi_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, mewtwo_special_hi_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_HI, mewtwo_special_hi_exit_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, mewtwo_special_lw_pre_status)
    .install()
    ;
    Agent::new("mewtwo_bindball")
    .status(Pre, *WEAPON_MEWTWO_BINDBALL_STATUS_KIND_SHOOT, mewtwo_futuresight_ball_shoot_pre_status)
    .status(Main, *WEAPON_MEWTWO_BINDBALL_STATUS_KIND_SHOOT, mewtwo_futuresight_ball_shoot_main_status)
    .status(End, *WEAPON_MEWTWO_BINDBALL_STATUS_KIND_SHOOT, mewtwo_futuresight_ball_shoot_end_status)
    .install()
    ;
}