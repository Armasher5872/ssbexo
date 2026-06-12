use super::*;

unsafe extern "C" fn demon_landing_attack_air_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    let motion_kind = MotionModule::motion_kind(boma);
    if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&prev_status_kind) {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP); //Makes wavedashes edge cancelable
    }
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
        ControlModule::clear_command(boma, true);
    }
    if motion_kind == hash40("attack_air_lw") {
        StatusModule::set_status_kind_interrupt(boma, *FIGHTER_STATUS_KIND_DOWN);
        return 1.into();
    }
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_LANDING_ATTACK_AIR_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_LANDING_ATTACK_AIR_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_LANDING_ATTACK_AIR_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, true, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32, 0);
    0.into()
}

unsafe extern "C" fn demon_landing_attack_air_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let attack_air_motion_kind = WorkModule::get_int64(boma, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    let mut motion_rate: f32 = 1.0;
    let mut landing_mot: u64 = 0;
    let mut landing_lag: f32 = 0.0;
    match attack_air_motion_kind {
        _ if attack_air_motion_kind == hash40("attack_air_n") => {
            if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_HIGH_POUNCE_ACTIVE) {
                landing_mot = hash40("high_pounce");
                landing_lag = 20.0;
            }
            else {
                landing_mot = hash40("landing_air_n");
                landing_lag = WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_n"), 0);
            }
        },
        _ if attack_air_motion_kind == hash40("attack_air_f") => {
            landing_mot = hash40("landing_air_f");
            landing_lag = WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_f"), 0);
        },
        _ if attack_air_motion_kind == hash40("attack_air_b") => {
            landing_mot = hash40("landing_air_b");
            landing_lag = WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_b"), 0);
        },
        _ if attack_air_motion_kind == hash40("attack_air_hi") => {
            landing_mot = hash40("landing_air_hi");
            landing_lag = WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_hi"), 0);
        },
        _ if attack_air_motion_kind == hash40("attack_air_lw") => {
            landing_mot = hash40("landing_air_lw");
            landing_lag = WorkModule::get_param_float(boma, hash40("landing_attack_air_frame_lw"), 0);
        },
        _ => {}
    }
    landing_lag *= motion_rate;
    if landing_lag != 0.0 {
        motion_rate = fighter.sub_get_landing_motion_rate(landing_mot.into(), landing_lag.into()).get_f32();
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    }
    MotionModule::change_motion(boma, Hash40::new_raw(landing_mot), 0.0, motion_rate, false, 0.0, false, false);
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_RUN_FALL) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_RUN_FALL);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_RUN_FALL);
    }
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP) {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP);
    }
    0.into()
}

unsafe extern "C" fn demon_landing_attack_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_LandingAttackAirSub();
    fighter.sub_shift_status_main(L2CValue::Ptr(demon_landing_attack_air_main_loop as *const () as _))
}

unsafe extern "C" fn demon_landing_attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let is_attack = ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK);
    let motion_kind = MotionModule::motion_kind(boma);
    if [hash40("landing_air_n"), hash40("landing_air_f"), hash40("landing_air_hi")].contains(&motion_kind) {
        if is_attack {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_7.into(), false.into());
            return 0.into();
        }
    }
    if motion_kind == hash40("landing_air_b") {
        if is_attack {
            fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_7.into(), false.into());
            return 0.into();
        }
        if WorkModule::is_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_SOBAT_TURN) {
            WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_SOBAT_TURN);
            PostureModule::reverse_lr(boma);
            PostureModule::update_rot_y_lr(boma);
        }
    }
    fighter.status_LandingAttackAir_Main()
}

unsafe extern "C" fn demon_landing_attack_air_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_HIGH_POUNCE_ACTIVE);
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_SOBAT_TURN);
    fighter.status_end_LandingAttackAir()
}

unsafe extern "C" fn demon_landing_attack_air_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_HIGH_POUNCE_ACTIVE);
    WorkModule::off_flag(boma, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_SOBAT_TURN);
    fighter.sub_landing_uniq_process_exit()
}

pub fn install() {
    Agent::new("demon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, demon_landing_attack_air_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, demon_landing_attack_air_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, demon_landing_attack_air_main_status)
    .status(End, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, demon_landing_attack_air_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, demon_landing_attack_air_exit_status)
    .install()
    ;
}