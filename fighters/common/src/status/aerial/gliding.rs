//Credit to: Liam Estares (LKE Studios), Ben Hall (arthur!), and WuBoytH
use super::*;

unsafe extern "C" fn status_glidestart_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_GLIDE_START, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, 0, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, 0, 0);
    0.into()
}

unsafe extern "C" fn status_glidestart_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut smash::app::KineticEnergy;
    let motion = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION) as *mut smash::app::KineticEnergy;
    let lr = PostureModule::lr(fighter.module_accessor);
    let glide_start_air_accel_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("glide_start_air_accel_y_mul"));
    let glide_start_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("glide_start_speed_x_mul"));
    let glide_start_speed_y_add = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("glide_start_speed_y_add"));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GLIDE_START);
    smash::app::lua_bind::KineticEnergy::reset_energy(gravity, *ENERGY_GRAVITY_RESET_TYPE_GLIDE_START, &Vector2f{x: 0.0, y: -glide_start_air_accel_y_mul}, &Vector3f{x: 0.0, y: -glide_start_air_accel_y_mul, z: 0.0}, fighter.module_accessor);
    smash::app::lua_bind::KineticEnergy::reset_energy(motion, *ENERGY_GRAVITY_RESET_TYPE_GLIDE_START, &Vector2f{x: glide_start_speed_x_mul * lr, y: 0.0}, &Vector3f{x: glide_start_speed_x_mul * lr, y: 0.0, z: 0.0}, fighter.module_accessor);
    KineticUtility::reset_enable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, fighter.module_accessor, *ENERGY_STOP_RESET_TYPE_GLIDE_START, &Vector2f{x: 0.0, y: glide_start_speed_y_add}, &Vector3f{x: 0.0, y: glide_start_speed_y_add, z: 0.0});
    0.into()
}

unsafe extern "C" fn status_glidestart_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE_LANDING);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_start"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(glidestart_main_sub as *const () as _))
}

unsafe extern "C" fn glidestart_main_sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE) || 
    WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE_LANDING) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_LANDING.into(), false.into());
            return 0.into();
        }
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR && 
        MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_GLIDE.into(), false.into());
            return 0.into();
        }
    }
    0.into()
}

pub unsafe extern "C" fn status_glidestart_end(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn status_glide_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, 0, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, 0, 0);
    0.into()
}

unsafe extern "C" fn status_glide_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let base_power = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("base_power"));
    WorkModule::set_float(fighter.module_accessor, base_power, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
    WorkModule::set_float(fighter.module_accessor, -sum_speed_y, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GRAVITY);
    let initial_speed = base_power * lr;
    KineticUtility::reset_enable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, fighter.module_accessor, *ENERGY_STOP_RESET_TYPE_FREE, &Vector2f{x: initial_speed, y: 0.0}, &Vector3f{x: initial_speed, y: 0.0, z: 0.0});
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fighter.module_accessor);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
    0.into()
}

unsafe extern "C" fn status_glide_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_GLIDE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE_LANDING);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GLIDE_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_direction"), 90.0, 0.0, true, 0.0, false, false);
    MotionModule::add_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, Hash40::new("glide_wing"), 0.0, 1.0, true, false, 0.0, false, true, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(glide_main_sub as *const () as _))
}

unsafe extern "C" fn glide_main_sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let frame = MotionModule::frame(fighter.module_accessor);
        let glide_landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("glide_landing_frame"));
        if glide_landing_frame <= frame {
            let sum_speed_length = KineticModule::get_sum_speed_length(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let glide_landing_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("glide_landing_speed"));
            if glide_landing_speed <= sum_speed_length {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_LANDING.into(), false.into());
                return 0.into();
            }
        }
    }
    if fighter.sub_transition_group_check_air_landing().get_bool() {
        return 0.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
        || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_STOP) {
            fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_END.into(), false.into());
            return 0.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GLIDE_ATTACK)
        && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_ATTACK.into(), true.into());
            return 0.into();
        }
    }
    0.into()
}

