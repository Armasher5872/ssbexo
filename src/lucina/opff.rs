use super::*;

#[fighter_frame( agent = FIGHTER_KIND_LUCINA )]
fn lucina_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(module_accessor);
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let lr = PostureModule::lr(module_accessor);
        let stick_x = ControlModule::get_stick_x(module_accessor) * lr;
        let stick_y = ControlModule::get_stick_y(module_accessor);
        let frame = MotionModule::frame(module_accessor);
        let valuea = Vector3f{x: 0.0, y: 0.0, z: 2.0};
        let valueb = Vector3f{x: 0.0, y: 0.0, z: 5.0};
        let valuec = Vector3f{x: 0.0, y: 0.0, z: 8.0};
        let statuses = [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT].contains(&status_kind); 
        LUCINA_GFX_COUNTER[entry_id] += 1;
        if LUCINA_GFX_COUNTER[entry_id] >= 6
        && statuses {
            EffectModule::req_follow(module_accessor, Hash40{hash: hash40("sys_damage_elec")}, Hash40::new("sword2"), &valuea, &valuea, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.3, 1.0);
            EffectModule::req_follow(module_accessor, Hash40{hash: hash40("sys_damage_elec")}, Hash40{hash: hash40("sword2")}, &valueb, &valueb, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.3, 1.0);
            EffectModule::req_follow(module_accessor, Hash40{hash: hash40("sys_damage_elec")}, Hash40::new("sword2"), &valuec, &valuec, 0.25, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.3, 1.0);
            LUCINA_GFX_COUNTER[entry_id] = 0;
        }
        else {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_elec"), false, false);
        }
        if motion_kind == hash40("attack_12")
        && frame >= 10.0
        && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            MotionModule::change_motion(module_accessor, Hash40::new("attack_13"), 1.0, 1.0, false, 0.0, false, false);
        }
        if [hash40("special_n_start"), hash40("special_air_n_start")].contains(&motion_kind)
        && MotionModule::end_frame(module_accessor) - frame <= 2.0 {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX, true);
        }
        if StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND
        || [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FALL].contains(&status_kind) {
            USE_SWORDSMAN_DASH[entry_id] = true;
            USE_UP_SPECIAL[entry_id] = true;
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            fighter.sub_transition_group_check_air_cliff();
            USE_SWORDSMAN_DASH[entry_id] = false;
            USE_UP_SPECIAL[entry_id] = false;
        }
        if status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX
        && frame < 5.0
        && stick_x < -0.2 {
            PostureModule::reverse_lr(module_accessor);
            PostureModule::update_rot_y_lr(module_accessor);
        }
        if motion_kind == hash40("special_s1")
        && (12.0..=30.0).contains(&frame)
        && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if stick_y > 0.75 {
                MotionModule::change_motion(module_accessor, Hash40::new("special_s2_hi"), 1.0, 1.0, false, 0.0, false, false);
            }
            else if stick_y < -0.75 {
                MotionModule::change_motion(module_accessor, Hash40::new("special_s2_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                DID_ASTRA_2_S[entry_id] = true;
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, true);
            }
        }
        if motion_kind == hash40("special_air_s1")
        && (12.0..=30.0).contains(&frame)
        && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if stick_y > 0.75 {
                MotionModule::change_motion(module_accessor, Hash40::new("special_air_s2_hi"), 1.0, 1.0, false, 0.0, false, false);
            }
            else if stick_y < -0.75 {
                MotionModule::change_motion(module_accessor, Hash40::new("special_air_s2_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                DID_ASTRA_2_S[entry_id] = true;
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, true);
            }
        }
        if status_kind == *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2
        && DID_ASTRA_2_S[entry_id] {
            DID_ASTRA_2_S[entry_id] = false;
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                MotionModule::change_motion(module_accessor, Hash40::new("special_s2_s"), 1.0, 1.0, false, 0.0, false, false);
            }
            else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                MotionModule::change_motion(module_accessor, Hash40::new("special_air_s2_s"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion_kind == hash40("special_s4_s")
        && (10.0..=27.0).contains(&frame)
        && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if stick_y > 0.75 {
                DID_ASTRA_5_HI[entry_id] = true;
                LANDING_HIT[entry_id] = false;
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            else if stick_y < -0.75 {
                MotionModule::change_motion(module_accessor, Hash40::new("special_s5_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(module_accessor, Hash40::new("special_s5_s"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion_kind == hash40("special_air_s4_s")
        && (10.0..=27.0).contains(&frame)
        && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if stick_y > 0.75 {
                DID_ASTRA_5_HI[entry_id] = true;
                LANDING_HIT[entry_id] = false;
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            else if stick_y < -0.75 {
                MotionModule::change_motion(module_accessor, Hash40::new("special_air_s5_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(module_accessor, Hash40::new("special_air_s5_s"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion_kind == hash40("special_s4_hi")
        && (11.0..=28.0).contains(&frame)
        && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if stick_y > 0.75 {
                DID_ASTRA_5_HI[entry_id] = true;
                LANDING_HIT[entry_id] = false;
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            else if stick_y < -0.75 {
                MotionModule::change_motion(module_accessor, Hash40::new("special_s5_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(module_accessor, Hash40::new("special_s5_s"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion_kind == hash40("special_air_s4_hi")
        && (11.0..=28.0).contains(&frame)
        && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if stick_y > 0.75 {
                DID_ASTRA_5_HI[entry_id] = true;
                LANDING_HIT[entry_id] = false;
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            else if stick_y < -0.75 {
                MotionModule::change_motion(module_accessor, Hash40::new("special_air_s5_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(module_accessor, Hash40::new("special_air_s5_s"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion_kind == hash40("special_s4_lw")
        && (22.0..=39.0).contains(&frame)
        && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if stick_y > 0.75 {
                DID_ASTRA_5_HI[entry_id] = true;
                LANDING_HIT[entry_id] = false;
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            else if stick_y < -0.75 {
                MotionModule::change_motion(module_accessor, Hash40::new("special_s5_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(module_accessor, Hash40::new("special_s5_s"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion_kind == hash40("special_air_s4_lw")
        && (22.0..=39.0).contains(&frame)
        && ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            if stick_y > 0.75 {
                DID_ASTRA_5_HI[entry_id] = true;
                LANDING_HIT[entry_id] = false;
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
            else if stick_y < -0.5 {
                MotionModule::change_motion(module_accessor, Hash40::new("special_air_s5_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(module_accessor, Hash40::new("special_air_s5_s"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
        && DID_ASTRA_5_HI[entry_id] {
            DID_ASTRA_5_HI[entry_id] = false;
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                MotionModule::change_motion(module_accessor, Hash40::new("special_s5_hi"), 1.0, 1.0, false, 0.0, false, false);
            }
            else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                MotionModule::change_motion(module_accessor, Hash40::new("special_air_s5_hi"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
        if motion_kind == hash40("special_air_s5_hi") 
        && KineticModule::get_kinetic_type(module_accessor) != *FIGHTER_KINETIC_TYPE_MOTION_AIR {
            KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        };
        if motion_kind == hash40("special_hi_fall") {
            LANDING_HIT[entry_id] = true;
            if StatusModule::is_situation_changed(fighter.module_accessor) 
            && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
            }
            if stick_x >= 0.2 {
                macros::SET_SPEED_EX(fighter, 1.0, -6.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            else if stick_x <= -0.2 {
                macros::SET_SPEED_EX(fighter, -1.0, -6.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            else {
                macros::SET_SPEED_EX(fighter, 0.0, -6.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            if MotionModule::end_frame(module_accessor) - frame <= 2.0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true);
            }
        }
        if [hash40("special_lw"), hash40("special_air_lw")].contains(&motion_kind)
        && frame > 28.0 {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, true);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        lucina_frame
    );
}