#![allow(improper_ctypes_definitions)] //Addresses warning: `extern` fn uses type `str`, which is not FFI-safe
use super::*;

//Handles angling of moves
pub unsafe extern "C" fn change_angle(boma: *mut BattleObjectModuleAccessor, current_degree: f32, max_degree: f32, motion_kind_max: &str, motion_kind_min: &str) {
    let frame = MotionModule::frame(boma);
    let motion_kind_2nd = MotionModule::motion_kind_2nd(boma);
    let rate = MotionModule::rate(boma);
    let motion = if current_degree <= 0.0 {hash40(motion_kind_min)} else {hash40(motion_kind_max)};
    if motion_kind_2nd != motion {
        if current_degree <= 0.0 {
            MotionModule::add_motion_2nd(boma, Hash40::new(motion_kind_min), frame, rate, true, -(current_degree/max_degree));
            MotionModule::set_weight(boma, 1.0+(current_degree/max_degree), true);
        }
        else {
            MotionModule::add_motion_2nd(boma, Hash40::new(motion_kind_max), frame, rate, true, current_degree/max_degree);
            MotionModule::set_weight(boma, 1.0-(current_degree/max_degree), true);
        }
    }
    else {
        if current_degree < 0.0 {
            MotionModule::set_weight(boma, 1.0+(current_degree/max_degree), true);
        }
        else if current_degree > 0.0 {
            MotionModule::set_weight(boma, 1.0-(current_degree/max_degree), true);
        }
        else {
            MotionModule::set_weight(boma, 1.0, true);
        }
    }
}

//Indicates when moves are off cooldown
pub unsafe extern "C" fn gimmick_flash(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let lr = PostureModule::lr(boma);
    let offset = WorkModule::get_param_float(boma, hash40("height"), 0);
    if !sv_information::is_ready_go() {
        return;
    }
    FighterUtil::flash_eye_info(boma);
    if WorkModule::get_param_int(boma, hash40("param_motion"), hash40("flip")) != 0 {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("top"), -5, offset, 2, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
    }
    else {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -5.0*lr, offset, 2, 0, 0, 0, 1.0, true);
    }
    LAST_EFFECT_SET_COLOR(fighter, 0.831, 0.686, 0.216);
}

pub unsafe extern "C" fn final_zoom_effect_remove(boma: *mut BattleObjectModuleAccessor, effect_handle: i32) {
    let agent = get_fighter_common_from_accessor(&mut *boma);
    if effect_handle != *EFFECT_HANDLE_NULL || EffectModule::is_exist_effect(boma, effect_handle as u32) {
        EffectModule::kill(boma, effect_handle as u32, true, true);
		EFFECT_OFF_KIND(agent, Hash40::new("sys_bg_criticalhit"), true, true);
		EFFECT_OFF_KIND(agent, Hash40::new("sys_bg_finishhit"), true, true);
		EFFECT_OFF_KIND(agent, Hash40::new("sys_bg_black"), true, true);
		EFFECT_OFF_KIND(agent, Hash40::new("sys_bg_vortex"), true, true);
		EFFECT_OFF_KIND(agent, Hash40::new("sys_bg_vortex2"), true, true);
		EFFECT_OFF_KIND(agent, Hash40::new("sys_bg_lightning"), true, true);
		EFFECT_OFF_KIND(agent, Hash40::new("critical_hit"), true, true);
		EFFECT_OFF_KIND(agent, Hash40::new("edge_win_fire"), true, true);
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
		EffectModule::remove_screen(boma, Hash40::new("bg_wario_final2"), -1);
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
		EffectModule::remove_screen(boma, Hash40::new("bg_cloud_final"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_kamui_final"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_bayonetta_final"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_inkling_final_l2"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_ridley_final"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_simon_final"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_richter_final"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_krool_final"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_shizue_final"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_gaogaen_final"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_packun_final2"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_jack_final"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_brave_final"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_buddy_final"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_dolly_final"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_master_final"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_tantan_final_end"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_pickel_final_end"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_eflame_final2"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_eelight_final2"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_demon_final"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_trail_final"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_miifighter_final"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_miiswordsman_final"), -1);
		EffectModule::remove_screen(boma, Hash40::new("bg_miigunner_final"), -1);
		WorkModule::set_int(boma, *EFFECT_HANDLE_NULL, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
    }
}