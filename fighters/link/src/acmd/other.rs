use super::*;

//Up Taunt ACMD
unsafe extern "C" fn ssbexo_link_up_taunt_acmd(_agent: &mut L2CAgentBase) {}

//Up Taunt Effect
unsafe extern "C" fn ssbexo_link_up_taunt_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword1"), 11, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword1"), 11, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

//Up Taunt Sound
unsafe extern "C" fn ssbexo_link_up_taunt_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_appeal_h01"));
    }
    frame(agent.lua_state_agent, 16.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_appeal_h02"));
    }
    frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_swing_s"));
    }
    frame(agent.lua_state_agent, 47.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_swing_s"));
    }
    frame(agent.lua_state_agent, 60.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_swing_s"));
    }
    frame(agent.lua_state_agent, 76.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_appeal_h03"));
    }
}

//Up Taunt Expression
unsafe extern "C" fn ssbexo_link_up_taunt_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 35.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 47.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 60.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 76.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beams"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 105.0);
    if is_excute(agent) {
        if ItemModule::is_have_item(agent.module_accessor, 0) {
            ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
        }
        else {
            VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        }
    }
}

//Mortal Draw Loop ACMD
unsafe extern "C" fn ssbexo_link_mortal_draw_loop_acmd(_agent: &mut L2CAgentBase) {}

//Mortal Draw Loop Effect
unsafe extern "C" fn ssbexo_link_mortal_draw_loop_effect(_agent: &mut L2CAgentBase) {}

//Mortal Draw Loop Sound
unsafe extern "C" fn ssbexo_link_mortal_draw_loop_sound(_agent: &mut L2CAgentBase) {}

//Mortal Draw Loop Expression
unsafe extern "C" fn ssbexo_link_mortal_draw_loop_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
}

//Mortal Draw Attack ACMD
unsafe extern "C" fn ssbexo_link_mortal_draw_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("sword2"), 1.0, 270, 10, 200, 0, 3.2, 8.6, 0.0, -2.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 1, 0, Hash40::new("sword2"), 1.0, 270, 10, 200, 0, 3.5, 3.0, 0.0, -2.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 2, 0, Hash40::new("armr"), 1.0, 270, 10, 200, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(agent, 3, 0, Hash40::new("top"), 1.0, 270, 10, 200, 0, 2.5, 0.0, 14.0, 14.0, Some(0.0), Some(14.0), Some(7.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 3, 0, Hash40::new("top"), 1.0, 270, 10, 200, 0, 2.5, 0.0, 2.5, 16.0, Some(0.0), Some(2.5), Some(7.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 13.0);
    FT_MOTION_RATE(agent, 1.5);
}

//Mortal Draw Attack Effect
unsafe extern "C" fn ssbexo_link_mortal_draw_attack_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_link_sword1"), Hash40::new("tex_link_sword2"), 5, Hash40::new("sword1"), 1, 0, 0, Hash40::new("sword1"), 14.6, 0.2, -0.2, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        EFFECT_FOLLOW(agent, Hash40::new("link_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        AFTER_IMAGE_OFF(agent, 4);
        EFFECT_OFF_KIND(agent, Hash40::new("link_sword"), false, false);
    }
}

//Mortal Draw Attack Sound
unsafe extern "C" fn ssbexo_link_mortal_draw_attack_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_link_swing_ll"));
        PLAY_SE(agent, Hash40::new("vc_link_attack06"));
    }
}

//Mortal Draw Attack Expression
unsafe extern "C" fn ssbexo_link_mortal_draw_attack_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_R, 3);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(agent.lua_state_agent, 1.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
    frame(agent.lua_state_agent, 33.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
        if !ItemModule::is_have_item(agent.module_accessor, 0) {
            VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        }
    }
}

//wall Cling Sound
unsafe extern "C" fn ssbexo_link_wall_cling_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        STOP_SE(agent, Hash40::new("se_link_step_left_s_ft"));
        STOP_SE(agent, Hash40::new("se_link_step_right_s_ft"));
    }
}

//Wall Cling Expression
unsafe extern "C" fn ssbexo_link_wall_cling_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

//Wall Climb Sound
unsafe extern "C" fn ssbexo_link_wall_climb_sound(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_link_step_right_s_ft"));
        }
        wait(agent.lua_state_agent, 5.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_link_step_left_s_ft"));
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

//Wall Climb Expression
unsafe extern "C" fn ssbexo_link_wall_climb_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(agent.lua_state_agent, 1.0);
    for _ in 0..i32::MAX {
        if is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(agent.lua_state_agent, 6.0);
        if is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(agent.lua_state_agent, 1.0);
    }
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_appealhir", ssbexo_link_up_taunt_acmd, Low)
    .effect_acmd("effect_appealhir", ssbexo_link_up_taunt_effect, Low)
    .sound_acmd("sound_appealhir", ssbexo_link_up_taunt_sound, Low)
    .expression_acmd("expression_appealhir", ssbexo_link_up_taunt_expression, Low)
    .game_acmd("game_appealhil", ssbexo_link_up_taunt_acmd, Low)
    .effect_acmd("effect_appealhil", ssbexo_link_up_taunt_effect, Low)
    .sound_acmd("sound_appealhil", ssbexo_link_up_taunt_sound, Low)
    .expression_acmd("expression_appealhil", ssbexo_link_up_taunt_expression, Low)
    .game_acmd("game_mortaldrawloop", ssbexo_link_mortal_draw_loop_acmd, Low)
    .effect_acmd("effect_mortaldrawloop", ssbexo_link_mortal_draw_loop_effect, Low)
    .sound_acmd("sound_mortaldrawloop", ssbexo_link_mortal_draw_loop_sound, Low)
    .expression_acmd("expression_mortaldrawloop", ssbexo_link_mortal_draw_loop_expression, Low)
    .game_acmd("game_mortaldrawattack", ssbexo_link_mortal_draw_attack_acmd, Low)
    .effect_acmd("effect_mortaldrawattack", ssbexo_link_mortal_draw_attack_effect, Low)
    .sound_acmd("sound_mortaldrawattack", ssbexo_link_mortal_draw_attack_sound, Low)
    .expression_acmd("expression_mortaldrawattack", ssbexo_link_mortal_draw_attack_expression, Low)
    .sound_acmd("sound_attachwall", ssbexo_link_wall_cling_sound, Priority::Low)
    .expression_acmd("expression_attachwall", ssbexo_link_wall_cling_expression, Priority::Low)
    .sound_acmd("sound_attachwallclimb", ssbexo_link_wall_climb_sound, Priority::Low)
    .expression_acmd("expression_attachwallclimb", ssbexo_link_wall_climb_expression, Priority::Low)
    .install()
    ;
}