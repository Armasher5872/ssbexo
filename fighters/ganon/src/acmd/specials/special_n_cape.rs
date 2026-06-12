use super::*;

//Neutral Special Cape ACMD
unsafe extern "C" fn ssbexo_ganon_neutral_special_cape_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 7.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 8.0, 0.0, 4.0, 10.0, 0.0, 10.0, 10.0, 1.3, 1.3, 50, false, 1.3, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 30, 60, 0, 60, 7.5, 0.0, 4.0, 10.0, Some(0.0), Some(10.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    frame(lua_state, 20.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
}

//Neutral Special Cape Effect
unsafe extern "C" fn ssbexo_ganon_neutral_special_cape_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    frame(lua_state, 11.0);
    if is_excute(agent) {
        LANDING_EFFECT_FLIP(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
    }
}

//Neutral Special Cape Sound
unsafe extern "C" fn ssbexo_ganon_neutral_special_cape_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    frame(lua_state, 8.0);
    if is_excute(agent) {
        let swipe = SoundModule::play_se(boma, Hash40::new("se_ganon_special_n07"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(boma, swipe as i32, 2.0, 0);
    }
}

//Neutral Special Cape Expression
unsafe extern "C" fn ssbexo_ganon_neutral_special_cape_expression(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    frame(lua_state, 12.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 14.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_01_mantle"), 0);
    }
    frame(lua_state, 15.0);
    if is_excute(agent) {
        AREA_WIND_2ND_arg10(agent, 0, 1, 0, 300, 0.5, 12, 10, 30, 20, 50);
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        AreaModule::erase_wind(boma, 0);
    }
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_specialncape", ssbexo_ganon_neutral_special_cape_acmd, Low)
    .acmd("effect_specialncape", ssbexo_ganon_neutral_special_cape_effect, Low)
    .acmd("sound_specialncape", ssbexo_ganon_neutral_special_cape_sound, Low)
    .acmd("expression_specialncape", ssbexo_ganon_neutral_special_cape_expression, Low)
    .acmd("game_specialairncape", ssbexo_ganon_neutral_special_cape_acmd, Low)
    .acmd("effect_specialairncape", ssbexo_ganon_neutral_special_cape_effect, Low)
    .acmd("sound_specialairncape", ssbexo_ganon_neutral_special_cape_sound, Low)
    .acmd("expression_specialairncape", ssbexo_ganon_neutral_special_cape_expression, Low)
    .install()
    ;
}