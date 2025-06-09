use super::*;

unsafe extern "C" fn ssbexo_tantan_hurtbox_disable_acmd(_agent: &mut L2CAgentBase) {}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_tantan_dash_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 5.5, 6.5, 9.0, 1.5);
    }
    frame(agent.lua_state_agent, 7.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), 8.0, 40, 50, 0, 85, 4.0, 0.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 10.0, 40, 50, 0, 85, 3.5, 0.0, 0.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("footl"), 10.0, 40, 50, 0, 85, 3.5, 0.0, 0.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    }
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("legl"), 6.0, 40, 50, 0, 85, 4.0, 0.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 1, 0, Hash40::new("kneel"), 8.0, 40, 50, 0, 85, 3.0, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(agent, 2, 0, Hash40::new("footl"), 8.0, 40, 50, 0, 85, 3.0, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 23.0);
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 4.0, 2.9, 7.2, 7.2);
    }
}

pub fn install() {
    Agent::new("tantan")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("0x172240c033", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("0x1778b4545c", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("0x18bc531aa8", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_aircatchhang", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_aircatchhit", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_aircatchloop", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_aircatchpose", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_attacklongstartl2", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_attacklongstartl3", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_attacklongstartlb2", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_attacklongstartlb3", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_attacklongstartr2", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_attacklongstartr3", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_attacklongstartrb2", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_attacklongstartrb3", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_attackshortstartl2", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_attackshortstartl3", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_attackshortstartlb1", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_attackshortstartlb2", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_attackshortstartlb3", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_attackshortstartr2", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_attackshortstartr3", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_attackshortstartrb1", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_attackshortstartrb2", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_attackshortstartrb3", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_catchpull", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_damageair2", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_specialairhi2", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_specialhilong", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_specialhishort", ssbexo_tantan_hurtbox_disable_acmd, Low)
    .game_acmd("game_attackdash", ssbexo_tantan_dash_attack_acmd, Low)
    .install()
    ;
}