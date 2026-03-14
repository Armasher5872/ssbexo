use super::*;

//Side Special Loop ACMD
unsafe extern "C" fn ssbexo_mario_side_special_loop_acmd(agent: &mut L2CAgentBase) {
    let attack_frame = WorkModule::get_float(agent.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_ATTACK_FRAME);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HIT) && attack_frame <= 32.0 {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("bust"), 7.0, 65, 60, 0, 50, 4.5, 1.9, -0.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
    }
}

//Side Special Loop Effect
unsafe extern "C" fn ssbexo_mario_side_special_loop_effect(_agent: &mut L2CAgentBase) {}

//Side Special Loop Sound
unsafe extern "C" fn ssbexo_mario_side_special_loop_sound(_agent: &mut L2CAgentBase) {}

//Side Special Loop Expression
unsafe extern "C" fn ssbexo_mario_side_special_loop_expression(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    Agent::new("mario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specialsloop", ssbexo_mario_side_special_loop_acmd, Low)
    .effect_acmd("effect_specialsloop", ssbexo_mario_side_special_loop_effect, Low)
    .sound_acmd("sound_specialsloop", ssbexo_mario_side_special_loop_sound, Low)
    .expression_acmd("expression_specialsloop", ssbexo_mario_side_special_loop_expression, Low)
    .install()
    ;
}