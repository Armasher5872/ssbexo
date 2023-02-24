#![allow(
	unused_macros,
	unused_mut
)]
use {
    crate::{
        custom::momentumtransfer::momentum_install,
        functions::{
            ATTACK_100_ON,
            ATTACK_ENABLE_COMBO_ON,
            ATTACK_NO_HIT_COMBO_ON,
            CAN_JAB,
            CAN_RAPID_JAB,
            DAMAGED,
            DAMAGED_PREVENT,
            DID_GLIDE_TOSS,
            DID_MAX_JUMP_COUNT,
            FIGHTER_KIND,
            GLIDE_TOSS_ENABLE,
            OFFENSE_UP_ACTIVE,
            PAD_FLAG,
            SHIELD_SPECIAL,
            SITUATION_KIND,
            SPECIAL_SMASH_STATUS,
            STATUS_KIND,
            BomaExt,
            FrameInfo
        }
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lua2cpp::L2CFighterCommon,
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
        let status_kind = StatusModule::status_kind(module_accessor);
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let frame = MotionModule::frame(module_accessor);
        let stick_x = ControlModule::get_stick_x(module_accessor) * PostureModule::lr(module_accessor);
        let stick_y = ControlModule::get_stick_y(module_accessor);
        let lr = PostureModule::lr(module_accessor);
        let cbm_veca1 = Vector4f{/* Red */ x: 1.0, /* Green */ y: 1.0, /* Blue */ z: 1.0, /* Alpha */ w: 0.2};
        let cbm_veca2 = Vector4f{/* Red */ x: 0.0, /* Green */ y: 0.0, /* Blue */ z: 0.0, /* Alpha */w: 0.8};
        //Invincibility: let cbm_vecb1 = Vector4f{/* Red */ x: 0.0, /* Green */ y: 1.0, /* Blue */ z: 0.0, /* Alpha */ w: 0.2};
        let cbm_vecb2 = Vector4f{/* Red */ x: 0.0, /* Green */ y: 0.0, /* Blue */ z: 0.0, /* Alpha */w: 0.6};
        let cbm_vecc1 = Vector4f{/* Red */ x: 1.0, /* Green */ y: 0.0, /* Blue */ z: 0.0, /* Alpha */ w: 0.2}; //Super Armor/Heavy Armor
        let pad = fighter.global_table[PAD_FLAG].get_i32();
        //Platform Dropping
        if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE, *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP, *FIGHTER_STATUS_KIND_ITEM_THROW].contains(&status_kind) {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR
            && stick_y <= -0.6875 {
                if GroundModule::is_passable_ground(module_accessor) {
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
                }
            }
        }
        if fighter_kind == *FIGHTER_KIND_SNAKE {
            if motion_kind == hash40("attack_dash_item_light_throw") {
                if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR
                && stick_y <= -0.6875 {
                    if GroundModule::is_passable_ground(module_accessor) {
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
                    }
                }
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_LANDING
        && GroundModule::is_passable_ground(module_accessor)
        && fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
            if stick_y <= -0.6875
            && (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD_HOLD) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_CATCH)) != true
            && ((StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_ESCAPE_AIR) || (StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE)) {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_PASS, true);
            }
        }
        //Shield Dropping
        if status_kind == *FIGHTER_STATUS_KIND_GUARD
        && GroundModule::is_passable_ground(module_accessor) {
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
        //Movement Canceled Crouches
        if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE, *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE].contains(&status_kind)
        && stick_y <= -0.6875 {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SQUAT, true);
        }
        //Grounded Glide Tossing
        let mut motion_value = 2.8 * (MotionModule::end_frame(module_accessor) - frame) / MotionModule::end_frame(module_accessor);
        let mut motion_vec = Vector3f{x: 0.0, y: 0.0, z: 0.0};
        if [*FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&status_kind) {
            if frame <= 5.0 {
                GLIDE_TOSS_ENABLE[entry_id] = true;
            }
            else {
                GLIDE_TOSS_ENABLE[entry_id] = false;
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ITEM_THROW && GLIDE_TOSS_ENABLE[entry_id] == true {
            motion_vec.x = motion_value * lr;
            motion_vec.y = 0.0;
            motion_vec.z = 0.0;
            KineticModule::add_speed_outside(module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
        if status_kind != *FIGHTER_STATUS_KIND_ITEM_THROW || GLIDE_TOSS_ENABLE[entry_id] == false {
            motion_vec.x = 0.0;
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
        //DACSA
		let f5 = [*FIGHTER_KIND_FOX, *FIGHTER_KIND_SONIC, *FIGHTER_KIND_LUIGI, *FIGHTER_KIND_PFUSHIGISOU];
        let f6 = [*FIGHTER_KIND_PURIN, *FIGHTER_KIND_SHEIK, *FIGHTER_KIND_WARIO];
        let f7 = [*FIGHTER_KIND_DAISY, *FIGHTER_KIND_MARIOD, *FIGHTER_KIND_JACK, *FIGHTER_KIND_MARIO, *FIGHTER_KIND_MIIFIGHTER, *FIGHTER_KIND_GAMEWATCH, *FIGHTER_KIND_PALUTENA, *FIGHTER_KIND_PEACH, *FIGHTER_KIND_PICHU, *FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_ROSETTA, *FIGHTER_KIND_SNAKE, *FIGHTER_KIND_WIIFIT, *FIGHTER_KIND_ZELDA];
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
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH 
        && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_SHIELD) != true
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
            if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0
            || (stick_y > 0.7 && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
            } 
            else if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0
            || (stick_y > 0.7 && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
            };
        };
        //ROA Airdodges/Aerial Glide Tossing
        if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
            if frame <= 5.0
            && DID_GLIDE_TOSS[entry_id] == false {
                GLIDE_TOSS_ENABLE[entry_id] = true;
            }
            else {
                GLIDE_TOSS_ENABLE[entry_id] = false;
            }
            if GLIDE_TOSS_ENABLE[entry_id] == true {
                WorkModule::enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW) && pad & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0 && ItemModule::is_have_item(fighter.module_accessor, 0) {
                    fighter.clear_lua_stack();
                    lua_args!(fighter, *MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, *ITEM_TRAIT_FLAG_NO_THROW);
                    smash::app::sv_module_access::item(fighter.lua_state_agent);
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ITEM_THROW, false);
                    KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: 1.0, y: 1.0, z: 1.0}, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
                    DID_GLIDE_TOSS[entry_id] = true;
                }
            }
            if (18.0..=34.0).contains(&frame) {
                KineticModule::unable_energy_all(module_accessor);
                KineticModule::clear_speed_all(module_accessor);
            }
            if (35.0..=40.0).contains(&frame) {
                KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                fighter.sub_transition_group_check_air_cliff();
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
            }
            if frame >= 60.0 {
                CancelModule::enable_cancel(fighter.module_accessor);
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
                ColorBlendModule::set_main_color(module_accessor, /* Brightness */&cbm_veca1, /* Diffuse */&cbm_veca2, 0.21, 2.2, 5, /* Display Color */ true);
                macros::PLAY_SE(fighter, Hash40::new("se_ingame_reach"));
            }
        }
        if DID_MAX_JUMP_COUNT[entry_id] == false {
            if OFFENSE_UP_ACTIVE[entry_id] != true {
                ColorBlendModule::cancel_main_color(module_accessor, 0);
            }
        }
        //Heavy Armor/Super Armor/Invincibility Indicator
        if DamageModule::is_no_reaction_mode_perfect(module_accessor) == true {
            ColorBlendModule::set_main_color(module_accessor, /* Brightness */&cbm_vecc1, /* Diffuse */&cbm_vecb2, 0.21, 2.2, 5, /* Display Color */ true);
        }
        if DamageModule::is_no_reaction_mode_perfect(module_accessor) == false
        && DID_MAX_JUMP_COUNT[entry_id] != true {
            ColorBlendModule::cancel_main_color(module_accessor, 0);
        }
        //Jab Overriding Prevention
		if [*FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO].contains(&status_kind) && fighter_kind != *FIGHTER_KIND_MURABITO {
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
        //Installations
        if let Some(info) = FrameInfo::update_and_get(fighter) {
            if fighter_kind == *FIGHTER_KIND_ALL {
                momentum_install(fighter, &info);
            }
        } 
        else {
            panic!("Could not get the FrameInfo for this fighter! Is this even a fighter?")
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

//Parry Sound
#[common_status_script(status = FIGHTER_STATUS_KIND_GUARD_OFF, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS, symbol = "_ZN7lua2cpp16L2CFighterCommon42sub_ftStatusUniqProcessGuardOff_exitStatusEv")]
unsafe fn ft_status_uniq_process_guard_off_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if FighterUtil::is_valid_just_shield(fighter.module_accessor) {
        ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NONE), 0);
        let shield_type = FighterUtil::get_shield_type_of_guard(fighter.global_table[FIGHTER_KIND].get_i32()) as i32;
        ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(shield_type), *FIGHTER_SHIELD_KIND_GUARD, 0);
        ReflectorModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
        ShieldModule::set_hit_stop_mul(fighter.module_accessor, 1.0);
    }
    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_GUARD_DAMAGE && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_JUST_SHIELD) {
        effect!(fighter, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new_raw(0xafae75f05), true, true);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x262a7a102d));
    }
    let shield_type = FighterUtil::get_shield_type_of_guard(fighter.global_table[FIGHTER_KIND].get_i32()) as i32;
    ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(shield_type), *FIGHTER_SHIELD_KIND_GUARD, 0);
    if fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_SPECIAL_HI || fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_JUMP_SQUAT || fighter.global_table[STATUS_KIND] == FIGHTER_STATUS_KIND_ATTACK_HI4_START {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_GUARD_FRAME);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_DISABLE_ESCAPE_FRAME);
    }
    let just_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
    if 0 < just_frame {
        if fighter.global_table[FIGHTER_KIND] == *FIGHTER_KIND_CAPTAIN {
            macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_special_h03"));
            macros::PLAY_SE(fighter, Hash40::new("vc_captain_appeal03"));
        }
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
        if (just_frame - 1) == 0 {
            ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, ShieldStatus(*SHIELD_STATUS_NONE), 0);
            let type_of_guard = FighterUtil::get_shield_type_of_guard(fighter.global_table[0x2].get_i32()) as i32;
            ShieldModule::set_shield_type(fighter.module_accessor, ShieldType(type_of_guard), *FIGHTER_SHIELD_KIND_GUARD, 0);
            ReflectorModule::set_status(fighter.module_accessor, 0, ShieldStatus(*SHIELD_STATUS_NONE), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
        }
    }
    let cancel_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME);
    if 0 < cancel_frame {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_OFF_WORK_INT_CANCEL_FRAME);
        if (cancel_frame - 1) == 0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
    L2CValue::I32(0)
}

//Shield Specials
unsafe extern "C" fn if_shield_special(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        SHIELD_SPECIAL[entry_id] = true;
        if smash::app::utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_NESS {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_N.into(),true.into());
            return true.into();
        }
        if smash::app::utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_CAPTAIN
        || smash::app::utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_PICHU
        || smash::app::utility::get_kind(&mut *fighter.module_accessor) == *FIGHTER_KIND_MEWTWO {
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
    install_status_scripts!(ft_status_uniq_process_guard_off_exit_status);
    skyline::install_hooks!(change_kinetic_hook);
    skyline::install_hook!(is_valid_just_shield_reflector);
    skyline::install_hook!(is_valid_just_shield_replace);
}