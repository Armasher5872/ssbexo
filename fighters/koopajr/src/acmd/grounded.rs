use super::*;

//Rapid Jab ACMD
unsafe extern "C" fn ssbexo_koopajr_rapid_jab_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    loop {
        ssbexo_koopajr_rapid_jab_sub_right_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_koopajr_rapid_jab_sub_left_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_koopajr_rapid_jab_sub_right_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_koopajr_rapid_jab_sub_left_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_koopajr_rapid_jab_sub_right_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_koopajr_rapid_jab_sub_left_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_koopajr_rapid_jab_sub_right_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_koopajr_rapid_jab_sub_left_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_koopajr_rapid_jab_sub_right_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_koopajr_rapid_jab_sub_left_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_koopajr_rapid_jab_sub_right_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_koopajr_rapid_jab_sub_left_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_koopajr_rapid_jab_sub_right_acmd(agent);
        wait(lua_state, 2.0);
        ssbexo_koopajr_rapid_jab_sub_left_acmd(agent);
        wait(lua_state, 2.0);
        wait_loop_clear(agent);
    }
}

//Rapid Jab Sub Left ACMD
unsafe extern "C" fn ssbexo_koopajr_rapid_jab_sub_left_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 10, 0, 8, 5.5, 0.0, 6.5, 21.0, Some(0.0), Some(6.5), Some(13.0), 0.5, 0.35, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("clownhandl"), 0.5, 361, 10, 0, 8, 4.0, 2.0, 0.0, 0.0, Some(-4.0), Some(0.0), Some(0.0), 0.5, 0.35, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 3.0, false);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 8.0);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

//Rapid Jab Sub Right ACMD
unsafe extern "C" fn ssbexo_koopajr_rapid_jab_sub_right_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 361, 10, 0, 8, 5.5, 0.0, 6.5, 21.0, Some(0.0), Some(6.5), Some(13.0), 0.5, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(agent, 1, 0, Hash40::new("clownhandr"), 0.5, 361, 10, 0, 8, 4.0, 2.0, 0.0, 0.0, Some(-4.0), Some(0.0), Some(0.0), 0.5, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(boma, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 3.0, false);
        ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 8.0);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_koopajr_dash_attack_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 30, 10, 0, 45, 4.0, 0.0, 8.0, 9.0, Some(0.0), Some(8.0), Some(8.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 34, 10, 0, 45, 4.0, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(8.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 65, 10, 0, 20, 4.0, 0.0, 8.0, 18.0, Some(0.0), Some(8.0), Some(8.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    wait(lua_state, 4.0);
    if is_excute(agent) {
        AttackModule::clear(boma, 0, false);
        ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 32, 10, 0, 35, 4.0, 0.0, 8.0, 9.0, Some(0.0), Some(8.0), Some(8.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 65, 10, 0, 20, 4.0, 0.0, 8.0, 14.0, Some(0.0), Some(8.0), Some(8.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 8.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 25, 70, 0, 80, 5.0, 0.0, 8.0, 8.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        ATTACK(agent, 1, 0, Hash40::new("clowntongue3"), 4.0, 25, 70, 0, 80, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    Agent::new("koopajr")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attack100", ssbexo_koopajr_rapid_jab_acmd, Low)
    .acmd("game_attackdash", ssbexo_koopajr_dash_attack_acmd, Low)
    .install()
    ;
}