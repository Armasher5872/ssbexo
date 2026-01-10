use super::*;

//Up Special ACMD
unsafe extern "C" fn ssbexo_metaknight_up_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        HitModule::set_whole(agent.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        HitModule::set_whole(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 70, 50, 0, 90, 6.8, 0.0, 12.0, 12.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 80, 50, 0, 90, 6.8, 0.0, 0.0, 12.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 80, 30, 0, 70, 6.0, 0.0, 14.0, 12.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("top"), 6.0, 80, 30, 0, 70, 6.0, 0.0, 2.0, 12.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        GroundModule::set_passable_check(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        HitModule::set_whole(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        damage!(agent, MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        ATTACK(agent, 0, 0, Hash40::new("body"), 6.0, 45, 100, 0, 0, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 50, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        GroundModule::set_passable_check(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
}

//Up Special Effect
unsafe extern "C" fn ssbexo_metaknight_up_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_sword"), Hash40::new("haver"), 0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("metaknight_shuttleloop1"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.2, true);
        EffectModule::set_disable_render_offset_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 20, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("metaknight_sword"), false, false);
    }
}

//Up Special Sound
unsafe extern "C" fn ssbexo_metaknight_up_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_metaknight_special_h01"));
        PLAY_STATUS(agent, Hash40::new("vc_metaknight_special_h01"));
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        PLAY_SE_REMAIN(agent, Hash40::new("se_metaknight_special_h02"));
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        PLAY_STATUS(agent, Hash40::new("se_metaknight_special_h03"));
    }
}

pub fn install() {
    Agent::new("metaknight")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialhi", ssbexo_metaknight_up_special_acmd, Low)
    .effect_acmd("effect_specialhi", ssbexo_metaknight_up_special_effect, Low)
    .sound_acmd("sound_specialhi", ssbexo_metaknight_up_special_sound, Low)
    .install()
    ;
}