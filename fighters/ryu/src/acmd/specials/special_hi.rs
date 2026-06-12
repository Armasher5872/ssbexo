use super::*;

//Shoryuken ACMD
unsafe extern "C" fn ssbexo_ryu_shoryuken_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let strength = WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if strength == *FIGHTER_RYU_STRENGTH_W {
        MotionModule::set_rate(boma, 1.2);
    }
    else if strength ==  *FIGHTER_RYU_STRENGTH_M {
        MotionModule::set_rate(boma, 1.0);
        HitModule::set_check_catch(boma, false, 0);
    }
    else {
        MotionModule::set_rate(boma, 0.857);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_XLU), 0);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
        ATTACK(agent, 0, 0, Hash40::new("top"), 14.0, 80, 64, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 1.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("handr"), 12.0, 80, 64, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        HitModule::set_check_catch(boma, true, 0);
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 9.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
        ATTACK(agent, 0, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 60, 5.5, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install() {
    Agent::new("ryu")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialhi", ssbexo_ryu_shoryuken_acmd, Low)
    .acmd("game_specialairhi", ssbexo_ryu_shoryuken_acmd, Low)
    .install()
    ;
}