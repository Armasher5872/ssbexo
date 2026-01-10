use super::*;

unsafe extern "C" fn ssbexo_metaknight_down_special_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 66, 100, 100, 0, 13.0, 0.0, 6.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn ssbexo_metaknight_down_special_attack_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("metaknight_final_thunder_start"), Hash40::new("top"), 0, 32, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("metaknight_final_thunder"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("metaknight_final_thunder2"), Hash40::new("top"), 0, 9, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn ssbexo_metaknight_down_special_attack_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_metaknight_final01"));
    }
}

unsafe extern "C" fn ssbexo_metaknight_down_special_attack_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        VisibilityModule::set_status_default_int64(agent.module_accessor, hash40("mantle") as i64, hash40("mantle_wing") as i64);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attack_critical"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        VisibilityModule::set_status_default_int64(agent.module_accessor, hash40("mantle") as i64, hash40("mantle_normal") as i64);
    }
}

pub fn install() {
    Agent::new("metaknight")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallw", ssbexo_metaknight_down_special_attack_acmd, Low)
    .game_acmd("game_specialairlw", ssbexo_metaknight_down_special_attack_acmd, Low)
    .effect_acmd("effect_speciallw", ssbexo_metaknight_down_special_attack_effect, Low)
    .effect_acmd("effect_specialairlw", ssbexo_metaknight_down_special_attack_effect, Low)
    .sound_acmd("sound_speciallw", ssbexo_metaknight_down_special_attack_sound, Low)
    .sound_acmd("sound_specialairlw", ssbexo_metaknight_down_special_attack_sound, Low)
    .expression_acmd("expression_speciallw", ssbexo_metaknight_down_special_attack_expression, Low)
    .expression_acmd("expression_specialairlw", ssbexo_metaknight_down_special_attack_expression, Low)
    .install()
    ;
}