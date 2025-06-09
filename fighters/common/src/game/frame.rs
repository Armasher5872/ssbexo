use super::*;

//Credited to WuBoyTH, the global opff for fighters
#[skyline::hook(offset = 0x614630)]
unsafe extern "C" fn global_once_per_fighter_frame(fighter: &mut Fighter) {
    original!()(fighter);
	let boma = fighter.battle_object.module_accessor;
	let agent = get_fighter_common_from_accessor(&mut *boma);
	let status_kind = agent.global_table[STATUS_KIND].get_i32();
	let prev_status_kind = agent.global_table[PREV_STATUS_KIND].get_i32();
	let mashing = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
	let final_zoom_attacker_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_ATTACKER_ID);
	let final_zoom_defender_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_DEFENDER_ID);
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
	//Final Zoom Effect Clearing
	if final_zoom_attacker_id != *BATTLE_OBJECT_ID_INVALID {
		let attacker_battle_object = *get_battle_object_from_id(final_zoom_attacker_id as u32);
		let attacker_boma = attacker_battle_object.module_accessor;
		let attacker_agent = get_fighter_common_from_accessor(&mut *attacker_boma);
		let counter = WorkModule::get_int(attacker_boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
		if final_zoom_defender_id != *BATTLE_OBJECT_ID_INVALID {
			let defender_battle_object = *get_battle_object_from_id(final_zoom_defender_id as u32);
			let defender_boma = defender_battle_object.module_accessor;
			let defender_agent = get_fighter_common_from_accessor(&mut *defender_boma);
			if counter > 0 {
				if counter == 80 {
					if WorkModule::is_flag(defender_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK) {
						set_stage_visibility(attacker_boma, 1);
						UiManager::set_ui_state(0, true);
					}
				}
				if counter <= 40 {
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_black"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_finishhit"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_mario_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_donkey_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_link_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_samus_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_samusd_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_yoshi_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_kirby_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_pikachu_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_luigi_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_ness_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_captain_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_purin_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_peach_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_daisy_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_koopa_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_popo_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_sheik_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_zelda_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_mariod_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_pichu_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_marth_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_lucina_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_younglink_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_ganon_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_mewtwo_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_roy_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_chrom_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_gamewatch_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_metaknight_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_pit_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_pitb_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_szerosuit_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_wario_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_snake_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_ike_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_ptrainer_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_diddy_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_lucas_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_sonic_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_dedede_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_pikmin_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_lucario_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_robot_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_toonlink_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_murabito_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_rockman_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_wiifit_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_rosetta_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_littlemac_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_gekkouga_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_palutena_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_pacman_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_reflet_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_shulk_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_koopajr_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_duckhunt_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_ryu_final_shinsyoryu"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_ken_final_shinryuken"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_cloud_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_kamui_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_bayonetta_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_inkling_final_l"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_ridley_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_simon_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_richter_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_krool_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_shizue_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_gaogaen_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_packun_final1"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_jack_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_brave_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_buddy_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_dolly_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_master_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_tantan_final_l"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_pickel_final_l"), 0);
					EFFECT_OFF_KIND(attacker_agent, Hash40::new("edge_win_fire"), false, false);
					EFFECT_OFF_KIND(attacker_agent, Hash40::new("edge_win_sprks_b"), false, false);
					EFFECT_OFF_KIND(attacker_agent, Hash40::new("sys_bg_black"), false, false);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_eflame_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_eelight_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_demon_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_trail_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_miifighter_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_miiswordsman_final"), 0);
					EffectModule::remove_screen(attacker_boma, Hash40::new("bg_miigunner_final"), 0);
					CAM_ZOOM_OUT(defender_agent);
				}
				if counter == 20 {
					SlowModule::clear_whole(attacker_boma);
					SlowModule::clear_whole(defender_boma);
				}
				WorkModule::dec_int(attacker_boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
			}
			else {
				WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_DEFENDER_ID);
			}
		}
		if counter == 0 {
			WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_ATTACKER_ID);
		}
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