use super::*;

//Up Special Open ACMD
unsafe extern "C" fn ssbexo_peach_up_special_open_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, Hash40::new("special_hi_open"), false, -1.0);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("havel"), 2.0, 80, 15, 0, 65, 2.0, -3.1, 6.5, 0.0, Some(3.1), Some(6.5), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 18, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PEACH_PARASOL, *ATTACK_REGION_PARASOL);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 20.0, false);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
}

//Grounded Down Special ACMD
unsafe extern "C" fn ssbexo_peach_grounded_down_special_acmd(agent: &mut L2CAgentBase) {
    let uniq_item_kind = WorkModule::get_int64(agent.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_UNIQ_ITEM_KIND);
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_PEACH_STATUS_SPECIAL_LW_WORK_INT_IS_DAIKON_GENERATABLE) == 1 {
        match uniq_item_kind {
           _ if uniq_item_kind == *ITEM_KIND_STARRING as u64 => {
                if macros::is_excute(agent) {
                    ItemModule::have_item(agent.module_accessor, ItemKind(*ITEM_KIND_STARRING), 0, 0, false, false);
                }
            }
            _ if uniq_item_kind == *ITEM_KIND_POWBLOCK as u64 => {
                if macros::is_excute(agent) {
                    ItemModule::have_item(agent.module_accessor, ItemKind(*ITEM_KIND_POWBLOCK), 0, 0, false, false);
                }
            }
            _ if uniq_item_kind == *ITEM_KIND_SOCCERBALL as u64 => {
                if macros::is_excute(agent) {
                    ItemModule::have_item(agent.module_accessor, ItemKind(*ITEM_KIND_SOCCERBALL), 0, 0, false, false);
                }
            }
            _ if uniq_item_kind == *ITEM_KIND_GREENSHELL as u64 => {
                if macros::is_excute(agent) {
                    ItemModule::have_item(agent.module_accessor, ItemKind(*ITEM_KIND_GREENSHELL), 0, 0, false, false);
                }
            }
            _ if uniq_item_kind == *ITEM_KIND_FIREBAR as u64 => {
                if macros::is_excute(agent) {
                    ItemModule::have_item(agent.module_accessor, ItemKind(*ITEM_KIND_FIREBAR), 0, 0, false, false);
                }
            }
            _ if uniq_item_kind == *ITEM_KIND_BEAMSWORD as u64 => {
                if macros::is_excute(agent) {
                    ItemModule::have_item(agent.module_accessor, ItemKind(*ITEM_KIND_BEAMSWORD), 0, 0, false, false);
                }
            }
            _ if uniq_item_kind == *ITEM_KIND_BOMBHEI as u64 => {
                if macros::is_excute(agent) {
                    ItemModule::have_item(agent.module_accessor, ItemKind(*ITEM_KIND_BOMBHEI), 0, 0, false, false);
                }
            }
            _ if uniq_item_kind == *ITEM_KIND_BOOMERANG as u64 => {
                if macros::is_excute(agent) {
                    ItemModule::have_item(agent.module_accessor, ItemKind(*ITEM_KIND_BOOMERANG), 0, 0, false, false);
                }
            }
            _ if uniq_item_kind == *ITEM_KIND_BANANA as u64 => {
                if macros::is_excute(agent) {
                    ItemModule::have_item(agent.module_accessor, ItemKind(*ITEM_KIND_BANANA), 0, 0, false, false);
                }
            }
            _ if uniq_item_kind == *ITEM_KIND_DOSEISAN as u64 => {
                if macros::is_excute(agent) {
                    ItemModule::have_item(agent.module_accessor, ItemKind(*ITEM_KIND_DOSEISAN), 0, 0, false, false);
                }
            }
            _ if uniq_item_kind == *ITEM_KIND_FREEZER as u64 => {
                if macros::is_excute(agent) {
                    ItemModule::have_item(agent.module_accessor, ItemKind(*ITEM_KIND_FREEZER), 0, 0, false, false);
                }
            }
            _ if uniq_item_kind == *ITEM_KIND_FIREFLOWER as u64 => {
                if macros::is_excute(agent) {
                    ItemModule::have_item(agent.module_accessor, ItemKind(*ITEM_KIND_FIREFLOWER), 0, 0, false, false);
                }
            }
            _ => {
                if macros::is_excute(agent) {
                    ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_DAIKON, false, -1);
                }
            }
        }
    }
}

//Aerial Down Special ACMD
unsafe extern "C" fn ssbexo_peach_aerial_down_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 0.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("haver"), 6.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
}

//Aerial Down Special Effect
unsafe extern "C" fn ssbexo_peach_aerial_down_special_effect(_agent: &mut L2CAgentBase) {}

//Aerial Down Special Sound
unsafe extern "C" fn ssbexo_peach_aerial_down_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_10"));
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_swing_10"));
    }
}

//Aerial Down Special Expression
unsafe extern "C" fn ssbexo_peach_aerial_down_special_expression(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_nohits"), 0);
    }
}

//Aerial Down Special Throw ACMD
unsafe extern "C" fn ssbexo_peach_aerial_down_special_throw_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 0, 65, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, 18, 10);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.0);
        smash::app::lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 18.0, y: 10.0, z: 0.0});
        macros::FT_CATCH_STOP(agent, 5, 1);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

//Aerial Down Special Throw Effect
unsafe extern "C" fn ssbexo_peach_aerial_down_special_throw_effect(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("peach_levitation"), Hash40::new("toer"), 1, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 5, 10, 1, 0, -90, -130, 1, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(agent, 1, 0.85, 0.9);
    }
}

//Aerial Down Special Throw Sound
unsafe extern "C" fn ssbexo_peach_aerial_down_special_throw_sound(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        macros::PLAY_STATUS(agent, Hash40::new("se_peach_jump04"));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    wait(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_02"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_peach_rnd_attack"));
    }
}

//Aerial Down Special Throw Expression
unsafe extern "C" fn ssbexo_peach_aerial_down_special_throw_expression(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_13_floating"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("top"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install() {
    Agent::new("peach")
    .game_acmd("game_specialhiopen", ssbexo_peach_up_special_open_acmd, Priority::Low)
    .game_acmd("game_speciallw", ssbexo_peach_grounded_down_special_acmd, Priority::Low)
    .game_acmd("game_specialairlw", ssbexo_peach_aerial_down_special_acmd, Priority::Low)
    .effect_acmd("effect_specialairlw", ssbexo_peach_aerial_down_special_effect, Priority::Low)
    .sound_acmd("sound_specialairlw", ssbexo_peach_aerial_down_special_sound, Priority::Low)
    .expression_acmd("expression_specialairlw", ssbexo_peach_aerial_down_special_expression, Priority::Low)
    .game_acmd("game_specialairlwthrow", ssbexo_peach_aerial_down_special_throw_acmd, Priority::Low)
    .effect_acmd("effect_specialairlwthrow", ssbexo_peach_aerial_down_special_throw_effect, Priority::Low)
    .sound_acmd("sound_specialairlwthrow", ssbexo_peach_aerial_down_special_throw_sound, Priority::Low)
    .expression_acmd("expression_specialairlwthrow", ssbexo_peach_aerial_down_special_throw_expression, Priority::Low)
    .install()
    ;
}