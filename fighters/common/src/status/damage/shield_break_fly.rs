use super::*;

//Sub Status Shield Break Fly Common, initiates Shield Break Zoom
#[skyline::hook(replace = L2CFighterCommon_sub_status_shield_break_fly_common)]
unsafe extern "C" fn sub_status_shield_break_fly_common(fighter: &mut L2CFighterCommon, bool_check: L2CValue) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    let group = sv_fighter_util::get_dead_up_camera_hit_my_distance_group(lua_state);
    let gravity_speed = {fighter.clear_lua_stack(); lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY); sv_kinetic_energy::get_speed_y(lua_state)};
    let gravity_accel = {fighter.clear_lua_stack(); lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY); sv_kinetic_energy::get_accel_y(lua_state)};
    let gravity_limit_speed_y = {fighter.clear_lua_stack(); lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY); sv_kinetic_energy::get_limit_speed_y(lua_state)};
    let hard_break = WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK);
    let shield_reset = WorkModule::get_param_float(boma, hash40("common"), hash40("shield_reset"));
    let shield_break_xlu_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("shield_break_xlu_frame"));
    WorkModule::set_float(boma, shield_reset, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHECK_DEAD_AREA_FORCE);
    if FighterUtil::check_melee_rule_time(300.0, smash::app::FighterCheckMeleeRuleTime{0: group}, true) {
        if hard_break {
            HitModule::set_whole(boma, HitStatus(*HIT_STATUS_XLU), 0);
        }
    }
    MotionModule::change_motion(boma, Hash40::new("shield_break_fly"), 0.0, 1.0, false, 0.0, false, false);
    if bool_check.get_bool() {
        SoundModule::play_se(boma, Hash40::new("se_common_guardbreak"), true, false, false, false, enSEType(0));
    }
    WorkModule::set_int(boma, shield_break_xlu_frame, *FIGHTER_STATUS_FURAFURA_STAND_WORK_INT_TERMINATE_XLU_COUNT);
    WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    if !hard_break {
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, gravity_speed*0.4);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, gravity_accel*0.2);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, gravity_limit_speed_y*0.3);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_FB);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP_BUTTON);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL_JUMP);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_WALL);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_PASSIVE_CEIL);
    }
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_SHIELD_BREAK_FLY_NUM);
}

//Status Shield Break Fly Main, makes soft breaks go into untechable knockdown instead of Shield Break Downed and adds Special Zoom on break
#[skyline::hook(replace = L2CFighterCommon_status_ShieldBreakFly_Main)]
unsafe extern "C" fn status_shield_break_fly_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let special_zoom_gfx = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    if special_zoom_gfx >= 0 && special_zoom_gfx < 4 {
        WorkModule::inc_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    }
    if special_zoom_gfx == 1 {
        SlowModule::set_whole(boma, 8, 80);
        CAM_ZOOM_IN_arg5(fighter, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
        EffectModule::req_follow(boma, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, false, 0, 0, 0, 0, 0, false, false);
        PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
        QUAKE(fighter, *CAMERA_QUAKE_KIND_XL);
    }
    if special_zoom_gfx >= 4 {
        SlowModule::clear_whole(boma);
        CameraModule::reset_all(boma);
        EffectModule::kill_kind(boma, Hash40::new("sys_bg_criticalhit"), false, false);
        CAM_ZOOM_OUT(fighter);
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK) {
            fighter.change_status(FIGHTER_STATUS_KIND_DOWN.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_SHIELD_BREAK_DOWN.into(), false.into());
        }
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_STATUS_KIND_SHIELD_BREAK_FALL.into(), false.into());
    }
    0.into()
}

//Status End Shield Break Fly
#[skyline::hook(replace = L2CFighterCommon_status_end_ShieldBreakFly)]
unsafe extern "C" fn status_end_shield_break_fly(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if status_kind != *FIGHTER_STATUS_KIND_SHIELD_BREAK_FALL {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK);
    }
    SlowModule::clear_whole(boma);
    CameraModule::reset_all(boma);
    EffectModule::kill_kind(boma, Hash40::new("sys_bg_criticalhit"), false, false);
    CAM_ZOOM_OUT(fighter);
    WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_status_shield_break_fly_common,
            status_shield_break_fly_main,
            status_end_shield_break_fly
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}