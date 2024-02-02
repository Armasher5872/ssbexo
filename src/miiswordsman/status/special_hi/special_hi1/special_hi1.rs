use super::*;

pub unsafe extern "C" fn miiswordsman_special_hi1_start_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_hi1_start_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable") as u64, 0);
    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul") as u64, 0);
    let hi1_jump_mul_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi") as u64, hash40("hi1_jump_mul_x") as u64);
    WorkModule::set_int(fighter.module_accessor, fighter.global_table[SITUATION_KIND].get_i32(), *FIGHTER_MIISWORDSMAN_STATUS_WORK_ID_INT_ROKET_UNDER_THRUST_START_SITUATION);
    KineticModule::clear_speed_all(fighter.module_accessor);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, *ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(mul_x_speed_max, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, (air_accel_x_mul*hi1_jump_mul_x));
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable);
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_hi1_start_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
    /*
    let hi1_jump_speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi") as u64, hash40("hi1_jump_speed_mul") as u64);
    fun_71000207c0(fighter);
    WorkModule::set_float(fighter.module_accessor, hi1_jump_speed_mul, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_hi_start") as i64, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_hi_start") as i64, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_AIR_MOT);
    fun_71000208c0(fighter);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_07 - 1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_07 - 1);
    fighter.sub_shift_status_main(L2CValue::Ptr(miiswordsman_special_hi1_start_main_loop as *const () as _))
    */
}

/*
unsafe extern "C" fn fun_71000207c0(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_EX1);
    0.into()
}

unsafe extern "C" fn fun_71000208c0(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ground_mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_GROUND_MOT);
    let air_mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_AIR_MOT);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(air_mot), -1.0, 1.0, 0.0, false, false);
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(air_mot), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(ground_mot), -1.0, 1.0, 0.0, false, false);
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(ground_mot), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);
    }
    0.into()
}

unsafe extern "C" fn miiswordsman_special_hi1_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut ret: i32 = 0;
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if !MotionModule::is_end(fighter.module_accessor) {
            if !fun_7100021060(fighter) {
                fun_71000208c0(fighter);
                if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                    fighter.set_situation(SITUATION_KIND_AIR.into());
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
                }
                else {
                    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                    fighter.set_situation(SITUATION_KIND_GROUND.into());
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
                }
            }
            if fun_71000212b0(fighter) {
                fun_71000207c0(fighter);
            }
            ret = 0.into();
        }
        fighter.change_status(FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_JUMP.into(), false.into());
    }
    ret = 1.into();
    ret.into()
}

unsafe extern "C" fn fun_7100021060(fighter: &mut L2CFighterCommon) -> bool {
    let mut ret: bool = false;
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR)
    && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END) {
            ret = true;
        }
        else {
            ret = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_EX1);
        }
        if !ret {
            ret = false;
        }
        else {
            ret = true;
        }
    }
    else {
        ret = true;
    }
    ret
}

unsafe extern "C" fn fun_71000212b0(fighter: &mut L2CFighterCommon) -> bool {
    let mut ret: bool = false;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR) && situation_kind != *SITUATION_KIND_AIR {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND) && situation_kind == *SITUATION_KIND_GROUND {
            ret = true;
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END) {
                ret = MotionModule::is_end(fighter.module_accessor);
            }
            else {
                ret = false;
            }
        }
        if !ret {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_EX1) {
                ret = false;
            }
            else {
                ret = true;
            }
        }
    }
    else {
        ret = true;
    }
    ret
}
*/

pub unsafe extern "C" fn miiswordsman_special_hi1_start_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_hi1_start_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_JUMP {
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_EFFECT_AFTER_IMAGE_OFF, 0);
        sv_module_access::effect(fighter.lua_state_agent);
        fighter.pop_lua_stack(1.into());
    }
    0.into()
}

pub unsafe extern "C" fn miiswordsman_special_hi1_start_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_HI1_JUMP {
        EffectModule::kill_all(fighter.module_accessor, *EFFECT_SUB_ATTRIBUTE_SYNC_STOP as u32, false, true);
    }
    0.into()
}