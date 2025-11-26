use super::*;

//Aerial Down Special Loop ACMD
unsafe extern "C" fn ssbexo_wario_aerial_down_special_loop_acmd(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STRONG) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("hip"), 19.6, 51, 65, 0, 80, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("hip"), 14.2, 51, 65, 0, 44, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
    }
}

//Aerial Down Special Loop Effect
unsafe extern "C" fn ssbexo_wario_aerial_down_special_loop_effect(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STRONG) {
	    if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("wario_attack_dash"), Hash40::new("top"), 0, 0, 0, 90, 0, 0, 0.8, true);
            LAST_EFFECT_SET_COLOR(agent, 0.88, 0.35, 0.13);
            agent.clear_lua_stack();
            lua_args!(agent, 0.15, 0.08, 1.0, 0.5);
            sv_animcmd::FLASH_NO_STOP(agent.lua_state_agent);
        }
    }
    else {
	    if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("wario_attack_dash"), Hash40::new("top"), 0, 0, 0, 90, 0, 0, 0.6, true);
            LAST_EFFECT_SET_COLOR(agent, 1.0, 1.0, 0.0);
        }
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 3.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_STRONG) {
	    if is_excute(agent) {
            agent.clear_lua_stack();
            lua_args!(agent, 1.0, 1.0, 1.0, 0.5);
            sv_animcmd::FLASH_NO_STOP(agent.lua_state_agent);
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
    }
}

//Aerial Down Special Loop Sound
unsafe extern "C" fn ssbexo_wario_aerial_down_special_loop_sound(_agent: &mut L2CAgentBase) {}

//Aerial Down Special Loop Expression
unsafe extern "C" fn ssbexo_wario_aerial_down_special_loop_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("wario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialairlwloop", ssbexo_wario_aerial_down_special_loop_acmd, Low)
    .effect_acmd("effect_specialairlwloop", ssbexo_wario_aerial_down_special_loop_effect, Low)
    .sound_acmd("sound_specialairlwloop", ssbexo_wario_aerial_down_special_loop_sound, Low)
    .expression_acmd("expression_specialairlwloop", ssbexo_wario_aerial_down_special_loop_expression, Low)
    .install()
    ;
}