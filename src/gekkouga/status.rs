use super::*;

unsafe extern "C" fn gekkouga_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let chance = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("chance"));
    let rate = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("special_s"))/(chance as f32);
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
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let warp_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_INT_SPECIAL_S_WARP_COUNT);
    let chance = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("chance"));
    let stick_play = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("stick_play"));
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_START_WARP) {
        if chance-1 <= warp_count {
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
    if stick_y < 0.7 && stick_y > -0.7 {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_VERTICAL);
        if stick_play >= stick_x*lr {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_FRONT);
        }
        else {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_FRONT);
        }
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_VERTICAL);
        if stick_y > 0.7 {
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_HI);
        }
        else {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_HI);
        }
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
    let warp_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_INT_SPECIAL_S_WARP_COUNT);
    let work_warp_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_INT_WARP_FRAME);
    let shadow_x_pos = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_SHADOW_X_POS);
    let shadow_y_pos = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_SHADOW_Y_POS);
    let warp_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("warp_frame"));
    let chance = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("chance"));
    let pos_x = PostureModule::pos_y(fighter.module_accessor);
    let pos_y = PostureModule::pos_y(fighter.module_accessor);
    let pos_z = PostureModule::pos_z(fighter.module_accessor);
    let doll_id = WorkModule::get_int(fighter.module_accessor, 0x100000C2);
    let doll_boma = sv_battle_object::module_accessor(doll_id as u32);
    let doll_pos_x = PostureModule::pos_x(doll_boma);
    let doll_pos_y = PostureModule::pos_y(doll_boma);
    let doll_pos_z = PostureModule::pos_z(doll_boma);
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_INT_SPECIAL_S_WARP_COUNT);
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_INT_WARP_FRAME);
    if chance > warp_count {
        return 0.into();
    }
    if work_warp_frame == 0 {
        VisibilityModule::set_whole(fighter.module_accessor, false);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        AreaModule::set_whole(fighter.module_accessor, false);
    }
    if warp_frame <= work_warp_frame {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_WARP_GIMMICK) {
            if sv_battle_object::is_active(doll_id as u32) && WorkModule::is_flag(fighter.module_accessor, FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_FOUND_DOLL) {
                WorkModule::off_flag(fighter.module_accessor, FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_FOUND_DOLL);
                GroundModule::set_shape_safe_pos(fighter.module_accessor, &Vector2f{x: doll_pos_x, y: doll_pos_y});
            }
            else {
                GroundModule::set_shape_safe_pos(fighter.module_accessor, &Vector2f{x: shadow_x_pos, y: shadow_y_pos});
            }
        }
        else {
            GroundModule::set_shape_safe_pos(fighter.module_accessor, &Vector2f{x: pos_x, y: pos_y+0.1});
        }
        if sv_battle_object::is_active(doll_id as u32) && WorkModule::is_flag(fighter.module_accessor, FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_FOUND_DOLL) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_FOUND_DOLL);
            PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: doll_pos_x-(3.0*PostureModule::lr(fighter.module_accessor)), y: doll_pos_y, z: doll_pos_z});
        }
        else {
            PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: shadow_x_pos, y: shadow_y_pos, z: pos_z});
        }
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
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24b1b29e66));
    fighter.pop_lua_stack(1);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_FLAG_ATTACK_FRONT);
    WorkModule::set_flag(fighter.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOLD_FRONT) || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_FLAG_FIND_ENEMY) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_FRONT) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_FLAG_ATTACK_FRONT);
        }
        else {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_FLAG_ATTACK_FRONT);
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_VERTICAL) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_STATUS_SPECIAL_S_WORK_FLAG_ATTACK_FRONT) {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_attack_f"), L2CValue::Hash40s("special_air_s_attack_f"), false.into());
            if situation_kind != *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            }
            else {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            }
        }
        else {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_attack_b"), L2CValue::Hash40s("special_air_s_attack_b"), false.into());
            if situation_kind != *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            }
            else {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            }
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_HI) {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_attack_hi"), L2CValue::Hash40s("special_air_s_attack_hi"), false.into());
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            macros::SET_SPEED_EX(fighter, 0.0, 5.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else {
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_attack_lw"), L2CValue::Hash40s("special_air_s_attack_lw"), false.into());
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            macros::SET_SPEED_EX(fighter, 0.0, -5.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(gekkouga_special_s_attack_main_loop as *const () as _))
}

unsafe extern "C" fn gekkouga_special_s_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if !StatusModule::is_changing(fighter.module_accessor) {
        if prev_situation_kind != *SITUATION_KIND_GROUND {
            if situation_kind == *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                if [hash40("special_s_attack_hi"), hash40("special_air_s_attack_hi"), hash40("special_s_attack_lw"), hash40("special_air_s_attack_lw")].contains(&motion_kind) {
                    fighter.change_status(FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_S_END.into(), false.into());
                }
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
    let mut ground_motion_kind = "special_s_end_f";
    let mut air_motion_kind = "special_air_s_end_f";
    if [hash40("special_s_attack_b"), hash40("special_air_s_attack_b")].contains(&motion_kind) {
        ground_motion_kind = "special_s_end_b";
        air_motion_kind = "special_air_s_end_b";
        PostureModule::reverse_lr(fighter.module_accessor);
    }
    if [hash40("special_s_attack_hi"), hash40("special_air_s_attack_hi")].contains(&motion_kind) {
        ground_motion_kind = "special_s_end_hi";
        air_motion_kind = "special_air_s_end_hi";
    }
    if [hash40("special_s_attack_lw"), hash40("special_air_s_attack_lw")].contains(&motion_kind) {
        ground_motion_kind = "special_s_end_lw";
        air_motion_kind = "special_air_s_end_lw";
    }
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    }
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s(ground_motion_kind), L2CValue::Hash40s(air_motion_kind), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(gekkouga_special_s_end_main_loop as *const () as _))
}

unsafe extern "C" fn gekkouga_special_s_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let mut ground_motion_kind = "special_s_end_f";
    let mut air_motion_kind = "special_air_s_end_f";
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if [hash40("special_s_end_b"), hash40("special_air_s_end_b")].contains(&motion_kind) {
        ground_motion_kind = "special_s_end_b";
        air_motion_kind = "special_air_s_end_b";
    }
    if [hash40("special_s_end_hi"), hash40("special_air_s_end_hi")].contains(&motion_kind) {
        ground_motion_kind = "special_s_end_hi";
        air_motion_kind = "special_air_s_end_hi";
    }
    if [hash40("special_s_end_lw"), hash40("special_air_s_end_lw")].contains(&motion_kind) {
        ground_motion_kind = "special_s_end_lw";
        air_motion_kind = "special_air_s_end_lw";
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new(ground_motion_kind), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new(air_motion_kind), -1.0, 1.0, 0.0, false, false);
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
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn gekkouga_special_lw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    0.into()
}

