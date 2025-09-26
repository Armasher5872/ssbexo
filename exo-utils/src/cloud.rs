use super::*;

pub unsafe extern "C" fn cloud_can_limit_break(fighter: &mut L2CFighterCommon, required_limit: i32) -> L2CValue {
    let limit_level = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_LEVEL);
    let special_wait_timer = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_INPUT_WAIT_TIMER);
    if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && special_wait_timer <= 0 {
        WorkModule::set_int(fighter.module_accessor, 15, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_INPUT_WAIT_TIMER);
    }
    if special_wait_timer > 0 {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_INPUT_WAIT_TIMER);
    }
    if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    && special_wait_timer > 0 
    && limit_level >= required_limit {
        return true.into();
    }
    false.into()
}

pub unsafe extern "C" fn cloud_training_mode_features(boma: *mut BattleObjectModuleAccessor) {
    let get_team_color = FighterUtil::get_team_color(boma) as i32;
    let status_kind = StatusModule::status_kind(boma);
    let limit_gauge_effect = WorkModule::get_int(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_GAUGE_EFFECT);
    let limit_gauge_offset_y = WorkModule::get_param_float(boma, hash40("param_special_lw"), hash40("limit_gauge_offset_y"));
    if smashball::is_training_mode()
    && status_kind == *FIGHTER_STATUS_KIND_APPEAL
    && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
        WorkModule::set_int(boma, 4, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_LEVEL);
        WorkModule::set_float(boma, 100.0, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLOAT_LIMIT_GAUGE);
        EffectModule::req_follow(boma, Hash40::new("cloud_limitbreak_aura"), Hash40::new("hip"), &Vector3f::zero(), &Vector3f::zero(), 2.0, false, *EFFECT_SUB_ATTRIBUTE_SYNC_INIT_POS as u32, 0, -1, 0, 0, false, false);
        EffectModule::req_common(boma, Hash40::new("cloud_limitbreak"), 0.0);
        EffectModule::req_follow(boma, Hash40::new_raw(0x16102a334b), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, true, 0, 0, -1, 0, 0, false, false);
        if limit_gauge_effect != 0 {
            EffectModule::detach(boma, limit_gauge_effect as u32, 0);
            EffectModule::req_follow(boma, Hash40::new_raw(0x14d013ba16), Hash40::new("top"), &Vector3f{x: 0.0, y: limit_gauge_offset_y, z: 0.0}, &Vector3f{x: 0.0, y: limit_gauge_offset_y, z: 0.0}, 1.0, true, 0, 0, get_team_color, 0, 0, false, false);
            EffectModule::set_rot(boma, limit_gauge_effect as u32, &Vector3f{x: 0.0, y: limit_gauge_offset_y, z: 0.0});
            WorkModule::set_int64(boma, limit_gauge_effect as i64, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_GAUGE_EFFECT);
        }
        SoundModule::play_se(boma, Hash40::new("se_cloud_special_l03"), true, false, false, false, enSEType(0));
        FighterUtil::flash_eye_info(boma);
        WorkModule::on_flag(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK);
        WorkModule::set_int(boma, i32::MAX, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_BREAK_CLEAR_FRAME);
    }
}

pub unsafe extern "C" fn cloud_set_vec_target_pos(boma: *mut BattleObjectModuleAccessor, hit_id: i32, bone: Hash40, pos: Vector2f, time_to_position: c_uint) {
    let angle = WorkModule::get_int(boma, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_ATTACK_ANGLE);
    let x = pos.x;
    let y = pos.y;
    let fighter = get_fighter_common_from_accessor(&mut *boma);
    let mut vector = fighter.Vector2__create(x.into(), y.into());
    let vec_x = vector["x"].get_f32();
    let vec_y = vector["y"].get_f32();
    if angle != 0 {
        let rad = (angle as f32)*-0.01745329;
        if rad != 0.0 {
            let sin = rad.sin();
            let cos = rad.cos();
            let new_x = sin*vec_x+cos*vec_y;
            let new_y = cos*vec_x+sin*vec_y;
            vector["x"].assign(&L2CValue::F32(new_x));
            vector["y"].assign(&L2CValue::F32(new_y));
        }
    }
    AttackModule::set_vec_target_pos(boma, hit_id, bone, &Vector2f{x: vec_x, y: vec_y}, time_to_position, false);
}