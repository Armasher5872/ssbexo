use super::*;

unsafe extern "C" fn edge_special_hi_end_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_RUSH_CANCEL);
    EffectModule::kill_kind(boma, Hash40::new("edge_octaslash_line"), true, true);
    fun_7100014650(fighter);
    fun_71000147f0(fighter);
    0.into()
}

unsafe extern "C" fn fun_7100014650(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::set_float(boma, 0.0, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_RUSH_DEGREE);
    ModelModule::set_joint_rotate(boma, Hash40::new("rot"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: 0 as u8}, MotionNodeRotateOrder{_address: 0 as u8});
    0.into()
}

unsafe extern "C" fn fun_71000147f0(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    let charged_rush = WorkModule::is_flag(boma, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_CHARGED_RUSH);
    let landing_lag = edge_special_hi_param_int_helper(fighter, Hash40::new("landing_fix_frame").into(), charged_rush.into());
    if status_kind != *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL {
        if status_kind != *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            return 0.into();
        }
    }
    WorkModule::set_float(boma, landing_lag.get_f32(), *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    0.into()
}

unsafe extern "C" fn edge_special_hi_param_int_helper(fighter: &mut L2CFighterCommon, hash: L2CValue, charged_rush: L2CValue) -> L2CValue {
    let boma = fighter.module_accessor;
    let param = edge_special_hi_param_helper_inner(hash, charged_rush).get_u64();
    WorkModule::get_param_int(boma, hash40("param_special_hi"), param).into()
}

unsafe extern "C" fn edge_special_hi_param_helper_inner(hash: L2CValue, charged_rush: L2CValue) -> L2CValue {
    let hash = hash.get_u64();
    if !charged_rush.get_bool() {
        return hash.into();
    }
    let new_hash = if hash == hash40("rot_decide_frame") {
        hash40("charged_rot_decide_frame")
    }
    else if hash == hash40("rot_end_frame") {
        hash40("charged_rot_end_frame")
    }
    else if hash == hash40("rush_frame") {
        hash40("charged_rush_frame")
    }
    else if hash == hash40("rush_speed") {
        hash40("charged_rush_speed")
    }
    else if hash == hash40("rush_brake_frame") {
        hash40("charged_rush_brake_frame")
    }
    else if hash == hash40("rush_brake") {
        hash40("charged_rush_brake")
    }
    else if hash == hash40("ground_speed_x_mul") {
        hash40("charged_ground_speed_x_mul")
    }
    else if hash == hash40("landing_speed_x_mul") {
        hash40("charged_landing_speed_x_mul")
    }
    else if hash == hash40("landing_brake_x") {
        hash40("charged_landing_brake_x")
    }
    else if hash == hash40("landing_fix_frame") {
        hash40("charged_landing_fix_frame")
    }
    else if hash == hash40("rotate_back_begin_frame") {
        hash40("charged_rotate_back_begin_frame")
    }
    else if hash == hash40("rotate_back_end_frame") {
        hash40("charged_rotate_back_end_frame")
    }
    else if hash == hash40("rush_end_speed_mul") {
        hash40("charged_rush_end_speed_mul")
    }
    else if hash == hash40("rush_end_brake_x") {
        hash40("charged_rush_end_brake_x")
    }
    else if hash == hash40("rush_end_gravity_accel") {
        hash40("charged_rush_end_gravity_accel")
    }
    else if hash == hash40("control_accel_x_mul") {
        hash40("charged_control_accel_x_mul")
    }
    else if hash == hash40("control_speed_x_max_mul") {
        hash40("charged_control_speed_x_max_mul")
    }
    else{
        hash
    };
    new_hash.into()
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END, edge_special_hi_end_end_status)
    .install()
    ;
}