use super::*;

//Punisher Down Special ACMD
unsafe extern "C" fn ssbexo_cloud_punisher_down_special_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.4);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 60, 55, 0, 45, 9.5, 0.0, 9.0, 16.5, Some(0.0), Some(9.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

//Grounded Punisher Down Special Effect
unsafe extern "C" fn ssbexo_cloud_grounded_punisher_down_special_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.4);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.19, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0);
        AFTER_IMAGE_OFF(agent, 4);
    }
    wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}

//Aerial Punisher Down Special Effect
unsafe extern "C" fn ssbexo_cloud_aerial_punisher_down_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.4);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 4, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.19, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    wait(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        FLASH(agent, 1, 1, 1, 0);
        AFTER_IMAGE_OFF(agent, 4);
    }
    wait(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        COL_NORMAL(agent);
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}

//Punisher Down Special Sound
unsafe extern "C" fn ssbexo_cloud_punisher_down_special_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_special_l02"));
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_cloud_rnd_attack"));
    }
}

//Punisher Down Special Expression
unsafe extern "C" fn ssbexo_cloud_punisher_down_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_punishspeciallw", ssbexo_cloud_punisher_down_special_acmd, Low)
    .game_acmd("game_punishspecialairlw", ssbexo_cloud_punisher_down_special_acmd, Low)
    .effect_acmd("effect_punishspeciallw", ssbexo_cloud_grounded_punisher_down_special_effect, Low)
    .effect_acmd("effect_punishspecialairlw", ssbexo_cloud_aerial_punisher_down_special_effect, Low)
    .sound_acmd("sound_punishspeciallw", ssbexo_cloud_punisher_down_special_sound, Low)
    .sound_acmd("sound_punishspecialairlw", ssbexo_cloud_punisher_down_special_sound, Low)
    .expression_acmd("expression_punishspeciallw", ssbexo_cloud_punisher_down_special_expression, Low)
    .expression_acmd("expression_punishspecialairlw", ssbexo_cloud_punisher_down_special_expression, Low)
    .install()
    ;
}