use super::*;

#[acmd_script( agent = "tantan", scripts = ["game_attacklongstartl1", "game_attacklongstartlb1", "game_attacklongstartr1", "game_attacklongstartrb1"], category = ACMD_GAME)]
unsafe fn ssbuexo_tantan_attacklongstart1_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.5, 3.5);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_START_HOLD);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_tantan_attacklongstart1_acmd
    );
}