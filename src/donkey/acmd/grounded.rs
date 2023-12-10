use super::*;

//Heavy Throw Forward
unsafe extern "C" fn ssbexo_donkey_heavy_throw_forward_acmd(agent: &mut L2CAgentBase) {
    let timer = WorkModule::get_int(agent.module_accessor, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_TIMER);
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE) && timer > 0 {
            if ItemModule::get_have_item_kind(agent.module_accessor, 0) == *ITEM_KIND_BARREL {
                ItemModule::throw_item(agent.module_accessor, 10.0, 1.5, 0.0, 0, true, 0.0);
            }
        }
        else {
            agent.clear_lua_stack();
            lua_args!(agent, 21, 10, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_ANGLE, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_SPEED, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_POWER);
            smash::app::sv_animcmd::THROW_ITEM_OFFSET(agent.lua_state_agent);
        }
    }
}

//Dash Attack ACMD
unsafe extern "C" fn ssbexo_donkey_dash_attack_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::set_flag(agent.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
        WorkModule::set_flag(agent.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
        WorkModule::set_float(agent.module_accessor, 0.5, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 53, 46, 0, 85, 10.0, 0.0, 8.0, 2.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 45, 60, 0, 60, 10.0, 0.0, 8.0, 2.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::set_flag(agent.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
        WorkModule::set_flag(agent.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
        WorkModule::set_flag(agent.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
    }
}

//Aerial Dash Attack ACMD
unsafe extern "C" fn ssbexo_donkey_aerial_dash_attack_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 53, 46, 0, 85, 10.0, 0.0, 8.0, 2.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 45, 60, 0, 60, 10.0, 0.0, 8.0, 2.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::set_flag(agent.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
        WorkModule::set_flag(agent.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
        WorkModule::set_flag(agent.module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
    }
}

pub fn install() {
    Agent::new("donkey")
    .game_acmd("game_itemheavythrowf", ssbexo_donkey_heavy_throw_forward_acmd)
    .game_acmd("game_attackdash", ssbexo_donkey_dash_attack_acmd)
    .game_acmd("game_attackairdash", ssbexo_donkey_aerial_dash_attack_acmd)
    .install()
    ;
}