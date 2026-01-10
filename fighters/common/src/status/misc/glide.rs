//Credit to: Liam Estares (LKE Studios), Ben Hall (arthur!), and WuBoytH
use super::*;

unsafe extern "C" fn status_glide_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let params = GlideParams::get();
    WorkModule::set_float(fighter.module_accessor, params.base_speed, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
    WorkModule::set_float(fighter.module_accessor, -sum_speed_y, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GRAVITY);
    let initial_speed = params.base_speed * lr;
    KineticUtility::reset_enable_energy(*FIGHTER_KINETIC_ENERGY_ID_STOP, fighter.module_accessor, *ENERGY_STOP_RESET_TYPE_FREE, &Vector2f{x: initial_speed, y: 0.0}, &Vector3f{x: initial_speed, y: 0.0, z: 0.0});
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fighter.module_accessor);
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_Glide)]
unsafe extern "C" fn status_glide(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_GLIDE);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_GLIDE_LANDING);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GLIDE_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("glide_direction"), 90.0, 0.0, true, 0.0, false, false);
    MotionModule::add_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, Hash40::new("glide_wing"), 0.0, 1.0, true, false, 0.0, false, true, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Glide_Main as *const () as _))
}

#[skyline::hook(replace = L2CFighterCommon_status_Glide_Main)]
unsafe extern "C" fn status_glide_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let sum_speed_length = KineticModule::get_sum_speed_length(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let frame = MotionModule::frame(fighter.module_accessor);
    let glide_timer = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_GLIDE_TIMER);
    let glide_landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("glide_landing_frame"));
    let glide_landing_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("glide_landing_speed"));
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_group_check_air_landing().get_bool() {
        return 0.into();
    }
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_GLIDE_TIMER);
    if glide_timer >= 300 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_STOP);
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        if glide_landing_frame <= frame {
            if glide_landing_speed <= sum_speed_length {
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_LANDING.into(), false.into());
                return 0.into();
            }
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_STOP) {
            fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_END.into(), false.into());
            return 0.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GLIDE_ATTACK) && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            fighter.change_status(FIGHTER_STATUS_KIND_GLIDE_ATTACK.into(), true.into());
            return 0.into();
        }
    }
    0.into()
}

unsafe extern "C" fn status_glide_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let params = GlideParams::get();
    let lr = PostureModule::lr(fighter.module_accessor);
    let _energy_stop = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let mut angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
    let mut angle_speed = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
    let mut stick_angle = ControlModule::get_stick_angle(fighter.module_accessor);
    if lr <= 0.0 {
        let mut above_or_below = -1.0;
        if stick_angle > 0.0 {
            above_or_below = 1.0;
        }
        stick_angle = (180.0 * above_or_below) - (stick_angle * 180.0 / std::f32::consts::PI);
    }
    else {
        stick_angle = stick_angle * 180.0 / std::f32::consts::PI;
    }
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let stick_magnitude = (stick_x * stick_x + stick_y * stick_y).sqrt();
    if stick_magnitude > params.radial_stick {
        let angle_accel = if stick_angle < 0.0 {
            if stick_angle >= -135.0 {
                -params.down_angle_accel //What is angle_accel here?
            }
            else {
                params.up_angle_accel
            }   
        }
        else {
            if stick_angle >= 45.0 {
                params.up_angle_accel
            }
            else {
                -params.down_angle_accel
            }
        };
        let scaled_angle_accel = angle_accel * (stick_magnitude - params.radial_stick) / (1.0 - params.radial_stick);
        if angle_speed * scaled_angle_accel < 0.0 {
            angle_speed = 0.0;
        }
        let mut new_angle_speed = angle_speed + scaled_angle_accel;
        new_angle_speed = new_angle_speed.clamp(-params.max_angle_speed, params.max_angle_speed);
        WorkModule::set_float(fighter.module_accessor, new_angle_speed, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
        angle += new_angle_speed;
    }
    angle = angle.clamp(params.angle_max_down, params.angle_max_up);
    let mut power = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
    power -= angle * params.speed_change / 90.0;
    // instead of setting the status flag for touching a wall,
    // we can just check it directly in this code
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) {
        power -= 0.01;
    }
    if power < 0.0 {
        power = 0.0
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_RAPID_FALL) {
        if angle < params.angle_more_speed {
            power += params.down_speed_add * (params.angle_more_speed - angle) / (params.angle_more_speed - params.angle_max_down);
        }
    }
    else if angle > 0.0 {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_RAPID_FALL);
    }
    let gravity = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GRAVITY);
    let mut new_gravity = gravity + params.gravity_accel;
    if new_gravity > params.gravity_speed {
        new_gravity = params.gravity_speed;
    }
    WorkModule::set_float(fighter.module_accessor, new_gravity, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GRAVITY);
    //let unrotated = Vector2f { x: power * lr, y: 0.0 };
    // TODO: probably want to make a new function for this, it doesn't seem like
    // the vec2_rot function from the game does what we want
    //let mut angled = smash::app::sv_math::vec2_rot(angle * lr * std::f32::consts::PI / 180.0, unrotated, 0.0 /*There's 3rd arg here*/);
    let mut angled = Vector2f {x: power * angle.to_radians().cos() * lr, y: power * angle.to_radians().sin()};
    angled.y -= new_gravity;
    let speed = (angled.x * angled.x + angled.y * angled.y).sqrt();
    let ratio = params.max_speed / speed;
    if speed > params.max_speed {
        angled.x *= ratio;
        angled.y *= ratio;
    }
    if speed < params.end_speed || power <= 0.0 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_FLAG_STOP);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE_SPEED);
    }
    // TODO: figure out how to set X and Y speed directly in an Energy
    //energy_stop.speed_x = angled.x;
    //energy_stop.speed_y = angled.y;
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, angled.x, angled.y);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, angled.x, angled.y);
    WorkModule::set_float(fighter.module_accessor, power, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
    MotionModule::set_frame(fighter.module_accessor, 90.0 - angle, false);
    WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
    let kind = fighter.global_table[0x2].get_i32();
    if kind == *FIGHTER_KIND_METAKNIGHT {
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_metaknight_glide_loop"), 1.0 + angle * -0.0035);
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_bind_address_call_status_end_Glide)]
unsafe extern "C" fn bind_address_call_status_end_glide(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    fighter.status_end_Glide()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_Glide)]
unsafe extern "C" fn status_glide_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_GLIDE_TIMER);
    0.into()
}

unsafe extern "C" fn status_glide_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_GLIDE_TIMER);
    MotionModule::remove_motion_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, false);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_glide,
            status_glide_main,
            bind_address_call_status_end_glide, 
            status_glide_end
        );
    }
}

pub fn install() {
    Agent::new("fighter")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *FIGHTER_STATUS_KIND_GLIDE, status_glide_init)
    .status(Exec, *FIGHTER_STATUS_KIND_GLIDE, status_glide_exec)
    .status(Exit, *FIGHTER_STATUS_KIND_GLIDE, status_glide_exit)
    .install()
    ;
	let _ = skyline::nro::add_hook(nro_hook).unwrap();
}