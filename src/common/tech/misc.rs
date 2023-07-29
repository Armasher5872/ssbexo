use super::*;

//This opff really only exists to deal with status kinds I couldn't translate, or have far too many status kinds to account for
#[fighter_frame_callback]
pub fn all_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let fighter_kind = smash::app::utility::get_kind(boma);
        let status_kind = StatusModule::status_kind(boma);
        let lr = PostureModule::lr(boma);
        let stick_x = ControlModule::get_stick_x(boma) * lr;
        let stick_y = ControlModule::get_stick_y(boma);
        let frame = MotionModule::frame(boma);
        let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma, Hash40::new_raw(MotionModule::motion_kind(boma)), false) as f32; //Cancel frame
        let cbm_vec1 = Vector4f{/* Red */ x: 1.0, /* Green */ y: 1.0, /* Blue */ z: 1.0, /* Alpha */ w: 0.2};
        let cbm_vec2 = Vector4f{/* Red */ x: 0.0, /* Green */ y: 0.0, /* Blue */ z: 0.0, /* Alpha */w: 0.8};
        let mut pos = Vector3f {x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)}; // get current pos
        let mashing = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
        let special_zoom_gfx = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
        //Jostle
        if ![
            *FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_TURN, *FIGHTER_RYU_STATUS_KIND_TURN_AUTO, *FIGHTER_DOLLY_STATUS_KIND_TURN_AUTO, *FIGHTER_DEMON_STATUS_KIND_TURN_AUTO, *FIGHTER_STATUS_KIND_WALK, *FIGHTER_STATUS_KIND_WALK_BRAKE, 
            *FIGHTER_RYU_STATUS_KIND_WALK_BACK, *FIGHTER_RYU_STATUS_KIND_WALK_BRAKE_BACK, *FIGHTER_DOLLY_STATUS_KIND_WALK_BACK, *FIGHTER_DOLLY_STATUS_KIND_WALK_BRAKE_BACK, *FIGHTER_DEMON_STATUS_KIND_WALK_BACK, *FIGHTER_DEMON_STATUS_KIND_WALK_BRAKE_BACK, 
            *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_RYU_STATUS_KIND_DASH_BACK, *FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, *FIGHTER_DEMON_STATUS_KIND_DASH_BACK, *FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_RUN_BRAKE, *FIGHTER_STATUS_KIND_TURN_RUN, 
            *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE, *FIGHTER_RYU_STATUS_KIND_TURN_RUN_BACK, *FIGHTER_DOLLY_STATUS_KIND_TURN_RUN_BACK, *FIGHTER_DEMON_STATUS_KIND_TURN_RUN_BACK, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP, *FIGHTER_STATUS_KIND_SQUAT, 
            *FIGHTER_STATUS_KIND_SQUAT_WAIT, *FIGHTER_STATUS_KIND_SQUAT_RV, *FIGHTER_STATUS_KIND_SQUAT_F, *FIGHTER_STATUS_KIND_SQUAT_B, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_S4_START,  *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, 
            *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4, 
            *FIGHTER_STATUS_KIND_CATCH_DASH
        ].contains(&status_kind) {
            JostleModule::set_status(boma, false);
        }
        if status_kind == *FIGHTER_STATUS_KIND_DASH {
            if ![*FIGHTER_KIND_SHEIK, *FIGHTER_KIND_GEKKOUGA].contains(&fighter_kind) {
                JostleModule::set_status(boma, true);
            }
            else {
                JostleModule::set_status(boma, false);
            }
        }
        //Lost Double Jump Indicator
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) >= WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
            WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
        }
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
            WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
        }
        if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT) {
            ColorBlendModule::set_main_color(boma, /* Brightness */&cbm_vec1, /* Diffuse */&cbm_vec2, 0.21, 2.2, 5, /* Display Color */ true);
        }
        if !WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT) && ![*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_OFF].contains(&status_kind) {
            if OFFENSE_UP_ACTIVE[entry_id] != true {
                ColorBlendModule::cancel_main_color(boma, 0);
            }
        }
        //Zair Platform Dropping
        if status_kind == *FIGHTER_STATUS_KIND_AIR_LASSO {
            if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_PASS {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) != true {
                    GroundModule::set_passable_check(boma, true);
                }
                if fighter.global_table[CURRENT_FRAME].get_f32() <= 1.0 {
                    pos.y += 4.5;
                    PostureModule::set_pos(boma, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
                }
            }
        }
        //Damage
        if [*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL].contains(&status_kind)
        && !WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT) {
            WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
        }
        if ![*FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_SONG, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL].contains(&status_kind) {
            WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
            WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
        }
        //ASDI
        if StopModule::is_damage(boma) {
            WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
        };
        if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START)  && !StopModule::is_damage(boma) {
            let asdi_stick_x = if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {ControlModule::get_sub_stick_x(boma)} else {stick_x}; // get stick x length, uses cstick's value if cstick is on (for Double Stick DI)
            let asdi_stick_y = if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {ControlModule::get_sub_stick_y(boma)} else {stick_y}; // get stick y length, uses cstick's value if cstick is on (for Double Stick DI)
            let asdi = WorkModule::get_param_float(boma, hash40("common"), hash40("hit_stop_delay_auto_mul")); // get base asdi distance
            let asdi_x = asdi * asdi_stick_x*lr; // mul asdi stick_x by total asdi
            let asdi_y = asdi * asdi_stick_y; // mul asdi stick_y by total asdi
            pos.x += asdi_x; //add asdi_x to pos_x
            pos.y += asdi_y; //add asdi_y to pos_y
            PostureModule::set_pos(boma, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
            WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
        };
        //Mashing
        if [*FIGHTER_STATUS_KIND_CAPTURE_BEETLE, *FIGHTER_STATUS_KIND_CAPTURE_BEITCRANE, *FIGHTER_STATUS_KIND_CAPTURE_BOSSGALAGA, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE, *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_MIMIKKYU, *FIGHTER_STATUS_KIND_CAPTURE_NABBIT, *FIGHTER_STATUS_KIND_CAPTURE_WAIT, *FIGHTER_STATUS_KIND_CAPTURE_WAIT_OCTOPUS, *FIGHTER_STATUS_KIND_CAPTURE_WAIT_YOSHI, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *FIGHTER_STATUS_KIND_BURY, *FIGHTER_STATUS_KIND_BURY_WAIT, *FIGHTER_STATUS_KIND_FURAFURA_STAND, *FIGHTER_STATUS_KIND_ICE, *FIGHTER_STATUS_KIND_SLEEP_START, *FIGHTER_STATUS_KIND_SLEEP, *FIGHTER_STATUS_KIND_SLEEP_FALL, *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_START, *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY, *FIGHTER_STATUS_KIND_YOSHI_EGG, *FIGHTER_STATUS_KIND_SWALLOWED_CAPTURE, *FIGHTER_STATUS_KIND_SWALLOWED_THROWN_STAR, *FIGHTER_STATUS_KIND_BITTEN_WARIO_START, *FIGHTER_STATUS_KIND_BITTEN_WARIO, *FIGHTER_STATUS_KIND_CLUNG_DAMAGE_DIDDY, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY].contains(&status_kind) {
            WorkModule::inc_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
            if mashing >= 5 && (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)) {
                ControlModule::add_clatter_time(boma, -15.0, 0);
                WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
            }
        }
        //Bury Knockback Resistance
        if [*FIGHTER_STATUS_KIND_BURY, *FIGHTER_STATUS_KIND_BURY_WAIT].contains(&status_kind) {
            DamageModule::set_reaction_mul(boma, 0.77);
        }
        if status_kind == *FIGHTER_STATUS_KIND_BURY_JUMP || (WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) && (fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_BURY || fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_BURY_WAIT)) {
            DamageModule::set_reaction_mul(boma, 1.0);
        }
        //Shield Breaks
        if status_kind == *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY
        && fighter_kind != *FIGHTER_KIND_PURIN {
            GroundModule::set_attach_ground(boma, true);
            StatusModule::set_situation_kind(boma, SituationKind(*SITUATION_KIND_GROUND), true);
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion(boma, Hash40::new("furafura_start_u"), 0.0, 1.0, false, 0.0, false, false);
            if special_zoom_gfx < 4 {
                WorkModule::inc_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
            }
            if special_zoom_gfx < 2 {
                SlowModule::set_whole(boma, 8, 80);
                macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
                EffectModule::req_follow(boma, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
                macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_XL);
            }
            if special_zoom_gfx >= 3 {
                SlowModule::clear_whole(boma);
                CameraModule::reset_all(boma);
                EffectModule::kill_kind(boma, Hash40::new("sys_bg_criticalhit"), false, false);
                macros::CAM_ZOOM_OUT(fighter);
            }
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FURAFURA_STAND, true);
        }
        //DJC (For Lucas cause they don't wanna work)
        let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        if fighter_kind == *FIGHTER_KIND_LUCAS {
            if [*FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL].contains(&KineticModule::get_kinetic_type(boma)) {
                if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) && [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_AIR_LASSO].contains(&status_kind) {
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
                    macros::SET_SPEED_EX(fighter, speed_x*lr, -0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                };
                if KineticModule::get_kinetic_type(boma) == *FIGHTER_KINETIC_TYPE_JUMP_AERIAL {
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION);
                };
            };
        }
        //Sword Scaling
        if fighter_kind == *FIGHTER_KIND_MARTH {
            let long_sword_scale = Vector3f{x: 1.0, y: 1.2, z: 1.0};
            ModelModule::set_joint_scale(boma, Hash40::new("havel"), &long_sword_scale);
            ModelModule::set_joint_scale(boma, Hash40::new("haver"), &long_sword_scale);
        }
        if fighter_kind == *FIGHTER_KIND_LUCINA {
            let long_sword_scale = Vector3f{x: 1.015, y: 1.115, z: 1.045};
            ModelModule::set_joint_scale(boma, Hash40::new("havel"), &long_sword_scale);
            ModelModule::set_joint_scale(boma, Hash40::new("haver"), &long_sword_scale);
        }
        if fighter_kind == *FIGHTER_KIND_CHROM {
            let long_sword_scale = Vector3f{x: 1.5, y: 1.5, z: 1.25};
            ModelModule::set_joint_scale(boma, Hash40::new("havel"), &long_sword_scale);
            ModelModule::set_joint_scale(boma, Hash40::new("haver"), &long_sword_scale);
        }
        //Guilty Gear Strive COUNTER!
        if [
            *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, 
            *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_DASH, 
            *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F, *FIGHTER_RYU_STATUS_KIND_ATTACK_NEAR, 
            *FIGHTER_SIMON_STATUS_KIND_ATTACK_HOLD_START, *FIGHTER_SIMON_STATUS_KIND_ATTACK_HOLD, *FIGHTER_SIMON_STATUS_KIND_ATTACK_LW32, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_FALL_AERIAL, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP, 
            *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WAIT, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_LANDING, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_WALK_BACK, *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1, *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2, 
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_COMBO, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WAIT, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_RV, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LANDING, 
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LADDER, *FIGHTER_METAKNIGHT_STATUS_KIND_ATTACK_S3, *FIGHTER_METAKNIGHT_STATUS_KIND_ATTACK_LW3
        ].contains(&status_kind) {
            if estimate_frame(boma, 0.0) {
                COUNTERHIT_CHECK[get_player_number(boma)] = true;
            }
            if AttackModule::is_attack(boma, 0, false) {
                COUNTERHIT_CHECK[get_player_number(boma)] = false;
            }
        }
        else {
            COUNTERHIT_SUCCESS[get_player_number(boma)] = false;
        }
        if COUNTERHIT_SUCCESS[get_player_number(boma)] {
            if special_zoom_gfx < 10 {
                WorkModule::inc_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
            }
            if special_zoom_gfx < 1 {
                let counter_sound = SoundModule::play_se(boma, Hash40::new("se_common_counter"), true, false, false, false, app::enSEType(0));
                SoundModule::set_se_pitch_ratio(boma, Hash40::new("se_common_counter"), 1.17);
                SoundModule::set_se_vol(boma, counter_sound as i32, 2.75, 0);
                SlowModule::set_whole(boma, 4, 40);
                macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 1.0,/*no*/ 0.0,/*zoom*/ 1.5,/*yrot*/ 0.0,/*xrot*/ 0.0);
            }
            if special_zoom_gfx >= 10 {
                SlowModule::clear_whole(boma);
                CameraModule::reset_all(boma);
                macros::CAM_ZOOM_OUT(fighter);
                COUNTERHIT_SUCCESS[get_player_number(boma)] = false;
            }
        }
        //Training Mode
        let faf_status = [
            *FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_DASH, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, 
            *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_CATCH, *FIGHTER_STATUS_KIND_CATCH_DASH, *FIGHTER_STATUS_KIND_CATCH_TURN, *FIGHTER_STATUS_KIND_THROW, *FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, 
            *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL
        ].contains(&status_kind);
        if !smash::app::smashball::is_training_mode() {
            FEATURES = false;
        } 
        else {
            if [*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_OFF].contains(&status_kind)
            && ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                if FEATURES {
                    FEATURES = false;
                } 
                else {
                    FEATURES = true;
                };
            };
        };
        if FEATURES {
            macros::COL_NORMAL(fighter);
            if WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) > 0.0 && (*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind) {
                //Glows Red during Hitstun
                macros::FLASH(fighter, 2.55, 0.0, 0.0, 0.7);
            }
            else if ((cancel_frame > 0.0 && frame >= cancel_frame) || (CancelModule::is_enable_cancel(boma))) && !(*FIGHTER_STATUS_KIND_DAMAGE..*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind) && faf_status {
                //Glows Green during Cancel Frames
                macros::FLASH(fighter, 0.0, 2.55, 0.0, 0.7);
            }
            else if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
            && (fighter.dacsa_check() == 1 || fighter.dacsa_check() == 2) {
                //Glows Yellow during DACUS/DACDS Frames
                macros::FLASH(fighter, 2.55, 2.55, 0.0, 0.7);
            }
        }
        //Held Buffer
        let control_pad = [
            *CONTROL_PAD_BUTTON_APPEAL_HI, *CONTROL_PAD_BUTTON_APPEAL_LW, *CONTROL_PAD_BUTTON_APPEAL_S_L, *CONTROL_PAD_BUTTON_APPEAL_S_R, *CONTROL_PAD_BUTTON_ATTACK, *CONTROL_PAD_BUTTON_ATTACK_RAW, *CONTROL_PAD_BUTTON_CATCH, *CONTROL_PAD_BUTTON_CSTICK_ON,
            *CONTROL_PAD_BUTTON_FLICK_JUMP, *CONTROL_PAD_BUTTON_GUARD, *CONTROL_PAD_BUTTON_GUARD_HOLD, *CONTROL_PAD_BUTTON_INVALID, *CONTROL_PAD_BUTTON_JUMP, *CONTROL_PAD_BUTTON_JUMP_MINI, *CONTROL_PAD_BUTTON_SMASH, *CONTROL_PAD_BUTTON_SPECIAL, 
            *CONTROL_PAD_BUTTON_SPECIAL_RAW, *CONTROL_PAD_BUTTON_SPECIAL_RAW2, *CONTROL_PAD_BUTTON_STOCK_SHARE, *CONTROL_PAD_BUTTON_TERM, *CONTROL_PAD_CLATTER_CAUSE_NONE, *CONTROL_PAD_CLATTER_FLOWER, *CONTROL_PAD_CLATTER_MAIN, *CONTROL_PAD_CLATTER_NONE,
            *CONTROL_PAD_CLATTER_TERM, *CONTROL_PAD_STICK_REVERSE_ALL, *CONTROL_PAD_STICK_REVERSE_NONE, *CONTROL_PAD_STICK_REVERSE_X, *CONTROL_PAD_STICK_REVERSE_Y
        ];
        for i in control_pad {
            if ControlModule::get_trigger_count(boma, i as u8) > 15 && ControlModule::check_button_on(boma, i)
            && ![*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_OFF].contains(&status_kind) {
                ControlModule::reset_trigger(boma);
                ControlModule::clear_command(boma, true);
            }
        }
        //Fullhop
        if FULL_HOP_ENABLE_DELAY[entry_id] > 0 {
			FULL_HOP_ENABLE_DELAY[entry_id] -= 1;
		};
        //This checks if the Full Hop button is pressed
		let triggered_buttons: Buttons = unsafe {
			Buttons::from_bits_unchecked(ControlModule::get_button(boma) & !ControlModule::get_button_prev(boma))
		};
		if triggered_buttons.intersects(Buttons::FullHop) {
			FULL_HOP_ENABLE_DELAY[entry_id] = 14;
		};
		if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) { 
            //Removes possibility of FH coming out of a SH. Shorthop button has priority over Fullhop
			FULL_HOP_ENABLE_DELAY[entry_id] = 0;
		};
    };
}

//Shield Specials
unsafe extern "C" fn if_shield_special(fighter: &mut L2CFighterCommon) -> L2CValue {
    let kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
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

//Installation of Shield Specials
#[smashline::fighter_init]
fn character_init(fighter: &mut L2CFighterCommon) {
    fighter.global_table[GUARD_CONT_UNIQ].assign(&L2CValue::Ptr(if_shield_special as *const () as _));
}

pub fn install() {
    install_agent_frame_callbacks!(all_frame);
    install_agent_init_callbacks!(character_init);
}