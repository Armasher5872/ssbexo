use super::*;

//Aerial Check Attack Status
unsafe extern "C" fn sheik_attack_air_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    let get_attack_air_kind = ControlModule::get_attack_air_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(fighter.module_accessor);
    let speed_x = get_sum_speed_x*lr;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT && collision_kind != *COLLISION_KIND_SHIELD {
            if get_attack_air_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW
            || motion_kind == hash40("attack_air_lw") {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE) {
                    SET_SPEED_EX(fighter, speed_x, 2.35, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
                }
            }
        }
    }
    0.into()
}

unsafe extern "C" fn sheik_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn sheik_special_s_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    0.into()
}

unsafe extern "C" fn sheik_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s"), L2CValue::Hash40s("special_air_s"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(sheik_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn sheik_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_SPECIAL_S_OBJECT_ID);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SHEIK_GENERATE_ARTICLE_KNIFE) {
        let knife_boma = get_article_boma(fighter.module_accessor, *FIGHTER_SHEIK_GENERATE_ARTICLE_KNIFE);
        LinkModule::set_model_constraint_pos_ort(knife_boma, *LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("throw"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_OFFSET_ROT) as u32, true);
        LinkModule::set_constraint_translate_offset(knife_boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
        LinkModule::set_constraint_rot_offset(knife_boma, &Vector3f{x: 90.0, y: 0.0, z: 0.0});
    }
    if object_id != *BATTLE_OBJECT_ID_INVALID {
        if frame < 30.0 {
            let opponent_boma = sv_battle_object::module_accessor(object_id as u32);
            MotionModule::set_rate(opponent_boma, 0.0);
        }
        else {
            let opponent_boma = sv_battle_object::module_accessor(object_id as u32);
            let opponent_agent = get_fighter_common_from_accessor(&mut *opponent_boma);
            LinkModule::remove_model_constraint(opponent_boma, true);
            if LinkModule::is_link(opponent_boma, *LINK_NO_CAPTURE) {
                LinkModule::unlink(opponent_boma, *LINK_NO_CAPTURE);
            }
            GroundModule::set_ignore_boss(opponent_boma, false);
            GroundModule::set_passable_check(opponent_boma, true);
            GroundModule::set_collidable(opponent_boma, true);
            JostleModule::set_status(opponent_boma, true);
            MotionModule::set_rate(opponent_boma, 1.0);
            opponent_agent.FighterStatusCapture_set_invalid_capture();
            WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_SPECIAL_S_OBJECT_ID);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn sheik_special_s_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn sheik_special_s_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    let opponent_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
    let opponent_boma = sv_battle_object::module_accessor(opponent_id);
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT && collision_kind != *COLLISION_KIND_SHIELD {
            if current_frame < 24.0 {
                if WorkModule::get_int(opponent_boma, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CAPTURE_FRAME) <= 0 {
                    WorkModule::set_int(fighter.module_accessor, opponent_id as i32, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_SPECIAL_S_OBJECT_ID);
                    LinkModule::remove_model_constraint(opponent_boma, true);
                    if LinkModule::is_link(opponent_boma, *LINK_NO_CAPTURE) {
                        LinkModule::unlink(opponent_boma, *LINK_NO_CAPTURE);
                    }
                    if !LinkModule::is_link(opponent_boma, *LINK_NO_CAPTURE) {
                        VisibilityModule::set_whole(opponent_boma, true);
                        LinkModule::link(opponent_boma, *LINK_NO_CAPTURE, fighter.battle_object_id);
                        LinkModule::set_model_constraint_pos_ort(opponent_boma, *LINK_NO_CAPTURE, Hash40::new("trans"), Hash40::new("throw"), *CONSTRAINT_FLAG_ORIENTATION as u32 | *CONSTRAINT_FLAG_POSITION as u32 | *CONSTRAINT_FLAG_OFFSET_TRANSLATE as u32, true);
                        LinkModule::set_constraint_translate_offset(opponent_boma, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
                    }
                    GroundModule::set_ignore_boss(opponent_boma, true);
                    GroundModule::set_passable_check(opponent_boma, false);
                    GroundModule::set_collidable(opponent_boma, false);
                    JostleModule::set_status(opponent_boma, false);
                    MotionModule::change_motion(opponent_boma, Hash40::new("damage_n_1"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
        }
    }
    0.into()
}

unsafe extern "C" fn sheik_special_s_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_SPECIAL_S_OBJECT_ID);
    if object_id != *BATTLE_OBJECT_ID_INVALID {
        let opponent_boma = sv_battle_object::module_accessor(object_id as u32);
        let opponent_agent = get_fighter_common_from_accessor(&mut *opponent_boma);
        LinkModule::remove_model_constraint(opponent_boma, true);
        if LinkModule::is_link(opponent_boma, *LINK_NO_CAPTURE) {
            LinkModule::unlink(opponent_boma, *LINK_NO_CAPTURE);
        }
        GroundModule::set_ignore_boss(opponent_boma, false);
        GroundModule::set_passable_check(opponent_boma, true);
        GroundModule::set_collidable(opponent_boma, true);
        JostleModule::set_status(opponent_boma, true);
        MotionModule::set_rate(opponent_boma, 1.0);
        opponent_agent.FighterStatusCapture_set_invalid_capture();
    }
    ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_SHEIK_GENERATE_ARTICLE_KNIFE, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SHEIK_GENERATE_ARTICLE_KNIFE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_SPECIAL_S_OBJECT_ID);
    0.into()
}

unsafe extern "C" fn sheik_special_s_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let object_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_SPECIAL_S_OBJECT_ID);
    if object_id != *BATTLE_OBJECT_ID_INVALID {
        let opponent_boma = sv_battle_object::module_accessor(object_id as u32);
        let opponent_agent = get_fighter_common_from_accessor(&mut *opponent_boma);
        LinkModule::remove_model_constraint(opponent_boma, true);
        if LinkModule::is_link(opponent_boma, *LINK_NO_CAPTURE) {
            LinkModule::unlink(opponent_boma, *LINK_NO_CAPTURE);
        }
        GroundModule::set_ignore_boss(opponent_boma, false);
        GroundModule::set_passable_check(opponent_boma, true);
        GroundModule::set_collidable(opponent_boma, true);
        JostleModule::set_status(opponent_boma, true);
        MotionModule::set_rate(opponent_boma, 1.0);
        opponent_agent.FighterStatusCapture_set_invalid_capture();
    }
    ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_SHEIK_GENERATE_ARTICLE_KNIFE, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SHEIK_GENERATE_ARTICLE_KNIFE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    WorkModule::set_int(fighter.module_accessor, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_SPECIAL_S_OBJECT_ID);
    0.into()
}

pub fn install() {
    Agent::new("sheik")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_AIR, sheik_attack_air_check_attack_status)
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, sheik_special_s_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, sheik_special_s_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, sheik_special_s_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, sheik_special_s_exec_status)
    .status(CheckAttack, *FIGHTER_STATUS_KIND_SPECIAL_S, sheik_special_s_check_attack_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, sheik_special_s_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, sheik_special_s_exit_status)
    .install()
    ;
}