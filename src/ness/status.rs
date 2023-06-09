use super::*;

#[status_script(agent = "ness", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn ness_attack_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_air_main_status(fighter)
}

pub fn install() {
    install_status_scripts!(
        ness_attack_air_main_status
    );
}