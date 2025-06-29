use super::*;

/*FUN_71000192f0*/
pub unsafe extern "C" fn littlemac_mtrans_smpl_off_flag(fighter: &mut L2CFighterCommon) {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_EX1);
}

/*FUN_7100019c20*/
pub unsafe extern "C" fn littlemac_special_s_ray_check(fighter: &mut L2CFighterCommon) {
    let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let get_sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let pos = *PostureModule::pos(fighter.module_accessor);
    let special_s_ray_check_start_frame_ = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_ray_check_start_frame_"));
    let special_s_start_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_START_Y);
    let special_s_frame_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_FRAME_COUNT);
    let x = 0.0;
    let y = 0.0;
    let z = 0.0;
    let mut vector = fighter.Vector3__create(x.into(), y.into(), z.into());
    let vec_x = vector["x"].get_f32();
    let vec_y = vector["y"].get_f32();
    let mut start_y = 0.0;
    if special_s_ray_check_start_frame_-1 <= special_s_frame_count {
        vector["x"].assign(&L2CValue::F32(pos.x));
        vector["y"].assign(&L2CValue::F32(pos.y));
        vector["z"].assign(&L2CValue::F32(pos.z));
        if special_s_start_y < vec_y {
            start_y = vec_y-special_s_start_y;
        }
        vector["x"].assign(&L2CValue::F32(vec_x+get_sum_speed_x));
        vector["y"].assign(&L2CValue::F32(vec_y+get_sum_speed_y));
        if GroundModule::ray_check(fighter.module_accessor, &Vector2f{x: vec_x, y: vec_y}, &Vector2f{x: 0.0, y: start_y-1.0}, true) == 1 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_RAY_CHECK_RESULT);
        }
        else {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_RAY_CHECK_RESULT);
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_RAY_CHECK_RESULT) {
                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, fighter.module_accessor);
            }
        }
    }
}