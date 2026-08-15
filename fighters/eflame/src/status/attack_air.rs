use super::*;

unsafe extern "C" fn eflame_attack_air_end_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter);
    let boma = fighter.module_accessor;
    WorkModule::off_flag(boma, *FIGHTER_ELEMENT_INSTANCE_WORK_ID_FLAG_CAN_BLADE_SWITCH);
    ret
}

pub fn install() {
    Agent::new("eflame")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_AIR, eflame_attack_air_end_status)
    .install()
    ;
}