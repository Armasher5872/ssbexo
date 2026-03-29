use super::*;

//Side Special Fall ACMD
unsafe extern "C" fn ssbexo_armstrong_side_special_fall_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 4.0, 0, 10, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0.0);
    }
}

//Side Special Fall Effect
unsafe extern "C" fn ssbexo_armstrong_side_special_fall_effect(_agent: &mut L2CAgentBase) {}

//Side Special Fall Sound
unsafe extern "C" fn ssbexo_armstrong_side_special_fall_sound(_agent: &mut L2CAgentBase) {}

//Side Special Fall Expression
unsafe extern "C" fn ssbexo_armstrong_side_special_fall_expression(_agent: &mut L2CAgentBase) {}

pub fn install() {
    let mut costume = &mut Vec::new();
    unsafe {
        for i in 0..MARKED_COLORS.len() {
            if MARKED_COLORS[i] {
                costume.push(i);
            }
        }
    }
    Agent::new("ganon")
    .set_costume(costume.to_vec())
    .game_acmd("game_specialairsfall", ssbexo_armstrong_side_special_fall_acmd, Low)
    .effect_acmd("effect_specialairsfall", ssbexo_armstrong_side_special_fall_effect, Low)
    .sound_acmd("sound_specialairsfall", ssbexo_armstrong_side_special_fall_sound, Low)
    .expression_acmd("expression_specialairsfall", ssbexo_armstrong_side_special_fall_expression, Low)
    .install()
    ;
}