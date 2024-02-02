use super::*;

unsafe extern "C" fn samusd_special_s1a_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn samusd_special_s2a_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_SHOOT | *FIGHTER_LOG_MASK_FLAG_HAJIKI) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn samusd_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn samusd_special_hi_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn samusd_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi"), L2CValue::Hash40s("special_air_hi"), false.into());
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        fighter.set_situation(SITUATION_KIND_AIR.into());
    }
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(samusd_special_hi_loop as *const () as _))
}

unsafe extern "C" fn samusd_special_hi_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut x_add;
    let mut y_add;
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(fighter.module_accessor);
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        WorkModule::set_float(fighter.module_accessor, 24.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        return 0.into();
    }
    if (22.0..=94.0).contains(&frame) && situation_kind == *SITUATION_KIND_AIR {
        if stick_x > 0.2 {
            x_add = ((stick_x-0.2)*0.02/*X Accel Mul*/)+0.01/*X Accel Add*/;
            if speed_x > 0.25/*X Max*/ || speed_x < -0.25/*X Max*/ {
                x_add = 0.0;
            };
        };
        if stick_x < -0.2 {
            x_add = ((stick_x+0.2)*0.02/*X Accel Mul*/)+0.01/*X Accel Add*/;
            if speed_x > 0.25/*X Max*/ || speed_x < -0.25/*X Max*/ {
                x_add = 0.0;
            };
        };
        if stick_y > 0.2 {
            y_add = ((stick_y-0.2)*0.04/*Y Accel Mul*/)+0.02/*Y Accel Add*/;
            if speed_y > 0.5/*Y Max*/ || speed_y < -0.5/*Y Max*/ {
                y_add = 0.0;
            };
        };
        if stick_y < -0.2 {
            y_add = ((stick_y+0.2)*0.04/*Y Accel Mul*/)+0.02/*Y Accel Add*/;
            if speed_y > 0.5/*Y Max*/ || speed_y < -0.5/*Y Max*/ {
                y_add = 0.0;
            };
        };
        if stick_x > -0.2 && stick_x < 0.2 && stick_y > -0.2 && stick_y < 0.2 {
            if speed_y > 0.0 {
                y_add = -0.04/*Y Accel Mul*/-0.02/*Y Accel Add*/;
            } 
            else if speed_y < 0.0 {
                y_add = 0.04/*Y Accel Mul*/+0.02/*Y Accel Add*/;
            };
            if speed_x > 0.0 {
                x_add = -0.02/*X Accel Mul*/-0.01/*X Accel Add*/;
            } 
            else if speed_x < 0.0 {
                x_add = 0.02/*X Accel Mul*/+0.01/*X Accel Add*/;
            };
        };
        x_add = (stick_x)*0.02/*X Accel Mul*/;
        y_add = (stick_y)*0.04/*Y Accel Mul*/;
        if x_add > 0.0 && SAMUSD_X[entry_id] > 0.25/*X Max*/ {
            x_add = 0.0;
        };
        if x_add < 0.0 && SAMUSD_X[entry_id] < 0.25*-1.0/*X Max*/ {
            x_add = 0.0;
        };
        if y_add > 0.0 && SAMUSD_Y[entry_id] > 0.5/*Y Max*/ {
            y_add = 0.0;
        };
        if y_add < 0.0 && SAMUSD_Y[entry_id] < 0.5*-1.0/*Y Max*/ {
            y_add = 0.0;
        };
        let speed = smash::phx::Vector3f{ x: x_add, y: y_add, z: 0.0 };
        KineticModule::add_speed(fighter.module_accessor, &speed);
        SAMUSD_X[entry_id] += x_add;
        SAMUSD_Y[entry_id] += y_add;
    }
    else {
        SAMUSD_X[entry_id] = 0.0;
        SAMUSD_Y[entry_id] = 0.0;
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn samusd_special_hi_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn samusd_special_hi_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn samusd_special_hi_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("samusd")
    .status(Pre, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S1A, samusd_special_s1a_pre_status)
    .status(Pre, *FIGHTER_SAMUS_STATUS_KIND_SPECIAL_S2A, samusd_special_s2a_pre_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, samusd_special_hi_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, samusd_special_hi_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, samusd_special_hi_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, samusd_special_hi_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, samusd_special_hi_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_HI, samusd_special_hi_exit_status)
    .install()
    ;
}