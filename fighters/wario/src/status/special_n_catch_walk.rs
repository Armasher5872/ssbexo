use super::*;

//Neutral Special Catch Walk Pre Status
unsafe extern "C" fn wario_special_n_catch_walk_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

//Neutral Special Catch Walk Init Status
unsafe extern "C" fn wario_special_n_catch_walk_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    0.into()
}

//Neutral Special Catch Walk Main Status
unsafe extern "C" fn wario_special_n_catch_walk_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    grabbed_anim_selector(fighter, "barrel_screw", 0.0, 0.0);
    MotionModule::change_motion(boma, Hash40::new("special_n_catch_walk"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(wario_special_n_catch_walk_main_loop as *const () as _))
}

unsafe extern "C" fn wario_special_n_catch_walk_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let boma = fighter.module_accessor;
    let is_attack = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK);
    let lr = PostureModule::lr(boma);
    let x_stick = stick_x*lr;
    let walk_stick_x = WorkModule::get_param_float(boma, hash40("common"), hash40("walk_stick_x"));
    let run_stick_x = WorkModule::get_param_float(boma, hash40("common"), hash40("run_stick_x"));
    let jump_neutral_y = WorkModule::get_param_float(boma, hash40("common"), hash40("jump_neutral_y"));
    handle_mash(fighter);
    if x_stick > run_stick_x && is_attack {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_F.into(), false.into());
    }
    if x_stick < -run_stick_x && is_attack {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_B.into(), false.into());
    }
    if stick_y > jump_neutral_y && is_attack {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_HI.into(), false.into());
    }
    if stick_y < -jump_neutral_y && is_attack {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_THROW_LW.into(), false.into());
    }
    if x_stick < walk_stick_x && x_stick > -walk_stick_x {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WAIT.into(), false.into());
    }
    if x_stick < -walk_stick_x {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_TURN.into(), false.into());
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WAIT.into(), false.into());
    }
    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() || fighter.sub_transition_group_check_ground_jump().get_bool() {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_JUMP.into(), false.into());
        }
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WALK.into(), false.into());
        return 1.into();
    }
    0.into()
}

//Neutral Special Catch Walk Exec Status
unsafe extern "C" fn wario_special_n_catch_walk_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//Neutral Special Catch Walk End Status
unsafe extern "C" fn wario_special_n_catch_walk_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    wario_special_n_end(fighter, false);
    0.into()
}

//Neutral Special Catch Walk Exit Status
unsafe extern "C" fn wario_special_n_catch_walk_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    wario_special_n_exit(fighter, false);
    0.into()
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WALK, wario_special_n_catch_walk_pre_status)
    .status(Init, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WALK, wario_special_n_catch_walk_init_status)
    .status(Main, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WALK, wario_special_n_catch_walk_main_status)
    .status(Exec, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WALK, wario_special_n_catch_walk_exec_status)
    .status(End, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WALK, wario_special_n_catch_walk_end_status)
    .status(Exit, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_CATCH_WALK, wario_special_n_catch_walk_exit_status)
    .install()
    ;
}