use super::*;

//Up Special Loop ACMD
unsafe extern "C" fn ssbexo_metaknight_up_special_loop_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        ATTACK(agent, 0, 0, Hash40::new("body"), 9.0, 30, 60, 0, 60, 7.0, 0.0, 0.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 9.0, 30, 60, 0, 60, 7.0, 0.0, 4.0, 4.0, Some(0.0), Some(20.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        ATTACK(agent, 0, 0, Hash40::new("body"), 5.0, 361, 80, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 5.0, 361, 80, 0, 30, 3.0, 0.0, 4.0, 4.0, Some(0.0), Some(20.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        GroundModule::set_passable_check(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        GroundModule::set_passable_check(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
}

//Up Special Loop Effect
unsafe extern "C" fn ssbexo_metaknight_up_special_loop_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_shuttleloop1"), Hash40::new("top"), 0, -5, 2.5, 4, 0, 0, 1, true);
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 20, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 31.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_sword"), false, false);
    }
}

//Up Special Loop Sound
unsafe extern "C" fn ssbexo_metaknight_up_special_loop_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_metaknight_special_h01"));
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("vc_metaknight_special_h01"));
        PLAY_SE_REMAIN(agent, Hash40::new("se_metaknight_special_h02"));
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
       PLAY_STATUS(agent, Hash40::new("se_metaknight_special_h03"));
    }
}

pub fn install() {
    Agent::new("metaknight")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialhiloop", ssbexo_metaknight_up_special_loop_acmd, Low)
    .effect_acmd("effect_specialhiloop", ssbexo_metaknight_up_special_loop_effect, Low)
    .sound_acmd("sound_specialhiloop", ssbexo_metaknight_up_special_loop_sound, Low)
    .install()
    ;
}