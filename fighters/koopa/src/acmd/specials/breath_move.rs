use super::*;

//Fire Breath Move ACMD
unsafe extern "C" fn ssbexo_koopa_firebreath_move_acmd(agent: &mut L2CAgentBase) {
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 361, 40, 0, 30, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(boma);
    }
}

//Fire Breath Move Effect
unsafe extern "C" fn ssbexo_koopa_firebreath_move_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, true);
            EFFECT_FOLLOW(agent, Hash40::new("koopa_breath_m_fire"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
        wait(lua_state, 15.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, true);
        }
        wait(lua_state, 15.0);
    }
}

//Fire Breath Move Sound
unsafe extern "C" fn ssbexo_koopa_firebreath_move_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_koopa_special_n02"));
    }
}

pub fn install() {
    Agent::new("koopa_breath")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_move", ssbexo_koopa_firebreath_move_acmd, Low)
    .acmd("effect_move", ssbexo_koopa_firebreath_move_effect, Low)
    .acmd("sound_move", ssbexo_koopa_firebreath_move_sound, Low)
    .install()
    ;
}