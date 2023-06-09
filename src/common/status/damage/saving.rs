/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

/*   DAMAGE STATUSES   */
//Crumple
#[common_status_script(status = FIGHTER_STATUS_KIND_SAVING_DAMAGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN, symbol = "_ZN7lua2cpp16L2CFighterCommon24status_SavingDamage_MainEv")]
unsafe fn status_saving_damage_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_SavingDamage_Main_common();
    if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_FURAFURA {
        fighter.sub_shift_status_main(L2CValue::Ptr(saving_status_loop as *const () as _));
    }
    L2CValue::I32(0)
}

pub unsafe fn saving_status_loop(fighter: &mut L2CFighterCommon) -> bool {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_FURAFURA {
        WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
        MotionModule::set_rate(boma, 0.4286);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_DOWN_WAIT.into(), true.into());
    }
    return false.into()
}

pub fn install() {
	install_status_scripts!(
        status_saving_damage_main
    );
}