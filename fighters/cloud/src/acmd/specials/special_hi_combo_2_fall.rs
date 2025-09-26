use super::*;

//Up Special Combo 2 Fall ACMD
unsafe extern "C" fn ssbexo_cloud_up_special_combo_2_fall_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 8.0, 280, 100, 0, 40, 5.0, 0.0, 6.0, 0.0, Some(0.0), Some(6.0), Some(0.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 8.0, 280, 100, 0, 40, 5.0, 0.0, 14.5, 0.0, Some(0.0), Some(6.0), Some(0.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
}

//Up Special Combo 2 Fall Effect
unsafe extern "C" fn ssbexo_cloud_up_special_combo_2_fall_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1_purple"), Hash40::new("tex_cloud_sword2"), 12, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.1);
    }
}

//Up Special Combo 2 Fall Sound
unsafe extern "C" fn ssbexo_cloud_up_special_combo_2_fall_sound(_agent: &mut L2CAgentBase) {}

//Up Special Combo 2 Fall Expression
unsafe extern "C" fn ssbexo_cloud_up_special_combo_2_fall_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialhicombo2fall", ssbexo_cloud_up_special_combo_2_fall_acmd, Low)
    .effect_acmd("effect_specialhicombo2fall", ssbexo_cloud_up_special_combo_2_fall_effect, Low)
    .sound_acmd("sound_specialhicombo2fall", ssbexo_cloud_up_special_combo_2_fall_sound, Low)
    .expression_acmd("expression_specialhicombo2fall", ssbexo_cloud_up_special_combo_2_fall_expression, Low)
    .install()
    ;
}