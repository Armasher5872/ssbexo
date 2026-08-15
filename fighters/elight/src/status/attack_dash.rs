use super::*;

unsafe extern "C" fn elight_attack_dash_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_DASH)(fighter);
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    ret
}

pub fn install() {
    Agent::new("elight")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_DASH, elight_attack_dash_end_status)
    .install()
    ;
}