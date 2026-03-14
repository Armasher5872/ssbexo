use super::*;

//Side Special ACMD
unsafe extern "C" fn ssbexo_mario_side_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        let lr = PostureModule::lr(agent.module_accessor);
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_AIR {
            SA_SET(agent, *SITUATION_KIND_AIR);
            GroundModule::correct(agent.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 2.3);
        }
        else {
            KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 1.2);
        }
        sv_kinetic_energy!(set_speed, agent, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 2.3*lr, 0.0);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::enable_energy(agent.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(set_accel, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -0.18);
        sv_kinetic_energy!(set_limit_speed, agent, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 3.9);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("bust"), 7.0, 65, 60, 0, 50, 4.5, 1.9, -0.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
}

//Side Special Effect
unsafe extern "C" fn ssbexo_mario_side_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

//Side Special Sound
unsafe extern "C" fn ssbexo_mario_side_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("vc_mario_012"));
        PLAY_SE(agent, Hash40::new("se_mario_jump01"));
    }
}

//Side Special Expression
unsafe extern "C" fn ssbexo_mario_side_special_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 15.0);
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6, true);
    }
}

pub fn install() {
    Agent::new("mario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specials", ssbexo_mario_side_special_acmd, Low)
    .game_acmd("game_specialairs", ssbexo_mario_side_special_acmd, Low)
    .effect_acmd("effect_specials", ssbexo_mario_side_special_effect, Low)
    .effect_acmd("effect_specialairs", ssbexo_mario_side_special_effect, Low)
    .sound_acmd("sound_specials", ssbexo_mario_side_special_sound, Low)
    .sound_acmd("sound_specialairs", ssbexo_mario_side_special_sound, Low)
    .expression_acmd("expression_specials", ssbexo_mario_side_special_expression, Low)
    .expression_acmd("expression_specialairs", ssbexo_mario_side_special_expression, Low)
    .install()
    ;
}