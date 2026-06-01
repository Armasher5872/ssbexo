use super::*;

pub unsafe extern "C" fn littlemac_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_HAS_STAR);
    WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_USED_AIR_SPECIAL_N);
    WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_FLAG_IS_START_AIR);
    WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_CAN_INPUT_DREAMLAND_EXPRESS);
    WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DREAMLAND_EXPRESS);
    WorkModule::set_float(boma, 0.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_STAR_DAMAGE);
    WorkModule::set_int(boma, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
    WorkModule::set_int(boma, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_HELD_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SUCCESSFUL_DREAMLAND_EXPRESS_INPUTS);
}

pub unsafe extern "C" fn littlemac_handle_star_strength(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let ko_gauge = WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    let star_punch_strength = WorkModule::get_int(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
    let special_held_timer = WorkModule::get_int(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_HELD_TIMER);
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) && star_punch_strength < 3 {
        WorkModule::inc_int(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_HELD_TIMER);
    }
    if special_held_timer == 3 && star_punch_strength != 1 && ko_gauge >= 34.0 {
        WorkModule::set_int(boma, 1, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
    }
    if special_held_timer == 5 && star_punch_strength != 2 && ko_gauge >= 68.0 {
        WorkModule::set_int(boma, 2, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
    }
    if special_held_timer == 7 && star_punch_strength != 3 && ko_gauge == 100.0 {
        WorkModule::set_int(boma, 3, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
    }
}

pub unsafe extern "C" fn littlemac_can_cancel_into_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    let ko_gauge = WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
    let ret;
    if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_LW {
        ret = ko_gauge == 100.0 && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD)
    }
    else {
        ret = ko_gauge > 0.0 && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD)
    }
    ret.into()
}

/*FUN_71000192f0*/
pub unsafe extern "C" fn littlemac_mtrans_smpl_off_flag(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
    WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
    WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END);
    WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_EX1);
}

/*FUN_7100019c20*/
pub unsafe extern "C" fn littlemac_special_s_ray_check(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let get_sum_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let get_sum_speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let pos = *PostureModule::pos(boma);
    let special_s_ray_check_start_frame_ = WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("special_s_ray_check_start_frame_"));
    let special_s_start_y = WorkModule::get_float(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_START_Y);
    let special_s_frame_count = WorkModule::get_int(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_SPECIAL_S_FRAME_COUNT);
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
        if GroundModule::ray_check(boma, &Vector2f{x: vec_x, y: vec_y}, &Vector2f{x: 0.0, y: start_y-1.0}, true) == 1 {
            WorkModule::on_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_RAY_CHECK_RESULT);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_RAY_CHECK_RESULT);
            if WorkModule::is_flag(boma, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_RAY_CHECK_RESULT) {
                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, boma);
            }
        }
    }
}

pub unsafe extern "C" fn littlemac_training_mode_features(boma: *mut BattleObjectModuleAccessor) {
    let status_kind = StatusModule::status_kind(boma);
    if smashball::is_training_mode()
    && status_kind == *FIGHTER_STATUS_KIND_APPEAL
    && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
        WorkModule::set_float(boma, 100.0, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE);
        EffectModule::req_on_joint(boma, Hash40::new("sys_flash"), Hash40::new("top"), &Vector3f::new(6.0, 15.0, 0.0), &Vector3f::zero(), 0.4, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
    }
}