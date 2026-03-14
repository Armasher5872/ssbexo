use super::*;

unsafe extern "C" fn armstrong_special_hi_throw_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let damage_multiplier = WorkModule::get_float(fighter.module_accessor, *FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x32e468d950), hash40("throw"), hash40("invalid"));
    AttackModule::set_power_up(fighter.module_accessor, damage_multiplier);
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_throw_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    armstrong_clear_charge(fighter.module_accessor);
    0.into()
}

unsafe extern "C" fn armstrong_special_hi_throw_exit_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    armstrong_clear_charge(fighter.module_accessor);
    if LinkModule::is_link(fighter.module_accessor, *LINK_NO_CAPTURE) {
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
    .status(Init, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, armstrong_special_hi_throw_init_status)
    .status(End, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, armstrong_special_hi_throw_end_status)
    .status(Exit, *FIGHTER_GANON_STATUS_KIND_SPECIAL_HI_THROW, armstrong_special_hi_throw_exit_status)
    .install()
    ;
}