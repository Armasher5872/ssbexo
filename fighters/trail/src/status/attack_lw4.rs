use super::*;

unsafe extern "C" fn trail_attack_lw4_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let smash_restart_frame = WorkModule::get_float(boma, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME);
    WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK);
    ControlModule::reset_trigger(boma);
    WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    ComboModule::set(boma, *FIGHTER_COMBO_KIND_S4);
    MotionModule::change_motion_force_inherit_frame(boma, Hash40::new("attack_lw4"), smash_restart_frame, 1.0, 0.0);
    fighter.sub_shift_status_main(L2CValue::Ptr(trail_attack_lw4_main_loop as *const () as _))   
}

unsafe extern "C" fn trail_attack_lw4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let boma = fighter.module_accessor;
    let current_frame = MotionModule::frame(boma);
    let pass_stick_y = WorkModule::get_param_float(boma, hash40("common"), hash40("pass_stick_y"));
    let start_air_frame = 7.0;
    let fall_loop_frame = 18.0;
    let landing_frame = 19.0;
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if current_frame == start_air_frame {
        SA_SET(fighter, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
        sv_kinetic_energy!(set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
        sv_kinetic_energy!(set_accel_x_add, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    if situation_kind == *SITUATION_KIND_AIR {
        if current_frame >= fall_loop_frame {
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -7.0);
            MotionModule::set_frame(boma, fall_loop_frame, true);
            MotionModule::set_rate(boma, 0.0);
        }
    }
    else {
        if current_frame >= fall_loop_frame && current_frame < landing_frame {
            if stick_y <= pass_stick_y {
                GroundModule::set_passable_check(boma, true);
            }
            else {
                GroundModule::set_passable_check(boma, false);
            }
            if GroundModule::is_passable_check(boma) && GroundModule::is_passable_ground(boma) {
                SA_SET(fighter, *SITUATION_KIND_AIR);
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
                KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -7.0);
                sv_kinetic_energy!(set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
                sv_kinetic_energy!(set_accel_x_add, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0);
                MotionModule::set_frame(boma, fall_loop_frame, true);
                MotionModule::set_rate(boma, 0.0);
            }
            else {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                MotionModule::set_frame(boma, landing_frame, true);
                MotionModule::set_rate(boma, 1.0);
            }
        }
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("trail")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, trail_attack_lw4_main_status)
    .install()
    ;
}