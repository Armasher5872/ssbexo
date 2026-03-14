use super::*;

//Finale Fireball Regular ACMD
unsafe extern "C" fn ssbexo_mario_finale_fireball_regular_acmd(agent: &mut L2CAgentBase) {
    let owner_boma = get_owner_boma(agent);
    if WorkModule::is_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE) {
        frame(agent.lua_state_agent, 60.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("fire1"), 2.5, 50, 40, 0, 25, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("fire2"), 2.5, 48, 40, 0, 25, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 100.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("fire1"), 3.0, 23, 90, 0, 20, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("fire2"), 3.0, 20, 90, 0, 20, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 2, 0, Hash40::new("fire1"), 3.0, 50, 100, 0, 15, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 3, 0, Hash40::new("fire2"), 3.0, 48, 100, 0, 15, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 150.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("fire1"), 3.5, 23, 120, 0, 10, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("fire2"), 3.5, 20, 120, 0, 10, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 2, 0, Hash40::new("fire1"), 3.5, 50, 130, 0, 5, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 3, 0, Hash40::new("fire2"), 3.5, 48, 130, 0, 5, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 220.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    else {
        frame(agent.lua_state_agent, 60.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("fire1"), 2.5, 50, 40, 0, 25, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("fire2"), 2.5, 48, 40, 0, 25, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 100.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("fire1"), 3.0, 23, 90, 0, 20, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("fire2"), 3.0, 20, 90, 0, 20, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 2, 0, Hash40::new("fire1"), 3.0, 50, 100, 0, 15, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 3, 0, Hash40::new("fire2"), 3.0, 48, 100, 0, 15, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 150.0);
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("fire1"), 3.5, 23, 120, 0, 10, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 1, 0, Hash40::new("fire2"), 3.5, 20, 120, 0, 10, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 2, 0, Hash40::new("fire1"), 3.5, 50, 130, 0, 5, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            ATTACK(agent, 3, 0, Hash40::new("fire2"), 3.5, 48, 130, 0, 5, 16.0, 0.0, -3.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 0.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        frame(agent.lua_state_agent, 220.0);
        if is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
}

pub fn install() {
    Agent::new("mario_hugeflame")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_regular", ssbexo_mario_finale_fireball_regular_acmd, Low)
    .install()
    ;
}