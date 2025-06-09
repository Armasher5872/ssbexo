use super::*;

unsafe extern "C" fn cloud_special_hi_combo_2_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn cloud_special_hi_combo_2_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    0.into()
}

unsafe extern "C" fn cloud_special_hi_combo_2_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_combo_2"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(cloud_special_hi_combo_2_main_loop as *const () as _))
}

unsafe extern "C" fn cloud_special_hi_combo_2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let scale = PostureModule::scale(fighter.module_accessor);
    let pos = PostureModule::pos(fighter.module_accessor);
    let color = FighterUtil::get_team_color(fighter.module_accessor);
    let effect_color = FighterUtil::get_effect_team_color(EColorKind(color as i32), Hash40::new("direction_effect_color"));
    let effect_handle = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_HANDLE);
    let move_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_MOVE_FRAME);
    let stick = fighter.Vector2__create(stick_x.into(), stick_y.into());
    let vec_stick_x = stick["x"].get_f32();
    let vec_stick_y = stick["y"].get_f32();
    let stick_angle = vec_stick_y.atan2(vec_stick_x);
    let stick_degrees = stick_angle.to_degrees();
    let radius = scale*14.0;
    let pos_x = radius*vec_stick_x+(*pos).x;
    let pos_y = (radius*vec_stick_y+(*pos).y)+10.0;
    let speed_x = (stick_degrees+90.0).to_radians().sin()*1.8;
    let speed_y = (stick_degrees-90.0).to_radians().cos()*1.8;
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return 0.into();
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_BRAKE);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_combo_2"), -1.0, 1.0, 0.0, false, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DIRECTION_DECIDE) {
        if effect_handle != *EFFECT_HANDLE_NULL {
            EffectModule::set_pos(fighter.module_accessor, effect_handle as u32, &Vector3f{x: pos_x, y: pos_y, z: 0.0});
            EffectModule::set_rot(fighter.module_accessor, effect_handle as u32, &Vector3f{x: 0.0, y: 0.0, z: stick_degrees-90.0});
        }
        else {
            let effect = EffectModule::req(fighter.module_accessor, Hash40::new("sys_direction2"), &Vector3f{x: pos_x, y: pos_y, z: 0.0}, &Vector3f::zero(), 1.0, 0, -1, false, 0);
            EffectModule::set_rot(fighter.module_accessor, effect as u32, &Vector3f{x: 0.0, y: 0.0, z: stick_degrees-90.0});
            EffectModule::set_rgb_partial_last(fighter.module_accessor, effect_color.value[0], effect_color.value[1], 0.0);
            WorkModule::set_int(fighter.module_accessor, effect as i32, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_HANDLE);
        }
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    }
    else {
        if EffectModule::is_exist_effect(fighter.module_accessor, effect_handle as u32) {
            EffectModule::kill(fighter.module_accessor, effect_handle as u32, true, true);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_HANDLE);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DIRECTION_CHOSEN) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_BRAKE);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed_x, speed_y);
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.04, 0.04);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DIRECTION_CHOSEN);
    }
    if current_frame > 10.0 && move_frame < 20 {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_MOVE_FRAME);
    }
    if move_frame >= 20 {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI_COMBO_2_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn cloud_special_hi_combo_2_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let effect_handle = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_HANDLE);
    EffectModule::kill(fighter.module_accessor, effect_handle as u32, true, true);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_EFFECT_HANDLE);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_ATTACK_ANGLE);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_MOVE_FRAME);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_ROT_ANGLE);
    ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &Vector3f::zero(), MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    0.into()
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI_COMBO_2, cloud_special_hi_combo_2_pre_status)
    .status(Init, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI_COMBO_2, cloud_special_hi_combo_2_init_status)
    .status(Main, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI_COMBO_2, cloud_special_hi_combo_2_main_status)
    .status(End, *FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI_COMBO_2, cloud_special_hi_combo_2_end_status)
    .install()
    ;
}