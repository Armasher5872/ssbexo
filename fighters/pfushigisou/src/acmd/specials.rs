use super::*;

//Neutral Special ACMD
unsafe extern "C" fn ssbexo_pfushigisou_neutral_special_acmd(agent: &mut L2CAgentBase) {
    let speed = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(agent.module_accessor);
    frame(agent.lua_state_agent, 20.0);
    if !IS_EXIST_ARTICLE(agent, *FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_SLUDGE)  {
        if is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PFUSHIGISOU_GENERATE_ARTICLE_SLUDGE, false, -1);
        }
    }
    if is_excute(agent) {
        SET_SPEED_EX(agent, speed*lr, -1.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
}

//Grounded Neutral Special Effect
unsafe extern "C" fn ssbexo_pfushigisou_grounded_neutral_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("pfushigisou_tanemg"), Hash40::new("flower"), 5.7, 0, 0, 0, 0, 0, 1.3, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, -4, 2, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pfushigisou_leaf"), Hash40::new("flower"), 0, 0, 0, 0, 0, -90, 1, 0, 0, 0, 0, 360, 0, true);
    }
}

//Aerial Neutral Special Effect
unsafe extern "C" fn ssbexo_pfushigisou_aerial_neutral_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        EFFECT_FLW_POS(agent, Hash40::new("pfushigisou_tanemg"), Hash40::new("flower"), 5.7, 0, 0, 0, 0, 0, 1.3, true);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("pfushigisou_leaf"), Hash40::new("flower"), 0, 0, 0, 0, 0, -90, 1, 0, 0, 0, 0, 360, 0, true);
    }
}

//Neutral Special Sound
unsafe extern "C" fn ssbexo_pfushigisou_neutral_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_pfushigisou_special_n01"));
        PLAY_SE(agent, Hash40::new("se_pfushigisou_special_n03"));
    }
}

//Grounded Neutral Special Expression
unsafe extern "C" fn ssbexo_pfushigisou_grounded_neutral_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 44.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

//Aerial Neutral Special Expression
unsafe extern "C" fn ssbexo_pfushigisou_aerial_neutral_special_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Sludge Bomb ACMD
unsafe extern "C" fn ssbexo_pfushigisou_sludge_bomb_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 80, 100, 20, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_poison_param(agent.module_accessor, 0, 40, 5, 0.5, false);
    }
}

//Sludge Bomb Effect
unsafe extern "C" fn ssbexo_pfushigisou_sludge_bomb_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("packun_poison_gas"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 500, true);
    }
}

pub fn install() {
    Agent::new("pfushigisou")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialnstart", ssbexo_pfushigisou_neutral_special_acmd, Low)
    .game_acmd("game_specialairnstart", ssbexo_pfushigisou_neutral_special_acmd, Low)
    .effect_acmd("effect_specialnstart", ssbexo_pfushigisou_grounded_neutral_special_effect, Low)
    .effect_acmd("effect_specialairnstart", ssbexo_pfushigisou_aerial_neutral_special_effect, Low)
    .sound_acmd("sound_specialnstart", ssbexo_pfushigisou_neutral_special_sound, Low)
    .sound_acmd("sound_specialairnstart", ssbexo_pfushigisou_neutral_special_sound, Low)
    .expression_acmd("expression_specialnstart", ssbexo_pfushigisou_grounded_neutral_special_expression, Low)
    .expression_acmd("expression_specialairnstart", ssbexo_pfushigisou_aerial_neutral_special_expression, Low)
    .install()
    ;
    Agent::new("pfushigisou_sludge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_shoot", ssbexo_pfushigisou_sludge_bomb_acmd, Low)
    .effect_acmd("effect_shoot", ssbexo_pfushigisou_sludge_bomb_effect, Low)
    .install()
    ;
}