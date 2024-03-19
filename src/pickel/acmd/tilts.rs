use super::*;

unsafe extern "C" fn ssbexo_pickel_forward_tilt_acmd(agent: &mut L2CAgentBase) {
    let situation_kind = WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_SITUATION_KIND);
    let material_kind = WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    let wood = *FIGHTER_PICKEL_MATERIAL_KIND_WOOD;
    let stone = *FIGHTER_PICKEL_MATERIAL_KIND_STONE;
    let iron = *FIGHTER_PICKEL_MATERIAL_KIND_IRON;
    let gold = *FIGHTER_PICKEL_MATERIAL_KIND_GOLD;
    let diamond = *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND;
    let mut sfx = *COLLISION_SOUND_ATTR_PUNCH;
    let mut damage = 1.0; //Default Damage
    let mut hitstun: [f32; 2] = [0.0, 0.0]; //Additional Hitstun
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(agent.lua_state_agent, 1.0);
    if situation_kind == *SITUATION_KIND_GROUND {
        if macros::is_excute(agent) {
            WorkModule::set_int(agent.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
        }
    }
    frame(agent.lua_state_agent, 2.0);
    if material_kind == gold {
        if macros::is_excute(agent) {
            MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
        }
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        if [wood, stone, iron, gold, diamond].contains(&material_kind) {
            if [iron, gold, diamond].contains(&material_kind) {
                sfx = *COLLISION_SOUND_ATTR_CUTUP;
            }
            match material_kind {
                _ if material_kind == wood => {
                    hitstun = [-7.0, -5.0];
                }
                _ if material_kind == stone => {
                    damage = 1.5;
                    hitstun = [-5.0, -3.0];
                }
                _ if material_kind == iron => {
                    damage = 2.0;
                    hitstun = [-3.0, -1.0];
                }
                _ if material_kind == gold => {
                    hitstun = [-4.0, -2.0];
                }
                _ if material_kind == diamond => {
                    damage = 2.5;
                }
                _ => {}
            }
            macros::ATTACK(agent, 0, 0, Hash40::new("haver"), damage, 45, 134, 0, 27, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("haver"), damage, 45, 134, 0, 27, 2.3, 0.0, 3.1, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("haver"), damage, 45, 134, 0, 27, 2.3, 0.0, 6.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), damage, 45, 134, 0, 27, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 4, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 5, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 3.1, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 6, 0, Hash40::new("haver"), damage, 57, 134, 0, 20, 2.3, 0.0, 6.2, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 7, 0, Hash40::new("top"), damage, 57, 134, 0, 20, 2.3, 0.0, 6.8, 5.4, Some(0.0), Some(6.8), Some(10.2), 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, sfx, *ATTACK_REGION_SWORD);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 0.25);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 4, 5, 6, 7, 0.25);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, hitstun[0], false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, hitstun[0], false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, hitstun[0], false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, hitstun[0], false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 4, hitstun[1], false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 5, hitstun[1], false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 6, hitstun[1], false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 7, hitstun[1], false);
            WorkModule::set_float(agent.module_accessor, 2.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        else {
            macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 0.5, 60, 50, 0, 72, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(agent, 4, 0, Hash40::new("haver"), 0.5, 60, 50, 0, 72, 2.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 0, 1, 2, 3, 0.25);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg5(agent, 4, 5, 6, 7, 0.25);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, -10.0, false);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 4, -10.0, false);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 1.0);
    if material_kind == gold {
        if macros::is_excute(agent) {
            MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.25);
        }
        frame(agent.lua_state_agent, 16.0);
        if macros::is_excute(agent) {
            MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        }
        else {
            if macros::is_excute(agent) {
                MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
            }
        }
    }
}

unsafe extern "C" fn ssbexo_pickel_up_tilt_acmd(agent: &mut L2CAgentBase) {
    let material_kind = WorkModule::get_int(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    let wood = *FIGHTER_PICKEL_MATERIAL_KIND_WOOD;
    let stone = *FIGHTER_PICKEL_MATERIAL_KIND_STONE;
    let iron = *FIGHTER_PICKEL_MATERIAL_KIND_IRON;
    let gold = *FIGHTER_PICKEL_MATERIAL_KIND_GOLD;
    let diamond = *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND;
    let mut damage = 3.5; //Default Damage
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::set_int(agent.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_AXE, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
    frame(agent.lua_state_agent, 2.0);
    if material_kind == gold {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 0.8);
            MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 0.8);
        }
    }
    else {
        if macros::is_excute(agent) {
            MotionModule::set_rate(agent.module_accessor, 0.571);
            MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 0.571);
        }
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        MotionModule::set_rate(agent.module_accessor, 1.0);
        MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
        if [wood, stone, iron, gold, diamond].contains(&material_kind) {
            match material_kind {
                _ if material_kind == stone => {
                    damage = 4.0;
                }
                _ if material_kind == iron => {
                    damage = 4.5;
                }
                _ if material_kind == diamond => {
                    damage = 5.0;
                }
                _ => {}
            }
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), damage, 86, 78, 0, 52, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("haver"), damage, 86, 78, 0, 52, 5.4, 0.0, 4.2, 0.0, None, None, None, 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            WorkModule::set_float(agent.module_accessor, 5.5, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
        else {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 3.0, 86, 78, 0, 52, 4.4, 0.6, 0.4, 0.0, None, None, None, 1.0, 1.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    if material_kind == gold {
        if macros::is_excute(agent) {
            MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 0.5);
        }
        frame(agent.lua_state_agent, 18.0);
        if macros::is_excute(agent) {
            MotionModule::set_rate_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
        }
    }
}

pub fn install() {
    Agent::new("pickel")
    .game_acmd("game_attacks3", ssbexo_pickel_forward_tilt_acmd)
    .game_acmd("game_attackhi3", ssbexo_pickel_up_tilt_acmd)
    .install()
    ;
}