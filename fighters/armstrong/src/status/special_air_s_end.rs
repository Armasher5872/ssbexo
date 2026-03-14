use super::*;

unsafe extern "C" fn armstrong_special_air_s_end_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK_ABSOLUTE);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn armstrong_special_air_s_end_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x32e468d950), hash40("catched_ganon"), hash40("catched_air_end_ganon"));
    0.into()
}

unsafe extern "C" fn armstrong_special_air_s_end_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charge_frames = WorkModule::get_int(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    let damage_multiplier = WorkModule::get_float(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
    if charge_frames > 0 {
        AttackModule::set_power_up(fighter.module_accessor, damage_multiplier);
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(armstrong_special_air_s_end_loop as *const () as _))
}

unsafe extern "C" fn armstrong_special_air_s_end_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn armstrong_special_air_s_end_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn armstrong_special_air_s_end_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_NANOMACHINES);
    armstrong_clear_charge(fighter.module_accessor);
    0.into()
}

unsafe extern "C" fn armstrong_special_air_s_end_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_NANOMACHINES);
    armstrong_clear_charge(fighter.module_accessor);
    CatchModule::catch_cut(fighter.module_accessor, false, false);
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([8, 9, 10, 11, 12, 13, 14, 15].to_vec())
    .status(Pre, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END, armstrong_special_air_s_end_pre_status)
    .status(Init, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END, armstrong_special_air_s_end_init_status)
    .status(Main, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END, armstrong_special_air_s_end_main_status)
    .status(Exec, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END, armstrong_special_air_s_end_exec_status)
    .status(End, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END, armstrong_special_air_s_end_end_status)
    .status(Exit, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END, armstrong_special_air_s_end_exit_status)
    .install()
    ;
}