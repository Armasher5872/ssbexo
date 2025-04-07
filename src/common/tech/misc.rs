use super::*;

//This opff really only exists to deal with status kinds I couldn't translate, or have far too many status kinds to account for
unsafe extern "C" fn all_frame(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let mashing = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
    //let special_zoom_gfx = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    let full_hop_enable_delay = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FULL_HOP_ENABLE_DELAY);
    let triggered_buttons: Buttons = unsafe {Buttons::from_bits_retain(ControlModule::get_button(boma) & !ControlModule::get_button_prev(boma))}; //This checks if the Full Hop button is pressed
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
}

pub fn install() {
    Agent::new("fighter")
	.on_line(Main, all_frame)
	.install()
	;
}