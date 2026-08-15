use super::*;

unsafe extern "C" fn simon_attack_air_exec_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_uniq_process_exec()
}

unsafe extern "C" fn simon_attack_air_check_attack_status(_fighter: &mut L2CFighterCommon, _param_2: &L2CValue, _param_3: &L2CValue) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("simon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Exec, *FIGHTER_STATUS_KIND_ATTACK_AIR, simon_attack_air_exec_status)
    .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_AIR, simon_attack_air_check_attack_status)
    .install()
    ;
}