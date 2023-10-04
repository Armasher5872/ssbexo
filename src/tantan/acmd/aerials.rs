use super::*;

#[acmd_script( agent = "tantan", script = "game_attackshortstartr1", category = ACMD_GAME)]
unsafe fn ssbuexo_tantan_attackshortstartr1_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_COMBO_ENABLE);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_COMBO_ENABLE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_F);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_tantan_attackshortstartr1_acmd
    );
}