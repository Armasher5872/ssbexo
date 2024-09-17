use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_FuraFura_Main)]
unsafe fn status_furafura_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let motion_kind = MotionModule::motion_kind(boma);
    let special_zoom_gfx = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    let shield_break_timer = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        let c_hash = Hash40::new_raw(0x10708f1e7c);
        let e_hash = Hash40::new_raw(0x101a3f3e8e);
        if [c_hash.hash as u64, e_hash.hash as u64].contains(&motion_kind) {
            return 1.into();
        }
        else {
            if MotionModule::is_end(boma) {
                MotionModule::change_motion(boma, Hash40::new("furafura"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        /* START OF NEW ADDITIONS */
        WorkModule::inc_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
        if shield_break_timer >= 120 {
            WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
            fighter.change_status(FIGHTER_STATUS_KIND_SAVING_DAMAGE.into(), false.into());
        }
        if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&fighter.global_table[PREV_STATUS_KIND].get_i32()) {
            if special_zoom_gfx < 4 {
                WorkModule::inc_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
            }
            if special_zoom_gfx >= 3 {
                SlowModule::clear_whole(boma);
                CameraModule::reset_all(boma);
                EffectModule::kill_kind(boma, Hash40::new("sys_bg_criticalhit"), false, false);
                macros::CAM_ZOOM_OUT(fighter);
            }
            if shield_break_timer == 0 {
                EffectModule::req_follow(boma, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
                macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
                macros::EFFECT_FLW_POS(fighter, Hash40::new("sys_piyopiyo"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1.3, true);
                macros::PLAY_SE(fighter, Hash40::new("se_common_guardbreak"));
                macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_XL);
                SlowModule::set_whole(boma, 8, 80);
            }
        }
        /* END OF NEW ADDITIONS */
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_FALL.into(), false.into());
    }
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_FuraFuraEnd)]
unsafe fn status_furafuraend(fighter: &mut L2CFighterCommon) -> L2CValue {
    if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&fighter.global_table[PREV_STATUS_KIND].get_i32()) {
        MotionModule::set_rate(fighter.module_accessor, 0.6);
        macros::EFFECT(fighter, Hash40::new("sys_piyo"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_piyopiyo"), false, false);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(0xcacd460a4), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_FuraFuraEnd_Main as *const () as _))
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_furafura_main,
            status_furafuraend
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}