use super::*;

//Grounded Neutral Special Attack ACMD
unsafe extern "C" fn ssbexo_luigi_grounded_neutral_special_attack_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 66, 120, 0, 35, 5.0, 0.0, 9.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 66, 120, 0, 35, 3.0, 0.0, 9.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 12.0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Aerial Neutral Special Attack ACMD
unsafe extern "C" fn ssbexo_luigi_aerial_neutral_special_attack_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 66, 120, 0, 35, 5.0, 0.0, 9.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 66, 120, 0, 35, 3.0, 0.0, 9.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 12.0, false);
        KineticModule::add_speed(boma, &Vector3f{x: -0.5, y: 0.0, z: 0.0});
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

//Grounded Neutral Special Attack Effect
unsafe extern "C" fn ssbexo_luigi_grounded_neutral_special_attack_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 5, 15, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let rand = &Vector3f{x: sv_math::randf(hash40("fighter"), 50.0), y: sv_math::randf(hash40("stage"), 50.0), z: sv_math::randf(hash40("luigi"), 50.0)};
        let flip = &Vector3f{x: if sv_math::randf(hash40("fighter"), 1.0) == 0.0 {-1.0} else {1.0}, y: if sv_math::randf(hash40("stage"), 1.0) == 0.0 {-1.0} else {1.0}, z: if sv_math::randf(hash40("luigi"), 1.0) == 0.0 {-1.0} else {1.0}};
        EFFECT_FOLLOW(agent, Hash40::new("sys_mball_beam"), Hash40::new("top"), 0.0, 9.0, 12.0, 0.0 + (rand.x*flip.x), 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_mball_beam"), Hash40::new("top"), 0.0, 9.0, 12.0, 120.0 + (rand.y*flip.y), 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_mball_beam"), Hash40::new("top"), 0.0, 9.0, 12.0, 240.0 + (rand.z*flip.z), 0, 0, 0.5, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0.25, 1.0, 0.7);
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 9.0, 12.0, 0, 90, 90, 0.4, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0.0, 9.0, 12.0, 0, 90, 90, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, true);
        LAST_EFFECT_SET_COLOR(agent, 0.2, 0.2, 0.2);
        LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

//Aerial Neutral Special Attack Effect
unsafe extern "C" fn ssbexo_luigi_aerial_neutral_special_attack_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 5, 15, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        let rand = &Vector3f{x: sv_math::randf(hash40("fighter"), 50.0), y: sv_math::randf(hash40("stage"), 50.0), z: sv_math::randf(hash40("luigi"), 50.0)};
        let flip = &Vector3f{x: if sv_math::randf(hash40("fighter"), 1.0) == 0.0 {-1.0} else {1.0}, y: if sv_math::randf(hash40("stage"), 1.0) == 0.0 {-1.0} else {1.0}, z: if sv_math::randf(hash40("luigi"), 1.0) == 0.0 {-1.0} else {1.0}};
        EFFECT_FOLLOW(agent, Hash40::new("sys_mball_beam"), Hash40::new("top"), 0.0, 9.0, 12.0, 0.0 + (rand.x*flip.x), 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_mball_beam"), Hash40::new("top"), 0.0, 9.0, 12.0, 120.0 + (rand.y*flip.y), 0, 0, 0.5, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_mball_beam"), Hash40::new("top"), 0.0, 9.0, 12.0, 240.0 + (rand.z*flip.z), 0, 0, 0.5, true);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        FLASH(agent, 0, 0.25, 1.0, 0.7);
        EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), 0.0, 9.0, 12.0, 0, 90, 90, 0.4, true);
        EFFECT_FOLLOW(agent, Hash40::new("sys_sp_flash"), Hash40::new("top"), 0.0, 9.0, 12.0, 0, 90, 90, 0.5, true);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

//Neutral Special Attack Sound
unsafe extern "C" fn ssbexo_luigi_neutral_special_attack_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 5.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_luigi_attack07"));
        PLAY_SE(agent, Hash40::new("se_common_electric_hit_l"));
    }
}

//Neutral Special Attack Expression
unsafe extern "C" fn ssbexo_luigi_neutral_special_attack_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_55_smash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialnattack", ssbexo_luigi_grounded_neutral_special_attack_acmd, Low)
    .game_acmd("game_specialairnattack", ssbexo_luigi_aerial_neutral_special_attack_acmd, Low)
    .effect_acmd("effect_specialnattack", ssbexo_luigi_grounded_neutral_special_attack_effect, Low)
    .effect_acmd("effect_specialairnattack", ssbexo_luigi_aerial_neutral_special_attack_effect, Low)
    .sound_acmd("sound_specialnattack", ssbexo_luigi_neutral_special_attack_sound, Low)
    .sound_acmd("sound_specialairnattack", ssbexo_luigi_neutral_special_attack_sound, Low)
    .expression_acmd("expression_specialnattack", ssbexo_luigi_neutral_special_attack_expression, Low)
    .expression_acmd("expression_specialairnattack", ssbexo_luigi_neutral_special_attack_expression, Low)
    .install()
    ;
}