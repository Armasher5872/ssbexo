use super::*;

unsafe extern "C" fn falco_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_CONTINUE);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_FALCO_BLASTER_STEP_START, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_INT_STEP);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_INT_STEP_PREV);
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        fun_71000171e0(fighter);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fun_7100016fb0(fighter);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fun_7100017410(fighter);
    }
    fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::Ptr(fun_7100017410 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(falco_special_n_main_loop as *const () as _))
}

unsafe extern "C" fn fun_71000171e0(fighter: &mut L2CFighterCommon) {
    MotionModule::set_whole_rate(fighter.module_accessor, 1.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_CONTINUE) {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_start"), -1.0, 1.0, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_start"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_CONTINUE);
    }
}

unsafe extern "C" fn fun_7100016fb0(fighter: &mut L2CFighterCommon) {
    MotionModule::set_whole_rate(fighter.module_accessor, 1.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_CONTINUE) {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_start"), -1.0, 1.0, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_start"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_CONTINUE);
    }
}

unsafe extern "C" fn fun_7100017410(fighter: &mut L2CFighterCommon) -> L2CValue {
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP_ACCEPT) {
        if pad_flag & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP);
        }
    }
    0.into()
}

unsafe extern "C" fn falco_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let blaster_step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_INT_STEP);
    let get_kinetic_type = KineticModule::get_kinetic_type(fighter.module_accessor);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
            if blaster_step == *FIGHTER_FALCO_BLASTER_STEP_START {
                fun_7100016fb0(fighter);
            }
            if blaster_step == *FIGHTER_FALCO_BLASTER_STEP_SHOT {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            }
            if blaster_step == *FIGHTER_FALCO_BLASTER_STEP_END {
                MotionModule::set_whole_rate(fighter.module_accessor, 1.0);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_end"), -1.0, 1.0, 0.0, false, false);
            }
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
            if blaster_step == *FIGHTER_FALCO_BLASTER_STEP_START {
                fun_71000171e0(fighter);
            }
            if blaster_step == *FIGHTER_FALCO_BLASTER_STEP_SHOT {
                MotionModule::set_whole_rate(fighter.module_accessor, 1.0);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_loop"), -1.0, 1.0, 0.0, false, false);
            }
            if blaster_step == *FIGHTER_FALCO_BLASTER_STEP_END {
                MotionModule::set_whole_rate(fighter.module_accessor, 1.0);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_end"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.sub_air_check_dive();
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if[*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&get_kinetic_type) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
                sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                sv_kinetic_energy!(enable, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if blaster_step == *FIGHTER_FALCO_BLASTER_STEP_START {
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_INT_STEP);
            if situation_kind == *SITUATION_KIND_GROUND {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_loop"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_loop"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if blaster_step == *FIGHTER_FALCO_BLASTER_STEP_SHOT {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP) {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_FALCO_BLASTER_STEP_END, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_INT_STEP);
                if situation_kind == *SITUATION_KIND_GROUND {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_end"), 0.0, 1.0, false, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_end"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
            else {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_SPECIAL_N, true);
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, *FIGHTER_LOG_DATA_INT_SHOOT_NUM);
                if situation_kind == *SITUATION_KIND_GROUND {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_loop"), 0.0, 1.0, false, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_loop"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_FALCO_BLASTER_STATUS_WORK_ID_FLAG_LOOP);
            ControlModule::clear_command(fighter.module_accessor, true);
            MotionModule::set_whole_rate(fighter.module_accessor, 1.0);
        }
        if blaster_step == *FIGHTER_FALCO_BLASTER_STEP_END {
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
    }
    0.into()
}

unsafe extern "C" fn falco_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, *FS_SUCCEEDS_KEEP_SOUND);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("falco")
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, falco_special_n_main_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, falco_special_lw_pre_status)
    .install()
    ;
}