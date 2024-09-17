/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//Status Attack Air Main Common, used for continual platform drops and ECB Shifts
#[skyline::hook(replace = L2CFighterCommon_status_AttackAir_Main_common)]
unsafe extern "C" fn status_attackair_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.attack_air_common_strans().get_bool() {
        /* START OF NEW ADDITIONS */
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
        let frame = fighter.global_table[CURRENT_FRAME].get_f32();
        let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
        let motion_kind = MotionModule::motion_kind(boma);
        let lr = PostureModule::lr(boma);
        let stick_x = fighter.global_table[STICK_X].get_f32()*lr;
        let pos = Vector3f{x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)}; // get current pos
        let get_sum_speed_x = lr*KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if prev_status_kind == *FIGHTER_STATUS_KIND_PASS {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) != true {
                GroundModule::set_passable_check(boma, true);
            }
        }
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) 
        && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
        }
        //Yoshi
        if fighter_kind == *FIGHTER_KIND_YOSHI
        && prev_status_kind != *FIGHTER_STATUS_KIND_TREAD_JUMP
        && motion_kind == hash40("attack_air_lw")
        && (12.0..=36.0).contains(&frame)
        && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.015);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.05);
        }
        //Captain Falcon Voice Exclamation
        if fighter_kind == *FIGHTER_KIND_CAPTAIN
        && motion_kind == hash40("attack_air_f")
        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
        && frame == 14.0
        && LAST_ATTACK_HITBOX_ID == 0 {
            macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_special_h03"));
            macros::PLAY_SE(fighter, Hash40::new("vc_captain_appeal03"));
        }
        //Sheik Dair Bounce
        if fighter_kind == *FIGHTER_KIND_SHEIK 
        && motion_kind == hash40("attack_air_lw")
        && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD))
        && WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE) {
            fighter.clear_lua_stack();
            lua_args!(fighter, get_sum_speed_x, 2.35, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            sv_animcmd::SET_SPEED_EX(fighter.lua_state_agent);
            WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
        }
        //Angleable Metaknight Dair
        if fighter_kind == *FIGHTER_KIND_METAKNIGHT 
        && motion_kind == hash40("attack_air_lw") 
        && frame < 7.0 {
            if stick_x > 0.5 {
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("attack_air_lw_diagonal_r"), -1.0, 1.0, 0.0, false, false);
            }
            if stick_x < -0.5 {
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("attack_air_lw_diagonal_l"), -1.0, 1.0, 0.0, false, false);
            }
        }
        //Ike Aerial Bounce
        if fighter_kind == *FIGHTER_KIND_IKE {
            let ike_bound_angle = WorkModule::get_float(boma, FIGHTER_IKE_INSTANCE_WORK_ID_FLOAT_BOUND_ANGLE);
            let ike_x_check = WorkModule::get_float(boma, FIGHTER_IKE_INSTANCE_WORK_ID_FLOAT_X_CHECK);
            let ike_y_check = WorkModule::get_float(boma, FIGHTER_IKE_INSTANCE_WORK_ID_FLOAT_Y_CHECK);
            let data = AttackModule::attack_data(boma, 0, false);
            let ike_damage: f32 = (*data).power;
            let hit_pos = &mut Vector2f{x: 0.0, y: 0.0};
            let ray_check = GroundModule::ray_check_hit_pos(boma, &Vector2f{x: pos.x, y: pos.y}, &Vector2f{x: 0.0, y: ike_y_check}, hit_pos, true);
            let wall_check = (
                GroundModule::ray_check(boma, &Vector2f{x: pos.x+ike_x_check, y: pos.y+ike_y_check}, &Vector2f{x: 6.0*lr, y: 0.0}, true) == 1
                || GroundModule::ray_check(boma, &Vector2f{x: pos.x+(ike_x_check*0.9), y: pos.y+(ike_y_check*0.9)}, &Vector2f{x: 6.0*lr, y: 0.0}, true) == 1
                || GroundModule::ray_check(boma, &Vector2f{x: pos.x+(ike_x_check*0.8), y: pos.y+(ike_y_check*0.8)}, &Vector2f{x: 6.0*lr, y: 0.0}, true) == 1
                || GroundModule::ray_check(boma, &Vector2f{x: pos.x+(ike_x_check*0.7), y: pos.y+(ike_y_check*0.7)}, &Vector2f{x: 6.0*lr, y: 0.0}, true) == 1
                || GroundModule::ray_check(boma, &Vector2f{x: pos.x+(ike_x_check*0.6), y: pos.y+(ike_y_check*0.6)}, &Vector2f{x: 6.0*lr, y: 0.0}, true) == 1
                || GroundModule::ray_check(boma, &Vector2f{x: pos.x+(ike_x_check*0.5), y: pos.y+(ike_y_check*0.5)}, &Vector2f{x: 6.0*lr, y: 0.0}, true) == 1
                || GroundModule::ray_check(boma, &Vector2f{x: pos.x+(ike_x_check*0.4), y: pos.y+(ike_y_check*0.4)}, &Vector2f{x: 6.0*lr, y: 0.0}, true) == 1
                || GroundModule::ray_check(boma, &Vector2f{x: pos.x+(ike_x_check*0.3), y: pos.y+(ike_y_check*0.3)}, &Vector2f{x: 6.0*lr, y: 0.0}, true) == 1
                || GroundModule::ray_check(boma, &Vector2f{x: pos.x+(ike_x_check*0.2), y: pos.y+(ike_y_check*0.2)}, &Vector2f{x: 6.0*lr, y: 0.0}, true) == 1
                || GroundModule::ray_check(boma, &Vector2f{x: pos.x+(ike_x_check*0.1), y: pos.y+(ike_y_check*0.1)}, &Vector2f{x: 6.0*lr, y: 0.0}, true) == 1
            );
            if WorkModule::is_flag(boma, FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_CAN_BOUND) 
            && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || ray_check || wall_check) {
                WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
                WorkModule::set_flag(boma, false, FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_CAN_BOUND);
            }
            if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE) {
                let mut bound_speed_x = ike_bound_angle.to_radians().sin()*(ike_damage/5.0);
                let mut bound_speed_y = ike_bound_angle.to_radians().cos()*(ike_damage/5.0);
                if ike_bound_angle > 180.0 && ike_bound_angle < 0.0 {
                    bound_speed_x = -(ike_bound_angle.to_radians().sin()*(ike_damage/5.0));
                    bound_speed_y = -(ike_bound_angle.to_radians().cos()*(ike_damage/5.0));
                }
                if motion_kind == hash40("attack_air_b") {
                    fighter.clear_lua_stack();
                    lua_args!(fighter, bound_speed_x/4.0, bound_speed_y/2.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    sv_animcmd::SET_SPEED_EX(fighter.lua_state_agent);
                }
                WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
            }
        }
        //R.O.B. Power Boosting
        if fighter_kind == *FIGHTER_KIND_ROBOT {
            let energy = WorkModule::get_float(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
            if WorkModule::is_flag(boma, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST) {
                if motion_kind == hash40("attack_air_n") {
                    if energy > 72.0 && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                        WorkModule::set_flag(boma, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST);
                        WorkModule::set_flag(boma, true, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST);
                        WorkModule::set_flag(boma, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_EARLY_CANCEL);
                    }
                    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST) {
                        MotionModule::set_rate(fighter.module_accessor, 1.0);
                    }
                }
                if motion_kind == hash40("attack_air_b") {
                    if energy > 120.0 && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                        WorkModule::set_flag(boma, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST);
                        WorkModule::set_flag(boma, true, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST);
                        WorkModule::set_flag(boma, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_EARLY_CANCEL);
                    }
                    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST) && frame < 19.0 {
                        let rate = (19.0-frame)/(4.0+(19.0-frame));
                        MotionModule::set_rate(fighter.module_accessor, rate);
                    }
                }
                if motion_kind == hash40("attack_air_lw") {
                    if energy > 144.0 && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                        WorkModule::set_flag(boma, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST);
                        WorkModule::set_flag(boma, true, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST);
                        WorkModule::set_flag(boma, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_EARLY_CANCEL);
                    }
                    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST) && frame < 20.0 {
                        let rate = (20.0-frame)/(4.0+(20.0-frame));
                        MotionModule::set_rate(fighter.module_accessor, rate);
                    }
                }
            }
            if WorkModule::is_flag(boma, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_EARLY_CANCEL) {
                if motion_kind == hash40("attack_air_n") {
                    if frame >= 35.0 {
                        CancelModule::enable_cancel(boma);
                    }
                }
                if motion_kind == hash40("attack_air_b") {
                    if frame >= 40.0 {
                        CancelModule::enable_cancel(boma);
                    }
                }
                if motion_kind == hash40("attack_air_lw") {
                    if frame >= 45.0 {
                        CancelModule::enable_cancel(boma);
                    }
                }
            }
        }
        /* END OF NEW ADDITIONS */
        if !CancelModule::is_enable_cancel(boma) {
            if MotionModule::is_end(boma) {
                WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            return false.into();
        }
        else {
            if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                if fighter.sub_air_check_fall_common().get_bool() {
                    return true.into();
                }
                if !MotionModule::is_end(boma) {
                    return false.into();
                }
                WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
        }
    }
    true.into()
}

