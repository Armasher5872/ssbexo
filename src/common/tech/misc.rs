use super::*;

//This opff really only exists to deal with status kinds I couldn't translate, or have far too many status kinds to account for
unsafe extern "C" fn all_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(boma);
    let cbm_vec1 = Vector4f{/* Red */ x: 1.0, /* Green */ y: 1.0, /* Blue */ z: 1.0, /* Alpha */ w: 0.2};
    let cbm_vec2 = Vector4f{/* Red */ x: 0.0, /* Green */ y: 0.0, /* Blue */ z: 0.0, /* Alpha */w: 0.8};
    let mut pos = Vector3f {x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma), z: PostureModule::pos_z(boma)}; // get current pos
    let mashing = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
    let special_zoom_gfx = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
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
    //Mashing
    if [*FIGHTER_STATUS_KIND_BURY, *FIGHTER_STATUS_KIND_BURY_WAIT, *FIGHTER_STATUS_KIND_ICE].contains(&status_kind) {
        WorkModule::inc_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
        if mashing >= 5 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                ControlModule::add_clatter_time(boma, -15.0, 0);
            }
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
            SoundModule::set_se_pitch_ratio(boma, Hash40::new("se_common_counter"), 1.0);
            SoundModule::set_se_vol(boma, counter_sound as i32, 5.0, 0);
            SlowModule::set_whole(boma, 4, 30);
            macros::CAM_ZOOM_IN_arg5(fighter, 5.0, 0.0, PostureModule::scale(boma)*1.5, 0.0, 0.0);
        }
        if special_zoom_gfx >= 10 {
            SlowModule::clear_whole(boma);
            CameraModule::reset_all(boma);
            macros::CAM_ZOOM_OUT(fighter);
            COUNTERHIT_SUCCESS[get_player_number(boma)] = false;
            WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
        }
    }
    //Held Buffer
    let control_pad = [
        *CONTROL_PAD_BUTTON_ATTACK, *CONTROL_PAD_BUTTON_JUMP, *CONTROL_PAD_BUTTON_CATCH, *CONTROL_PAD_BUTTON_GUARD, *CONTROL_PAD_BUTTON_SMASH, *CONTROL_PAD_BUTTON_SPECIAL, *CONTROL_PAD_BUTTON_CSTICK_ON, *CONTROL_PAD_BUTTON_JUMP_MINI,
        *CONTROL_PAD_BUTTON_ATTACK_RAW, *CONTROL_PAD_BUTTON_SPECIAL_RAW, *CONTROL_PAD_BUTTON_SPECIAL_RAW2
    ];
    for i in control_pad {
        if ControlModule::get_trigger_count(boma, i as u8) > 15 && ControlModule::check_button_on(boma, i)
        && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) 
        && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R)
        && ![*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_OFF, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&status_kind) {
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
}

pub fn install() {
    smashline::api::install_line_callback(None, StatusLine::Main, all_frame as _);
}