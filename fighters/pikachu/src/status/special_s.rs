use super::*;

unsafe extern "C" fn pikachu_special_s_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn pikachu_special_s_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    }
    else {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    0.into()
}

unsafe extern "C" fn pikachu_special_s_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    ArticleModule::generate_article(boma, FIGHTER_PIKACHU_GENERATE_ARTICLE_TAIL, false, -1);
    if ArticleModule::is_exist(boma, FIGHTER_PIKACHU_GENERATE_ARTICLE_TAIL) {
        let tail_boma = get_article_boma(boma, FIGHTER_PIKACHU_GENERATE_ARTICLE_TAIL);
        ModelModule::set_joint_scale(boma, Hash40::new("tail1"), &Vector3f{x: 0.001, y: 0.001, z: 0.001});
        LinkModule::set_model_constraint_pos_ort(tail_boma, *LINK_NO_CONSTRAINT, Hash40::new("tail1"), Hash40::new("tail1"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION) as u32, true);
    }
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s"), L2CValue::Hash40s("special_air_s"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(pikachu_special_s_loop as *const () as _))
}

unsafe extern "C" fn pikachu_special_s_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(boma) {
        if prev_situation_kind == *SITUATION_KIND_GROUND
        && situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if WorkModule::is_flag(boma, *FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ENABLE_LANDING) {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
            }
        }
        if prev_situation_kind == *SITUATION_KIND_AIR
        && situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_s"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if ArticleModule::is_exist(boma, FIGHTER_PIKACHU_GENERATE_ARTICLE_TAIL) {
        let tail_boma = get_article_boma(boma, FIGHTER_PIKACHU_GENERATE_ARTICLE_TAIL);
        ModelModule::set_joint_scale(boma, Hash40::new("tail1"), &Vector3f{x: 0.001, y: 0.001, z: 0.001});
        if [8.0, 12.0].contains(&frame) {
            ModelModule::set_joint_scale(tail_boma, Hash40::new("tail1"), &Vector3f{x: 1.1, y: 1.1, z: 1.1});
            ModelModule::set_joint_scale(tail_boma, Hash40::new("tail2"), &Vector3f{x: 1.1, y: 1.1, z: 1.1});
            ModelModule::set_joint_scale(tail_boma, Hash40::new("tail3"), &Vector3f{x: 1.1, y: 1.1, z: 1.1});
            ModelModule::set_joint_scale(tail_boma, Hash40::new("tail4"), &Vector3f{x: 1.1, y: 1.1, z: 1.1});
        }
        if [9.0, 11.0].contains(&frame) {
            ModelModule::set_joint_scale(tail_boma, Hash40::new("tail1"), &Vector3f{x: 1.175, y: 1.175, z: 1.175});
            ModelModule::set_joint_scale(tail_boma, Hash40::new("tail2"), &Vector3f{x: 1.175, y: 1.175, z: 1.175});
            ModelModule::set_joint_scale(tail_boma, Hash40::new("tail3"), &Vector3f{x: 1.175, y: 1.175, z: 1.175});
            ModelModule::set_joint_scale(tail_boma, Hash40::new("tail4"), &Vector3f{x: 1.175, y: 1.175, z: 1.175});
        }
        if frame == 10.0 {
            ModelModule::set_joint_scale(tail_boma, Hash40::new("tail1"), &Vector3f{x: 1.25, y: 1.25, z: 1.25});
            ModelModule::set_joint_scale(tail_boma, Hash40::new("tail2"), &Vector3f{x: 1.25, y: 1.25, z: 1.25});
            ModelModule::set_joint_scale(tail_boma, Hash40::new("tail3"), &Vector3f{x: 1.25, y: 1.25, z: 1.25});
            ModelModule::set_joint_scale(tail_boma, Hash40::new("tail4"), &Vector3f{x: 1.25, y: 1.25, z: 1.25});
        }
        if frame == 13.0 {
            ModelModule::set_joint_scale(tail_boma, Hash40::new("tail1"), &Vector3f{x: 1.05, y: 1.05, z: 1.05});
            ModelModule::set_joint_scale(tail_boma, Hash40::new("tail2"), &Vector3f{x: 1.05, y: 1.05, z: 1.05});
            ModelModule::set_joint_scale(tail_boma, Hash40::new("tail3"), &Vector3f{x: 1.05, y: 1.05, z: 1.05});
            ModelModule::set_joint_scale(tail_boma, Hash40::new("tail4"), &Vector3f{x: 1.05, y: 1.05, z: 1.05});
        }
        if frame == 14.0 {
            ModelModule::set_joint_scale(tail_boma, Hash40::new("tail1"), &Vector3f{x: 1.025, y: 1.025, z: 1.025});
            ModelModule::set_joint_scale(tail_boma, Hash40::new("tail2"), &Vector3f{x: 1.025, y: 1.025, z: 1.025});
            ModelModule::set_joint_scale(tail_boma, Hash40::new("tail3"), &Vector3f{x: 1.025, y: 1.025, z: 1.025});
            ModelModule::set_joint_scale(tail_boma, Hash40::new("tail4"), &Vector3f{x: 1.025, y: 1.025, z: 1.025});
        }
    }
    if MotionModule::is_end(boma) {
        if situation_kind != *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn pikachu_special_s_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn pikachu_special_s_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if collision_kind == *COLLISION_KIND_HIT {
            PLAY_SE(fighter, Hash40::new("se_pikachu_special_s02_l"));
        }
    }
    0.into()
}

unsafe extern "C" fn pikachu_special_s_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    WorkModule::off_flag(boma, *FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ENABLE_LANDING);
    ArticleModule::remove_exist(boma, FIGHTER_PIKACHU_GENERATE_ARTICLE_TAIL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ModelModule::set_joint_scale(boma, Hash40::new("tail1"), &Vector3f{x: 1.0, y: 1.0, z: 1.0});
    0.into()
}

unsafe extern "C" fn pikachu_special_s_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    WorkModule::off_flag(boma, *FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ENABLE_LANDING);
    ArticleModule::remove_exist(boma, FIGHTER_PIKACHU_GENERATE_ARTICLE_TAIL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    ModelModule::set_joint_scale(boma, Hash40::new("tail1"), &Vector3f{x: 1.0, y: 1.0, z: 1.0});
    0.into()
}

pub fn install() {
    Agent::new("pikachu")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, pikachu_special_s_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, pikachu_special_s_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, pikachu_special_s_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, pikachu_special_s_exec_status)
    .status(CheckAttack, *FIGHTER_STATUS_KIND_SPECIAL_S, pikachu_special_s_check_attack_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, pikachu_special_s_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_S, pikachu_special_s_exit_status)
    .install()
    ;
}