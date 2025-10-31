use super::*;

//Nair ACMD
unsafe extern "C" fn ssbexo_cloud_nair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 5.0);
    FT_MOTION_RATE(agent, 0.75);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 8.0, 45, 95, 0, 23, 3.8, 3.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 8.0, 45, 95, 0, 23, 3.8, 3.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 8.0, 45, 95, 0, 23, 3.8, 3.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 34.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Punisher Nair ACMD
unsafe extern "C" fn ssbexo_cloud_punisher_nair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 6.0, 80, 55, 0, 50, 6.0, 3.0, 13.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 6.0, 80, 55, 0, 50, 6.0, 3.0, 9.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 6.0, 80, 55, 0, 50, 6.0, 3.0, 4.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 12.0, false);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 29.0);
    FT_MOTION_RATE(agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 10.0, 75, 60, 0, 60, 6.0, 3.0, 13.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 10.0, 75, 60, 0, 60, 6.0, 3.0, 9.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 10.0, 75, 60, 0, 60, 6.0, 3.0, 4.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 37.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Punisher Nair Effect
unsafe extern "C" fn ssbexo_cloud_punisher_nair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.2);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 7, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 18.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
    frame(agent.lua_state_agent, 29.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.2);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword1"), Hash40::new("tex_cloud_sword2"), 7, Hash40::new("haver"), 0.0, 1.5, -1.2, Hash40::new("haver"), 0.0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 34.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
    }
    frame(agent.lua_state_agent, 40.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}

//Punisher Nair Sound
unsafe extern "C" fn ssbexo_cloud_punisher_nair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_cloud_rnd_attack_air"));
        PLAY_SE(agent, Hash40::new("se_cloud_attackair_n01"));
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_attackair_n01"));
    }
}

//Punisher Nair Expression
unsafe extern "C" fn ssbexo_cloud_punisher_nair_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

//Punisher Landing Nair ACMD
unsafe extern "C" fn ssbexo_cloud_punisher_landing_nair_acmd(_agent: &mut L2CAgentBase) {}

//Punisher Landing Nair Effect
unsafe extern "C" fn ssbexo_cloud_punisher_landing_nair_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Punisher Landing Nair Sound
unsafe extern "C" fn ssbexo_cloud_punisher_landing_nair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_cloud_landing02"));
    }
}

//Punisher Landing Nair Expression
unsafe extern "C" fn ssbexo_cloud_punisher_landing_nair_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Punisher Fair ACMD
unsafe extern "C" fn ssbexo_cloud_punisher_fair_acmd(agent: &mut L2CAgentBase) {
    let lr = PostureModule::lr(agent.module_accessor);
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 1.5);
        sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.5*lr);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 18.0, 361, 40, 0, 57, 6.0, 3.0, 13.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 18.0, 361, 40, 0, 57, 6.0, 3.0, 9.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 18.0, 361, 40, 0, 57, 6.0, 3.0, 4.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -4.2);
    }
}

//Punisher Fair Effect
unsafe extern "C" fn ssbexo_cloud_punisher_fair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.2);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword3"), Hash40::new("tex_cloud_sword4"), 3, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
}

//Punisher Fair Sound
unsafe extern "C" fn ssbexo_cloud_punisher_fair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 21.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_cloud_rnd_special_l03"));
        PLAY_SE(agent, Hash40::new("se_cloud_attackair_f01"));
    }
}

//Punisher Fair Expression
unsafe extern "C" fn ssbexo_cloud_punisher_fair_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

//Punisher Landing Fair ACMD
unsafe extern "C" fn ssbexo_cloud_punisher_landing_fair_acmd(_agent: &mut L2CAgentBase) {}

//Punisher Landing Fair Effect
unsafe extern "C" fn ssbexo_cloud_punisher_landing_fair_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 1.5, 0, 18, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LANDING_EFFECT(agent, Hash40::new("sys_quake"), Hash40::new("top"), 1.5, 0, 18, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("ganon_ground_crack"), Hash40::new("top"), 0, 0, 18, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Punisher Landing Fair Sound
unsafe extern "C" fn ssbexo_cloud_punisher_landing_fair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_cloud_landing02"));
        PLAY_SE(agent, Hash40::new("se_cloud_special_h04"));
    }
}

