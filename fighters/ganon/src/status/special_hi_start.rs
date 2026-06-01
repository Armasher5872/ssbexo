use super::*;

unsafe extern "C" fn ganon_special_hi_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn ganon_special_hi_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if situation_kind == *SITUATION_KIND_AIR {
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_NONE);
        KineticModule::clear_speed_all(boma);
        KineticModule::unable_energy_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_STARTED_GROUNDED);
    }
    else {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        WorkModule::on_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_STARTED_GROUNDED);
    }
    0.into()
}

unsafe extern "C" fn ganon_special_hi_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let lr = PostureModule::lr(boma);
    let pos = PostureModule::pos(boma);
    let pos_x = if situation_kind == *SITUATION_KIND_AIR {(*pos).x} else {(*pos).x+(6.0*lr)};
    let effect = EffectModule::req(boma, Hash40::new("ganon_entry"), &Vector3f{x: pos_x, y: (*pos).y+15.0, z: 0.0}, &Vector3f::zero(), 1.0, 0, -1, false, 0);
    WorkModule::set_int(boma, effect as i32, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_hi"), L2CValue::Hash40s("special_air_hi"), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(ganon_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn ganon_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if !StatusModule::is_changing(boma) {
        if situation_kind == *SITUATION_KIND_GROUND
        && prev_situation_kind == *SITUATION_KIND_AIR {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_MOTION);
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_hi"), -1.0, 1.0, 0.0, false, false);
        }
        if situation_kind == *SITUATION_KIND_AIR
        && prev_situation_kind == *SITUATION_KIND_GROUND {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            MotionModule::change_motion_inherit_frame(boma, Hash40::new("special_air_hi"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_MOVE.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn ganon_special_hi_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let boma = fighter.module_accessor;
    let special_hi_hold_frame = WorkModule::get_int(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_HOLD_FRAME);
    let effect_handle = WorkModule::get_int(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    let mut stick = fighter.Vector2__create(stick_x.into(), stick_y.into());
    if stick["x"].get_f32().abs()+stick["y"].get_f32().abs() < 0.5 {
        stick["x"].assign(&L2CValue::F32(0.0));
        stick["y"].assign(&L2CValue::F32(1.0));
    }
    let normalize = fighter.Vector2__normalize(stick);
    let vec_stick_x = normalize["x"].get_f32();
    let vec_stick_y = normalize["y"].get_f32();
    let stick_angle = vec_stick_y.atan2(vec_stick_x);
    let stick_degrees = stick_angle.to_degrees();
    if current_frame >= 30.0 {
        WorkModule::set_int(boma, stick_degrees as i32, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_ROT_ANGLE);
    }
    else {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) && special_hi_hold_frame < 25 {
            WorkModule::inc_int(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_HOLD_FRAME);
        }
        if special_hi_hold_frame >= 25 && !WorkModule::is_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED) {
            MotionModule::set_rate(boma, 0.4375);
            WorkModule::on_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED);
        }
        if WorkModule::is_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED) {
            EffectModule::set_scale(boma, effect_handle as u32, &Vector3f{x: 0.5, y: 0.5, z: 0.5});
            EffectModule::set_rgb(boma, effect_handle as u32, 1.0, 0.0, 0.325);
        }
    }
    0.into()
}

unsafe extern "C" fn ganon_special_hi_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    let effect_handle = WorkModule::get_int(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    if ![*FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_MOVE, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING].contains(&status_kind) {
        WorkModule::set_int(boma, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_ROT_ANGLE);
        WorkModule::off_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED);
        WorkModule::off_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_STARTED_GROUNDED);
        WorkModule::set_int(boma, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    }
    EffectModule::kill(boma, effect_handle as u32, true, true);
    WorkModule::set_int(boma, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_HOLD_FRAME);
    0.into()
}

unsafe extern "C" fn ganon_special_hi_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    let effect_handle = WorkModule::get_int(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    if ![*FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_MOVE, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING].contains(&status_kind) {
        WorkModule::set_int(boma, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_ROT_ANGLE);
        WorkModule::off_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CHARGED);
        WorkModule::off_flag(boma, *FIGHTER_GANON_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_STARTED_GROUNDED);
        WorkModule::set_int(boma, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_EFFECT_HANDLE);
    }
    EffectModule::kill(boma, effect_handle as u32, true, true);
    WorkModule::set_int(boma, 0, *FIGHTER_GANON_INSTANCE_WORK_ID_INT_SPECIAL_HI_HOLD_FRAME);
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, ganon_special_hi_pre_status)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, ganon_special_hi_init_status)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, ganon_special_hi_main_status)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, ganon_special_hi_exec_status)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, ganon_special_hi_end_status)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_HI, ganon_special_hi_exit_status)
    .install()
    ;
}