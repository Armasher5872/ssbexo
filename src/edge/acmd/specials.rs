use super::*;

//Flare Charge ACMD
#[acmd_script( agent = "edge", scripts = ["game_specialnstart", "game_specialairnstart"], category = ACMD_GAME )]
unsafe fn ssbuexo_sephiroth_flare_charge_acmd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_EDGE_SPECIAL_N_S, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_EDGE_SPECIAL_N_M, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(fighter.lua_state_agent, 79.0);
    MotionModule::set_rate(fighter.module_accessor, 0.8);
    frame(fighter.lua_state_agent, 99.0);
    MotionModule::set_rate(fighter.module_accessor, 1.0);
    frame(fighter.lua_state_agent, 100.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_EDGE_SPECIAL_N_L, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(fighter.lua_state_agent, 105.0);
    MotionModule::set_rate(fighter.module_accessor, 0.625);
    frame(fighter.lua_state_agent, 115.0);
    MotionModule::set_rate(fighter.module_accessor, 1.0);
    frame(fighter.lua_state_agent, 120.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_EDGE_GENERATE_ARTICLE_FIRE, false, 0);
    }
    frame(fighter.lua_state_agent, 140.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    }
}

//Flare ACMD
#[acmd_script( agent = "edge_fire", script = "game_specialn1", category = ACMD_GAME )]
unsafe fn ssbuexo_sephiroth_flare_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::disable_tip(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 82, 15, 0, 55, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, -2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::set_size(fighter.module_accessor, 0, 4.5);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::set_size(fighter.module_accessor, 0, 5.0);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        AttackModule::set_size(fighter.module_accessor, 0, 5.5);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_sephiroth_flare_acmd,
        ssbuexo_sephiroth_flare_charge_acmd
    );
}