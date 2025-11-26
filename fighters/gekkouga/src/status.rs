use super::*;

unsafe extern "C" fn gekkouga_attack_air_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    let get_attack_air_kind = ControlModule::get_attack_air_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT && collision_kind != *COLLISION_KIND_SHIELD {
            if get_attack_air_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW || motion_kind == hash40("attack_air_lw") {
                let object_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
                let opponent_boma = sv_battle_object::module_accessor(object_id);
                StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR, false);  
            }
        }
    }
    0.into()
}

unsafe extern "C" fn gekkouga_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let end_frame_from_hash = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("special_s"));
    let chance = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("chance")) as f32;
    let rate = end_frame_from_hash/chance;
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_INT_WARP_FRAME);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_FLAG_ATTACK_BACK);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, rate, false, 0.0, false, false);
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_START_WARP);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_INT_SPECIAL_S_WARP_COUNT);
    fighter.sub_shift_status_main(L2CValue::Ptr(gekkouga_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn gekkouga_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let special_s_warp_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_INT_SPECIAL_S_WARP_COUNT);
    let stick_play = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("stick_play"));
    let chance = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("chance"));
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_START_WARP) {
        if chance-1 <= special_s_warp_count {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_START_WARP);
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind != *SITUATION_KIND_GROUND {
            if situation_kind == *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            }
        }
        else {
            if situation_kind == *SITUATION_KIND_AIR {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            }
        }
    }
    if stick_play >= stick_x*lr {
        if stick_play >= stick_x*-lr {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_FRONT);
        }
        else {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_FRONT);
        }
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_FRONT);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_FLAG_RESET_GRAVITY) {
        if situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_FLAG_RESET_GRAVITY);
        }
    }
    0.into()
}

unsafe extern "C" fn gekkouga_special_s_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let pos_x = PostureModule::pos_x(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let pos_z = PostureModule::pos_z(fighter.module_accessor);
    let shadow_x_pos = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_SHADOW_X_POS);
    let shadow_y_pos = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_SHADOW_Y_POS);
    let warp_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_INT_SPECIAL_S_WARP_COUNT);
    let work_warp_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_INT_WARP_FRAME);
    let chance = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("chance"));
    let warp_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("warp_frame"));
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_INT_SPECIAL_S_WARP_COUNT);
    if chance > warp_count {
        return 0.into();
    }
    if work_warp_frame == 0 {
        VisibilityModule::set_whole(fighter.module_accessor, false);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        AreaModule::set_whole(fighter.module_accessor, false);
    }
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_INT_WARP_FRAME);
    if warp_frame <= work_warp_frame {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_WARP_GIMMICK) {
            GroundModule::set_shape_safe_pos(fighter.module_accessor, &Vector2f{x: shadow_x_pos, y: shadow_y_pos});
        }
        else {
            GroundModule::set_shape_safe_pos(fighter.module_accessor, &Vector2f{x: pos_x, y: pos_y+0.1});
        }
        PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: shadow_x_pos, y: shadow_y_pos, z: pos_z});
        if situation_kind != *SITUATION_KIND_GROUND {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_WARP_AIR) {
                fighter.set_situation(SITUATION_KIND_GROUND.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_WARP_AIR) {
                fighter.set_situation(SITUATION_KIND_AIR.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            }
        }
        fighter.change_status(FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_S_ATTACK.into(), false.into());
    }
    1.into()
}

unsafe extern "C" fn gekkouga_special_s_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_FLAG_ATTACK_FRONT);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOLD_FRONT) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_FRONT) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_FLAG_ATTACK_FRONT);
        }
    }
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24b1b29e66));
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_FLAG_FIND_ENEMY) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_FRONT) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_FLAG_ATTACK_FRONT);
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_FLAG_ATTACK_FRONT);
        }
    }
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_FLAG_ATTACK_FRONT) {
        if situation_kind != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_attack_f"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_attack_f"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_attack_b"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_attack_b"), 0.0, 1.0, false, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(gekkouga_special_s_attack_main_loop as *const () as _))
}

unsafe extern "C" fn gekkouga_special_s_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind != *SITUATION_KIND_GROUND {
            if situation_kind == *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            }
        }
        else {
            if situation_kind == *SITUATION_KIND_AIR {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_S_END.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn gekkouga_special_s_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let mut front = true;
    if ![hash40("special_s_attack_f"), hash40("special_air_s_attack_f")].contains(&motion_kind) {
        front = false;
    }
    if !front {
        if situation_kind != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_end_b"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_end_b"), 0.0, 1.0, false, 0.0, false, false);
        }
        PostureModule::reverse_lr(fighter.module_accessor);
    }
    else {
        if situation_kind != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_end_f"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_end_f"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(gekkouga_special_s_end_main_loop as *const () as _))
}

