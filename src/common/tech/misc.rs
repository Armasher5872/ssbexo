use super::*;

//This opff really only exists to deal with status kinds I couldn't translate, or have far too many status kinds to account for
unsafe extern "C" fn all_frame(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let cbm_vec1 = Vector4f{/* Red */ x: 1.0, /* Green */ y: 1.0, /* Blue */ z: 1.0, /* Alpha */ w: 0.2};
    let cbm_vec2 = Vector4f{/* Red */ x: 0.0, /* Green */ y: 0.0, /* Blue */ z: 0.0, /* Alpha */w: 0.8};
    let mashing = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
    //let special_zoom_gfx = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    let jump_count = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    let max_jump_count = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX);
    let counter = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
    let handle = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
    let full_hop_enable_delay = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FULL_HOP_ENABLE_DELAY);
    let triggered_buttons: Buttons = unsafe {Buttons::from_bits_unchecked(ControlModule::get_button(boma) & !ControlModule::get_button_prev(boma))}; //This checks if the Full Hop button is pressed
    //Lost Double Jump Indicator
    if jump_count >= max_jump_count {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
    }
    else {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
    }
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT) {
        ColorBlendModule::set_main_color(boma, /* Brightness */&cbm_vec1, /* Diffuse */&cbm_vec2, 0.21, 2.2, 5, /* Display Color */ true);
    }
    if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT) && ![*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_OFF].contains(&status_kind) {
        ColorBlendModule::cancel_main_color(boma, 0);
    }
    //Zair Platform Dropping
    if status_kind == *FIGHTER_STATUS_KIND_AIR_LASSO && prev_status_kind == *FIGHTER_STATUS_KIND_PASS && !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
        GroundModule::set_passable_check(boma, true);
    }
    //Mashing
    if [*FIGHTER_STATUS_KIND_BURY, *FIGHTER_STATUS_KIND_BURY_WAIT, *FIGHTER_STATUS_KIND_ICE].contains(&status_kind) {
        WorkModule::inc_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
        if mashing >= 5 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                ControlModule::add_clatter_time(boma, -15.0, 0);
            }
            WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
        }
    }
    //Bury Knockback Resistance
    if [*FIGHTER_STATUS_KIND_BURY, *FIGHTER_STATUS_KIND_BURY_WAIT].contains(&status_kind) {
        DamageModule::set_reaction_mul(boma, 0.77);
    }
    if status_kind == *FIGHTER_STATUS_KIND_BURY_JUMP || (WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) && [*FIGHTER_STATUS_KIND_BURY, *FIGHTER_STATUS_KIND_BURY_WAIT].contains(&prev_status_kind)) {
        DamageModule::set_reaction_mul(boma, 1.0);
    }
    /*
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
        if estimate_frame(&mut *boma, 0.0) {
            COUNTERHIT_CHECK[get_player_number(&mut *boma)] = true;
        }
        if AttackModule::is_attack(boma, 0, false) {
            COUNTERHIT_CHECK[get_player_number(&mut *boma)] = false;
        }
    }
    else {
        COUNTERHIT_SUCCESS[get_player_number(&mut *boma)] = false;
    }
    if COUNTERHIT_SUCCESS[get_player_number(&mut *boma)] {
        if special_zoom_gfx < 10 {
            WorkModule::inc_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
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
            COUNTERHIT_SUCCESS[get_player_number(&mut *boma)] = false;
            WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
        }
    }
    */
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
    if full_hop_enable_delay > 0 {
        WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FULL_HOP_ENABLE_DELAY);
    }
    if triggered_buttons.intersects(Buttons::FullHop) {
        WorkModule::set_int(boma, 14, *FIGHTER_INSTANCE_WORK_ID_INT_FULL_HOP_ENABLE_DELAY);
    }
    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FULL_HOP_ENABLE_DELAY); //Removes possibility of FH coming out of a SH. Shorthop button has priority over Fullhop
    }
    //Final Zoom Effect Clearing
    if counter > 0 {
        if counter <= 16 {
            if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK) {
                EffectModule::set_rate(boma, handle as u32, 1.0);
            }
            set_stage_visibility(boma, 1);
            set_vis_hud(true);
        }
        if counter == 10 {
            if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK) {
                EffectModule::remove_screen(boma, Hash40::new("bg_finishhit"), -1);
            }
            else {
                EffectModule::remove_screen(boma, Hash40::new("bg_mario_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_donkey_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_link_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_samus_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_samusd_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_yoshi_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_kirby_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_pikachu_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_luigi_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_ness_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_captain_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_purin_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_peach_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_daisy_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_koopa_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_popo_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_sheik_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_zelda_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_mariod_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_pichu_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_marth_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_lucina_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_younglink_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_ganon_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_mewtwo_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_roy_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_chrom_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_gamewatch_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_metaknight_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_pit_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_pitb_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_szerosuit_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_wario_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_snake_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_ike_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_ptrainer_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_diddy_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_lucas_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_sonic_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_dedede_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_pikmin_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_lucario_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_robot_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_toonlink_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_murabito_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_rockman_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_wiifit_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_rosetta_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_littlemac_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_gekkouga_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_palutena_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_pacman_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_reflet_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_shulk_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_koopajr_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_duckhunt_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_ryu_final_shinsyoryu"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_ken_final_shinryuken"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_cloud_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_kamui_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_bayonetta_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_inkling_final_l"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_ridley_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_simon_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_richter_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_krool_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_shizue_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_gaogaen_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_packun_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_jack_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_brave_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_buddy_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_dolly_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_master_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_tantan_final_l"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_pickel_final_l"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_edge_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_eflame_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_eelight_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_demon_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_trail_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_miifighter_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_miiswordsman_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_miigunner_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_criticalhit"), -1);
            }
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_bg_black"), false, false);
            macros::CAM_ZOOM_OUT(fighter);
        }
        if counter == 5 {
            SlowModule::clear_whole(boma);
        }
        WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
    }
    else {
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
    }
    println!("Jump Speed Ratio: {}", WorkModule::get_float(boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_JUMP_SPEED_RATIO));
    println!("Current X Speed: {}", KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN));
}

