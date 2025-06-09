use super::*;

unsafe extern "C" fn cloud_landing_attack_air_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attack_air_motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    let mut motion_rate: f32 = 1.0;
    let landing_param_type;
    let mut landing_mot: u64 = 0;
    let mut landing_lag: f32 = 0.0;
    match attack_air_motion_kind {
        _ if attack_air_motion_kind == hash40("attack_air_n") => {
            landing_param_type = hash40("landing_attack_air_frame_n");
            landing_mot = hash40("landing_air_n");
            landing_lag = WorkModule::get_param_float(fighter.module_accessor, landing_param_type, 0);
        },
        _ if attack_air_motion_kind == hash40("punish_attack_air_n") => {
            landing_mot = hash40("punish_landing_air_n");
            landing_lag = 10.0;
        },
        _ if attack_air_motion_kind == hash40("attack_air_f") => {
            landing_param_type = hash40("landing_attack_air_frame_f");
            landing_mot = hash40("landing_air_f");
            landing_lag = WorkModule::get_param_float(fighter.module_accessor, landing_param_type, 0);
        },
        _ if attack_air_motion_kind == hash40("punish_attack_air_f") => {
            landing_mot = hash40("punish_landing_air_f");
            landing_lag = 30.0;
        },
        _ if attack_air_motion_kind == hash40("attack_air_b") => {
            landing_param_type = hash40("landing_attack_air_frame_b");
            landing_mot = hash40("landing_air_b");
            landing_lag = WorkModule::get_param_float(fighter.module_accessor, landing_param_type, 0);
        },
        _ if attack_air_motion_kind == hash40("punish_attack_air_b") => {
            landing_mot = hash40("punish_landing_air_b");
            landing_lag = 16.0;
        },
        _ if attack_air_motion_kind == hash40("attack_air_hi") => {
            landing_param_type = hash40("landing_attack_air_frame_hi");
            landing_mot = hash40("landing_air_hi");
            landing_lag = WorkModule::get_param_float(fighter.module_accessor, landing_param_type, 0);
        },
        _ if attack_air_motion_kind == hash40("punish_attack_air_hi") => {
            landing_mot = hash40("punish_landing_air_hi");
            landing_lag = 14.0;
        },
        _ if attack_air_motion_kind == hash40("attack_air_lw") => {
            landing_param_type = hash40("landing_attack_air_frame_lw");
            landing_mot = hash40("landing_air_lw");
            landing_lag = WorkModule::get_param_float(fighter.module_accessor, landing_param_type, 0);
        },
        _ if attack_air_motion_kind == hash40("punish_attack_air_lw") => {
            landing_mot = hash40("punish_landing_air_lw");
            landing_lag = 14.0;
        },
        _ => {}
    }
    landing_lag *= motion_rate;
    if landing_lag != 0.0 {
        motion_rate = fighter.sub_get_landing_motion_rate(landing_mot.into(), landing_lag.into()).get_f32();
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(landing_mot), 0.0, motion_rate, false, 0.0, false, false);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RUN_FALL) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_RUN_FALL);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RUN_FALL);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_LANDING_CLIFF_STOP);
    }
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, cloud_landing_attack_air_init_status)
    .install()
    ;
}