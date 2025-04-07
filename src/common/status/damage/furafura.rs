use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_FuraFura)]
unsafe extern "C" fn status_furafura(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if ![hash40("furafura_start_u"), hash40("furafura_start_d")].contains(&motion_kind) && MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("furafura"), 0.0, 1.0, false, 0.0, false, false);
    }
    WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_FuraFura_Main as *const () as _))
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_FuraFura_Main)]
unsafe extern "C" fn status_furafura_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let special_zoom_gfx = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    let shield_break_timer = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        if ![hash40("furafura_start_u"), hash40("furafura_start_d")].contains(&motion_kind) && MotionModule::is_end(fighter.module_accessor) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("furafura"), 0.0, 1.0, false, 0.0, false, false);
        }
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
        if shield_break_timer >= 120 {
            fighter.change_status(FIGHTER_STATUS_KIND_SAVING_DAMAGE.into(), false.into());
        }
        if special_zoom_gfx > 0 {
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
        }
        if special_zoom_gfx == 2 {
            SlowModule::set_whole(fighter.module_accessor, 8, 80);
            macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, false, 0, 0, 0, 0, 0, false, false);
            macros::EFFECT_FLW_POS(fighter, Hash40::new("sys_piyopiyo"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1.3, true);
            macros::PLAY_SE(fighter, Hash40::new("se_common_guardbreak"));
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_XL);
        }
        if special_zoom_gfx >= 4 {
            SlowModule::clear_whole(fighter.module_accessor);
            CameraModule::reset_all(fighter.module_accessor);
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
            macros::CAM_ZOOM_OUT(fighter);
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_DAMAGE_FALL.into(), false.into());
    }
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_end_FuraFura)]
unsafe extern "C" fn status_end_furafura(fighter: &mut L2CFighterCommon) -> L2CValue {
    SlowModule::clear_whole(fighter.module_accessor);
    CameraModule::reset_all(fighter.module_accessor);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_piyopiyo"), false, false);
    macros::CAM_ZOOM_OUT(fighter);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    0.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_FuraFuraEnd)]
unsafe fn status_furafuraend(fighter: &mut L2CFighterCommon) -> L2CValue {
    macros::PLAY_SE(fighter, Hash40::new("se_common_guardbreak"));
    macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("furafura_end"), 0.0, 0.6, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_FuraFuraEnd_Main as *const () as _))
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_furafura,
            status_furafura_main,
            status_end_furafura,
            status_furafuraend
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}