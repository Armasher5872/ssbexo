use super::*;

//Barrel Roll ACMD
unsafe extern "C" fn ssbexo_donkey_barrel_roll_acmd(agent: &mut L2CAgentBase) {
    let situation_kind = StatusModule::situation_kind(agent.module_accessor);
    if situation_kind == *SITUATION_KIND_GROUND {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("rotx"), 10.0, 361, 25, 0, 50, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
    else {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("rotx"), 10.0, 75, 25, 0, 50, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        }
    }
}

pub fn install() {
    Agent::new("donkey_barrel")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_roll", ssbexo_donkey_barrel_roll_acmd, Low)
    .install()
    ;
}