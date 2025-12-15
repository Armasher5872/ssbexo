use super::*;

//Aerial Down Special Loop Pre Status
unsafe extern "C" fn wario_special_air_lw_loop_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_FALL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

//Aerial Down Special Loop Init Status
unsafe extern "C" fn wario_special_air_lw_loop_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -4.0);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 4.0);
    sv_kinetic_energy!(controller_set_accel_x_add, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.015);
    sv_kinetic_energy!(controller_set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
    0.into()
}

//Aerial Down Special Loop Main Status
unsafe extern "C" fn wario_special_air_lw_loop_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_loop"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(wario_special_air_lw_loop_main_loop as *const () as _))
}

unsafe extern "C" fn wario_special_air_lw_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let special_lw_timer = WorkModule::get_int(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_TIMER);
    let pass_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y"));
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if special_lw_timer >= 29 && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STRONG) {
        PLAY_SE(fighter, Hash40::new("se_wario_special_l02"));
        //PLAY_SE(fighter, Hash40::new("se_wario_special_s02"));
        sv_kinetic_energy!(controller_set_accel_x_add, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STRONG);
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STRONG) {
        if stick_y > 0.7 {
            fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_LW_END.into(), false.into());
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        if GroundModule::is_passable_ground(fighter.module_accessor) {
            if stick_y <= pass_stick_y {
                GroundModule::pass_floor(fighter.module_accessor);
                SA_SET(fighter, *SITUATION_KIND_AIR);
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STRONG) {
                    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -4.0);
                    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 4.0);
                    sv_kinetic_energy!(controller_set_accel_x_add, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
                    sv_kinetic_energy!(controller_set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
                    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
                }
                else {
                    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -4.0);
                    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 4.0);
                    sv_kinetic_energy!(controller_set_accel_x_add, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.015);
                    sv_kinetic_energy!(controller_set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
                }
            }
            else {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_LW_LAND.into(), false.into());
            }
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_LW_LAND.into(), false.into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_loop"), 0.0, 1.0, false, 0.0, false, false);
    }
    0.into()
}

//Aerial Down Special Loop Exec Status
unsafe extern "C" fn wario_special_air_lw_loop_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STRONG) {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_TIMER);
    }
    0.into()
}

//Aerial Down Special Loop End Status
unsafe extern "C" fn wario_special_air_lw_loop_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_LW_LOOP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_LW_LAND].contains(&status_kind) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STRONG);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_TIMER);
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_wario_special_l02"), 0);
    }
    0.into()
}

//Aerial Down Special Loop Exit Status
unsafe extern "C" fn wario_special_air_lw_loop_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_LW_LOOP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_LW_LAND].contains(&status_kind) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STRONG);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_SPECIAL_LW_TIMER);
    
    }
    0.into()
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_LW_LOOP, wario_special_air_lw_loop_pre_status)
    .status(Init, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_LW_LOOP, wario_special_air_lw_loop_init_status)
    .status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_LW_LOOP, wario_special_air_lw_loop_main_status)
    .status(Exec, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_LW_LOOP, wario_special_air_lw_loop_exec_status)
    .status(End, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_LW_LOOP, wario_special_air_lw_loop_end_status)
    .status(Exit, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_AIR_LW_LOOP, wario_special_air_lw_loop_exit_status)
    .install()
    ;
}