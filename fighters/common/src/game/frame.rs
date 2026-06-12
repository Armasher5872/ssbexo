use super::*;

//Credited to WuBoyTH, the global opff for fighters
#[skyline::hook(offset = 0x614630)]
unsafe extern "C" fn global_once_per_fighter_frame(fighter: &mut Fighter) {
    original!()(fighter);
	let boma = fighter.battle_object.module_accessor;
	let agent = get_fighter_common_from_accessor(&mut *boma);
	let status_kind = agent.global_table[STATUS_KIND].get_i32();
	let final_zoom_counter = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
	let effect_handle = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
    let invalid_runback_states = !sv_information::is_ready_go() && [*FIGHTER_STATUS_KIND_DEMO, *FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_ROULETTE_FURAFURA, *FIGHTER_STATUS_KIND_ROULETTE, *FIGHTER_STATUS_KIND_STANDBY].contains(&status_kind);
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
}

pub fn install() {
    skyline::install_hook!(global_once_per_fighter_frame);
}