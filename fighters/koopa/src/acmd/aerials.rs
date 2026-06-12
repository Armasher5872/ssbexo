use super::*;

//Nair ACMD
unsafe extern "C" fn ssbexo_koopa_nair_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        MotionModule::set_rate(boma, 0.666);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 55, 55, 0, 50, 10.0, 0.0, 9.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
       ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 65, 60, 0, 60, 10.0, 0.0, 9.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 30.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//Nair Effect
unsafe extern "C" fn ssbexo_koopa_nair_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 6.0);
    for _ in 0..3 {
        if is_excute(agent) {
            let spv1 = Vector3f{x: 0.0, y: 18.2, z: 0.0};
            let spv2 = Vector3f{x: 0.0, y: 18.15, z: 0.0};
            let spv3 = Vector3f{x: 0.0, y: 18.3, z: 0.0};
            let spv4 = Vector3f{x: 0.0, y: 18.1, z: 0.0};
            let spv5 = Vector3f{x: 0.0, y: 18.25, z: 0.0};
            let stv1 = Vector3f{x: 0.0, y: 18.25, z: 18.0};
            let stv2 = Vector3f{x: 0.0, y: 18.25, z: -18.0};
            let stv3 = Vector3f{x: 0.0, y: 18.25, z: 9.5};
            let stv4 = Vector3f{x: 0.0, y: 18.25, z: -9.5};
            let spin1: u32 = EffectModule::req_follow(boma, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv1, &Vector3f::zero(), 1.6, true, 0, 0, 0, 0, 0, true, true) as u32;
            let spin2: u32 = EffectModule::req_follow(boma, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv2, &Vector3f::zero(), 1.6, true, 0, 0, 0, 0, 0, true, true) as u32;
            let spin3: u32 = EffectModule::req_follow(boma, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv3, &Vector3f::zero(), 1.6, true, 0, 0, 0, 0, 0, true, true) as u32;
            let spin4: u32 = EffectModule::req_follow(boma, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv4, &Vector3f::zero(), 1.6, true, 0, 0, 0, 0, 0, true, true) as u32;
            let spin5: u32 = EffectModule::req_follow(boma, Hash40::new("sys_spin_wind"), Hash40::new("top"), &spv5, &Vector3f::zero(), 1.6, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star1: u32 = EffectModule::req_follow(boma, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &spv5, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star2: u32 = EffectModule::req_follow(boma, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &stv1, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star3: u32 = EffectModule::req_follow(boma, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &stv2, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star4: u32 = EffectModule::req_follow(boma, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.4, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star5: u32 = EffectModule::req_follow(boma, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.4, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star6: u32 = EffectModule::req_follow(boma, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &stv3, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            let star7: u32 = EffectModule::req_follow(boma, Hash40::new("sys_starrod_splash"), Hash40::new("top"), &stv4, &Vector3f::zero(), 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
            EffectModule::set_rgb(boma, spin1, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(boma, spin2, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(boma, spin3, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(boma, spin4, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(boma, spin5, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(boma, star1, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(boma, star2, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(boma, star3, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(boma, star4, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(boma, star5, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(boma, star6, 1.0, 0.0, 1.0);
            EffectModule::set_rgb(boma, star7, 1.0, 0.0, 1.0);
            EffectModule::set_alpha(boma, spin1, 0.15);
            EffectModule::set_alpha(boma, spin2, 0.15);
            EffectModule::set_alpha(boma, spin3, 0.15);
            EffectModule::set_alpha(boma, spin4, 0.15);
            EffectModule::set_alpha(boma, spin5, 0.2);
            EffectModule::set_alpha(boma, star1, 0.6);
            EffectModule::set_alpha(boma, star2, 0.6);
            EffectModule::set_alpha(boma, star3, 0.6);
            EffectModule::set_alpha(boma, star4, 0.6);
            EffectModule::set_alpha(boma, star5, 0.6);
            EffectModule::set_alpha(boma, star6, 0.6);
            EffectModule::set_alpha(boma, star7, 0.6);
        }
        wait(lua_state, 5.0);
    }
}

//Nair Sound
unsafe extern "C" fn ssbexo_koopa_nair_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 6.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_koopa_attackhard_s01"));
    }
}

//Nair Expression
unsafe extern "C" fn ssbexo_koopa_nair_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 6.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitm"), 11, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//Dair ACMD
unsafe extern "C" fn ssbexo_koopa_dair_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 14.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        HIT_NODE(agent, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("mouth2"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("footr"), *HIT_STATUS_XLU);
        HIT_NODE(agent, Hash40::new("footl"), *HIT_STATUS_XLU);
    }
    for _ in 0..9 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 0.7, 367, 75, 75, 0, 7.0, 0.0, -3.0, -1.0, None, None, None, 0.6, 0.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        }
        wait(lua_state, 2.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 1.0);
    }
    frame(lua_state, 44.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 20, 60, 0, 40, 7.0, 0.0, -3.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(lua_state, 50.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 61.0);
    if is_excute(agent) {
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 70.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

pub fn install() {
    Agent::new("koopa")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attackairn", ssbexo_koopa_nair_acmd, Low)
    .acmd("effect_attackairn", ssbexo_koopa_nair_effect, Low)
    .acmd("sound_attackairn", ssbexo_koopa_nair_sound, Low)
    .acmd("expression_attackairn", ssbexo_koopa_nair_expression, Low)
    .acmd("game_attackairlw", ssbexo_koopa_dair_acmd, Low)
    .install()
    ;
}