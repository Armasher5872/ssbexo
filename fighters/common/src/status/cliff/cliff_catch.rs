use super::*;

#[skyline::hook(replace = L2CFighterCommon_sub_cliff_catch_move_uniq_process_init_common)]
unsafe extern "C" fn sub_cliff_catch_move_uniq_process_init_common(fighter: &mut L2CFighterCommon, motion_kind: L2CValue, bone: L2CValue) {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let hang_cliff_dir = GroundModule::hang_cliff_dir(fighter.module_accessor);
    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND, 0, 0.0, 0.0, 0.0, 0.0, 0.0);
    if !GroundModule::is_status_cliff(fighter.module_accessor) {
        return;
    }
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2dbe023abb));
    PostureModule::set_lr(fighter.module_accessor, -hang_cliff_dir);
    GroundModule::set_shape_flag(fighter.module_accessor, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FRONT_FIX as u16, true);
    sv_fighter_util::adjust_joint_pos_change_motion(fighter.lua_state_agent, bone.get_hash());
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, motion_kind.get_hash(), 0.0, 1.0, false, 0.0, false, false);
    GroundModule::set_shape_flag(fighter.module_accessor, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FRONT_FIX as u16, true);
    GroundModule::entry_cliff(fighter.module_accessor);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CLIFF_CATCH_MOVE);
    if prev_status_kind == *FIGHTER_STATUS_KIND_AIR_LASSO_HANG {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_CLIFF_CATCH_MOVE_FLAG_AIR_LASSO_CATCH) {
            return;
        }
    }
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_COUNT);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUB_FIGHTER) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_CLIFF_NUM);
    }
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_CliffCatchCommon)]
unsafe extern "C" fn sub_status_cliffcatchcommon(fighter: &mut L2CFighterCommon) {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RUN_FALL);
    FighterUtil::set_pickelblock_mode_ignoreandattack(fighter.module_accessor);
    GroundModule::set_ignore_boss(fighter.module_accessor, true);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_CliffCatch_MainCommon)]
unsafe extern "C" fn sub_status_cliffcatch_maincommon(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32().abs();
    let stick_y = fighter.global_table[STICK_Y].get_f32().abs();
    let length = sv_math::vec2_length(stick_x, stick_y);
    let normalize = sv_math::vec2_normalize(stick_x, stick_y);
    let dir_x = normalize.x;
    let dir_y = normalize.y;
    let atan2 = dir_y.atan2(dir_x.abs());
    let degrees = atan2.to_degrees();
    let speed_x = if length < 0.25 {1.0} else if degrees >= 70.0 {0.5} else if degrees >= 45.0 {1.0} else {1.3};
    let speed_y = if length < 0.25 {2.0} else if degrees >= 70.0 {2.6} else if degrees >= 45.0 {2.0} else {1.4};
    WorkModule::set_float(fighter.module_accessor, speed_x, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_X);
    WorkModule::set_float(fighter.module_accessor, speed_y, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_Y);
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if !GroundModule::is_status_cliff(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    if sv_information::stage_id() == 0x145 && 10.0 <= current_frame && FighterUtil::check_cliff_separated(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if current_frame >= 15.0 {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_CLIFF_FLAG_TO_ROB) {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) && situation_kind == *SITUATION_KIND_CLIFF {
                fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_ROBBED.into(), false.into());
                return 1.into();
            }
            return 0.into();
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHANGE_STATUS_DLAY_MOTION) {
        fighter.sub_cliff_uniq_process_main();
        return 0.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_CLIFF_WAIT.into(), false.into());
    }
    1.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_cliff_catch_move_uniq_process_init_common,
            sub_status_cliffcatchcommon,
            sub_status_cliffcatch_maincommon
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}