use super::*;

unsafe extern "C" fn ssbexo_pickel_forward_smash_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    let wood = *FIGHTER_PICKEL_MATERIAL_KIND_WOOD;
    let stone = *FIGHTER_PICKEL_MATERIAL_KIND_STONE;
    let iron = *FIGHTER_PICKEL_MATERIAL_KIND_IRON;
    let gold = *FIGHTER_PICKEL_MATERIAL_KIND_GOLD;
    let diamond = *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND;
    let mut damage = 7.0; //Default Damage
    let mut knockback_growth = 85;
    let mut durability = 14.0; //Defult Durability
    if is_excute(agent) {
        WorkModule::off_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        WorkModule::set_int(boma, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(lua_state, 6.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        if material_kind == gold {
            FT_MOTION_RATE(agent, 1.4);
        }
        else {
            FT_MOTION_RATE(agent, 2.0);
        }
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        FT_MOTION_RATE(agent, 1.0);
        if [wood, stone, iron, gold, diamond].contains(&material_kind) {
            match material_kind {
                _ if material_kind == wood => {
                    durability = 9.0;
                }
                _ if material_kind == stone => {
                    damage = 14.0;
                }
                _ if material_kind == iron => {
                    damage = 15.0;
                }
                _ if material_kind == gold => {
                    durability = 18.0;
                    knockback_growth = 101;
                }
                _ if material_kind == diamond => {
                    damage = 16.0;
                    knockback_growth = 90;
                }
                _ => {}
            }
            ATTACK(agent, 0, 0, Hash40::new("top"), damage, 42, knockback_growth, 0, 34, 3.8, 0.0, 8.0, 6.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 1, 0, Hash40::new("top"), damage, 42, knockback_growth, 0, 34, 4.4, 0.0, 8.0, 10.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            ATTACK(agent, 2, 0, Hash40::new("top"), damage, 42, knockback_growth, 0, 34, 4.6, 0.0, 8.0, 14.5, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            WorkModule::set_float(boma, durability, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        else {
            ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 42, 85, 0, 20, 3.8, 0.0, 8.0, 6.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 42, 85, 0, 20, 4.4, 0.0, 8.0, 10.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        if material_kind == gold {
            FT_MOTION_RATE(agent, 0.8);
        }
    }
}

unsafe extern "C" fn ssbexo_pickel_up_smash_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let material_kind = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    let wood = *FIGHTER_PICKEL_MATERIAL_KIND_WOOD;
    let stone = *FIGHTER_PICKEL_MATERIAL_KIND_STONE;
    let iron = *FIGHTER_PICKEL_MATERIAL_KIND_IRON;
    let gold = *FIGHTER_PICKEL_MATERIAL_KIND_GOLD;
    let diamond = *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND;
    let mut damage = 7.0; //Default Damage
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(lua_state, 7.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF, false, -1);
        ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 105, 100, 90, 0, 3.0, 0.0, 3.0, 8.0, Some(0.0), Some(3.0), Some(1.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    wait(lua_state, 2.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 13.0);
    FT_MOTION_RATE(agent, 10.0);
    frame(lua_state, 15.0);
    if is_excute(agent) {
        WorkModule::set_int(boma, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(lua_state, 16.0);
    FT_MOTION_RATE(agent, 1.0);
    frame(lua_state, 22.0);
    if is_excute(agent) {
        ArticleModule::remove_exist(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 80, 90, 0, 47, 8.0, 0.0, 26.5, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        if [wood, stone, iron, gold, diamond].contains(&material_kind) {
            match material_kind {
                _ if material_kind == stone => {
                    damage = 7.5;
                }
                _ if material_kind == iron => {
                    damage = 8.0;
                }
                _ if material_kind == diamond => {
                    damage = 8.5;
                }
                _ => {}
            }
            ATTACK(agent, 1, 0, Hash40::new("haver"), damage, 52, 82, 0, 56, 3.5, 4.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(boma, 0.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
    }
}

pub fn install() {
    Agent::new("pickel")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_attacks4", ssbexo_pickel_forward_smash_acmd, Low)
    .acmd("game_attackhi4", ssbexo_pickel_up_smash_acmd, Low)
    .install()
    ;
}