use super::*;

unsafe extern "C" fn gaogaen_throw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let motion_kind = WorkModule::get_int64(boma, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
    let mut thrown_type /*LStack_90*/ = hash40("thrown_lw");
    if [hash40("throw_f"), hash40("throw_f_revenge")].contains(&motion_kind) {
        thrown_type = hash40("thrown_f");
    }
    if [hash40("throw_b"), hash40("throw_b_revenge")].contains(&motion_kind) {
        thrown_type = hash40("thrown_b");
    }
    if [hash40("throw_hi"), hash40("throw_hi_revenge")].contains(&motion_kind) {
        thrown_type = hash40("thrown_hi");
    }
    if [hash40("throw_lw"), hash40("throw_lw_revenge")].contains(&motion_kind) {
        thrown_type = hash40("thrown_lw");
    }
    LinkModule::send_event_nodes_throw(boma, Hash40::new("throw"), Hash40::new_raw(thrown_type), true, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO, *FIGHTER_STATUS_THROW_WORK_FLOAT_MOTION_RATE);
    let get_node_object_id = LinkModule::get_node_object_id(boma, *LINK_NO_CAPTURE);
    WorkModule::set_int(boma, get_node_object_id as i32, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
    let throw_invincible_frame = WorkModule::get_param_int(boma, hash40("common"), hash40("throw_invincible_frame"));
    if 0 >= throw_invincible_frame {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_THROW_FLAG_INVINCIBLE);
    }
    else {
        HitModule::set_invincible_frame_global(boma, throw_invincible_frame, false, 0);
    }
    JostleModule::set_ignore_owner_id(boma, get_node_object_id as i32);
    0.into()
}

unsafe extern "C" fn gaogaen_throw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    gaogaen_throw_sub(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(gaogaen_throw_main_loop as *const () as _))
}

unsafe extern "C" fn gaogaen_throw_sub(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let motion_kind = WorkModule::get_int64(boma, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
    let motion_rate = WorkModule::get_float(boma, *FIGHTER_STATUS_THROW_WORK_FLOAT_MOTION_RATE);
    ItemModule::set_have_item_visibility(boma, false, 0);
    MotionModule::change_motion(boma, Hash40::new_raw(motion_kind), 0.0, motion_rate, false, 0.0, false, false);
    gaogaen_throw_log_common(fighter);
    if !StopModule::is_stop(boma) {
        gaogaen_throw_uniq(fighter);
    }
    fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::Ptr(gaogaen_throw_uniq as *const () as _));
}

unsafe extern "C" fn gaogaen_throw_log_common(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let motion_kind = WorkModule::get_int64(boma, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND);
    if [hash40("throw_f"), hash40("throw_f_revenge")].contains(&motion_kind) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_THROW_F);
    }
    if [hash40("throw_b"), hash40("throw_b_revenge")].contains(&motion_kind) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_THROW_B);
    }
    if [hash40("throw_hi"), hash40("throw_hi_revenge")].contains(&motion_kind) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_THROW_HI);
    }
    if [hash40("throw_lw"), hash40("throw_lw_revenge")].contains(&motion_kind) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), *FIGHTER_LOG_ACTION_CATEGORY_ATTACK, *FIGHTER_LOG_ATTACK_KIND_THROW_LW);
    }
}

unsafe extern "C" fn gaogaen_throw_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_THROW_FLAG_INVINCIBLE) {
        if !CatchModule::is_catch(boma) {
            HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_THROW_FLAG_INVINCIBLE);
        }
    }
    0.into()
}

unsafe extern "C" fn gaogaen_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let special_zoom_gfx = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    if CancelModule::is_enable_cancel(boma) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if WorkModule::is_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_THROW_CRITICAL_ZOOM) {
        WorkModule::set_int(boma, 1, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
        WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_THROW_CRITICAL_ZOOM);
    }
    if special_zoom_gfx > 0 && special_zoom_gfx < 4 {
        WorkModule::inc_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    }
    if special_zoom_gfx == 2 {
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
    if situation_kind == *SITUATION_KIND_AIR {
        let is_catch = {fighter.clear_lua_stack(); lua_args!(fighter, *MA_MSC_CMD_CATCH_IS_CATCH); sv_module_access::_catch(fighter.lua_state_agent); fighter.pop_lua_stack(1).get_bool()};
        if is_catch {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_JUMP.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    if MotionModule::is_end(boma) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn gaogaen_throw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_THROW_CRITICAL_ZOOM);
    0.into()
}

pub fn install() {
    Agent::new("gaogaen")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *FIGHTER_STATUS_KIND_THROW, gaogaen_throw_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_THROW, gaogaen_throw_main_status)
    .status(End, *FIGHTER_STATUS_KIND_THROW, gaogaen_throw_end_status)
    .install()
    ;
}