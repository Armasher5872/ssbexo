use super::*;

unsafe extern "C" fn armstrong_special_hi_cling_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_NONE as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, true, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_cling_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_cling_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_catch"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(armstrong_special_hi_cling_loop as *const () as _))
}

unsafe extern "C" fn armstrong_special_hi_cling_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_cling_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_cling_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind != *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW {
        armstrong_clear_charge(fighter.module_accessor);
        effect!(fighter, *MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("armstrong_flame_grab"), false, true);
    }
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_cling_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if ![*FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, *FIGHTER_STATUS_KIND_CATCH_CUT].contains(&status_kind) {
        if status_kind != *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW {
            armstrong_clear_charge(fighter.module_accessor);
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_CATCH_CLING_CUT);
        sv_module_access::_catch(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([8, 9, 10, 11, 12, 13, 14, 15].to_vec())
    .status(Pre, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, armstrong_special_hi_cling_pre_status)
    .status(Init, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, armstrong_special_hi_cling_init_status)
    .status(Main, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, armstrong_special_hi_cling_main_status)
    .status(Exec, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, armstrong_special_hi_cling_exec_status)
    .status(End, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, armstrong_special_hi_cling_end_status)
    .status(Exit, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_CLING, armstrong_special_hi_cling_exit_status)
    .install()
    ;
}