use super::*;

//Neutral Special Cape ACMD
unsafe extern "C" fn ssbexo_ganon_neutral_special_cape_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 8.0, 0.0, 4.0, 10.0, 0.0, 10.0, 10.0, 1.3, 1.3, 50, false, 1.3, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 30, 60, 0, 60, 7.5, 0.0, 4.0, 10.0, Some(0.0), Some(10.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
}

//Neutral Special Cape Effect
unsafe extern "C" fn ssbexo_ganon_neutral_special_cape_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
    }
}

//Neutral Special Cape Sound
unsafe extern "C" fn ssbexo_ganon_neutral_special_cape_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        let swipe = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ganon_special_n07"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, swipe as i32, 2.0, 0);
    }
}

//Neutral Special Cape Expression
unsafe extern "C" fn ssbexo_ganon_neutral_special_cape_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_01_mantle"), 0);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        AREA_WIND_2ND_arg10(agent, 0, 1, 0, 300, 0.5, 12, 10, 30, 20, 50);
    }
    frame(agent.lua_state_agent, 25.0);
    if is_excute(agent) {
        AreaModule::erase_wind(agent.module_accessor, 0);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialncape", ssbexo_ganon_neutral_special_cape_acmd, Low)
    .effect_acmd("effect_specialncape", ssbexo_ganon_neutral_special_cape_effect, Low)
    .sound_acmd("sound_specialncape", ssbexo_ganon_neutral_special_cape_sound, Low)
    .expression_acmd("expression_specialncape", ssbexo_ganon_neutral_special_cape_expression, Low)
    .game_acmd("game_specialairncape", ssbexo_ganon_neutral_special_cape_acmd, Low)
    .effect_acmd("effect_specialairncape", ssbexo_ganon_neutral_special_cape_effect, Low)
    .sound_acmd("sound_specialairncape", ssbexo_ganon_neutral_special_cape_sound, Low)
    .expression_acmd("expression_specialairncape", ssbexo_ganon_neutral_special_cape_expression, Low)
    .install()
    ;
}