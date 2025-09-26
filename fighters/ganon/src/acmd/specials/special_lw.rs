use super::*;

//Grounded Down Special ACMD
unsafe extern "C" fn ssbexo_ganon_grounded_down_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 3.0, 6.0, 8.5, 9.5);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 2.0, 6.0, 8.5, 10.0);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("shoulderr"), 14.0, 45, 65, 0, 65, 5.0, -1.1, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("armr"), 14.0, 45, 65, 0, 65, 5.0, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 16.0, 45, 65, 0, 65, 5.0, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        JostleModule::set_status(agent.module_accessor, false);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_VOLLEY, false, -1);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK);
    }
    frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 8.0, 8.0, 8.0, 4.0);
    }
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
}

//Grounded Down Special Effect
unsafe extern "C" fn ssbexo_ganon_grounded_down_special_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -5.5, 4.0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, 2, 0.5, 1);
        sv_animcmd::EFFECT_COLOR(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("ganon_rekkikyaku"), Hash40::new("top"), 0, 3, 10, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Grounded Down Special Sound
unsafe extern "C" fn ssbexo_ganon_grounded_down_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_dash_start"));
    }
    wait(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_ganon_special_l01"));
        PLAY_SE(agent, Hash40::new("vc_ganon_special_l01"));
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        let boar = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ganon_final04"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, boar as i32, 0.5, 0);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallw", ssbexo_ganon_grounded_down_special_acmd, Low)
    .effect_acmd("effect_speciallw", ssbexo_ganon_grounded_down_special_effect, Low)
    .sound_acmd("sound_speciallw", ssbexo_ganon_grounded_down_special_sound, Low)
    .install()
    ;
}