unsafe extern "C" fn gekkouga_special_s_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind != *SITUATION_KIND_GROUND {
            if situation_kind == *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                if motion_kind != hash40("special_air_s_end_f") {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_end_b"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_end_f"), -1.0, 1.0, 0.0, false, false);
                }
            }
        }
        else {
            if situation_kind == *SITUATION_KIND_AIR {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                if motion_kind != hash40("special_s_end_f") {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_end_b"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_end_f"), -1.0, 1.0, 0.0, false, false);
                }
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn gekkouga_special_lw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let status_attr = if situation_kind == *SITUATION_KIND_AIR {*FIGHTER_STATUS_ATTR_START_TURN} else {0};
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, status_attr as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn gekkouga_special_lw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_DISABLE);
    0.into()
}

unsafe extern "C" fn gekkouga_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw"), L2CValue::Hash40s("special_air_lw"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(gekkouga_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn gekkouga_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn gekkouga_special_lw_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn gekkouga_special_lw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MAT) {
        let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MAT);
        let article_motion_kind = MotionModule::motion_kind(article_boma);
        if article_motion_kind == hash40("special_lw") {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MAT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    0.into()
}

unsafe extern "C" fn gekkouga_special_lw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MAT) {
        let article_boma = get_article_boma(fighter.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MAT);
        let article_motion_kind = MotionModule::motion_kind(article_boma);
        if article_motion_kind == hash40("special_lw") {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MAT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    0.into()
}

unsafe extern "C" fn gekkouga_mat_fall_pre_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_NONE), *WEAPON_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn gekkouga_mat_fall_init_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_boma = get_owner_boma(weapon);
    let owner_situation_kind = StatusModule::situation_kind(owner_boma);
    if !StatusModule::is_changing(owner_boma) || !StatusModule::is_situation_changed(owner_boma) {
        if owner_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            weapon.set_situation(SITUATION_KIND_AIR.into());
        }
        else {
            GroundModule::correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            weapon.set_situation(SITUATION_KIND_GROUND.into());
        }
    }
    WorkModule::set_int(weapon.module_accessor, 45, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, 45, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn gekkouga_mat_fall_main_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let situation_kind = weapon.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_air_lw_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
    }
    weapon.fastshift(L2CValue::Ptr(gekkouga_mat_fall_main_loop as *const () as _))
}

unsafe extern "C" fn gekkouga_mat_fall_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let frame = weapon.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = weapon.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = weapon.global_table[PREV_SITUATION_KIND].get_i32();
    let motion_kind = MotionModule::motion_kind(weapon.module_accessor);
    if should_remove_projectile(weapon) || (situation_kind == *SITUATION_KIND_GROUND && prev_situation_kind == *SITUATION_KIND_AIR) {
        mat_removal(weapon);
    }
    if motion_kind == hash40("special_air_lw_start") && frame == 28.0 {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_air_lw"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -4.0);
        sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -4.0);
        sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    }
    0.into()
}

unsafe extern "C" fn gekkouga_mat_fall_exec_status(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if !StopModule::is_stop(weapon.module_accessor) {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

unsafe extern "C" fn gekkouga_mat_fall_end_status(_weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("gekkouga")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_AIR, gekkouga_attack_air_check_attack_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, gekkouga_special_s_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, gekkouga_special_s_exec_status)
    .status(Main, *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_S_ATTACK, gekkouga_special_s_attack_main_status)
    .status(Main, *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_S_END, gekkouga_special_s_end_main_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, gekkouga_special_lw_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_LW, gekkouga_special_lw_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, gekkouga_special_lw_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, gekkouga_special_lw_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, gekkouga_special_lw_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_LW, gekkouga_special_lw_exit_status)
    .install()
    ;
    Agent::new("gekkouga_mat")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *WEAPON_GEKKOUGA_MAT_STATUS_KIND_FALL, gekkouga_mat_fall_pre_status)
    .status(Init, *WEAPON_GEKKOUGA_MAT_STATUS_KIND_FALL, gekkouga_mat_fall_init_status)
    .status(Main, *WEAPON_GEKKOUGA_MAT_STATUS_KIND_FALL, gekkouga_mat_fall_main_status)
    .status(Exec, *WEAPON_GEKKOUGA_MAT_STATUS_KIND_FALL, gekkouga_mat_fall_exec_status)
    .status(End, *WEAPON_GEKKOUGA_MAT_STATUS_KIND_FALL, gekkouga_mat_fall_end_status)
    .install()
    ;
}