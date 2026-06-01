use super::*;

unsafe extern "C" fn armstrong_special_hi_throw_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_AIR_STOP, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_throw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    sv_kinetic_energy!(set_accel_x_mul, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.03);
    sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.02);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.07);
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 1.0);
    sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 1.0);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x32e468d950), Hash40::new("throw"), Hash40::new("invalid"));
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_throw_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let charge_frames = WorkModule::get_int(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
    let damage_multiplier = WorkModule::get_float(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
    if charge_frames > 0 {
        AttackModule::set_power_up(boma, damage_multiplier);
    }
    if !StopModule::is_stop(boma) {
        armstrong_special_hi_throw_sub_status(fighter, false.into());
    }
    MotionModule::change_motion(boma, Hash40::new("special_hi_throw"), 0.0, 1.0, false, 0.0, false, false);
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(armstrong_special_hi_throw_sub_status as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(armstrong_special_hi_throw_main_loop as *const () as _))
}

unsafe extern "C" fn armstrong_special_hi_throw_sub_status(fighter: &mut L2CFighterCommon, bool_check: L2CValue) -> L2CValue {
    let boma = fighter.module_accessor;
    if !bool_check.get_bool() {
        if !WorkModule::is_flag(boma, *FIGHTER_GANON_STATUS_SPECIAL_HI_THROW_FLAG_CHANGE_KINE) {
            if WorkModule::is_flag(boma, *FIGHTER_GANON_STATUS_SPECIAL_HI_THROW_FLAG_FALL) {
                WorkModule::on_flag(boma, *FIGHTER_GANON_STATUS_SPECIAL_HI_THROW_FLAG_CHANGE_KINE);
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GANON_SPECIAL_HI_THROW_FALL);
                WorkModule::enable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(boma) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(boma) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_throw_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_throw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    armstrong_clear_charge(boma);
    WorkModule::off_flag(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_THROW);
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_throw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    armstrong_clear_charge(boma);
    if LinkModule::is_link(boma, *LINK_NO_CAPTURE) {
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_CATCH_CLING_CUT);
        sv_module_access::_catch(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
    WorkModule::off_flag(boma, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_THROW);
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume(get_armstrong_costumes_acmd())
    .status(Pre, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, armstrong_special_hi_throw_pre_status)
    .status(Init, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, armstrong_special_hi_throw_init_status)
    .status(Main, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, armstrong_special_hi_throw_main_status)
    .status(Exec, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, armstrong_special_hi_throw_exec_status)
    .status(End, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, armstrong_special_hi_throw_end_status)
    .status(Exit, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, armstrong_special_hi_throw_exit_status)
    .install()
    ;
}