use super::*;

//Neutral Special Start ACMD
unsafe extern "C" fn ssbexo_snake_neutral_special_start_acmd(agent: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CORRECT(agent, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        if GRENADE_HOLD[entry_id] != true {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, false, -1);
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, false, -1);
            ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, true, smash::app::ArticleOperationTarget(0));
        }
        else {
            ItemModule::have_item(agent.module_accessor, smash::app::ItemKind(*ITEM_KIND_SMOKESCREEN), 0, 0, false, false);
        }
        macros::CORRECT(agent, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
    frame(agent.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(agent, 0.5);
}

//C4 ACMD
unsafe extern "C" fn ssbexo_snake_c4_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("rot"), 17.0, 80, 90, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        macros::ATTACK(agent, 1, 0, Hash40::new("rot"), 13.0, 60, 90, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        macros::ATTACK(agent, 2, 0, Hash40::new("rot"), 9.0, 40, 90, 0, 40, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        VisibilityModule::set_whole(agent.module_accessor, false);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, 0);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::set_size(agent.module_accessor, 0, 9.0);
        AttackModule::set_size(agent.module_accessor, 1, 14.0);
        AttackModule::set_size(agent.module_accessor, 2, 21.0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosion"), 0, false, 0);
    }
    frame(agent.lua_state_agent, 2.0);
    if WorkModule::is_flag(agent.module_accessor, *WEAPON_SNAKE_C4_INSTANCE_WORK_ID_FLAG_GROUND) == false {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 3.0);
    if WorkModule::is_flag(agent.module_accessor, *WEAPON_SNAKE_C4_INSTANCE_WORK_ID_FLAG_GROUND) == true {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install() {
    Agent::new("snake")
    .game_acmd("game_specialnstart", ssbexo_snake_neutral_special_start_acmd, Priority::Low)
    .game_acmd("game_specialairnstart", ssbexo_snake_neutral_special_start_acmd, Priority::Low)
    .install()
    ;
    Agent::new("snake_c4")
    .game_acmd("game_explosion", ssbexo_snake_c4_acmd, Priority::Low)
    .install()
    ;
}