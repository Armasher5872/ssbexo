use super::*;

//Fire Teraflare Burst ACMD
unsafe extern "C" fn ssbexo_edge_fire_teraflare_burst_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let owner_boma = get_owner_boma(agent);
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_explosionl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(agent, *CAMERA_QUAKE_KIND_XL);
        AttackModule::disable_tip(boma);
        ATTACK(agent, 0, 0, Hash40::new("top"), 40.0, 54, 115, 0, 105, 22.5, 0.0, 0.0, 0.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 1, 0, Hash40::new("top"), 30.0, 54, 115, 0, 105, 32.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        ATTACK(agent, 2, 0, Hash40::new("top"), 20.0, 54, 115, 0, 105, 40.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        WorkModule::off_flag(owner_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 51.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

//Fire Teraflare Burst Effect
unsafe extern "C" fn ssbexo_edge_fire_teraflare_burst_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        EFFECT(agent, Hash40::new("edge_fire4_sign"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        EffectModule::req_screen(boma, Hash40::new("edge_fire3_screen2"), false, false, false);
        EFFECT(agent, Hash40::new("edge_fire4_burst"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        EFFECT(agent, Hash40::new("edge_fire4_burst2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
}

//Fire Teraflare Burst Sound
unsafe extern "C" fn ssbexo_edge_fire_teraflare_burst_sound(_agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("edge_fire")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_burstxl", ssbexo_edge_fire_teraflare_burst_acmd, Low)
    .acmd("effect_burstxl", ssbexo_edge_fire_teraflare_burst_effect, Low)
    .acmd("sound_burstxl", ssbexo_edge_fire_teraflare_burst_sound, Low)
    .install()
    ;
}