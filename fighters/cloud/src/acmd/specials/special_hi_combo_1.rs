use super::*;

//Up Special Combo 1 ACMD
unsafe extern "C" fn ssbexo_cloud_up_special_combo_1_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DIRECTION_DECIDE);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DIRECTION_DECIDE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DIRECTION_CHOSEN);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 4.0, 367, 100, 100, 0, 5.0, 0.0, 6.0, 0.0, Some(0.0), Some(6.0), Some(0.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 4.0, 367, 100, 100, 0, 5.0, 0.0, 14.5, 0.0, Some(0.0), Some(6.0), Some(0.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        let attack_angle = WorkModule::get_int(agent.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_SPECIAL_HI_ATTACK_ANGLE);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 5.0, attack_angle as u64, 40, 0, 50, 5.0, 0.0, 6.0, 0.0, Some(0.0), Some(6.0), Some(0.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 5.0, attack_angle as u64, 40, 0, 50, 5.0, 0.0, 14.5, 0.0, Some(0.0), Some(6.0), Some(0.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 43.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 56.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT);
    }
    frame(agent.lua_state_agent, 61.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_CLOUD_STATUS_SPECIAL_HI_FLAG_SHIFT);
    }
}

//Up Special Combo 1 Effect
unsafe extern "C" fn ssbexo_cloud_up_special_combo_1_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1_purple"), Hash40::new("tex_cloud_sword2"), 12, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.1);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1_purple"), Hash40::new("tex_cloud_sword2"), 12, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.1);
    }
    frame(agent.lua_state_agent, 44.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 45.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_CLIMHAZZARD_SWORD, false, true);
    }
}

//Up Special Combo 1 Sound
unsafe extern "C" fn ssbexo_cloud_up_special_combo_1_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_smash_s03"));
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_cloud_rnd_special_h01"));
        PLAY_SE(agent, Hash40::new("se_cloud_smash_h02"));
    }
}

//Up Special Combo 1 Expression
unsafe extern "C" fn ssbexo_cloud_up_special_combo_1_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 0);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialhicombo1", ssbexo_cloud_up_special_combo_1_acmd, Low)
    .effect_acmd("effect_specialhicombo1", ssbexo_cloud_up_special_combo_1_effect, Low)
    .sound_acmd("sound_specialhicombo1", ssbexo_cloud_up_special_combo_1_sound, Low)
    .expression_acmd("expression_specialhicombo1", ssbexo_cloud_up_special_combo_1_expression, Low)
    .install()
    ;
}