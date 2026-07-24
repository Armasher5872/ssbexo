use super::*;

//Scintilla Attack ACMD
unsafe extern "C" fn ssbexo_edge_scintilla_attack_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let owner_boma = get_owner_boma(agent);
    let charge = WorkModule::get_int(owner_boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_RETALIATION_CHARGE);
    let multihit_damage = if charge >= 2 {2.0} else if charge == 1 {1.5} else {1.0};
    let final_damage = if charge >= 2 {8.0} else if charge == 1 {6.5} else {5.0};
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), multihit_damage, 366, 65, 60, 40, 9.5, 0.0, 0.0, 0.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting_flash"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), final_damage, 361, 75, 0, 66, 11.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, -1.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting_flash"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_MAGIC);
        SEARCH(agent, 0, 0, Hash40::new("top"), 11.0, 0.0, 0.0, 0.0, None, None, None, *COLLISION_KIND_MASK_AH, *HIT_STATUS_MASK_ALL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false);
    }
    frame(lua_state, 17.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

//Scintilla Attack Effect
unsafe extern "C" fn ssbexo_edge_scintilla_attack_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    if is_excute(agent) {
        EFFECT_FOLLOW_RND(agent, Hash40::new("edge_senkou_slash"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 0.5, 7, 7, 7, 0, 0, 0, false);
        agent.clear_lua_stack();
        lua_args!(agent, *WEAPON_EDGE_FLASH_INSTANCE_WORK_ID_EFFECT_ID);
        LAST_EFFECT_SET_WORK_INT(agent.lua_state_agent);
        EFFECT_FOLLOW(agent, Hash40::new("edge_senkou_hold"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
        agent.clear_lua_stack();
        lua_args!(agent, *WEAPON_EDGE_FLASH_INSTANCE_WORK_ID_EFFECT_ID);
        LAST_EFFECT_SET_WORK_INT(agent.lua_state_agent);
    }
    wait(lua_state, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("edge_senkou_hold_end"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
        agent.clear_lua_stack();
        lua_args!(agent, *WEAPON_EDGE_FLASH_INSTANCE_WORK_ID_EFFECT_ID);
        LAST_EFFECT_SET_WORK_INT(agent.lua_state_agent);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("edge_senkou_hold"), -1);
        EFFECT_DETACH_KIND(agent, Hash40::new("edge_senkou_hold_end"), -1);
    }
}

pub fn install() {
    Agent::new("edge_flash")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attack", ssbexo_edge_scintilla_attack_acmd, Low)
    .acmd("effect_attack", ssbexo_edge_scintilla_attack_effect, Low)
    .install()
    ;
}