unsafe extern "C" fn all_weapon_frame(weapon: &mut L2CFighterBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent);
    let link_owner = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = &mut *smash::app::sv_battle_object::module_accessor(link_owner);
    let counter = WorkModule::get_int(owner_boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
    //Final Zoom Effect Clearing
    if counter > 0 {
        let handle = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
        if counter <= 16 {
            if !WorkModule::is_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK) {
                EffectModule::set_rate(boma, handle as u32, 1.0);
            }
            set_stage_visibility(boma, 1);
            set_vis_hud(true);
        }
        if counter == 10 {
            if WorkModule::is_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK) {
                EffectModule::remove_screen(boma, Hash40::new("bg_finishhit"), -1);
            }
            else {
                EffectModule::remove_screen(boma, Hash40::new("bg_mario_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_donkey_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_link_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_samus_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_samusd_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_yoshi_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_kirby_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_pikachu_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_luigi_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_ness_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_captain_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_purin_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_peach_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_daisy_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_koopa_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_popo_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_sheik_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_zelda_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_mariod_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_pichu_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_marth_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_lucina_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_younglink_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_ganon_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_mewtwo_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_roy_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_chrom_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_gamewatch_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_metaknight_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_pit_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_pitb_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_szerosuit_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_wario_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_snake_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_ike_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_ptrainer_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_diddy_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_lucas_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_sonic_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_dedede_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_pikmin_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_lucario_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_robot_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_toonlink_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_murabito_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_rockman_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_wiifit_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_rosetta_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_littlemac_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_gekkouga_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_palutena_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_pacman_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_reflet_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_shulk_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_koopajr_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_duckhunt_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_ryu_final_shinsyoryu"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_ken_final_shinryuken"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_cloud_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_kamui_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_bayonetta_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_inkling_final_l"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_ridley_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_simon_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_richter_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_krool_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_shizue_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_gaogaen_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_packun_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_jack_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_brave_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_buddy_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_dolly_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_master_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_tantan_final_l"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_pickel_final_l"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_edge_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_eflame_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_eelight_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_demon_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_trail_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_miifighter_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_miiswordsman_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_miigunner_final"), -1);
                EffectModule::remove_screen(boma, Hash40::new("bg_criticalhit"), -1);
            }
            macros::EFFECT_OFF_KIND(weapon, Hash40::new("sys_bg_black"), false, false);
            macros::CAM_ZOOM_OUT(weapon);
        }
        if counter == 5 {
            SlowModule::clear_whole(boma);
        }
        WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
    }
    else {
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
    }
}

pub fn install() {
    Agent::new("fighter")
	.on_line(Main, all_frame)
	.install()
	;
    Agent::new("weapon")
    .on_line(Main, all_weapon_frame)
    .install()
    ;
}