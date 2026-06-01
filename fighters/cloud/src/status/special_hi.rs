use super::*;

unsafe extern "C" fn cloud_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn cloud_special_hi_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    0.into()
}

unsafe extern "C" fn cloud_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let log_attack_additions = *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_05;
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, *FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, log_attack_additions-1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), log_attack_additions-1);
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi"), L2CValue::Hash40s("special_air_hi"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(cloud_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn cloud_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let boma = fighter.module_accessor;
    let lr = PostureModule::lr(boma);
    let rot_angle = WorkModule::get_int(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_ROT_ANGLE);
    let move_frame = WorkModule::get_int(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_MOVE_FRAME);
    let mut stick = fighter.Vector2__create(stick_x.into(), stick_y.into());
    if stick["x"].get_f32().abs()+stick["y"].get_f32().abs() < 0.5 {
        stick["x"].assign(&L2CValue::F32(0.0));
        stick["y"].assign(&L2CValue::F32(1.0));
    }
    let normalize = fighter.Vector2__normalize(stick);
    let vec_stick_x = normalize["x"].get_f32();
    let vec_stick_y = normalize["y"].get_f32();
    let stick_angle = vec_stick_y.atan2(vec_stick_x);
    let stick_degrees = stick_angle.to_degrees();
    let speed_x = ((stick_degrees+90.0).to_radians().sin()*2.0)*lr;
    let speed_y = (stick_degrees-90.0).to_radians().cos()*2.0;
    if CancelModule::is_enable_cancel(boma) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if !StatusModule::is_changing(boma) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_BRAKE);
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_hi"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if cloud_can_limit_break(fighter, 2).get_bool() {
        WorkModule::on_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK);
        WorkModule::on_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL);
        WorkModule::on_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SET_CUSTOM);
        fighter.change_status(FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI_LIMIT_BREAK.into(), false.into());
    }
    if WorkModule::is_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DIRECTION_DECIDE) {
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    }
    if WorkModule::is_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DIRECTION_CHOSEN) {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_BRAKE);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, speed_y);
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.04, 0.04);
        if stick_degrees < 0.0 {
            WorkModule::set_int(boma, (stick_degrees+180.0) as i32, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_ATTACK_ANGLE);
        }
        else {
            WorkModule::set_int(boma, stick_degrees as i32, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_ATTACK_ANGLE);
        }
        if (stick_degrees >= 0.0 && stick_degrees <= 90.0) || (stick_degrees >= -90.0 && stick_degrees < 0.0) {
            WorkModule::set_int(boma, -stick_degrees as i32, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_ROT_ANGLE);
        }
        if stick_degrees > 90.0 && stick_degrees <= 180.0 {
            WorkModule::set_int(boma, -(stick_degrees-90.0) as i32, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_ROT_ANGLE);
        }
        if stick_degrees >= -180.0 && stick_degrees < -90.0 {
            WorkModule::set_int(boma, (stick_degrees+180.0) as i32, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_ROT_ANGLE);
        }
        if (stick_degrees > 90.0 && stick_degrees <= 180.0) || (stick_degrees >= -180.0 && stick_degrees < -90.0) {
            PostureModule::reverse_lr(boma);
            PostureModule::update_rot_y_lr(boma);
        }
        WorkModule::off_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DIRECTION_CHOSEN);
    }
    if current_frame > 10.0 && move_frame < 30 {
        WorkModule::inc_int(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_MOVE_FRAME);
        ModelModule::set_joint_rotate(boma, Hash40::new("rot"), &Vector3f{x: rot_angle as f32, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    }
    if move_frame >= 30 {
        ModelModule::set_joint_rotate(boma, Hash40::new("rot"), &Vector3f::zero(), MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        if move_frame == 30 {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.06);
        }
    }
    if WorkModule::is_flag(boma, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT) {
        if stick_x.abs()+stick_y.abs() >= 0.5 {
            fighter.change_status(FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI_COMBO_1.into(), false.into());
        }
    }
    if MotionModule::is_end(boma) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn cloud_special_hi_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::set_int(boma, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_ATTACK_ANGLE);
    WorkModule::set_int(boma, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_MOVE_FRAME);
    WorkModule::set_int(boma, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_ROT_ANGLE);
    WorkModule::set_int(boma, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_INPUT_WAIT_TIMER);
    ModelModule::set_joint_rotate(boma, Hash40::new("rot"), &Vector3f::zero(), MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, cloud_special_hi_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, cloud_special_hi_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, cloud_special_hi_main_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, cloud_special_hi_end_status)
    .install()
    ;
}