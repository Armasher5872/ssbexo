/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

/*   DAMAGE STATUSES   */
//Furafura Stand, clears the Shield Break Zoom Effect
#[skyline::hook(replace = L2CFighterCommon_status_FuraFuraStand)]
unsafe fn status_furafurastand(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DOWN_FLAG_UP) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(0x10708f1e7c), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(0x101a3f3e8e), 0.0, 1.0, false, 0.0, false, false);
    }
    let xlu_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("furafura_stand_whole_hit_xlu_frame"));
    WorkModule::set_int(fighter.module_accessor, xlu_frame+1, *FIGHTER_STATUS_FURAFURA_STAND_WORK_INT_FRAME_COUNT);
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_furafura_stand_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_furafura_stand_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(status_furafurastand_main as *const () as _))
}

unsafe extern "C" fn status_furafurastand_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    /* START OF NEW ADDITIONS */
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    SlowModule::clear_whole(boma);
    CameraModule::reset_all(boma);
    EffectModule::kill_kind(boma, Hash40::new("sys_bg_criticalhit"), false, false);
    macros::CAM_ZOOM_OUT(fighter);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SHIELD_COLOR_R);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SHIELD_COLOR_G);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SHIELD_COLOR_B);
    /* END OF NEW ADDITIONS */
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FURAFURA.into(), false.into());
            }
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHANGE_STATUS_DLAY_MOTION) {
                return 0.into();
            }
            else {
                return 1.into();
            }
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

//Furafura, forces a transition into Crumple and adds Automatic Mashing to the status
#[skyline::hook(replace = L2CFighterCommon_status_FuraFura)]
unsafe fn status_furafura(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let c_hash = Hash40::new_raw(0x10708f1e7c);
    let e_hash = Hash40::new_raw(0x101a3f3e8e);
    if [c_hash.hash as u64, e_hash.hash as u64].contains(&motion_kind) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("furafura"), 0.0, 1.0, false, 0.0, false, false);
        return true.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        return true.into();
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(status_furafura_main as *const () as _))
}

unsafe extern "C" fn status_furafura_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let motion_kind = MotionModule::motion_kind(boma);
    let shield_break_timer = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    let mashing = WorkModule::get_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
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
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
        WorkModule::inc_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
        WorkModule::inc_int(boma, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
        if shield_break_timer >= 120 {
            fighter.change_status(FIGHTER_STATUS_KIND_SAVING_DAMAGE.into(), true.into());
            WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
        }
        if mashing >= 5 && (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)) {
            ControlModule::add_clatter_time(boma, -15.0, 0);
            WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
        }
        /* END OF NEW ADDITIONS */
        if ControlModule::get_clatter_time(boma, 0) <= 0.0 {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FURAFURA_END.into(), false.into());
                WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
            }
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_furafurastand,
            status_furafura
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}