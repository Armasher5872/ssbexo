#![allow(unused_macros)]
use {
    crate::functions::{
        BOOST_INSTALL_ACTIVE,
        BOOST_INSTALL_GFX_COUNTER,
        BOOST_INSTALL_MOTION_RATE,
        BOOST_INSTALL_TIMER,
        CAN_ADD,
        CMD_CAT1,
        DASH_GRAB_SPEED,
        FALCON_PUNCH_HIT,
        FALCON_PUNCH_TURN_COUNT,
        HITFLOW,
        HYPE_HIT,
        KIRBY_FALCON_PUNCH_TURN_COUNT,
        PARRIED,
        PARRY_TIMER,
        SHIELD_SPECIAL,
        SITUATION_KIND
    },
    smash::{
        app::lua_bind::*,
        hash40,
        lua2cpp::L2CFighterCommon,
        lib::{
            lua_const::*,
            L2CValueType
        },
        phx::Hash40,
    },
    smashline::*,
    smash_script::*,
};

#[fighter_frame( agent = FIGHTER_KIND_CAPTAIN )]
fn captain_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let status_kind = StatusModule::status_kind(module_accessor);
        let frame = MotionModule::frame(module_accessor);
        let stick_y = ControlModule::get_stick_y(module_accessor);
        let mut globals = fighter.globals_mut().clone();
        //Parry Voice
        if [hash40("just_shield_off"), hash40("just_shield")].contains(&motion_kind) {
            if (0.0..5.0).contains(&frame) {
                PARRIED[entry_id] = 1;
                PARRY_TIMER[entry_id] = 60;
            }
        }
        if PARRY_TIMER[entry_id] > 0 {
            PARRY_TIMER[entry_id] -= 1;
        }
        if PARRY_TIMER[entry_id] == 30 {
            macros::PLAY_SE(fighter, Hash40::new("vc_captain_appeal02"));
        }
        if PARRY_TIMER[entry_id] <= 0
        && PARRIED[entry_id] == 1 {
            PARRIED[entry_id] = 0;
        }
        //Jab Cancels
        if [hash40("attack_11"), hash40("attack_12")].contains(&motion_kind)
        && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
            if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
            } 
            else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
            } 
            else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
            };
        };
        //Fair
        if motion_kind == hash40("attack_air_f")
        && (14.0..15.0).contains(&frame) {
            if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == true {
                macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_special_h03"));
                macros::PLAY_SE(fighter, Hash40::new("vc_captain_appeal03"));
            }
            else {
                macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_rnd_attack"));
                macros::PLAY_SE(fighter, Hash40::new("se_captain_swing_l"));
            };
            if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == true
            && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD) != true {
                HYPE_HIT[entry_id] = true;
            };
        };
        //Dash Grab
        if status_kind == *FIGHTER_STATUS_KIND_DASH {
            DASH_GRAB_SPEED[entry_id] = 2.0;
        }
        if status_kind == *FIGHTER_STATUS_KIND_RUN {
            DASH_GRAB_SPEED[entry_id] = 2.35;
        }
        if motion_kind == hash40("catch_pull")
        && status_kind == *FIGHTER_STATUS_KIND_CATCH_DASH_PULL {
            macros::SET_SPEED_EX(fighter, DASH_GRAB_SPEED[entry_id]/1.5, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        //Shield Special
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL
        && SHIELD_SPECIAL[entry_id] == true {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_shield"), 1.0, 1.0, false, 0.0, false, false);
        }
        if motion_kind == hash40("special_shield") {
            SHIELD_SPECIAL[entry_id] = false;
            if HYPE_HIT[entry_id] == true {
                BOOST_INSTALL_ACTIVE[entry_id] = true;
                BOOST_INSTALL_TIMER[entry_id] = 650;
            }
        }
        //Boost Install
        if BOOST_INSTALL_ACTIVE[entry_id] == true {
            HYPE_HIT[entry_id] = false;
            if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL].contains(&status_kind) {
                BOOST_INSTALL_TIMER[entry_id] -= 120;
            }
            if BOOST_INSTALL_TIMER[entry_id] > 0 {
                BOOST_INSTALL_TIMER[entry_id] -= 1;
            }
            if BOOST_INSTALL_TIMER[entry_id] <= 0 {
                BOOST_INSTALL_ACTIVE[entry_id] = false;
            }
            //GFX
            BOOST_INSTALL_GFX_COUNTER[entry_id] += 1;
            if BOOST_INSTALL_GFX_COUNTER[entry_id] >= 8 {
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 0.0);
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 0.0);
                BOOST_INSTALL_GFX_COUNTER[entry_id] = 0;
            }
            //Damage Increase
            DamageModule::set_damage_mul(module_accessor, 1.25);
            DamageModule::set_reaction_mul(module_accessor, 1.25);
            //Snowballing Hitflow
            if frame < 2.0 { // resets at the start of a move the inability to add further motion rate
                CAN_ADD[entry_id] = true;
            };
            if CAN_ADD[entry_id] == true 
            && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
                CAN_ADD[entry_id] = false;
                BOOST_INSTALL_MOTION_RATE[entry_id] += 0.05;
            };
            if BOOST_INSTALL_MOTION_RATE[entry_id] > 1.3 {
                BOOST_INSTALL_MOTION_RATE[entry_id] = 1.3;
            }
            if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)
            && !AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                HITFLOW[entry_id] = true;
            };
            if HITFLOW[entry_id] == true {
                MotionModule::set_rate(fighter.module_accessor, BOOST_INSTALL_MOTION_RATE[entry_id]);
            }
            if MotionModule::end_frame(module_accessor) - frame <= 2.0
            || CancelModule::is_enable_cancel(module_accessor) {
                HITFLOW[entry_id] = false;
                MotionModule::set_rate(module_accessor, 1.0);
            };
            //Hitfalling Aerials
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR
            && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT)
            && (ControlModule::get_command_flag_cat(module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP) != 0
            && stick_y < -0.66
            && KineticModule::get_sum_speed_y(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            };
            //Change Up Tilt
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_S_END, true);
            }
            //Aerial Down Special Return Double Jump
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
            && [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_WALL_END].contains(&status_kind)
            && WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) == WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                WorkModule::dec_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            }
        }
        //Reset Boost Install
        if status_kind == *FIGHTER_STATUS_KIND_DEAD {
            BOOST_INSTALL_ACTIVE[entry_id] = false;
        }
        if BOOST_INSTALL_ACTIVE[entry_id] == false {
            BOOST_INSTALL_TIMER[entry_id] = 0;
            BOOST_INSTALL_MOTION_RATE[entry_id] = 1.0;
            DamageModule::set_damage_mul(module_accessor, 1.0);
            DamageModule::set_reaction_mul(module_accessor, 1.0);
        }
        //Neutral Special
        if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind)
        && (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0
        && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == true {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
        };
        if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN].contains(&status_kind) {
            if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == true
            && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD) != true {
                HYPE_HIT[entry_id] = true;
                FALCON_PUNCH_HIT[entry_id] = true;
            };
            if FALCON_PUNCH_HIT[entry_id] == true
            && (54.0..57.0).contains(&frame) {
                macros::PLAY_SE(fighter, Hash40::new("vc_captain_cheer"));
            }
        };
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N
        && FALCON_PUNCH_HIT[entry_id] == true
        && frame > 70.0 {
            CancelModule::enable_cancel(module_accessor);
        }
        if status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN
        && FALCON_PUNCH_HIT[entry_id] == true
        && frame > 104.0 {
            CancelModule::enable_cancel(module_accessor);
        }
        if status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN 
        && (frame > 25.0 && frame < 40.0) 
        && (ControlModule::get_stick_x(module_accessor)*PostureModule::lr(module_accessor)) < -0.5
        && FALCON_PUNCH_TURN_COUNT[entry_id] <= 15.0 {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN, true);
        };
        if status_kind != *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN {
            FALCON_PUNCH_TURN_COUNT[entry_id] = 0.0;
        };
        if FALCON_PUNCH_TURN_COUNT[entry_id] == 0.0 {
            AttackModule::set_power_up(module_accessor, 1.0);
        };
        if ![*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_N_TURN].contains(&status_kind) {
            FALCON_PUNCH_HIT[entry_id] = false;
        }
        //Side Special
        if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_S_END].contains(&status_kind) {
            fighter.sub_transition_group_check_air_cliff();
        };
        //Down Special
        if let L2CValueType::Void = globals["reverse"].val_type {
            globals["reverse"] = false.into();
        }
        if motion_kind == hash40("special_air_lw") {
            let cat = fighter.global_table[CMD_CAT1].get_int() as i32;
            if ((cat & *FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_LEFT) != 0 && GroundModule::get_touch_flag(module_accessor) == *GROUND_TOUCH_FLAG_LEFT as u64) || ((cat & *FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_RIGHT) != 0 && GroundModule::get_touch_flag(module_accessor) == *GROUND_TOUCH_FLAG_RIGHT as u64) {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WALL_JUMP, true);
            }
            if frame < 6.0 {
                if ControlModule::get_stick_x(module_accessor) * PostureModule::lr(module_accessor) <= -0.66 {
                    globals["reverse"] = true.into();
                }
            }
            else if (1.0..6.0).contains(&frame)
            && globals["reverse"].get_bool() {
                PostureModule::reverse_lr(module_accessor);
                PostureModule::update_rot_y_lr(module_accessor);
                KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            }	
        }
        else {
            globals["reverse"] = false.into();
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn kirby_captain_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(module_accessor);
        let frame = MotionModule::frame(module_accessor);
        if (WorkModule::get_int(module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_CAPTAIN)
        && [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind)
        && (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0
        && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == true {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N, true);
        };
        if [*FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN].contains(&status_kind) {
            if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == true {
                FALCON_PUNCH_HIT[entry_id] = true;
            };
            if FALCON_PUNCH_HIT[entry_id] == true
            && (54.0..57.0).contains(&frame) {
                macros::PLAY_SE(fighter, Hash40::new("vc_kirby_cheer"));
            }
        };
        if status_kind == *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N
        && FALCON_PUNCH_HIT[entry_id] == true
        && frame > 70.0 {
            CancelModule::enable_cancel(module_accessor);
        }
        if status_kind == *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN
        && FALCON_PUNCH_HIT[entry_id] == true
        && frame > 104.0 {
            CancelModule::enable_cancel(module_accessor);
        }
        if ![*FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN].contains(&status_kind) {
            FALCON_PUNCH_HIT[entry_id] = false;
        }
        if status_kind == *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN 
        && (frame > 25.0 && frame < 40.0) 
        && (ControlModule::get_stick_x(module_accessor)*PostureModule::lr(module_accessor)) < -0.5
        && KIRBY_FALCON_PUNCH_TURN_COUNT[entry_id] <= 15.0 {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN, true);
        };
        if status_kind != *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN {
            KIRBY_FALCON_PUNCH_TURN_COUNT[entry_id] = 0.0;
        }
        if KIRBY_FALCON_PUNCH_TURN_COUNT[entry_id] == 0.0 {
            AttackModule::set_power_up(module_accessor, 1.0);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        captain_frame,
        kirby_captain_frame
    );
}