use super::*;

unsafe extern "C" fn snake_special_n_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let itemmanager = smash2::app::ItemManager::instance().unwrap();
    let grenade_count = smash2::app::ItemManager::get_num_of_ownered_item(itemmanager, fighter.battle_object_id, smash2::app::ItemKind::Snakegrenade);
    if ItemModule::is_have_item(boma, 0) {
        ItemModule::drop_item(boma, 90.0, 0.0, 0);
    }
    if !ArticleModule::is_generatable(boma, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE)
    || grenade_count >= 2 {
        fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_SPECIAL_N_THROW.into(), false.into());
        return 1.into();
    }
    else {
        WorkModule::set_float(boma, -1.0, *FIGHTER_SNAKE_STATUS_SPECIAL_N_WORK_FLOAT_THROW_RATE);
        if !StopModule::is_stop(boma) {
            fun_710001c850(fighter, false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(fun_710001c850 as *const () as _));
        WorkModule::set_int64(boma, hash40("special_n_start") as i64, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
        WorkModule::set_int64(boma, hash40("special_air_n_start") as i64, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);
        fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_n_start"), L2CValue::Hash40s("special_air_n_start"), false.into());
        if situation_kind == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        else {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(snake_special_n_main_loop as *const () as _));
    }
    0.into()
}

unsafe extern "C" fn fun_710001c850(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    let boma = fighter.module_accessor;
    if !bool_check.get_bool() {
        if ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::on_flag(boma, *FIGHTER_SNAKE_STATUS_SPECIAL_N_FLAG_BUTTON_SPECIAL_OFF);
            fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::I32(0));
            fighter.global_table[SUB_STATUS].assign(&L2CValue::I32(0));
        }
    }
    0.into()
}

unsafe extern "C" fn snake_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let mot_kind = WorkModule::get_int64(boma, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
    let mot_air_kind = WorkModule::get_int64(boma, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);
    if !StatusModule::is_changing(boma) {
        if prev_situation_kind == *SITUATION_KIND_GROUND 
        && situation_kind == *SITUATION_KIND_AIR {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(boma, Hash40::new_raw(mot_air_kind), -1.0, 1.0, 0.0, false, false);
        }
        if prev_situation_kind == *SITUATION_KIND_AIR
        && situation_kind == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion_inherit_frame(boma, Hash40::new_raw(mot_kind), -1.0, 1.0, 0.0, false, false);
        }
    }
    if MotionModule::is_end(boma) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("snake")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, snake_special_n_main_status)
    .install()
    ;
}