unsafe extern "C" fn status_glide_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let mut angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
    let mut angle_speed = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
    let mut stick_angle = ControlModule::get_stick_angle(fighter.module_accessor);
    if lr <= 0.0 {
        let mut above_or_below = -1.0;
        if stick_angle > 0.0 {
            above_or_below = 1.0;
        }
        stick_angle = (180.0 * above_or_below) - (stick_angle * 180.0 / PI);
    }
    else {
        stick_angle = stick_angle * 180.0 / PI;
    }
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let stick_magnitude = (stick_x.powi(2) + stick_y.powi(2)).sqrt(); //Square Root of Stick X^2 + Stick Y^2
    let radial_stick_sensitivity = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("radial_stick_sensitivity"));
    let angle_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_speed_max"));
    let angle_speed_stick_add = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_speed_stick_add"));
    if stick_magnitude <= radial_stick_sensitivity {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_STOP) {
            if angle_speed < 0.0 {
                angle_speed = 0.0;
            }
            let mut added_angle_speed = angle_speed + angle_speed_stick_add;
            if added_angle_speed < -angle_speed_max {
                added_angle_speed = -angle_speed_max;
            }
            if added_angle_speed > angle_speed_max {
                added_angle_speed = angle_speed_max;
            }
            WorkModule::set_float(fighter.module_accessor, added_angle_speed, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
            angle += added_angle_speed;
        }
    }
    let angle_accel_up = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_accel_up"));
    let angle_accel_down = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_accel_down"));
    if stick_magnitude > radial_stick_sensitivity {
        let angle_accel = if stick_angle < 0.0 {
            if stick_angle >= -135.0 {
                -angle_accel_down
            }
            else {
                angle_accel_up
            }   
        }
        else {
            if stick_angle >= 45.0 {
                angle_accel_up
            }
            else {
                -angle_accel_down
            }
        };
        let scaled_angle_accel = angle_accel * (stick_magnitude - radial_stick_sensitivity) / (1.0 - radial_stick_sensitivity);
        if angle_speed * scaled_angle_accel < 0.0 {
            angle_speed = 0.0;
        }
        let mut new_angle_speed = angle_speed + scaled_angle_accel;
        new_angle_speed = new_angle_speed.clamp(-angle_speed_max, angle_speed_max);
        WorkModule::set_float(fighter.module_accessor, new_angle_speed, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
        angle += new_angle_speed;
    }
    let angle_up_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_up_max"));
    let angle_down_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_down_max"));
    angle = angle.clamp(angle_down_max, angle_up_max);    
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_STOP) {
        let mut power = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
        let angle_power_rate = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_power_rate"));
        power -= angle * angle_power_rate / 90.0;
        //Instead of setting the status flag for touching a wall, we can just check it directly in this code
        if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) {
            let glide_touch_decel = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("glide_touch_decel"));
            power -= glide_touch_decel;
        }
        if power < 0.0 {
            power = 0.0
        }
        let angle_more_power = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_more_power"));
        let down_add_power_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("down_add_power_max"));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_RAPID_FALL) {
            if angle < angle_more_power {
                power += down_add_power_max * (angle_more_power - angle) / (angle_more_power - angle_down_max);
            }
        }
        else if angle > 0.0 {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_RAPID_FALL);
        }
        let gravity = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GRAVITY);
        let gravity_accel = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("gravity_accel"));
        let gravity_speed_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("gravity_speed_max"));
        let mut new_gravity = gravity + gravity_accel;
        if new_gravity > gravity_speed_max {
            new_gravity = gravity_speed_max;
        }
        WorkModule::set_float(fighter.module_accessor, new_gravity, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GRAVITY);
        let mut angled = Vector2f {x: power * angle.to_radians().cos() * lr, y: power * angle.to_radians().sin()};
        angled.y -= new_gravity;
        /*Made a new function for the Vector2f above ^, it doesn't seem like the vec2_rot function in Ultimate does what we want*/
        let speed = (angled.x * angled.x + angled.y * angled.y).sqrt(); //Square Root of angled X value + angled Y angle
        let max_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("max_speed"));
        let ratio = max_speed / speed;
        if speed > max_speed {
            angled.x *= ratio;
            angled.y *= ratio;
        }
        let end_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("end_speed"));
        if speed < end_speed || power <= 0.0 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_STOP);
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
        }
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, angled.x, angled.y);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, angled.x, angled.y);
        WorkModule::set_float(fighter.module_accessor, power, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
    }
    else {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        let brake = sv_kinetic_energy::get_brake_x(fighter.pop_lua_stack(0).into());
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        let limit = sv_kinetic_energy::get_limit_speed_x(fighter.pop_lua_stack(0).into());
        let brake_speed = Vector2f {x: brake * lr, y: brake};
        let limit_speed = Vector2f {x: limit * lr, y: limit};
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, brake_speed.x, brake_speed.y);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, limit_speed.x, limit_speed.y);
        let end_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("end_speed"));
        let angle_dummy = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_dummy"));
        if angle_up_max < angle_dummy {
            if end_speed < 0.0 {
                WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_STOP);
            }
        }
    }
    MotionModule::set_frame(fighter.module_accessor, 90.0 - angle, false);
    WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
    0.into()
}

unsafe extern "C" fn status_glide_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, false);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_Glide)]
unsafe extern "C" fn bind_address_call_status_end_glide(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    fighter.status_end_Glide()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_Glide)]
unsafe extern "C" fn status_glide_end(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            bind_address_call_status_end_glide, 
            status_glide_end
        );
    }
}

pub fn install() {
    Agent::new("fighter")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_GLIDE_START, status_glidestart_pre)
    .status(Init, *FIGHTER_STATUS_KIND_GLIDE_START, status_glidestart_init)
    .status(Main, *FIGHTER_STATUS_KIND_GLIDE_START, status_glidestart_main)
    .status(End, *FIGHTER_STATUS_KIND_GLIDE_START, status_glidestart_end)
    .status(Pre, *FIGHTER_STATUS_KIND_GLIDE, status_glide_pre)
    .status(Init, *FIGHTER_STATUS_KIND_GLIDE, status_glide_init)
    .status(Main, *FIGHTER_STATUS_KIND_GLIDE, status_glide_main)
    .status(Exec, *FIGHTER_STATUS_KIND_GLIDE, status_glide_exec)
    .status(Exit, *FIGHTER_STATUS_KIND_GLIDE, status_glide_exit)
    .install();
	let _ = skyline::nro::add_hook(nro_hook).unwrap();
}