//Punisher Landing Fair Expression
unsafe extern "C" fn ssbexo_cloud_punisher_landing_fair_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Bair ACMD
unsafe extern "C" fn ssbexo_cloud_bair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 95, 0, 20, 7.0, 0.0, 8.5, -15.0, Some(0.0), Some(8.5), Some(-7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 361, 95, 0, 20, 7.0, 0.0, 16.0, -15.0, Some(0.0), Some(16.0), Some(-7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 37.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Punisher Bair ACMD
unsafe extern "C" fn ssbexo_cloud_punisher_bair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        MotionModule::set_rate(agent.module_accessor, 1.17);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 18.0, 20, 60, 0, 50, 6.0, 0.0, 4.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 18.0, 250, 20, 0, 60, 6.0, 0.0, 9.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 18.0, 85, 70, 0, 40, 6.0, 0.0, 14.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        ATTACK(agent, 2, 0, Hash40::new("havel"), 18.0, 20, 60, 0, 50, 6.0, 0.0, 4.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("havel"), 18.0, 250, 20, 0, 60, 6.0, 0.0, 9.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 0, 0, Hash40::new("havel"), 18.0, 85, 70, 0, 40, 6.0, 0.0, 14.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 62.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Punisher Bair Effect
unsafe extern "C" fn ssbexo_cloud_punisher_bair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword3"), Hash40::new("tex_cloud_sword4"), 3, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.14);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
        AFTER_IMAGE_OFF(agent, 1);
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword3"), Hash40::new("tex_cloud_sword4"), 3, Hash40::new("havel"), 0, 1.5, -1.2, Hash40::new("havel"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.14);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(agent.lua_state_agent, 43.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}

//Punisher Bair Sound
unsafe extern "C" fn ssbexo_cloud_punisher_bair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_cloud_rnd_attack_air"));
        PLAY_SE(agent, Hash40::new("se_cloud_attackair_b01"));
    }
}

//Punisher Bair Expression
unsafe extern "C" fn ssbexo_cloud_punisher_bair_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_l") as i64);
    }
    frame(agent.lua_state_agent, 65.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_r") as i64);
    }
}

//Punisher Landing Bair ACMD
unsafe extern "C" fn ssbexo_cloud_punisher_landing_bair_acmd(_agent: &mut L2CAgentBase) {}

//Punisher Landing Bair Effect
unsafe extern "C" fn ssbexo_cloud_punisher_landing_bair_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Punisher Landing Bair Sound
unsafe extern "C" fn ssbexo_cloud_punisher_landing_bair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_cloud_landing02"));
    }
}

//Punisher Landing Bair Expression
unsafe extern "C" fn ssbexo_cloud_punisher_landing_bair_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_l") as i64);
    }
    frame(agent.lua_state_agent, 34.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_r") as i64);
    }
}

//Uair ACMD
unsafe extern "C" fn ssbexo_cloud_uair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 11.0, 80, 80, 0, 42, 4.7, 0.0, 4.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 11.0, 80, 80, 0, 42, 4.7, 0.0, 9.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 11.0, 80, 80, 0, 42, 4.7, 0.0, 14.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 8.0, 80, 80, 0, 42, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 8.0, 80, 80, 0, 42, 3.5, 0.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 8.0, 80, 80, 0, 42, 3.5, 0.0, 14.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 38.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Punisher Uair ACMD
unsafe extern "C" fn ssbexo_cloud_punisher_uair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        MotionModule::set_rate(agent.module_accessor, 1.43);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        ATTACK(agent, 0, 0, Hash40::new("haver"), 15.0, 90, 70, 0, 35, 5.5, 0.0, 4.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 15.0, 90, 70, 0, 35, 5.5, 0.0, 9.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 15.0, 90, 70, 0, 35, 5.5, 0.0, 14.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Punisher Uair Effect
unsafe extern "C" fn ssbexo_cloud_punisher_uair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_cloud_sword3"), Hash40::new("tex_cloud_sword4"), 3, Hash40::new("haver"), 0, 1.5, -1.2, Hash40::new("haver"), 0, 20.5, -1.2, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.14);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 3);
    }
    frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}

//Punisher Uair Sound
unsafe extern "C" fn ssbexo_cloud_punisher_uair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_attackair_h01"));
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_cloud_smash_h01"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_cloud_rnd_attack_air"));
    }
}

//Punisher Uair Expression
unsafe extern "C" fn ssbexo_cloud_punisher_uair_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

//Punisher Landing Uair ACMD
unsafe extern "C" fn ssbexo_cloud_punisher_landing_uair_acmd(_agent: &mut L2CAgentBase) {}

