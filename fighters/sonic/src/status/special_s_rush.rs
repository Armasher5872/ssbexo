use super::*;

unsafe extern "C" fn sonic_special_s_rush_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn sonic_special_s_rush_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let cooldown = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_N_COOLDOWN_TIMER);
    if situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    if cooldown <= 0 {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    }
    0.into()
}

unsafe extern "C" fn sonic_special_s_rush_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_rush"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_special_s_rush_main_loop as *const () as _))
}

unsafe extern "C" fn sonic_special_s_rush_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.global_table[CURRENT_FRAME].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let boost_value = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_BOOST_VALUE);
    let angle = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_S_ANGLE);
    let y_speed = if angle > 0 {0.8} else if angle < 0 {-0.8} else {0.0};
    let rot_angle = if angle > 0 {-20.0} else if angle < 0 {20.0} else {0.0};
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind == *SITUATION_KIND_GROUND
        && situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        }
        if prev_situation_kind == *SITUATION_KIND_AIR
        && situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_RUSH) {
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 3.4*lr, y_speed);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 3.4*lr, y_speed);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 3.4, y_speed);
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.03);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_RUSH);
    }
    if frame >= 2 {
        ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &Vector3f{x: rot_angle as f32, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            if boost_value > 33.0 {
                WorkModule::sub_float(fighter.module_accessor, 33.0, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_BOOST_VALUE);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_BOOSTED);
                fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), false.into());
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_RUSH_END.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn sonic_special_s_rush_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn sonic_special_s_rush_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_RUSH);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_S_ANGLE);
    EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spintrace"), true, true);
    EFFECT_OFF_KIND(fighter, Hash40::new("sonic_rush_shock"), true, true);
    ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &Vector3f::zero(), MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    0.into()
}

unsafe extern "C" fn sonic_special_s_rush_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_RUSH);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_S_ANGLE);
    EFFECT_OFF_KIND(fighter, Hash40::new("sonic_spintrace"), true, true);
    EFFECT_OFF_KIND(fighter, Hash40::new("sonic_rush_shock"), true, true);
    ModelModule::set_joint_rotate(fighter.module_accessor, Hash40::new("rot"), &Vector3f::zero(), MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    0.into()
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_RUSH, sonic_special_s_rush_pre_status)
    .status(Init, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_RUSH, sonic_special_s_rush_init_status)
    .status(Main, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_RUSH, sonic_special_s_rush_main_status)
    .status(Exec, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_RUSH, sonic_special_s_rush_exec_status)
    .status(End, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_RUSH, sonic_special_s_rush_end_status)
    .status(Exit, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_RUSH, sonic_special_s_rush_exit_status)
    .install()
    ;
}