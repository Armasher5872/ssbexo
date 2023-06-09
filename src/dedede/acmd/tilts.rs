use super::*;

//Forward Tilt ACMD
#[acmd_script( agent = "dedede", script = "game_attacks3s", category = ACMD_GAME)]
unsafe fn ssbuexo_dedede_ftilt_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("hammer1"), 2.0, 22, 100, 60, 0, 3.0, -8.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        macros::ATTACK(fighter, 1, 0, Hash40::new("hammer1"), 2.0, 22, 100, 50, 0, 3.5, 1.0, 0.0, 0.0, Some(8.0), Some(0.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 100, 100, 30, 0, 3.0, 0.0, 4.0, 19.2, Some(0.0), Some(4.0), Some(25.8), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 250, 100, 26, 0, 4.0, 0.0, 9.5, 20.0, Some(0.0), Some(9.5), Some(25.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("hammer1"), 3.0, 50, 190, 0, 30, 3.5, -8.0, 0.0, 0.0, Some(-2.0), Some(0.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        macros::ATTACK(fighter, 1, 0, Hash40::new("hammer1"), 3.0, 50, 190, 0, 50, 3.5, 1.0, 0.0, 0.0, Some(8.0), Some(0.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        macros::ATTACK(fighter, 2, 0, Hash40::new("hammer2"), 3.0, 50, 190, 0, 50, 8.3, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        macros::ATTACK(fighter, 3, 0, Hash40::new("hammer1"), 3.0, 50, 190, 0, 30, 3.5, 1.0, 0.0, 0.0, Some(8.0), Some(0.0), Some(0.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
        macros::ATTACK(fighter, 4, 0, Hash40::new("hammer2"), 3.0, 50, 190, 0, 30, 8.3, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_dedede_hammer"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEDEDE, *ATTACK_REGION_HAMMER);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_dedede_ftilt_acmd
    );
}