use super::*;

unsafe extern "C" fn koopajr_float_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_MOTION_FALL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, *FIGHTER_STATUS_ATTR_INTO_DOOR as u32, *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32, 0);
    0.into()
}

unsafe extern "C" fn koopajr_float_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    WorkModule::on_flag(boma, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT);
    WorkModule::set_int(boma, 60, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_INT_FLOAT_TIME);
    0.into()
}

unsafe extern "C" fn koopajr_float_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    EFFECT_FOLLOW(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("clownshaft3"), 0, 0, 0, 0, 0, 0, 0.7, true);
    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
    WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
    MotionModule::change_motion(boma, Hash40::new("fall"), 0.0, 1.5, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(koopajr_float_main_loop as *const () as _))
}

unsafe extern "C" fn koopajr_float_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let float_time = WorkModule::get_int(boma, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_INT_FLOAT_TIME);
    WorkModule::dec_int(boma, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_INT_FLOAT_TIME);
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_transition_group_check_air_attack().get_bool() {
        return 1.into();
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
    }
    if fighter.global_table[CURRENT_FRAME].get_f32() == 3.0 {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x, 0.0);
    }
    if MotionModule::is_end(boma) {
        MotionModule::change_motion(boma, Hash40::new("fall"), 0.0, 1.5, false, 0.0, false, false);
    }
    if float_time <= 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn koopajr_float_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    EffectModule::kill_kind(boma, Hash40::new("sys_erace_smoke"), false, true);
    0.into()
}

pub fn install() {
    Agent::new("koopajr")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_KOOPAJR_STATUS_KIND_FLOAT, koopajr_float_pre_status)
    .status(Init, *FIGHTER_KOOPAJR_STATUS_KIND_FLOAT, koopajr_float_init_status)
    .status(Main, *FIGHTER_KOOPAJR_STATUS_KIND_FLOAT, koopajr_float_main_status)
    .status(End, *FIGHTER_KOOPAJR_STATUS_KIND_FLOAT, koopajr_float_end_status)
    .install()
    ;
}