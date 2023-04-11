#![allow(
	unused_macros,
	unused_mut
)]
use {
    crate::{
        functions::{
            ext::*,
            variables::*,
        }
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lua2cpp::{
            L2CFighterBase,
            L2CFighterCommon
        },
        lib::{
            L2CValue,
            lua_const::*,
        },
        phx::{
            Hash40,
            Vector3f,
            Vector4f
        }
    },
    smash_script::*,
    smashline::*,
};

#[fighter_frame_callback]
pub fn all_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let status_kind = StatusModule::status_kind(module_accessor);
        let frame = MotionModule::frame(module_accessor);
        let lr = PostureModule::lr(module_accessor);
        let stick_x = ControlModule::get_stick_x(module_accessor) * lr;
        let stick_y = ControlModule::get_stick_y(module_accessor);
        let cbm_vec1 = Vector4f{/* Red */ x: 1.0, /* Green */ y: 1.0, /* Blue */ z: 1.0, /* Alpha */ w: 0.2};
        let cbm_vec2 = Vector4f{/* Red */ x: 0.0, /* Green */ y: 0.0, /* Blue */ z: 0.0, /* Alpha */w: 0.8};
        let pad = fighter.global_table[PAD_FLAG].get_i32();
        println!("Last Attack Hitbox Id: {}", LAST_ATTACK_HITBOX_ID);
        //Platform Dropping/Movement Canceled Crouches
        if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE, *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE, *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP, *FIGHTER_STATUS_KIND_ITEM_THROW].contains(&status_kind) {
            if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE, *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE].contains(&status_kind) {
                if stick_y <= -0.6875 {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SQUAT, true);
                }
            }
            if GroundModule::is_passable_ground(module_accessor) {
                WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS);
            }
            if WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS) && (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT2_FLAG_GUARD_TO_PASS) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
            }
        }
        //Shield Platform Dropping
        if [*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD].contains(&status_kind) {
            if GroundModule::is_passable_ground(module_accessor) {
                if stick_y < -0.2
                && stick_y > -0.6875 {
                    WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS);
                    WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
                }
                else if stick_y > 0.2 
                || stick_y <= -0.6875 {
                    WorkModule::unable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS);
                    WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
                }
                if WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_PASS)
                && fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
                }
            }
        }
        //Lost Double Jump Indicator
        if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) >= WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
            DID_MAX_JUMP_COUNT[entry_id] = true;
        }
        if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
            DID_MAX_JUMP_COUNT[entry_id] = false;
        }
        if DID_MAX_JUMP_COUNT[entry_id] == true {
            if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI) || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_FLICK_JUMP) {
                ColorBlendModule::set_main_color(module_accessor, /* Brightness */&cbm_vec1, /* Diffuse */&cbm_vec2, 0.21, 2.2, 5, /* Display Color */ true);
            }
        }
        if DID_MAX_JUMP_COUNT[entry_id] == false {
            if OFFENSE_UP_ACTIVE[entry_id] != true {
                ColorBlendModule::cancel_main_color(module_accessor, 0);
            }
        }
        //DJC
        if [*FIGHTER_KIND_YOSHI, *FIGHTER_KIND_NESS, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_LUCAS, *FIGHTER_KIND_TRAIL].contains(&fighter_kind) {
            if [*FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL].contains(&KineticModule::get_kinetic_type(module_accessor)) {
                if ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_JUMP) && [*FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F, *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_AIR_LASSO].contains(&status_kind) {
                    KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
                };
                if KineticModule::get_kinetic_type(module_accessor) == *FIGHTER_KINETIC_TYPE_JUMP_AERIAL {
                    KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION);
                };
            };
        }
        //Grounded Glide Tossing
        let mut motion_value = 2.8 * (MotionModule::end_frame(module_accessor) - frame) / MotionModule::end_frame(module_accessor);
        let mut motion_vec = Vector3f{x: 0.0, y: 0.0, z: 0.0};
        if [*FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&status_kind) {
            if frame <= 5.0 {
                CAN_GLIDE_TOSS[entry_id] = true;
            }
            else {
                CAN_GLIDE_TOSS[entry_id] = false;
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ITEM_THROW && ![hash40("special_n"), hash40("special_air_n")].contains(&motion_kind) && CAN_GLIDE_TOSS[entry_id] == true {
            motion_vec.x = motion_value * lr;
            motion_vec.y = 0.0;
            motion_vec.z = 0.0;
            KineticModule::add_speed_outside(module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
        if status_kind != *FIGHTER_STATUS_KIND_ITEM_THROW || CAN_GLIDE_TOSS[entry_id] == false {
            motion_vec.x = 0.0;
        }
        //ROA Airdodges/Aerial Glide Tossing
        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind) {
            if frame <= 5.0 {
                CAN_GLIDE_TOSS[entry_id] = true;
            }
            else {
                CAN_GLIDE_TOSS[entry_id] = false;
            }
            if CAN_GLIDE_TOSS[entry_id] == true {
                WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW) && pad & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 && ItemModule::is_have_item(fighter.module_accessor, 0) {
                    fighter.clear_lua_stack();
                    lua_args!(fighter, *MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, *ITEM_TRAIT_FLAG_NO_THROW);
                    smash::app::sv_module_access::item(fighter.lua_state_agent);
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ITEM_THROW, false);
                    KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: 1.0, y: 1.0, z: 1.0}, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                if (18.0..=34.0).contains(&frame) {
                    KineticModule::unable_energy_all(module_accessor);
                    KineticModule::clear_speed_all(module_accessor);
                }
                if frame > 34.0 {
                    KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                    fighter.sub_transition_group_check_air_cliff();
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
                }
            }
            if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE {
                if (27.0..=43.0).contains(&frame) {
                    KineticModule::unable_energy_all(module_accessor);
                    KineticModule::clear_speed_all(module_accessor);
                }
                if frame > 43.0 {
                    KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                    fighter.sub_transition_group_check_air_cliff();
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
                }
            }
        }
        //Wavedash Platform Drops
        if status_kind == *FIGHTER_STATUS_KIND_LANDING
        && GroundModule::is_passable_ground(module_accessor)
        && fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
            if stick_y <= -0.6875
            && (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD_HOLD) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CATCH)) != true
            && ((StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_ESCAPE_AIR) || (StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE)) {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
            }
        }
        //Jab Overriding Prevention
		if [*FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO].contains(&status_kind) 
        && ![*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_DEMON].contains(&fighter_kind) {
			if ((stick_x <= 0.2 && stick_x >= -0.2) && (stick_y <= 0.2 && stick_y >= -0.2))
            && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) 
            && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_CATCH) 
            && ((ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) == 0 && (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) == 0 && (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) == 0 && (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) == 0 && (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) == 0 && (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) == 0 && (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH) == 0 && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_JUMP)) {
				CAN_JAB[entry_id] = 0;
				CAN_RAPID_JAB[entry_id] = 0;
				if ATTACK_100_ON[entry_id] {
					WorkModule::set_flag(module_accessor, true, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
				};
				if ATTACK_ENABLE_COMBO_ON[entry_id] {
					WorkModule::set_flag(module_accessor, true, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
					if fighter_kind == *FIGHTER_KIND_DEMON {
						WorkModule::set_flag(module_accessor, true, *FIGHTER_DEMON_STATUS_ATTACK_COMBO_FLAG_CHANGE_STATUS);
					};
				};
				if ATTACK_NO_HIT_COMBO_ON[entry_id] {
					WorkModule::set_flag(module_accessor, true, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
				};
			} 
            else {
				CAN_JAB[entry_id] = 1;
				CAN_RAPID_JAB[entry_id] = 1;
				if ATTACK_100_ON[entry_id] {
					WorkModule::set_flag(module_accessor, false, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
				};
				if ATTACK_ENABLE_COMBO_ON[entry_id] {
					WorkModule::set_flag(module_accessor, false, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
				};
				if ATTACK_NO_HIT_COMBO_ON[entry_id] {
					WorkModule::set_flag(module_accessor, false, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
				};
			};
		} 
        else {
			CAN_JAB[entry_id] = 0;
			CAN_RAPID_JAB[entry_id] = 0;
		};
        //DACSA
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
            let f5 = [*FIGHTER_KIND_FOX, *FIGHTER_KIND_SONIC, *FIGHTER_KIND_LUIGI];
            let f6 = [*FIGHTER_KIND_PURIN, *FIGHTER_KIND_SHEIK, *FIGHTER_KIND_WARIO];
            let f7 = [*FIGHTER_KIND_DAISY, *FIGHTER_KIND_MARIOD, *FIGHTER_KIND_JACK, *FIGHTER_KIND_MARIO, *FIGHTER_KIND_MIIFIGHTER, *FIGHTER_KIND_GAMEWATCH, *FIGHTER_KIND_PALUTENA, *FIGHTER_KIND_PEACH, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_PICHU, *FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_ROSETTA, *FIGHTER_KIND_SNAKE, *FIGHTER_KIND_WIIFIT, *FIGHTER_KIND_ZELDA];
            let f8 = [*FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_PITB, *FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_SHIZUE, *FIGHTER_KIND_KEN, *FIGHTER_KIND_KROOL, *FIGHTER_KIND_LITTLEMAC, *FIGHTER_KIND_LUCARIO, *FIGHTER_KIND_ROCKMAN, *FIGHTER_KIND_METAKNIGHT, *FIGHTER_KIND_TANTAN, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_PIT, *FIGHTER_KIND_ROBOT, *FIGHTER_KIND_RYU, *FIGHTER_KIND_TRAIL, *FIGHTER_KIND_TOONLINK, *FIGHTER_KIND_SZEROSUIT];
            let f9 = [*FIGHTER_KIND_KOOPAJR, *FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_DIDDY, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_GAOGAEN, *FIGHTER_KIND_INKLING, *FIGHTER_KIND_NESS, *FIGHTER_KIND_PIKMIN, *FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_REFLET, *FIGHTER_KIND_SAMUS, *FIGHTER_KIND_PICKEL, *FIGHTER_KIND_YOUNGLINK];
            let f10 = [*FIGHTER_KIND_BUDDY, *FIGHTER_KIND_MASTER, *FIGHTER_KIND_CLOUD, *FIGHTER_KIND_DONKEY, *FIGHTER_KIND_POPO, *FIGHTER_KIND_NANA, *FIGHTER_KIND_KIRBY, *FIGHTER_KIND_MIISWORDSMAN, *FIGHTER_KIND_ELIGHT, *FIGHTER_KIND_MURABITO];
            let f11 = [*FIGHTER_KIND_DUCKHUNT, *FIGHTER_KIND_GANON, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_MIIGUNNER, *FIGHTER_KIND_PACMAN, *FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_RICHTER, *FIGHTER_KIND_SIMON, *FIGHTER_KIND_DOLLY, *FIGHTER_KIND_YOSHI];
            let f12 = [*FIGHTER_KIND_KOOPA, *FIGHTER_KIND_WOLF];
            let f13 = [*FIGHTER_KIND_KAMUI, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_SHULK];
            let f14 = [*FIGHTER_KIND_CHROM, *FIGHTER_KIND_LUCAS, *FIGHTER_KIND_LUCINA, *FIGHTER_KIND_ROY];
            let f15 = [*FIGHTER_KIND_EDGE];
            let f16 = [*FIGHTER_KIND_BAYONETTA, *FIGHTER_KIND_IKE, *FIGHTER_KIND_DEMON];
            let f18 = [*FIGHTER_KIND_EFLAME];
            let f21 = [*FIGHTER_KIND_LINK];
            let f22 = [*FIGHTER_KIND_BRAVE];
            let f23 = [*FIGHTER_KIND_DEDEDE];
            if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD) != true
            && ((f5.contains(&fighter_kind) && frame <= 5.0) 
            || (f6.contains(&fighter_kind) && frame <= 6.0) 
            || (f7.contains(&fighter_kind) && frame <= 7.0) 
            || (f8.contains(&fighter_kind) && frame <= 8.0) 
            || (f9.contains(&fighter_kind) && frame <= 9.0) 
            || (f10.contains(&fighter_kind) && frame <= 10.0)
            || (f11.contains(&fighter_kind) && frame <= 11.0)
            || (f12.contains(&fighter_kind) && frame <= 12.0)
            || (f13.contains(&fighter_kind) && frame <= 13.0)
            || (f14.contains(&fighter_kind) && frame <= 14.0)
            || (f15.contains(&fighter_kind) && frame <= 15.0)
            || (f16.contains(&fighter_kind) && frame <= 16.0)
            || (f18.contains(&fighter_kind) && frame <= 18.0)
            || (f21.contains(&fighter_kind) && frame <= 21.0)
            || (f22.contains(&fighter_kind) && frame <= 22.0)
            || (f23.contains(&fighter_kind) && frame <= 23.0)) {
                if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 || (stick_y > 0.7 && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
                } 
                else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 || (stick_y < -0.7 && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
                };
            };
        }
        //Damage
        if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL].contains(&status_kind)
        && DAMAGED_PREVENT[entry_id] != false {
            DAMAGED[entry_id] = true;
        }
        if ![*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL].contains(&status_kind) {
            DAMAGED[entry_id] = false;
            DAMAGED_PREVENT[entry_id] = false;
        }
        //ASDI
        if StopModule::is_damage(module_accessor) {
            ASDI_START[entry_id] = true;
        };
        if ASDI_START[entry_id] && !StopModule::is_damage(module_accessor) {
            let asdi_stick_x = if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {ControlModule::get_sub_stick_x(module_accessor)} else {stick_x}; // get stick x length, uses cstick's value if cstick is on (for Double Stick DI)
            let asdi_stick_y = if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {ControlModule::get_sub_stick_y(module_accessor)} else {stick_y}; // get stick y length, uses cstick's value if cstick is on (for Double Stick DI)
            let asdi = WorkModule::get_param_float(module_accessor, hash40("common"), hash40("hit_stop_delay_auto_mul")); // get base asdi distance
            let asdi_x = asdi * asdi_stick_x; // mul asdi stick_x by total asdi
            let asdi_y = asdi * asdi_stick_y; // mul asdi stick_y by total asdi
            let mut pos = Vector3f {x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor)}; // get current pos
            pos.x += asdi_x; //add asdi_x to pos_x
            pos.y += asdi_y; //add asdi_y to pos_y
            PostureModule::set_pos(module_accessor, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
            ASDI_START[entry_id] = false;
        };
        //Mashing
        if [*FIGHTER_STATUS_KIND_CAPTURE_BEETLE, *FIGHTER_STATUS_KIND_CAPTURE_BEITCRANE, *FIGHTER_STATUS_KIND_CAPTURE_BOSSGALAGA, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_MIMIKKYU, *FIGHTER_STATUS_KIND_CAPTURE_NABBIT, *FIGHTER_STATUS_KIND_CAPTURE_WAIT, *FIGHTER_STATUS_KIND_CAPTURE_WAIT_OCTOPUS, *FIGHTER_STATUS_KIND_CAPTURE_WAIT_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *FIGHTER_STATUS_KIND_BURY, *FIGHTER_STATUS_KIND_BURY_WAIT, *FIGHTER_STATUS_KIND_FURAFURA_STAND, *FIGHTER_STATUS_KIND_FURAFURA, *FIGHTER_STATUS_KIND_ICE, *FIGHTER_STATUS_KIND_SLEEP_START, *FIGHTER_STATUS_KIND_SLEEP, *FIGHTER_STATUS_KIND_SLEEP_FALL, *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_START, *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY, *FIGHTER_STATUS_KIND_YOSHI_EGG, *FIGHTER_STATUS_KIND_SWALLOWED_CAPTURE, *FIGHTER_STATUS_KIND_SWALLOWED_THROWN_STAR, *FIGHTER_STATUS_KIND_BITTEN_WARIO_START, *FIGHTER_STATUS_KIND_BITTEN_WARIO, *FIGHTER_STATUS_KIND_CLUNG_DAMAGE_DIDDY, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY].contains(&status_kind) {
            MASHING[entry_id] += 1;
            if MASHING[entry_id] >= 5 && (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)) {
                ControlModule::add_clatter_time(module_accessor, -15.0, 0);
                MASHING[entry_id] = 0;
            }
        }
        //Bury Knockback Resistance
        if [*FIGHTER_STATUS_KIND_BURY, *FIGHTER_STATUS_KIND_BURY_WAIT].contains(&status_kind) {
            DamageModule::set_reaction_mul(module_accessor, 0.77);
        }
        if status_kind == *FIGHTER_STATUS_KIND_BURY_JUMP {
            DamageModule::set_reaction_mul(module_accessor, 1.0);
        }
        //Shield Breaks
        if status_kind == *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY
        && fighter_kind != *FIGHTER_KIND_PURIN {
            GroundModule::set_attach_ground(module_accessor, true);
            StatusModule::set_situation_kind(module_accessor, SituationKind(*SITUATION_KIND_GROUND), true);
            GroundModule::correct(module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion(module_accessor, Hash40::new("furafura_start_u"), 0.0, 1.0, false, 0.0, false, false);
            if SPECIAL_ZOOM_GFX[entry_id] < 4 {
                SPECIAL_ZOOM_GFX[entry_id] += 1;
            }
            if SPECIAL_ZOOM_GFX[entry_id] < 2 {
                SlowModule::set_whole(module_accessor, 8, 80);
                macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
                EffectModule::req_follow(module_accessor, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
                macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_XL);
            }
            if SPECIAL_ZOOM_GFX[entry_id] >= 3 {
                SlowModule::clear_whole(module_accessor);
                CameraModule::reset_all(module_accessor);
                EffectModule::kill_kind(module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
                macros::CAM_ZOOM_OUT(fighter);
            }
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FURAFURA_STAND, true);
        }
        if status_kind == *FIGHTER_STATUS_KIND_FURAFURA_STAND {
            SlowModule::clear_whole(module_accessor);
            CameraModule::reset_all(module_accessor);
            EffectModule::kill_kind(module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
            macros::CAM_ZOOM_OUT(fighter);
            SPECIAL_ZOOM_GFX[entry_id] = 0;
        }
        if status_kind == *FIGHTER_STATUS_KIND_FURAFURA {
            SHIELD_BREAK_TIMER[entry_id] += 1;
            if SHIELD_BREAK_TIMER[entry_id] >= 120 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SAVING_DAMAGE, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SAVING_DAMAGE {
            if StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_FURAFURA {
                SHIELD_BREAK_TIMER[entry_id] = 0;
                MotionModule::set_rate(fighter.module_accessor, 0.4286);
            }
        }
    };
}

//Special Smash
#[skyline::hook(replace=smash::app::FighterUtil::is_valid_just_shield)]
unsafe fn is_valid_just_shield_replace(module_accessor: &mut BattleObjectModuleAccessor) -> bool {
	if SPECIAL_SMASH_STATUS == 2 {
		return false;
	}
	else {
		original!()(module_accessor)
	}
}

//Parry Reflects
#[skyline::hook(replace=smash::app::FighterUtil::is_valid_just_shield_reflector)]
unsafe fn is_valid_just_shield_reflector(_module_accessor: &mut BattleObjectModuleAccessor) -> bool {
	return true;
}

//Shield Specials
unsafe extern "C" fn if_shield_special(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        SHIELD_SPECIAL[entry_id] = true;
        if kind == *FIGHTER_KIND_NESS {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_N.into(),true.into());
            return true.into();
        }
        if [*FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_PICHU, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_PZENIGAME].contains(&kind) {
            fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(),true.into());
            return true.into();
        }
    }
    return false.into();
}

//Momentum Transfer Kinetic Hook
#[skyline::hook(replace=KineticModule::change_kinetic)]
unsafe fn change_kinetic_hook(boma: &mut BattleObjectModuleAccessor, kinetic_type: i32) -> i32 {
    let mut kinetic_type_new = kinetic_type;
        if boma.is_fighter() {
        /*   --------------  SPECIAL MOMENTUM  -----------------  */
        match crate::custom::momentumtransfer::change_kinetic_momentum_related(boma, kinetic_type_new) {
            Some(x) => kinetic_type_new = x,
            None => ()
        }
        /*   --------------------------------------------------------  */    
    }
    original!()(boma, kinetic_type_new)
}

//Installation of Shield Specials
#[smashline::fighter_init]
fn character_init(fighter: &mut L2CFighterCommon) {
    fighter.global_table[0x34].assign(&L2CValue::Ptr(if_shield_special as *const () as _));
}

pub fn install() {
    install_agent_frame_callbacks!(all_frame);
    install_agent_init_callbacks!(character_init);
    skyline::install_hooks!(change_kinetic_hook);
    skyline::install_hook!(is_valid_just_shield_reflector);
    skyline::install_hook!(is_valid_just_shield_replace);
}
