use super::*;

unsafe extern "C" fn armstrong_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let special_hi_lr_stick_x = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_lr_stick_x"));
    let special_hi_fall_x_mul = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_fall_x_mul"));
    let special_hi_landing_frame = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_landing_frame"));
    let special_hi_speed_coef = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("special_hi_speed_coef"));
    WorkModule::set_int64(boma, hash40("special_hi") as i64, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND);
    WorkModule::set_int64(boma, hash40("special_air_hi") as i64, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND_AIR);
    WorkModule::set_float(boma, special_hi_lr_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_LR_STICK_X);
    WorkModule::set_float(boma, special_hi_fall_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_FALL_X_MUL);
    WorkModule::set_int(boma, special_hi_landing_frame as i32, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_CONST_LANDING_FRAME);
    WorkModule::set_float(boma, special_hi_speed_coef, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
    WorkModule::set_int(boma, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    fighter.super_jump_punch(L2CValue::Ptr(armstrong_special_hi_super_jump_punch as *const () as _));
    if situation_kind != *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GANON_SPECIAL_HI);
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE);
    }
    else {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(armstrong_special_hi_loop as *const () as _))
}

unsafe extern "C" fn armstrong_special_hi_super_jump_punch(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32().abs();
    let boma = fighter.module_accessor;
    let sjp_stick_x = WorkModule::get_float(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_LR_STICK_X);
    if bool_check.get_bool() {
        return 0.into();
    }
    if !WorkModule::is_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE) {
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS) {
            WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_CHANGE_KINE);
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GANON_SPECIAL_HI);
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
        }
    }
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        if sjp_stick_x < stick_x {
            PostureModule::set_stick_lr(boma, 0.0);
            PostureModule::update_rot_y_lr(boma);
        }
    }
    if WorkModule::is_flag(boma, *FIGHTER_GANON_STATUS_SPECIAL_HI_FLAG_IS_CHECK_DIVE) {
        fighter.sub_air_check_dive();
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            let motion_speed_x = {fighter.clear_lua_stack(); lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION); sv_kinetic_energy::get_speed_x(fighter.lua_state_agent)};
            sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, motion_speed_x, 0.0);
        }
    }
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if situation_kind == *SITUATION_KIND_GROUND {
            armstrong_charge_move(fighter, 4.0, 8.0, 0.01, 7.0, ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL), true, "handl");
        }
        else {
            armstrong_charge_move(fighter, 4.0, 8.0, 0.01, 0.0, ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL), false, "handl");
        }
        fighter.super_jump_punch_main();
    }
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_NANOMACHINES);
    effect!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("armstrong_flame_grab_hold"), false, true);
    fighter.super_jump_punch_reset_common_condition();
    if status_kind != *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW {
        armstrong_clear_charge(boma);
    }
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_NANOMACHINES);
    if status_kind != *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW {
        armstrong_clear_charge(boma);
    }
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_armstrong_costumes_acmd())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, armstrong_special_hi_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, armstrong_special_hi_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, armstrong_special_hi_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, armstrong_special_hi_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, armstrong_special_hi_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_HI, armstrong_special_hi_exit_status)
    .install()
    ;
}