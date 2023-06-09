use super::*;

#[status_script(agent = "samusd", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn samusd_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

#[status_script(agent = "samusd", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn samusd_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if SAMUSD_HAS_FLOAT[entry_id] != true {
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi"), L2CValue::Hash40s("special_air_hi"), false.into());
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
        }
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        fighter.sub_shift_status_main(L2CValue::Ptr(samusd_special_hi_loop as *const () as _));
    }
    else {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

pub unsafe fn samusd_special_hi_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let stick_x = fighter.global_table[STICK_X].get_f32()*PostureModule::lr(fighter.module_accessor);
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
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
        if prev_situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            WorkModule::set_float(fighter.module_accessor, 24.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            return 0.into();
        }
    }
    if (22.0..=94.0).contains(&frame) && situation_kind == *SITUATION_KIND_AIR {
        let mut y_add = 0.0;
        let mut x_add = 0.0;
        if stick_x > 0.2 {
            x_add = ((stick_x-0.2)*SAMUSD_X_ACCEL_MUL) + SAMUSD_X_ACCEL_ADD;
            if speed_x > SAMUSD_X_MAX || speed_x < -SAMUSD_X_MAX {
                x_add = 0.0;
            };
        };
        if stick_x < -0.2 {
            x_add = ((stick_x+0.2)*SAMUSD_X_ACCEL_MUL) + SAMUSD_X_ACCEL_ADD;
            if speed_x > SAMUSD_X_MAX || speed_x < -SAMUSD_X_MAX {
                x_add = 0.0;
            };
        };
        if stick_y > 0.2 {
            y_add = ((stick_y-0.2)*SAMUSD_Y_ACCEL_MUL) + SAMUSD_Y_ACCEL_ADD;
            if speed_y > SAMUSD_Y_MAX || speed_y < -SAMUSD_Y_MAX {
                y_add = 0.0;
            };
        };
        if stick_y < -0.2 {
            y_add = ((stick_y+0.2)*SAMUSD_Y_ACCEL_MUL) + SAMUSD_Y_ACCEL_ADD;
            if speed_y > SAMUSD_Y_MAX || speed_y < -SAMUSD_Y_MAX {
                y_add = 0.0;
            };
        };
        if stick_x > -0.2 && stick_x < 0.2 && stick_y > -0.2 && stick_y < 0.2 {
            if speed_y > 0.0 {
                y_add = -SAMUSD_Y_ACCEL_MUL - SAMUSD_Y_ACCEL_ADD;
            } 
            else if speed_y < 0.0 {
                y_add = SAMUSD_Y_ACCEL_MUL + SAMUSD_Y_ACCEL_ADD;
            };
            let mut x_add = 0.0;
            if speed_x > 0.0 {
                x_add = -SAMUSD_X_ACCEL_MUL - SAMUSD_X_ACCEL_ADD;
            } 
            else if speed_x < 0.0 {
                x_add = SAMUSD_X_ACCEL_MUL + SAMUSD_X_ACCEL_ADD;
            };
        };
        x_add = (stick_x)*SAMUSD_X_ACCEL_MUL;
        y_add = (stick_y)*SAMUSD_Y_ACCEL_MUL;
        if x_add > 0.0 && SAMUSD_X[entry_id] > SAMUSD_X_MAX {
            x_add = 0.0;
        };
        if x_add < 0.0 && SAMUSD_X[entry_id] < SAMUSD_X_MAX*-1.0 {
            x_add = 0.0;
        };
        if y_add > 0.0 && SAMUSD_Y[entry_id] > SAMUSD_Y_MAX {
            y_add = 0.0;
        };
        if y_add < 0.0 && SAMUSD_Y[entry_id] < SAMUSD_Y_MAX*-1.0 {
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
        if situation_kind == *SITUATION_KIND_GROUND {
            if prev_situation_kind != *SITUATION_KIND_AIR {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 22.0, 1.0, false, 0.0, false, false);
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

#[status_script(agent = "samusd", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn samusd_special_hi_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[status_script(agent = "samusd", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn samusd_special_hi_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[status_script(agent = "samusd", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn samusd_special_hi_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    SAMUSD_HAS_FLOAT[entry_id] = true;
    0.into()
}

#[status_script(agent = "samusd", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn samusd_special_hi_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    install_status_scripts!(
        samusd_special_hi_pre_status,
        samusd_special_hi_init_status,
        samusd_special_hi_main_status,
        samusd_special_hi_exec_status,
        samusd_special_hi_end_status,
        samusd_special_hi_exit_status
    );
}