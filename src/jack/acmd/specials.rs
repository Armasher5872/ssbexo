use super::*;

//Eiha Hit ACMD
unsafe extern "C" fn ssbexo_jack_eiha_hit_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 75, 100, 0, 20, 8.5, 0.0, 1.5, 0.0, Some(0.0), Some(3.5), Some(0.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(agent.module_accessor, 0, 361, 45, 1.0, false);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Grounded Eiagon Fire ACMD
unsafe extern "C" fn ssbexo_jack_grounded_eiagon_fire_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            WorkModule::sub_float(agent.module_accessor, 5.0, 0x4D);
        }
    }
}

//Aerial Eiagon Fire ACMD
unsafe extern "C" fn ssbexo_jack_aerial_eiagon_fire_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            WorkModule::sub_float(agent.module_accessor, 5.0, 0x4D);
        }
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_ENABLE_CONTROL_ENERGY);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_S_FLAG_SET_FALL_NORMAL);
    }
}

//Eiagon Hit ACMD
unsafe extern "C" fn ssbexo_jack_eiagon_hit_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 367, 100, 0, 5, 4.0, 0.0, 1.5, 0.0, Some(0.0), Some(8.0), Some(0.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(agent.module_accessor, 0, 321, 40, 1.5, false);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 367, 100, 0, 5, 4.0, 0.0, 1.5, 0.0, Some(0.0), Some(8.0), Some(0.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(agent.module_accessor, 0, 321, 40, 1.5, false);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 367, 100, 0, 5, 4.0, 0.0, 1.5, 0.0, Some(0.0), Some(8.0), Some(0.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(agent.module_accessor, 0, 321, 40, 1.5, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 73, 60, 0, 105, 4.0, 0.0, 1.5, 0.0, Some(0.0), Some(8.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(agent.module_accessor, 0, 321, 40, 1.5, false);
    }
}

//Wings of Rebellion Start ACMD
unsafe extern "C" fn ssbexo_jack_wings_of_rebellion_start_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WING, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WING, Hash40::new("special_hi2"), false, -1.0);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(agent) {
            WorkModule::sub_float(agent.module_accessor, 8.0, 0x4D);
        }
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_HI2_FLAG_REVERSE_LR);
    }
}

pub fn install() {
    Agent::new("jack")
    .game_acmd("game_specials2", ssbexo_jack_grounded_eiagon_fire_acmd, Priority::Low)
    .game_acmd("game_specialairs2", ssbexo_jack_aerial_eiagon_fire_acmd, Priority::Low)
    .game_acmd("game_specialhistart", ssbexo_jack_wings_of_rebellion_start_acmd, Priority::Low)
    .game_acmd("game_specialairhistart", ssbexo_jack_wings_of_rebellion_start_acmd, Priority::Low)
    .install()
    ;
    Agent::new("jack_fire")
    .game_acmd("game_hit", ssbexo_jack_eiha_hit_acmd, Priority::Low)
    .install()
    ;
    Agent::new("jack_fire2")
    .game_acmd("game_hit", ssbexo_jack_eiagon_hit_acmd, Priority::Low)
    .install()
    ;
}