//Status End Attack Air, clears flags
#[skyline::hook(replace = L2CFighterCommon_status_end_AttackAir)]
unsafe extern "C" fn status_end_attackair(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    if fighter_kind == *FIGHTER_KIND_GANON {
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CURRENT_DAMAGE);
        WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);	
    }
    if fighter_kind == *FIGHTER_KIND_ROBOT {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST);
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST);
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_EARLY_CANCEL);
    }
    0.into()
}

//Sub Attack Air Inherit Jump Aerial Motion Uniq Process Init, inherits the initial motion of double jump
#[skyline::hook(replace = L2CFighterCommon_sub_attack_air_inherit_jump_aerial_motion_uniq_process_init)]
unsafe extern "C" fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    if ![*FIGHTER_KIND_NESS, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_LUCAS].contains(&fighter_kind) {
        call_original!(fighter)
    }
    else {
        let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        fighter.sub_attack_air_kind();
        if [hash40("jump_aerial_f"), hash40("jump_aerial_b")].contains(&motion_kind) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
                MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new_raw(motion_kind), frame, 1.0, false, 1.0);
                MotionModule::set_weight(fighter.module_accessor, 1.0, true);
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) || frame < 2.0 {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
                } 
                else {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
                }
            }
            else {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            }
        } 
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        }
        let _ = fighter.sub_attack_air_uniq_process_init();
        0.into()
    }
}

//Sub Attack Air Inherit Jump Aerial Motion Uniq Process Exec, inherits the momentum of double jump
#[skyline::hook(replace = L2CFighterCommon_sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec)]
unsafe extern "C" fn sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
    if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND 
    && [*FIGHTER_KIND_NESS, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_LUCAS].contains(&fighter_kind)
    && MotionModule::frame_2nd(fighter.module_accessor) >= 2.0
    && fighter.global_table[CURRENT_FRAME].get_f32() <= 5.0
    && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    }
    call_original!(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_attackair_main_common,
            status_end_attackair,
            sub_attack_air_inherit_jump_aerial_motion_uniq_process_init,
            sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}