//Punisher Landing Uair Effect
unsafe extern "C" fn ssbexo_cloud_punisher_landing_uair_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Punisher Landing Uair Sound
unsafe extern "C" fn ssbexo_cloud_punisher_landing_uair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_cloud_landing02"));
    }
}

//Punisher Landing Uair Expression
unsafe extern "C" fn ssbexo_cloud_punisher_landing_uair_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Dair ACMD
unsafe extern "C" fn ssbexo_cloud_dair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 15.0, 270, 100, 0, 10, 3.6, 0.0, 11.0, 0.0, Some(0.0), Some(12.5), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 13.0, 60, 105, 0, 30, 4.5, 0.0, 7.0, 0.0, Some(0.0), Some(13.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 8.0, 65, 55, 0, 80, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 8.0, 65, 55, 0, 80, 3.5, 0.0, 9.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 8.0, 65, 55, 0, 80, 3.5, 0.0, 14.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 46.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Punisher Dair ACMD
unsafe extern "C" fn ssbexo_cloud_punisher_dair_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 4.0, 367, 90, 50, 0, 6.0, 3.0, 13.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 4.0, 367, 90, 50, 0, 6.0, 3.0, 9.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 4.0, 367, 90, 50, 0, 6.0, 3.0, 4.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 10.0, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 4.0, 367, 90, 50, 0, 6.0, 3.0, 13.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 4.0, 367, 90, 50, 0, 6.0, 3.0, 9.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 4.0, 367, 90, 50, 0, 6.0, 3.0, 4.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 10.0, false);
    }
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("haver"), 6.0, 65, 95, 0, 50, 6.0, 3.0, 13.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("haver"), 6.0, 65, 95, 0, 50, 6.0, 3.0, 9.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("haver"), 6.0, 65, 95, 0, 50, 6.0, 3.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CLOUD_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 13.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 13.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 13.0, false);
    }
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 50.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Punisher Dair Effect
unsafe extern "C" fn ssbexo_cloud_punisher_dair_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, 0.5);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("cloud_speedline"), Hash40::new("haver"), 0, 4, 0, -90, 0, 0, 0.8, true, 0.627, 1, 0.674);
        sv_animcmd::EFFECT_FOLLOW_COLOR(agent.lua_state_agent);
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, -1, 0, 0, 0, -6, 0.28, true, 1);
        LAST_EFFECT_SET_RATE(agent, 0.9);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 2, 0, 0, 90, 6, 0.4, true, 0.8);
        LAST_EFFECT_SET_RATE(agent, 0.95);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 5.5, 0, 0, 180, -6, 0.52, true, 0.55);
    }
    frame(agent.lua_state_agent, 19.0);
    if is_excute(agent) {
        EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 9, 0, 0, 270, 6, 0.65, true, 0.4);
        LAST_EFFECT_SET_RATE(agent, 1.1);
    }
    frame(agent.lua_state_agent, 36.0);
    if is_excute(agent) {
        EFFECT_OFF_KIND_WORK(agent, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, true, true);
    }
}

//Punisher Dair Sound
unsafe extern "C" fn ssbexo_cloud_punisher_dair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_cloud_attack06"));
        PLAY_SE(agent, Hash40::new("se_cloud_attackair_l01"));
    }
}

//Punisher Dair Expression
unsafe extern "C" fn ssbexo_cloud_punisher_dair_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_NONE), AttackDirectionAxis(*ATTACK_DIRECTION_NONE));
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 13.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashs"), 8);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 28.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 48.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

//Punisher Landing Dair ACMD
unsafe extern "C" fn ssbexo_cloud_punisher_landing_dair_acmd(_agent: &mut L2CAgentBase) {}

//Punisher Landing Dair Effect
unsafe extern "C" fn ssbexo_cloud_punisher_landing_dair_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//Punisher Landing Dair Sound
unsafe extern "C" fn ssbexo_cloud_punisher_landing_dair_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_LANDING_SE(agent, Hash40::new("se_cloud_landing02"));
    }
}

