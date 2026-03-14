use super::*;

//Down Special Loop ACMD
unsafe extern "C" fn ssbexo_luigi_down_special_loop_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 0.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.2, 180, 100, 30, 0, 6.0, 0.0, 6.0, 8.0, Some(0.0), Some(6.0), Some(16.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 5.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
}

//Down Special Loop Effect
unsafe extern "C" fn ssbexo_luigi_down_special_loop_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("luigi_final_vacuum"), true, true);
        EFFECT_FOLLOW(agent, Hash40::new("luigi_final_vacuum"), Hash40::new("top"), 0, 6, 8, 0, 0, 0, 1, true);
    }
}

//Down Special Loop Sound
unsafe extern "C" fn ssbexo_luigi_down_special_loop_sound(_agent: &mut L2CAgentBase) {}

//Down Special Loop Expression
unsafe extern "C" fn ssbexo_luigi_down_special_loop_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_none"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallwloop", ssbexo_luigi_down_special_loop_acmd, Low)
    .effect_acmd("effect_speciallwloop", ssbexo_luigi_down_special_loop_effect, Low)
    .sound_acmd("sound_speciallwloop", ssbexo_luigi_down_special_loop_sound, Low)
    .expression_acmd("expression_speciallwloop", ssbexo_luigi_down_special_loop_expression, Low)
    .install()
    ;
}