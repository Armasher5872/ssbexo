use super::*;

//Side Special Rush ACMD
unsafe extern "C" fn ssbexo_sonic_side_special_rush_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_SPECIAL_S_RUSH);
        JostleModule::set_status(agent.module_accessor, false);
        FT_MOTION_RATE(agent, 8.0/11.0);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 6.0, 33, 20, 0, 100, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 12.0, false);
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 6.0, 44, 20, 0, 90, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 12.0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 6.0, 55, 20, 0, 80, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 12.0, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 6.0, 66, 20, 0, 70, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 12.0, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("hip"), 6.0, 77, 20, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 12.0, false);
    }
}

//Side Special Rush Effect
unsafe extern "C" fn ssbexo_sonic_side_special_rush_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        let angle = WorkModule::get_int(agent.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_ID_INT_SPECIAL_S_ANGLE);
        if angle > 0 {
            EFFECT_FOLLOW(agent, Hash40::new("sonic_rush_shock"), Hash40::new("top"), 6, 10, 0, -20, 0, 0, 0.85, false);
        }
        else if angle < 0 {
            EFFECT_FOLLOW(agent, Hash40::new("sonic_rush_shock"), Hash40::new("top"), 6, 10, 0, 20, 0, 0, 0.85, false);
        }
        else {
            EFFECT_FOLLOW(agent, Hash40::new("sonic_rush_shock"), Hash40::new("top"), 6, 10, 0, 0, 0, 0, 0.85, false);
        }
        LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sonic_spintrace"), Hash40::new("top"), 0, 5, 0, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

//Side Special Rush Sound
unsafe extern "C" fn ssbexo_sonic_side_special_rush_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_sonic_attack05"));
        PLAY_SE(agent, Hash40::new("se_sonic_special_s04"));
    }
}

//Side Special Rush Expression
unsafe extern "C" fn ssbexo_sonic_side_special_rush_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3, true);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install() {
    Agent::new("sonic")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialsrush", ssbexo_sonic_side_special_rush_acmd, Low)
    .effect_acmd("effect_specialsrush", ssbexo_sonic_side_special_rush_effect, Low)
    .sound_acmd("sound_specialsrush", ssbexo_sonic_side_special_rush_sound, Low)
    .expression_acmd("expression_specialsrush", ssbexo_sonic_side_special_rush_expression, Low)
    .install()
    ;
}