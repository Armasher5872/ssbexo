use super::*;

//Neutral Special Start ACMD
#[acmd_script( agent = "snake", scripts = ["game_specialnstart", "game_specialairnstart"], category = ACMD_GAME)]
unsafe fn ssbuexo_snake_neutral_special_start_acmd(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        if GRENADE_HOLD[entry_id] != true {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, false, -1);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, false, -1);
            ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE_PIN, true, smash::app::ArticleOperationTarget(0));
        }
        else {
            ItemModule::have_item(fighter.module_accessor, smash::app::ItemKind(*ITEM_KIND_SMOKESCREEN), 0, 0, false, false);
        }
        macros::CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
    frame(fighter.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
}

//C4 ACMD
#[acmd_script( agent = "snake_c4", script = "game_explosion", category = ACMD_GAME)]
unsafe fn ssbuexo_snake_c4_acmd(fighter: &mut L2CAgentBase) 
{
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 17.0, 80, 90, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        macros::ATTACK(fighter, 1, 0, Hash40::new("rot"), 13.0, 60, 90, 0, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        macros::ATTACK(fighter, 2, 0, Hash40::new("rot"), 9.0, 40, 90, 0, 40, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
        VisibilityModule::set_whole(fighter.module_accessor, false);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_erase"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::set_size(fighter.module_accessor, 0, 9.0);
        AttackModule::set_size(fighter.module_accessor, 1, 14.0);
        AttackModule::set_size(fighter.module_accessor, 2, 21.0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 2.0);
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_SNAKE_C4_INSTANCE_WORK_ID_FLAG_GROUND) == false {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 3.0);
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_SNAKE_C4_INSTANCE_WORK_ID_FLAG_GROUND) == true {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_snake_neutral_special_start_acmd,
        ssbuexo_snake_c4_acmd
    );
}