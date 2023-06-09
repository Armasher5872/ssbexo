use super::*;

#[status_script(agent = "trail", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn trail_attack_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    attack_air_main_status(fighter)
}

pub fn install() {
    install_status_scripts!(
        trail_attack_air_main_status
    );
}