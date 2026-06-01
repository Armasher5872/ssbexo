use super::*;

unsafe extern "C" fn krool_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    ArticleModule::change_status_exist(boma, *FIGHTER_KROOL_GENERATE_ARTICLE_BACKPACK, *WEAPON_KROOL_BACKPACK_STATUS_KIND_FLY);
    fighter.set_situation(SITUATION_KIND_AIR.into());
    GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    fun_710001dba0(fighter, Hash40::new("special_hi").into(), Hash40::new("special_hi").into(), false.into());
    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_UNIQ);
    fun_710001ea30(fighter);
    WorkModule::set_float(boma, 0.5, *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOTION_2ND_LERP_RATE);
    if !StopModule::is_stop(boma) {
        fun_71000210d0(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_71000210d0 as *const () as _));
    GroundModule::select_cliff_hangdata(boma, *FIGHTER_KROOL_CLIFF_HANG_DATA_SPECIAL_HI as u32);
    fighter.sub_shift_status_main(L2CValue::Ptr(krool_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn fun_71000210d0(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    let status_kind_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let boma = fighter.module_accessor;
    let special_hi_fly_acl_y_speed_minus = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fly_acl_y_speed_minus"));
    let special_hi_fly_acl_y = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fly_acl_y"));
    let special_hi_fly_touch_max_spd_x = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fly_touch_max_spd_x"));
    let special_hi_fly_max_spd_x = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fly_max_spd_x"));
    let special_hi_fly_brake_x = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fly_brake_x"));
    let special_hi_fly_stick_mul_max_spd_x = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fly_stick_mul_max_spd_x"));
    let special_hi_fly_stick_mul_acl_spd_y = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fly_stick_mul_acl_spd_y"));
    let special_hi_fly_brake_movement_y = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fly_brake_movement_y"));
    let special_hi_fly_brake_y = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fly_brake_y"));
    let special_hi_fall_max_spd_x = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fall_max_spd_x"));
    let special_hi_fall_stick_mul_max_spd_y = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fall_stick_mul_max_spd_y"));
    let special_hi_fall_max_spd_y = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fall_max_spd_y"));
    let lstack_140 = WorkModule::get_param_float(boma, hash40("param_special_hi"), 0x1f32a7c5bf);
    let special_hi_fly_mul_max_acl_x = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fly_mul_max_acl_x"));
    let movement_y = WorkModule::get_float(boma, *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOVEMENT_Y);
    let brake_after_frame = WorkModule::get_int(boma, *FIGHTER_KROOL_STATUS_SPECIAL_HI_INT_BRAKE_AFTER_FRAME);
    let stop_speed_x = {fighter.clear_lua_stack(); lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP); sv_kinetic_energy::get_speed_x(fighter.lua_state_agent)};
    let gravity_speed_y = {fighter.clear_lua_stack(); lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY); sv_kinetic_energy::get_speed_y(fighter.lua_state_agent)};
    let pyth = ((stop_speed_x*stop_speed_x)+(gravity_speed_y*gravity_speed_y)).sqrt();
    let lstack_150;
    let mut lstack_160;
    let lstack_170;
    let lstack_180;
    let lstack_190 = stick_x*special_hi_fly_mul_max_acl_x;
    if bool_check.get_bool() {
        if stick_x > 0.0 {
            lstack_150 = stick_x;
        }
        else {
            lstack_150 = -stick_x;
        }
        if 1.0 >= special_hi_fly_stick_mul_max_spd_x {
            lstack_160 = 1.0;
        }
        else {
            lstack_160 = ((special_hi_fly_stick_mul_max_spd_x-1.0)*lstack_150)+1.0;
        }
        if status_kind_interrupt != *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL {
            if status_kind_interrupt == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END {
                lstack_160 = 1.0;
            }
        }
        else {
            lstack_160 = 1.0;
        }
        if special_hi_fly_stick_mul_acl_spd_y >= 1.0 {
            lstack_170 = 1.0;
        }
        else {
            lstack_170 = 1.0-((1.0-special_hi_fly_stick_mul_acl_spd_y)*lstack_150);
        }
        if 1.0 >= special_hi_fall_stick_mul_max_spd_y {
            lstack_180 = 1.0;
        }
        else {
            lstack_180 = 1.0+((special_hi_fall_stick_mul_max_spd_y-1.0)*lstack_150);
        }
        if status_kind_interrupt != *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI {
            sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_hi_fall_max_spd_x*lstack_160, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_hi_fall_max_spd_y*lstack_180);
        }
        else {
            if GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_ALL as u32) {
                sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_hi_fly_touch_max_spd_x*lstack_160, 0.0);
            }
            else {
                sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_hi_fly_max_spd_x*lstack_160, 0.0);
            }
            if brake_after_frame > 0 {
                sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -special_hi_fly_brake_y);
                WorkModule::inc_int(boma, *FIGHTER_KROOL_STATUS_SPECIAL_HI_INT_BRAKE_AFTER_FRAME);
            }
            if special_hi_fly_brake_movement_y*10.0 > movement_y {
                sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_hi_fly_acl_y*lstack_170);
            }
            else {
                sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -special_hi_fly_brake_y);
                WorkModule::inc_int(boma, *FIGHTER_KROOL_STATUS_SPECIAL_HI_INT_BRAKE_AFTER_FRAME);
            }
            if gravity_speed_y < 0.0 {
                sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, special_hi_fly_acl_y_speed_minus);
            }
        }
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        if lstack_190 != 0.0 {
            sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, lstack_190, 0.0);
            if movement_y < 0.0 {
                sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, stick_x*lstack_140, 0.0);
            }
        }
        else {
            if lstack_190 == 0.0 {
                sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, special_hi_fly_brake_x, 0.0);
            }
        }
        if gravity_speed_y >= 0.0 {
            WorkModule::set_float(boma, pyth+movement_y, *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOVEMENT_Y);
        }
        else {
            WorkModule::set_float(boma, (-pyth)+movement_y, *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOVEMENT_Y);
        }
        if status_kind_interrupt != *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI {
            if status_kind_interrupt != *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END {
                fun_710001e090(fighter, Hash40::new("special_hi_air_end_f").into(), Hash40::new("special_hi_air_end_b").into());
            }
            if status_kind_interrupt != *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL {
                fun_710001e090(fighter, Hash40::new("special_hi_fall_f").into(), Hash40::new("special_hi_fall_b").into());
            }
        }
        else {
            fun_710001e090(fighter, Hash40::new("special_hi_f").into(), Hash40::new("special_hi_b").into());
        }
    }
    0.into()
}

