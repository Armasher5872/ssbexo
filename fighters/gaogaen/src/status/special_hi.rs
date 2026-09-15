use super::*;

unsafe extern "C" fn gaogaen_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let get_sum_speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let special_air_hi_start_y_mul = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_air_hi_start_y_mul"));
    let special_air_hi_start_x_mul = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_air_hi_start_x_mul"));
    let special_air_hi_accel_y = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_air_hi_accel_y"));
    let special_hi_pass_mul = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_pass_mul"));
    let special_air_hi_pass_mul = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_air_hi_pass_mul"));
    let special_hi_end_landing_frame = WorkModule::get_param_int(boma, hash40("param_special_hi"), hash40("special_hi_end_landing_frame"));
    let special_hi_lr_stick_x = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_lr_stick_x"));
    WorkModule::set_flag(boma, situation_kind != *SITUATION_KIND_GROUND, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_FLAG_START_AIR);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, get_sum_speed_y*special_air_hi_start_y_mul);
    KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    WorkModule::set_int64(boma, hash40("special_hi") as i64, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND);
    WorkModule::set_int64(boma, hash40("special_air_hi_start") as i64, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND_AIR);
    WorkModule::set_float(boma, special_air_hi_start_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_START_X_MUL);
    WorkModule::set_float(boma, special_air_hi_accel_y, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_ACCEL_Y);
    WorkModule::set_float(boma, special_hi_pass_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_PASS_MUL);
    WorkModule::set_float(boma, special_air_hi_pass_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_PASS_MUL);
    WorkModule::set_int(boma, special_hi_end_landing_frame, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_CONST_LANDING_FRAME);
    WorkModule::set_float(boma, special_hi_lr_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_LR_STICK_X);
    WorkModule::set_float(boma, 1.0, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_FALL_X_MUL);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_INT_FALL_HIT_OBJECT_ID);
    WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_FLAG_DISABLE_OPPONENT_PASSIVE);
    WorkModule::set_int(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    fighter.super_jump_punch(L2CValue::Void());
    fighter.sub_shift_status_main(L2CValue::Ptr(gaogaen_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn gaogaen_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32().abs();
    let boma = fighter.module_accessor;
    let frame = MotionModule::frame(boma) as i32;
    let special_air_hi_fall_play_all_rise_frame = WorkModule::get_param_int(boma, hash40("param_special_hi"), hash40("special_air_hi_fall_play_all_rise_frame"));
    if CancelModule::is_enable_cancel(boma) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if !WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_FLAG_REVERSE_LR) {
        if frame < special_air_hi_fall_play_all_rise_frame {
            if WorkModule::get_float(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_LR_STICK_X) < stick_x {
                PostureModule::set_stick_lr(boma, 0.0);
                PostureModule::update_rot_y_lr(boma);
                WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_FLAG_REVERSE_LR);
            }
        }
    }
    if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CAN_CANCEL) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            EFFECT_FOLLOW(fighter, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 0.6, true);
            LAST_EFFECT_SET_RATE(fighter, 1.25);
            WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CANCELED);
            WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CAN_CANCEL);
        }
    }
    fighter.super_jump_punch_main();
    fun_710001a8c0(fighter);
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_TURN.into(), false.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("gaogaen")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, gaogaen_special_hi_main_status)
    .install()
    ;
}