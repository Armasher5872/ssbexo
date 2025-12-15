use super::*;

unsafe extern "C" fn gaogaen_throw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
    let motion_rate = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_FLOAT_MOTION_RATE);
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion_kind), 0.0, motion_rate, false, 0.0, false, false);
    gaogaen_throw_log_common(fighter);
    if !StopModule::is_stop(fighter.module_accessor) {
        gaogaen_throw_uniq(fighter);
    }
    fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::Ptr(gaogaen_throw_uniq as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(gaogaen_throw_main_loop as *const () as _))
}

unsafe extern "C" fn gaogaen_throw_log_common(fighter: &mut L2CFighterCommon) {
    let motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
    if ![hash40("throw_f"), hash40("throw_f_revenge")].contains(&motion_kind) {
        if ![hash40("throw_b"), hash40("throw_b_revenge")].contains(&motion_kind) {
            if ![hash40("throw_hi"), hash40("throw_hi_revenge")].contains(&motion_kind) {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_THROW_LW);
                fighter.pop_lua_stack(1);
            }
            else {
                notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_THROW_HI);
                fighter.pop_lua_stack(1);
            }
        }
        else {
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_THROW_B);
            fighter.pop_lua_stack(1);
        }
    }
    else {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_THROW_F);
        fighter.pop_lua_stack(1);
    }
}

unsafe extern "C" fn gaogaen_throw_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_THROW_FLAG_INVINCIBLE) {
        if !CatchModule::is_catch(fighter.module_accessor) {
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_THROW_FLAG_INVINCIBLE);
        }
    }
    0.into()
}

unsafe extern "C" fn gaogaen_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let special_zoom_gfx = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() && fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_THROW_CRITICAL_ZOOM) {
        WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_THROW_CRITICAL_ZOOM);
    }
    if special_zoom_gfx > 0 {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    }
    if special_zoom_gfx == 2 {
        SlowModule::set_whole(fighter.module_accessor, 8, 80);
        CAM_ZOOM_IN_arg5(fighter, /*frames*/ 2.0,/*no*/ 0.0,/*zoom*/ 1.8,/*yrot*/ 0.0,/*xrot*/ 0.0);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), Hash40::new("top"), &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
        PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
        QUAKE(fighter, *CAMERA_QUAKE_KIND_XL);
    }
    if special_zoom_gfx >= 4 {
        SlowModule::clear_whole(fighter.module_accessor);
        CameraModule::reset_all(fighter.module_accessor);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
        CAM_ZOOM_OUT(fighter);
    }
    if situation_kind != *SITUATION_KIND_AIR {
        if MotionModule::is_end(fighter.module_accessor) {
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
        }
    }
    else {
        let is_catch = {fighter.clear_lua_stack(); lua_args!(fighter, *MA_MSC_CMD_CATCH_IS_CATCH); sv_module_access::_catch(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
        if is_catch {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_JUMP.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn gaogaen_throw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_THROW_CRITICAL_ZOOM);
    0.into()
}

pub fn install() {
    Agent::new("gaogaen")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_THROW, gaogaen_throw_main_status)
    .status(End, *FIGHTER_STATUS_KIND_THROW, gaogaen_throw_end_status)
    .install()
    ;
}