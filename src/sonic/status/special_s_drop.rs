use super::*;

unsafe extern "C" fn sonic_special_s_drop_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_FALL, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn sonic_special_s_drop_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 2.0);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 2.0);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.09);
    0.into()
}

unsafe extern "C" fn sonic_special_s_drop_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind == *SITUATION_KIND_GROUND {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_FLAG_SPECIAL_LW_HOLD);
        fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH.into(), false.into());
    }
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_drop"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_special_s_drop_main_loop as *const () as _))
}

unsafe extern "C" fn sonic_special_s_drop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if situation_kind == *SITUATION_KIND_GROUND
    && prev_situation_kind == *SITUATION_KIND_AIR {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_FLAG_SPECIAL_LW_HOLD);
        fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH.into(), false.into());
    }
    if frame >= 12.0 {
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -6.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -6.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -6.0);
    }
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
        fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_BOUNCE.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SONIC_STATUS_SPECIAL_S_DASH_FLAG_SPECIAL_LW_HOLD);
            fighter.change_status(FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn sonic_special_s_drop_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    lua_args!(fighter, Hash40::new("sys_machstamp"), true, true);
    sv_animcmd::EFFECT_OFF_KIND(fighter.lua_state_agent);
    0.into()
}

pub fn install() {
    Agent::new("sonic")
    .status(Pre, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DROP, sonic_special_s_drop_pre_status)
    .status(Init, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DROP, sonic_special_s_drop_init_status)
    .status(Main, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DROP, sonic_special_s_drop_main_status)
    .status(End, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DROP, sonic_special_s_drop_end_status)
    .install()
    ;
}