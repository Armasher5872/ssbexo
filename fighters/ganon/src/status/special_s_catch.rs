use super::*;

unsafe extern "C" fn ganon_special_s_catch_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    fighter.clear_lua_stack();
    lua_args!(fighter, *MA_MSC_CMD_CATCH_SET_CATCH);
    sv_module_access::_catch(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    KineticModule::clear_speed_all(boma);
    WorkModule::off_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_SET_THROW_WORK);
    WorkModule::off_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_SET_FALL);
    WorkModule::off_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_SET_FALL_CHECKED);
    WorkModule::off_flag(boma, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_EXPLOSION_GET_TARGET_TASK_ID);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_GANON_STATUS_WORK_ID_INT_EXPLOSION_TARGET_TASK_ID);
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Init, *FIGHTER_GANON_STATUS_KIND_SPECIAL_S_CATCH, ganon_special_s_catch_init_status)
    .install()
    ;
}