//Punisher Landing Dair Expression
unsafe extern "C" fn ssbexo_cloud_punisher_landing_dair_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("cloud")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_attackairn", ssbexo_cloud_nair_acmd, Low)
    .game_acmd("game_punishattackairn", ssbexo_cloud_punisher_nair_acmd, Low)
    .effect_acmd("effect_punishattackairn", ssbexo_cloud_punisher_nair_effect, Low)
    .sound_acmd("sound_punishattackairn", ssbexo_cloud_punisher_nair_sound, Low)
    .expression_acmd("expression_punishattackairn", ssbexo_cloud_punisher_nair_expression, Low)
    .game_acmd("game_punishlandingairn", ssbexo_cloud_punisher_landing_nair_acmd, Low)
    .effect_acmd("effect_punishlandingairn", ssbexo_cloud_punisher_landing_nair_effect, Low)
    .sound_acmd("sound_punishlandingairn", ssbexo_cloud_punisher_landing_nair_sound, Low)
    .expression_acmd("expression_punishlandingairn", ssbexo_cloud_punisher_landing_nair_expression, Low)
    .game_acmd("game_punishattackairf", ssbexo_cloud_punisher_fair_acmd, Low)
    .effect_acmd("effect_punishattackairf", ssbexo_cloud_punisher_fair_effect, Low)
    .sound_acmd("sound_punishattackairf", ssbexo_cloud_punisher_fair_sound, Low)
    .expression_acmd("expression_punishattackairf", ssbexo_cloud_punisher_fair_expression, Low)
    .game_acmd("game_punishlandingairf", ssbexo_cloud_punisher_landing_fair_acmd, Low)
    .effect_acmd("effect_punishlandingairf", ssbexo_cloud_punisher_landing_fair_effect, Low)
    .sound_acmd("sound_punishlandingairf", ssbexo_cloud_punisher_landing_fair_sound, Low)
    .expression_acmd("expression_punishlandingairf", ssbexo_cloud_punisher_landing_fair_expression, Low)
    .game_acmd("game_attackairb", ssbexo_cloud_bair_acmd, Low)
    .game_acmd("game_punishattackairb", ssbexo_cloud_punisher_bair_acmd, Low)
    .effect_acmd("effect_punishattackairb", ssbexo_cloud_punisher_bair_effect, Low)
    .sound_acmd("sound_punishattackairb", ssbexo_cloud_punisher_bair_sound, Low)
    .expression_acmd("expression_punishattackairb", ssbexo_cloud_punisher_bair_expression, Low)
    .game_acmd("game_punishlandingairb", ssbexo_cloud_punisher_landing_bair_acmd, Low)
    .effect_acmd("effect_punishlandingairb", ssbexo_cloud_punisher_landing_bair_effect, Low)
    .sound_acmd("sound_punishlandingairb", ssbexo_cloud_punisher_landing_bair_sound, Low)
    .expression_acmd("expression_punishlandingairb", ssbexo_cloud_punisher_landing_bair_expression, Low)
    .game_acmd("game_attackairhi", ssbexo_cloud_uair_acmd, Low)
    .game_acmd("game_punishattackairhi", ssbexo_cloud_punisher_uair_acmd, Low)
    .effect_acmd("effect_punishattackairhi", ssbexo_cloud_punisher_uair_effect, Low)
    .sound_acmd("sound_punishattackairhi", ssbexo_cloud_punisher_uair_sound, Low)
    .expression_acmd("expression_punishattackairhi", ssbexo_cloud_punisher_uair_expression, Low)
    .game_acmd("game_punishlandingairhi", ssbexo_cloud_punisher_landing_uair_acmd, Low)
    .effect_acmd("effect_punishlandingairhi", ssbexo_cloud_punisher_landing_uair_effect, Low)
    .sound_acmd("sound_punishlandingairhi", ssbexo_cloud_punisher_landing_uair_sound, Low)
    .expression_acmd("expression_punishlandingairhi", ssbexo_cloud_punisher_landing_uair_expression, Low)
    .game_acmd("game_attackairlw", ssbexo_cloud_dair_acmd, Low)
    .game_acmd("game_punishattackairlw", ssbexo_cloud_punisher_dair_acmd, Low)
    .effect_acmd("effect_punishattackairlw", ssbexo_cloud_punisher_dair_effect, Low)
    .sound_acmd("sound_punishattackairlw", ssbexo_cloud_punisher_dair_sound, Low)
    .expression_acmd("expression_punishattackairlw", ssbexo_cloud_punisher_dair_expression, Low)
    .game_acmd("game_punishlandingairlw", ssbexo_cloud_punisher_landing_dair_acmd, Low)
    .effect_acmd("effect_punishlandingairlw", ssbexo_cloud_punisher_landing_dair_effect, Low)
    .sound_acmd("sound_punishlandingairlw", ssbexo_cloud_punisher_landing_dair_sound, Low)
    .expression_acmd("expression_punishlandingairlw", ssbexo_cloud_punisher_landing_dair_expression, Low)
    .install()
    ;
}