unsafe extern "C" fn gekkouga_special_lw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_DOLL_LINK);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_lw"), L2CValue::Hash40s("special_air_lw"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(gekkouga_special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn gekkouga_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let doll_id = WorkModule::get_int(fighter.module_accessor, 0x100000C2);
    let doll_boma = sv_battle_object::module_accessor(doll_id as u32);
    let doll_pos_x = PostureModule::pos_x(doll_boma);
    let doll_pos_y = PostureModule::pos_y(doll_boma);
    let doll_pos_z = PostureModule::pos_z(doll_boma);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw"), -1.0, 1.0, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR
    && prev_situation_kind == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw"), -1.0, 1.0, 0.0, false, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_DOLL_LINK) {
        LinkModule::remove_model_constraint(doll_boma, true);
        if LinkModule::is_link(doll_boma, *ITEM_LINK_NO_HAVE) {
            LinkModule::unlink(doll_boma, *ITEM_LINK_NO_HAVE);
        }
        if !LinkModule::is_link(doll_boma, *ITEM_LINK_NO_HAVE) {
            VisibilityModule::set_whole(doll_boma, true);
            LinkModule::link(doll_boma, *ITEM_LINK_NO_HAVE, (*(fighter.module_accessor)).battle_object_id);
            LinkModule::set_model_constraint_pos_ort(doll_boma, *ITEM_LINK_NO_HAVE, Hash40::new("top"), Hash40::new("haver"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32, true);
            LinkModule::set_constraint_translate_offset(doll_boma, &Vector3f::zero());
        }
    }
    else {
        LinkModule::remove_model_constraint(doll_boma, true);
        if LinkModule::is_link(doll_boma, *ITEM_LINK_NO_HAVE) {
            LinkModule::unlink(doll_boma, *ITEM_LINK_NO_HAVE);
        }
        HitModule::set_whole(doll_boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_DOLL_LAUNCH) {
        PostureModule::set_pos(doll_boma, &Vector3f{x: doll_pos_x, y: doll_pos_y+3.0, z: doll_pos_z});
        KineticModule::change_kinetic(doll_boma, *ITEM_KINETIC_TYPE_NORMAL);
        KineticModule::add_speed(doll_boma, &Vector3f{x: 1.5*PostureModule::lr(fighter.module_accessor), y: 1.5, z: 0.0});
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_DOLL_LAUNCH);
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
    let doll_id = WorkModule::get_int(fighter.module_accessor, 0x100000C2);
    let doll_boma = sv_battle_object::module_accessor(doll_id as u32);
    LinkModule::remove_model_constraint(doll_boma, true);
    if LinkModule::is_link(doll_boma, *ITEM_LINK_NO_HAVE) {
        LinkModule::unlink(doll_boma, *ITEM_LINK_NO_HAVE);
    }
    HitModule::set_whole(doll_boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    0.into()
}

unsafe extern "C" fn gekkouga_special_lw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let doll_id = WorkModule::get_int(fighter.module_accessor, 0x100000C2);
    let doll_boma = sv_battle_object::module_accessor(doll_id as u32);
    LinkModule::remove_model_constraint(doll_boma, true);
    if LinkModule::is_link(doll_boma, *ITEM_LINK_NO_HAVE) {
        LinkModule::unlink(doll_boma, *ITEM_LINK_NO_HAVE);
    }
    HitModule::set_whole(doll_boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    0.into()
}

pub fn install() {
    Agent::new("gekkouga")
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
}

