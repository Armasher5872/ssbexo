use super::*;

//Down Special ACMD
unsafe extern "C" fn ssbexo_mario_down_special_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        SEARCH(agent, 0, 0, Hash40::new("top"), 8.0, 0.0, 7.5, 0.0, Some(0.0), Some(7.5), Some(0.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        WorkModule::set_float(agent.module_accessor, 4.0, *FIGHTER_MARIO_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }
    frame(agent.lua_state_agent,4.0);
    if is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        shield!(agent, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_JUMP);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 79, 25, 0, 90, 7.5, 0.0, 6.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BODY);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 10.0, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 79, 25, 0, 90, 7.5, 0.0, 6.0, -1.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 26.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        AttackModule::clear_all(agent.module_accessor);
        FT_MOTION_RATE(agent, 2.0);
    }
}

//Down Special Effect
unsafe extern "C" fn ssbexo_mario_down_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    for _ in 0..3 {
        if is_excute(agent) {
            let spv1 = Vector3f{x: 0.0, y: 10.2, z: 0.0};
            let spv2 = Vector3f{x: 0.0, y: 10.15, z: 0.0};
            let spv3 = Vector3f{x: 0.0, y: 10.3, z: 0.0};
            let spv4 = Vector3f{x: 0.0, y: 10.1, z: 0.0};
            let spv5 = Vector3f{x: 0.0, y: 10.25, z: 0.0};
            let stv1 = Vector3f{x: 0.0, y: 9.3, z: 9.0};
            let stv2 = Vector3f{x: 0.0, y: 9.3, z: -9.0};
            let stv3 = Vector3f{x: 0.0, y: 9.3, z: 4.5};
            let stv4 = Vector3f{x: 0.0, y: 9.3, z: -4.5};
            let spin1: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv1, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let spin2: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv2, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let spin3: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv3, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let spin4: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv4, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let spin5: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv5, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star1: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &spv5, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star2: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &stv1, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star3: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &stv2, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star4: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star5: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star6: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &stv3, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star7: u32 = EffectModule::req_follow(agent.module_accessor, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &stv4, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(agent.module_accessor, spin1, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, spin2, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, spin3, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, spin4, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, spin5, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, star1, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, star2, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, star3, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, star4, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, star5, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, star6, 0.045, 0.345, 2.05);
            EffectModule::set_rgb(agent.module_accessor, star7, 0.045, 0.345, 2.05);
            EffectModule::set_alpha(agent.module_accessor, spin1, 0.15);
            EffectModule::set_alpha(agent.module_accessor, spin2, 0.15);
            EffectModule::set_alpha(agent.module_accessor, spin3, 0.15);
            EffectModule::set_alpha(agent.module_accessor, spin4, 0.15);
            EffectModule::set_alpha(agent.module_accessor, spin5, 0.2);
            EffectModule::set_alpha(agent.module_accessor, star1, 0.6);
            EffectModule::set_alpha(agent.module_accessor, star2, 0.6);
            EffectModule::set_alpha(agent.module_accessor, star3, 0.6);
            EffectModule::set_alpha(agent.module_accessor, star4, 0.6);
            EffectModule::set_alpha(agent.module_accessor, star5, 0.6);
            EffectModule::set_alpha(agent.module_accessor, star6, 0.6);
            EffectModule::set_alpha(agent.module_accessor, star7, 0.6);
        }
        wait(agent.lua_state_agent, 5.0);
    }
}

//Down Special Sound
unsafe extern "C" fn ssbexo_mario_down_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_mario_special_l03"));
    }
    frame(agent.lua_state_agent, 4.0);
    if is_excute(agent) {
        PLAY_SEQUENCE(agent, Hash40::new("seq_mario_rnd_attack"));
    }
}

//Down Special Expression
unsafe extern "C" fn ssbexo_mario_down_special_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install() {
    Agent::new("mario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_speciallw", ssbexo_mario_down_special_acmd, Low)
    .game_acmd("game_specialairlw", ssbexo_mario_down_special_acmd, Low)
    .effect_acmd("effect_speciallw", ssbexo_mario_down_special_effect, Low)
    .effect_acmd("effect_specialairlw", ssbexo_mario_down_special_effect, Low)
    .sound_acmd("sound_speciallw", ssbexo_mario_down_special_sound, Low)
    .sound_acmd("sound_specialairlw", ssbexo_mario_down_special_sound, Low)
    .expression_acmd("expression_speciallw", ssbexo_mario_down_special_expression, Low)
    .expression_acmd("expression_specialairlw", ssbexo_mario_down_special_expression, Low)
    .install()
    ;
}