unsafe extern "C" fn fun_710001e090(fighter: &mut L2CFighterCommon, f_mot: L2CValue, b_mot: L2CValue) {
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let boma = fighter.module_accessor;
    let frame = MotionModule::frame(boma);
    let motion_kind_2nd = MotionModule::motion_kind_2nd(boma);
    let rate = MotionModule::rate(boma);
    let lr = PostureModule::lr(boma);
    let mut lerp_rate = WorkModule::get_float(boma, *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOTION_2ND_LERP_RATE);
    let turn_stick_x = WorkModule::get_param_float(boma, hash40("common"), hash40("turn_stick_x"));
    let mk2d;
    let weight;
    if stick_x >= -0.1 {
        if 0.1 >= stick_x {
            if lerp_rate >= 0.5 {
                lerp_rate = fighter.clamp((lerp_rate-0.05).into(), 0.5f32.into(), 1.0f32.into()).get_f32();
            }
            else {
                lerp_rate = fighter.clamp((lerp_rate+0.05).into(), 0.5f32.into(), 1.0f32.into()).get_f32();
            }
        }
    }
    if stick_x*lr > turn_stick_x {
        lerp_rate = fighter.clamp((lerp_rate-0.05).into(), 0.0f32.into(), 1.0f32.into()).get_f32();
    }
    else {
        lerp_rate = fighter.clamp((lerp_rate+0.05).into(), 0.0f32.into(), 1.0f32.into()).get_f32();
    }
    WorkModule::set_float(boma, lerp_rate, *FIGHTER_KROOL_STATUS_SPECIAL_HI_FLOAT_MOTION_2ND_LERP_RATE);
    if lerp_rate >= 0.5 {
        mk2d = b_mot.get_u64();
        weight = (lerp_rate-0.5)*2.0;
    }
    else {
        mk2d = f_mot.get_u64();
        weight = 1.0-(lerp_rate*2.0);
    }
    if motion_kind_2nd != mk2d {
        MotionModule::add_motion_2nd(boma, Hash40::new_raw(mk2d), frame, rate, true, weight);
        MotionModule::set_weight(boma, 1.0-weight, true);
    }
    else {
        MotionModule::set_weight(boma, 1.0-weight, true);
    }
}

unsafe extern "C" fn krool_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let special_hi_fly_brake_after_frame = if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE) {
        11
    }
    else {
        WorkModule::get_param_int(boma, hash40("param_special_hi"), hash40("special_hi_fly_brake_after_frame"))
    };
    let special_hi_fly_start_air_frame = WorkModule::get_param_int(boma, hash40("param_special_hi"), hash40("special_hi_fly_start_air_frame"));
    let brake_after_frame = WorkModule::get_int(boma, *FIGHTER_KROOL_STATUS_SPECIAL_HI_INT_BRAKE_AFTER_FRAME);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(boma) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if special_hi_fly_brake_after_frame <= brake_after_frame {
        fighter.change_status(FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END.into(), false.into());
        return 0.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        if special_hi_fly_start_air_frame < current_frame {
            fighter.change_status(FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_LANDING.into(), false.into());
            return 0.into();
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("krool")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI, krool_special_hi_main_status)
    .install()
    ;
}