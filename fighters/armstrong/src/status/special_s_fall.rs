use super::*;

unsafe extern "C" fn armstrong_special_s_fall_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_AIR_STOP, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn armstrong_special_s_fall_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.14);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -6.0);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 6.0);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 1.2);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 1.2);
    sv_kinetic_energy!(set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.03);
    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.02);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x31697c2b98), hash40("catched_ganon"), hash40("catched_air_fall_ganon"));
    0.into()
}

unsafe extern "C" fn armstrong_special_s_fall_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_fall"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(armstrong_special_s_fall_loop as *const () as _))
}

unsafe extern "C" fn armstrong_special_s_fall_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn armstrong_special_s_fall_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn armstrong_special_s_fall_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END].contains(&status_kind) {
        CatchModule::catch_cut(fighter.module_accessor, false, false);
    }
    0.into()
}

unsafe extern "C" fn armstrong_special_s_fall_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END].contains(&status_kind) {
        CatchModule::catch_cut(fighter.module_accessor, false, false);
    }
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([8, 9, 10, 11, 12, 13, 14, 15].to_vec())
    .status(Pre, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL, armstrong_special_s_fall_pre_status)
    .status(Init, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL, armstrong_special_s_fall_init_status)
    .status(Main, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL, armstrong_special_s_fall_main_status)
    .status(Exec, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL, armstrong_special_s_fall_exec_status)
    .status(End, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL, armstrong_special_s_fall_end_status)
    .status(Exit, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL, armstrong_special_s_fall_exit_status)
    .install()
    ;
}