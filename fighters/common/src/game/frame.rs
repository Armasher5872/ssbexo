use super::*;

//Credited to WuBoyTH, the global opff for fighters
#[skyline::hook(offset = 0x614630)]
unsafe extern "C" fn global_once_per_fighter_frame(fighter: &mut Fighter) {
    original!()(fighter);
	let boma = fighter.battle_object.module_accessor;
	let agent = get_fighter_common_from_accessor(&mut *boma);
    let kind = agent.global_table[FIGHTER_KIND].get_i32();
	let status_kind = agent.global_table[STATUS_KIND].get_i32();
    let situation_kind = agent.global_table[SITUATION_KIND].get_i32();
	let final_zoom_counter = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
	let effect_handle = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
    let invalid_runback_states = !sv_information::is_ready_go() && [*FIGHTER_STATUS_KIND_DEMO, *FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_ROULETTE_FURAFURA, *FIGHTER_STATUS_KIND_ROULETTE, *FIGHTER_STATUS_KIND_STANDBY].contains(&status_kind);
    let fighter_low_offset = [*FIGHTER_KIND_KIRBY, *FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_NESS, *FIGHTER_KIND_PURIN, *FIGHTER_KIND_GAMEWATCH, *FIGHTER_KIND_POPO, *FIGHTER_KIND_NANA, *FIGHTER_KIND_PICHU, *FIGHTER_KIND_METAKNIGHT, *FIGHTER_KIND_WARIO, *FIGHTER_KIND_PZENIGAME, *FIGHTER_KIND_PFUSHIGISOU, *FIGHTER_KIND_LUCAS, *FIGHTER_KIND_PIKMIN, *FIGHTER_KIND_TOONLINK, *FIGHTER_KIND_DUCKHUNT, *FIGHTER_KIND_MURABITO, *FIGHTER_KIND_INKLING, *FIGHTER_KIND_SHIZUE];
    let fighter_mid_offset = [*FIGHTER_KIND_MARIO, *FIGHTER_KIND_YOSHI, *FIGHTER_KIND_LUIGI, *FIGHTER_KIND_MARIOD, *FIGHTER_KIND_YOUNGLINK, *FIGHTER_KIND_PLIZARDON, *FIGHTER_KIND_DIDDY, *FIGHTER_KIND_DEDEDE, *FIGHTER_KIND_ROCKMAN, *FIGHTER_KIND_GEKKOUGA, *FIGHTER_KIND_PACMAN, *FIGHTER_KIND_KOOPAJR, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_MIIFIGHTER, *FIGHTER_KIND_MIISWORDSMAN, *FIGHTER_KIND_MIIGUNNER, *FIGHTER_KIND_PACKUN, *FIGHTER_KIND_BUDDY, *FIGHTER_KIND_PICKEL];
    let fighter_high_offset = [*FIGHTER_KIND_FOX, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_DAISY, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_PIT, *FIGHTER_KIND_PITB, *FIGHTER_KIND_SONIC, *FIGHTER_KIND_SZEROSUIT, *FIGHTER_KIND_LUCARIO, *FIGHTER_KIND_ROBOT, *FIGHTER_KIND_WOLF, *FIGHTER_KIND_LITTLEMAC, *FIGHTER_KIND_KROOL, *FIGHTER_KIND_GAOGAEN];
    let fighter_max_offset = [*FIGHTER_KIND_DONKEY, *FIGHTER_KIND_LINK, *FIGHTER_KIND_SAMUS, *FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_PEACH, *FIGHTER_KIND_KOOPA, *FIGHTER_KIND_SHEIK, *FIGHTER_KIND_ZELDA, *FIGHTER_KIND_MARTH, *FIGHTER_KIND_LUCINA, *FIGHTER_KIND_GANON, *FIGHTER_KIND_ROY, *FIGHTER_KIND_CHROM, *FIGHTER_KIND_SNAKE, *FIGHTER_KIND_IKE, *FIGHTER_KIND_WIIFIT, *FIGHTER_KIND_ROSETTA, *FIGHTER_KIND_PALUTENA, *FIGHTER_KIND_REFLET, *FIGHTER_KIND_SHULK, *FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN, *FIGHTER_KIND_CLOUD, *FIGHTER_KIND_KAMUI, *FIGHTER_KIND_BAYONETTA, *FIGHTER_KIND_RIDLEY, *FIGHTER_KIND_SIMON, *FIGHTER_KIND_RICHTER, *FIGHTER_KIND_JACK, *FIGHTER_KIND_BRAVE, *FIGHTER_KIND_DOLLY, *FIGHTER_KIND_MASTER, *FIGHTER_KIND_TANTAN, *FIGHTER_KIND_EDGE, *FIGHTER_KIND_EFLAME, *FIGHTER_KIND_ELIGHT, *FIGHTER_KIND_DEMON, *FIGHTER_KIND_TRAIL];
    let offset_y = if fighter_low_offset.contains(&kind) {2.0} else if fighter_mid_offset.contains(&kind) {3.5} else if fighter_high_offset.contains(&kind) {4.0} else if fighter_max_offset.contains(&kind) {5.0} else {1.0};
    //Chaingrab Mechanics
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CAPTURE_COOLDOWN) > 0 {
        WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CAPTURE_COOLDOWN);
    }
    else {
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CAPTURE_TIMER_MULTIPLIER) > 1 {
            WorkModule::set_int(boma, 1, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CAPTURE_TIMER_MULTIPLIER);
        }
    }
    //Final Zoom Clearing
	if final_zoom_counter > 0 {
		if final_zoom_counter == 40 {
			if is_final_killing_hit(&mut *boma) {
				set_stage_visibility(boma, 1);
				set_vis_hud(true);
			}
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_STOCK);
		}
		if final_zoom_counter <= 40 {
			EffectModule::set_alpha(boma, effect_handle as u32, 0.0+((final_zoom_counter-20) as f32/20.0));
		}
		if final_zoom_counter == 25 {
			SlowModule::clear_whole(boma);
            CameraModule::zoom_out(boma, 0);
            ControlModule::stop_rumble(boma, true);
            for quake_kind in *CAMERA_QUAKE_KIND_NONE..=*CAMERA_QUAKE_KIND_MAX {
                CameraModule::stop_quake(boma, quake_kind);
            }
		}
		if final_zoom_counter <= 20 {
			final_zoom_effect_remove(boma, effect_handle);
		}
		WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
	}
	else {
        final_zoom_effect_remove(boma, effect_handle);
	}
    //Credited to HDR. Salty Runback/Match Exit
    if invalid_runback_states {
        IS_SALTY_RUNBACK = false;
        IS_SALTY_MATCH_EXIT = false;
    }
    if salty_runback_check(agent) {
        IS_SALTY_RUNBACK = true;
    }
    if salty_quit_check(agent) {
        IS_SALTY_MATCH_EXIT = true;
    }
    //Credit to JoshuaSwagi. ECB tweaks
    if situation_kind == *SITUATION_KIND_AIR
    && [*FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_FALL_AERIAL, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind) {
        let mut final_offset = offset_y;
        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind) {
            final_offset -= 0.2;
        }
        GroundModule::set_offset_y(boma, final_offset);
        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) < 10 {
            GroundModule::set_rhombus_offset(boma, &Vector2f{x: 0.0, y: 0.0});
        }
    }
    else {
        if GroundModule::get_offset_y(boma) != 0.0 {
            GroundModule::set_offset_y(boma, 0.0);
            GroundModule::set_rhombus_offset(boma, &Vector2f{x: 0.0, y: 0.0});
        }
    }
}

pub fn install() {
    skyline::install_hook!(global_once_per_fighter_frame);
}