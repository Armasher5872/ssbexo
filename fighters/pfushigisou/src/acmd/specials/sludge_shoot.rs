use super::*;

//Sludge Bomb ACMD
unsafe extern "C" fn ssbexo_pfushigisou_sludge_bomb_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 80, 100, 20, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_poison_param(agent.module_accessor, 0, 40, 5, 0.5, false);
    }
}

//Sludge Bomb Effect
unsafe extern "C" fn ssbexo_pfushigisou_sludge_bomb_effect(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        EFFECT_FOLLOW(agent, Hash40::new("packun_poison_gas"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 500, true);
    }
}

pub fn install() {
    Agent::new("pfushigisou_sludge")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_shoot", ssbexo_pfushigisou_sludge_bomb_acmd, Low)
    .effect_acmd("effect_shoot", ssbexo_pfushigisou_sludge_bomb_effect, Low)
    .install()
    ;
}