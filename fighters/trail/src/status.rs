use super::*;

unsafe extern "C" fn trail_attack_lw4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let smash_restart_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    ControlModule::reset_trigger(fighter.module_accessor);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    ComboModule::set(fighter.module_accessor, *FIGHTER_COMBO_KIND_S4);
    MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("attack_lw4"), smash_restart_frame, 1.0, 0.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(trail_attack_lw4_main_loop as *const () as _))   
}

unsafe extern "C" fn trail_attack_lw4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let current_frame = MotionModule::frame(fighter.module_accessor);
    let pass_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y"));
    let start_air_frame = 7.0;
    let fall_loop_frame = 18.0;
    let landing_frame = 19.0;
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if current_frame == start_air_frame {
        SA_SET(fighter, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        sv_kinetic_energy!(set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
        sv_kinetic_energy!(set_accel_x_add, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    if situation_kind == *SITUATION_KIND_AIR {
        if current_frame >= fall_loop_frame {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -7.0);
            MotionModule::set_frame(fighter.module_accessor, fall_loop_frame, true);
            MotionModule::set_rate(fighter.module_accessor, 0.0);
        }
    }
    else {
        if current_frame >= fall_loop_frame && current_frame < landing_frame {
            if stick_y <= pass_stick_y {
                GroundModule::set_passable_check(fighter.module_accessor, true);
            }
            else {
                GroundModule::set_passable_check(fighter.module_accessor, false);
            }
            if GroundModule::is_passable_check(fighter.module_accessor) && GroundModule::is_passable_ground(fighter.module_accessor) {
                SA_SET(fighter, *SITUATION_KIND_AIR);
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -7.0);
                sv_kinetic_energy!(set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
                sv_kinetic_energy!(set_accel_x_add, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
                MotionModule::set_frame(fighter.module_accessor, fall_loop_frame, true);
                MotionModule::set_rate(fighter.module_accessor, 0.0);
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                MotionModule::set_frame(fighter.module_accessor, landing_frame, true);
                MotionModule::set_rate(fighter.module_accessor, 1.0);
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn trail_attack_air_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attack_air_kind = ControlModule::get_attack_air_kind(fighter.module_accessor);
    if attack_air_kind != *FIGHTER_COMMAND_ATTACK_AIR_KIND_F {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_FALL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_AIR_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_NONE as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32, 0);
        return 0.into()
    }
    else {
        StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F);
    }
    1.into()
}

unsafe extern "C" fn trail_attack_air_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    fighter.sub_attack_air_kind();
    if [hash40("jump_aerial_f"), hash40("jump_aerial_b")].contains(&motion_kind) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new_raw(motion_kind), frame, 1.0, false, 1.0);
            MotionModule::set_weight(fighter.module_accessor, 1.0, true);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
        }
    }
    fighter.sub_attack_air_uniq_process_init();
    0.into()
}

unsafe extern "C" fn trail_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

pub fn install() {
    Agent::new("trail")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, trail_attack_lw4_main_status)
    .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_AIR, trail_attack_air_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_ATTACK_AIR, trail_attack_air_init_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, trail_special_lw_pre_status)
    .install()
    ;
}