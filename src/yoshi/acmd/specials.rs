use super::*;

//Side Special Start ACMD
#[acmd_script( agent = "yoshi", script = "game_specialairsstart", category = ACMD_GAME)]
unsafe fn ssbuexo_yoshi_side_special_start_acmd(fighter: &mut L2CAgentBase) {
    MotionModule::set_rate(fighter.module_accessor, 1.333);
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_YOSHI_STATUS_SPECIAL_S_FLAG_DISP_EGG);
    }
}

//Side Special Loop ACMD
#[acmd_script( agent = "yoshi", scripts = ["game_specialairsloop", "game_specialsloop"], category = ACMD_GAME)]
unsafe fn ssbuexo_yoshi_side_special_loop_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 80, 50, 0, 43, 8.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 32, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        JostleModule::set_status(fighter.module_accessor, false);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_yoshi_side_special_start_acmd,
        ssbuexo_yoshi_side_special_loop_acmd
    );
}