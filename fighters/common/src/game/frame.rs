use super::*;

//Credited to WuBoyTH, the global opff for fighters
#[skyline::hook(offset = 0x614630)]
unsafe extern "C" fn global_once_per_fighter_frame(fighter: &mut Fighter) {
    original!()(fighter);
	let boma = fighter.battle_object.module_accessor;
	let agent = get_fighter_common_from_accessor(&mut *boma);
	let status_kind = agent.global_table[STATUS_KIND].get_i32();
	let prev_status_kind = agent.global_table[PREV_STATUS_KIND].get_i32();
	let final_zoom_counter = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
	let effect_handle = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
	//Zair Platform Falling
	if status_kind == *FIGHTER_STATUS_KIND_AIR_LASSO && prev_status_kind == *FIGHTER_STATUS_KIND_PASS && !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
        GroundModule::set_passable_check(boma, true);
    }
	//Bury Knockback Resistance
    if [*FIGHTER_STATUS_KIND_BURY, *FIGHTER_STATUS_KIND_BURY_WAIT].contains(&status_kind) {
        DamageModule::set_reaction_mul(boma, 0.77);
    }
    if status_kind == *FIGHTER_STATUS_KIND_BURY_JUMP || (is_damaged(boma) && [*FIGHTER_STATUS_KIND_BURY, *FIGHTER_STATUS_KIND_BURY_WAIT].contains(&prev_status_kind)) {
        DamageModule::set_reaction_mul(boma, 1.0);
    }
	//Final Zoom Clearing
	if final_zoom_counter > 0 {
		println!("Final Zoom Handle: {}", effect_handle);
		println!("Final Zoom Counter: {}", final_zoom_counter);
		if final_zoom_counter == 40 {
			WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_STOCK);
			if is_final_killing_hit(&mut *boma) {
				set_stage_visibility(boma, 1);
				set_vis_hud(true);
			}
		}
		if final_zoom_counter <= 40 {
			EffectModule::set_alpha(boma, effect_handle as u32, 0.0+((final_zoom_counter-20) as f32/20.0));
		}
		if final_zoom_counter == 25 {
			SlowModule::clear_whole(boma);
		}
		if final_zoom_counter <= 20 {
			EffectModule::kill(boma, effect_handle as u32, true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("sys_bg_criticalhit"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("sys_bg_finishhit"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("sys_bg_black"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("mario_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("donkey_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("link_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("samus_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("samusd_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("yoshi_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("kirby_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("pikachu_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("luigi_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("ness_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("captain_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("purin_final_bg_vortex"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("peach_final_gb"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("daisy_final_gb"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("koopa_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("popo_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("nana_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("sheik_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("zelda_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("mariod_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("pichu_final_bg_flash"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("marth_final_line_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("lucina_final_line_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("younglink_final_line_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("ganon_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("mewtwo_final_mega_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("roy_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("chrom_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("gamewatch_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("metaknight_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("pit_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("pitb_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("szero_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("wario_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("snake_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("ike_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("ptrainer_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("diddy_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("lucas_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("sonic_final_bg_vortex"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("dedede_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("pikmin_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("lucario_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("robot_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("toonlink_final_line_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("murabito_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("rockman_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("wiifit_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("rosetta_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("littlemac_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("gekkouga_final_bg_vortex"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("palutena_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("pacman_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("reflet_final_line_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("shulk_final_world_effect"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("koopajr_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("ryu_final_line_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("ken_final_line_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("cloud_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("kamui_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("bayonetta_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("inkling_final_bg_l"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("ridley_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("simon_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("richter_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("krool_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("shizue_final_bg2"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("gaogaen_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("packun_final_bg_1"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("jack_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("buddy_final_flow_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("dolly_fainal_bg1"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("master_final_bg_vortex"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("tantan_final_bg_l"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("pickel_final_bg_l"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("edge_win_fire"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("eflame_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("demon_final_bg_after_l"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("trail_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("miifighter_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("miiswordsman_final_bg"), true, true);
			EFFECT_OFF_KIND(agent, Hash40::new("miigunner_final_bg"), true, true);
			WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
		}
		WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
	}
	else {
		WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
	}
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
		CAM_ZOOM_IN_arg5(fighter, 5.0, 0.0, PostureModule::scale(boma)*1.5, 0.0, 0.0);
	}
	if special_zoom_gfx >= 10 {
		SlowModule::clear_whole(boma);
		CameraModule::reset_all(boma);
		CAM_ZOOM_OUT(fighter);
		COUNTERHIT_SUCCESS[get_player_number(&mut *boma)] = false;
		WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
	}
}
*/

pub fn install() {
    skyline::install_hook!(global_once_per_fighter_frame);
}