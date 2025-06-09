#![allow(unused_assignments)]
use super::*;

unsafe extern "C" fn link_attack_lw_4_hold_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut log_mask_flag = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_ATTACK_LW4 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_HAJIKI | *FIGHTER_STATUS_ATTR_ENABLE_ROCKETBELT_EJECT | *FIGHTER_STATUS_ATTR_INTO_DOOR;
    if 0 < WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND) {
        log_mask_flag = *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP;
    }
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_HOLD_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_HOLD_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_HOLD_FLOAT, (*FS_SUCCEEDS_KEEP_EFFECT | *FS_SUCCEEDS_KEEP_VISIBILITY | *FS_SUCCEEDS_KEEP_SOUND | *FS_SUCCEEDS_KEEP_NO_REACTION) as i32);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, log_mask_flag as u64, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn link_attack_lw_4_hold_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(fighter.module_accessor);
    let attack_4_hold_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("attack_4_hold_frame"), 0);
    let attack_lw4_hold_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_lw4_hold_frame"), 0);
    let attack_lw4_hold_keep_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_lw4_hold_keep_frame"), 0);
    let ratio = attack_lw4_hold_frame as f32/attack_4_hold_frame;
    WorkModule::set_int(fighter.module_accessor, ratio as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_BASE_FRAME);
    WorkModule::set_int(fighter.module_accessor, ratio as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_FRAME);
    WorkModule::set_int(fighter.module_accessor, ratio as i32, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_LOOP_TOTAL_FRAME);
    WorkModule::set_int(fighter.module_accessor, attack_lw4_hold_keep_frame, *FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_HOLD_KEEP_FRAME);
    if stick_x >= 0.5 {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_LOOP);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw4_hold_walk_f"), 0.0, attack_lw4_hold_keep_frame as f32/ratio, false, 0.0, false, false);
    }
    else if stick_x <= -0.5 {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_LOOP);
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw4_hold_walk_b"), 0.0, attack_lw4_hold_keep_frame as f32/ratio, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw4_hold"), 0.0, attack_lw4_hold_keep_frame as f32/ratio, false, 0.0, false, false);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_smash_hold_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_smash_hold_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(link_attack_lw_4_hold_main_loop as *const () as _))
}

unsafe extern "C" fn link_attack_lw_4_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_stop = fighter.global_table[IS_STOP].get_bool();
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(fighter.module_accessor);
    let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    let mini_jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let mut ret: i32 = 0;
    if situation_kind != *SITUATION_KIND_AIR {
        if 0 < mini_jump_attack_frame {
            if !StopModule::is_stop(fighter.module_accessor) {
                if fighter.sub_check_button_jump().get_bool() {
                    MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(0x10f40d7b92), -1);
                    WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                    fighter.change_status_jump_mini_attack(true.into());
                    ret = 1;
                }
            }
        }
        if mini_jump_attack_frame == 1 {
            if !is_stop {
                if 0 < attack_kind {
                    FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
                    WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                }
            }
        }
        if current_frame >= 59.0 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) && motion_kind == hash40("attack_lw4_hold") {
            if stick_x >= 0.5 {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_LOOP);
                GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("attack_lw4_hold_walk_f"), -1.0, 1.0, 0.0);
            }
            if stick_x <= -0.5 {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_LOOP);
                GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("attack_lw4_hold_walk_b"), -1.0, 1.0, 0.0);
            }
        }
        if [hash40("attack_lw4_hold_walk_f"), hash40("attack_lw4_hold_walk_b")].contains(&motion_kind) {
            if motion_kind == hash40("attack_lw4_hold_walk_f") {
                if stick_x <= -0.5 {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("attack_lw4_hold_walk_b"), -1.0, 1.0, 0.0);
                }
            }
            if motion_kind == hash40("attack_lw4_hold_walk_b") {
                if stick_x >= 0.5 {
                    MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("attack_lw4_hold_walk_f"), -1.0, 1.0, 0.0);
                }
            }
            if (-0.5..0.5).contains(&stick_x) {
                MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("attack_lw4_hold"), -1.0, 1.0, 0.0);
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("attack_lw4_hold"), -1.0, 1.0, 0.0);
            }
        }
        if !fighter.is_smash_hold(FIGHTER_STATUS_ATTACK_WORK_INT_SMASH_HOLD_KEEP_FRAME.into()).get_bool() {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW4.into(), true.into());
            ret = 1;
        }
        ret = 0;
    }
    else {
        if [hash40("attack_lw4_hold_walk_f"), hash40("attack_lw4_hold_walk_b")].contains(&motion_kind) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPIN_ATTACK_CAN_FALL);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), false.into());
        }
        else {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    ret.into()
}

unsafe extern "C" fn link_attack_lw_4_hold_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_4_hold(fighter);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
    }
    0.into()
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, link_attack_lw_4_hold_pre_status)
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, link_attack_lw_4_hold_main_status)
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, link_attack_lw_4_hold_end_status)
    .install()
    ;
}