use super::*;

//Octaslash Prime ACMD
unsafe extern "C" fn ssbexo_edge_octaslash_prime_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let hit_id = WorkModule::get_int(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_HIT_ID);
    if is_excute(agent) {
        WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 367, 0, 0, 25, 10.0, determine_opponent_hitbox_spawn_position(boma, hit_id).x, determine_opponent_hitbox_spawn_position(boma, hit_id).y+6.0, determine_opponent_hitbox_spawn_position(boma, hit_id).z, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_stop"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 90.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.5, 367, 60, 60, 0, 10.0, determine_opponent_hitbox_spawn_position(boma, hit_id).x, determine_opponent_hitbox_spawn_position(boma, hit_id).y+6.0, determine_opponent_hitbox_spawn_position(boma, hit_id).z, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 101.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 102.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 5.5, 36, 140, 0, 68, 10.0, determine_opponent_hitbox_spawn_position(boma, hit_id).x, determine_opponent_hitbox_spawn_position(boma, hit_id).y+6.0, determine_opponent_hitbox_spawn_position(boma, hit_id).z, None, None, None, 2.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(lua_state, 105.0);
    if is_excute(agent) {
        CAM_ZOOM_OUT(agent);
        WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        AttackModule::clear_all(boma);
    }
}

//Octaslash Prime Effect
unsafe extern "C" fn ssbexo_edge_octaslash_prime_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_sword_light"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("edge_sword_flash"), Hash40::new("swordl2"), 12, 0, -0.2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 34.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_sword_flare"), Hash40::new("swordl2"), 0, 0, 0, 0, 180, -90, 1, true);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_edge_sword1"), Hash40::new("tex_edge_sword2"), 6, Hash40::new("swordl2"), -4, 0, -0.7, Hash40::new("swordl2"), 29.2, 0, 1.5, true, Hash40::new("null"), Hash40::new("swordl2"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("rot"), 0, 4, 2, 0, -3.5, -196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND(agent, Hash40::new("edge_octaslash_sword_flare"), false, true);
        AFTER_IMAGE_OFF(agent, 6);
    }
    frame(lua_state, 90.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("top"), 0, 8, -22, 0, -3.5, 196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("top"), 0, 8, -22, 0, -3.5, -196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("top"), 0, 8, -22, 0, -176.5, 196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("top"), 0, 8, -22, 0, -176.5, -196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("edge_senkou_hold_attack"), Hash40::new("top"), 0, 8, -24, 0, 0, 0, 2.0, true);
    }
    frame(lua_state, 93.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("top"), 0, 8, -22, 0, -3.5, 196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("top"), 0, 8, -22, 0, -3.5, -196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("top"), 0, 8, -22, 0, -176.5, 196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("top"), 0, 8, -22, 0, -176.5, -196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("edge_senkou_hold_attack"), Hash40::new("top"), 0, 8, -24, 0, 0, 0, 2.0, true);
    }
    frame(lua_state, 96.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("top"), 0, 8, -22, 0, -3.5, 196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("top"), 0, 8, -22, 0, -3.5, -196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("top"), 0, 8, -22, 0, -176.5, 196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("top"), 0, 8, -22, 0, -176.5, -196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("edge_senkou_hold_attack"), Hash40::new("top"), 0, 8, -24, 0, 0, 0, 2.0, true);
    }
    frame(lua_state, 99.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("top"), 0, 8, -22, 0, -3.5, 196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("top"), 0, 8, -22, 0, -3.5, -196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("top"), 0, 8, -22, 0, -176.5, 196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("top"), 0, 8, -22, 0, -176.5, -196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("edge_senkou_hold_attack"), Hash40::new("top"), 0, 8, -24, 0, 0, 0, 2.0, true);
    }
    frame(lua_state, 102.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("top"), 0, 8, -22, 0, -3.5, 196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("top"), 0, 8, -22, 0, -3.5, -196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("top"), 0, 8, -22, 0, -176.5, 196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("edge_octaslash_arc2"), Hash40::new("top"), 0, 8, -22, 0, -176.5, -196, 0.4, true);
        EffectModule::set_disable_render_offset_last(boma);
        LAST_EFFECT_SET_RATE(agent, 1.5);
        EFFECT_FOLLOW(agent, Hash40::new("edge_senkou_hold_end"), Hash40::new("top"), 0, 8, -24, 0, 0, 0, 2.0, false);
    }
}

//Octaslash Prime Sound
unsafe extern "C" fn ssbexo_edge_octaslash_prime_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_appeal_s02"));
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_special_h03_08"));
    }
    frame(lua_state, 90.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_special_h03_01"));
    }
    frame(lua_state, 93.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_special_h03_02"));
    }
    frame(lua_state, 96.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_special_h03_03"));
    }
    frame(lua_state, 99.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_special_h03_04"));
    }
    frame(lua_state, 102.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_edge_final02_03"));
    }
}

//Octaslash Prime Expression
unsafe extern "C" fn ssbexo_edge_octaslash_prime_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        AttackModule::set_attack_reference_joint_id(boma, Hash40::new("swordl1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3, true);
    }
    frame(lua_state, 36.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl_l"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        RUMBLE_HIT(agent, Hash40::new("rbkind_78_smash"), 50);
    }
    frame(lua_state, 90.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_78_slash"), 1);
    }
    for _ in 0..5 {
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitll"), 4, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(lua_state, 3.0);
    }
}

pub fn install() {
    Agent::new("edge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialhi2endwing", ssbexo_edge_octaslash_prime_acmd, Low)
    .acmd("effect_specialhi2endwing", ssbexo_edge_octaslash_prime_effect, Low)
    .acmd("sound_specialhi2endwing", ssbexo_edge_octaslash_prime_sound, Low)
    .acmd("expression_specialhi2endwing", ssbexo_edge_octaslash_prime_expression, Low)
    .install()
    ;
}