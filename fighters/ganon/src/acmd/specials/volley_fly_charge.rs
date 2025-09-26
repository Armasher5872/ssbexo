use super::*;

//Volley Fly Charge ACMD
unsafe extern "C" fn ssbexo_ganon_volley_fly_charge_acmd(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ATTACK(agent, 0, 0, Hash40::new_raw(0x0c12cb676b), 8.0, 361, 80, 0, 60, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        ATTACK(agent, 1, 0, Hash40::new_raw(0x0be7a40ca7), 8.0, 361, 80, 0, 60, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        ATTACK(agent, 2, 0, Hash40::new_raw(0x0aa78d649a), 8.0, 361, 80, 0, 60, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        ATTACK(agent, 3, 0, Hash40::new_raw(0x080482867e), 8.0, 361, 80, 0, 60, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        ATTACK(agent, 4, 0, Hash40::new_raw(0x0ac11513c9), 8.0, 361, 80, 0, 60, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
}

//Volley Fly Charge Effect
unsafe extern "C" fn ssbexo_ganon_volley_fly_charge_effect(_agent: &mut L2CAgentBase) {}

//Volley Fly Charge Sound
unsafe extern "C" fn ssbexo_ganon_volley_fly_charge_sound(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        let glow = SoundModule::play_se(agent.module_accessor, Hash40::new("se_ganon_special_n05"), true, false, false, false, smash::app::enSEType(0));
        SoundModule::set_se_vol(agent.module_accessor, glow as i32, 2.0, 0);
        SoundModule::set_remain_se(agent.module_accessor, true);
    }
}

pub fn install() {
    Agent::new("ganon_volley")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_flycharge", ssbexo_ganon_volley_fly_charge_acmd, Low)
    .effect_acmd("effect_flycharge", ssbexo_ganon_volley_fly_charge_effect, Low)
    .sound_acmd("sound_flycharge", ssbexo_ganon_volley_fly_charge_sound, Low)
    .install()
    ;
}