use super::*;

//Fire Fox ACMD
#[acmd_script( agent = "fox", script = "game_specialhi", category = ACMD_GAME)]
unsafe fn ssbuexo_fox_firefox_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 16.0, 60, 60, 0, 70, 5.0, 3.0, -1.5, 0.0, Some(2.0), Some(-1.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 10.0, 60, 50, 0, 85, 5.0, 3.0, -1.5, 0.0, Some(-2.0), Some(-1.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
}

//Reflector ACMD
#[acmd_script( agent = "fox", scripts = ["game_speciallwstart", "game_specialairlwstart"], category = ACMD_GAME)]
unsafe fn ssbuexo_fox_reflector_acmd(fighter: &mut L2CAgentBase) 
{
    let parried = WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        if parried == 1 {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 330, 32, 0, 66, 7.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 330, 32, 0, 66, 7.5, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_fox_firefox_acmd,
        ssbuexo_fox_reflector_acmd
    );
}