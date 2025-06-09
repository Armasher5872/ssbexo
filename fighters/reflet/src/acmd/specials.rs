use super::*;

//Side Special ACMD
unsafe extern "C" fn ssbexo_reflet_side_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_SHOOT_OK) {
            WorkModule::sub_float(agent.module_accessor, 1.0, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT);
            if WorkModule::get_float(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT) <= 0.0 {
                FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_GIGA_FIRE, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
            }
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_REFLET_GENERATE_ARTICLE_GIGAFIRE, false, -1);
        }
    }
}

//Side Special Effect
unsafe extern "C" fn ssbexo_reflet_side_special_effect(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_SHOOT_OK) {
        frame(agent.lua_state_agent, 4.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("reflet_gigafire_hold"), Hash40::new("top"), -1, 22, 1.5, 0, 0, 0, 0.8, true);
        }
        frame(agent.lua_state_agent, 15.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("reflet_gigafire_hand"), Hash40::new("handr"), 1, 1, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
        frame(agent.lua_state_agent, 16.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("reflet_gigafire_hand"), Hash40::new("handl"), 1, 1, 0, 0, 0, 0, 1, true);
            EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        }
    }
    else {
        frame(agent.lua_state_agent, 15.0);
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("reflet_book_smoke"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.7, true);
        }
    }
}

//Side Special Sound
unsafe extern "C" fn ssbexo_reflet_side_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_S_FLAG_SHOOT_OK) {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_ENHANCED_MAGIC_ACTIVE) {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_reflet_special_s01"));
                PLAY_SE(agent, Hash40::new("vc_reflet_final02"));
            }
        }
        else {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_reflet_special_s01"));
                PLAY_SEQUENCE(agent, Hash40::new("seq_reflet_rnd_special_s"));
            }
        }
    }
    else {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_reflet_mp_empty"));
            PLAY_SEQUENCE(agent, Hash40::new("seq_reflet_rnd_special_empty"));
        }
    }
}

//Arcfire Shoot ACMD
unsafe extern "C" fn ssbexo_reflet_arcfire_shoot_acmd(agent: &mut L2CAgentBase) {
    let owner_boma = sv_battle_object::module_accessor((WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if !WorkModule::is_flag(agent.module_accessor, *WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_IS_GOEITA) {
        if WorkModule::is_flag(owner_boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_ENHANCED_MAGIC_ACTIVE) {
            if is_excute(agent) {
                WorkModule::on_flag(agent.module_accessor, *WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_IS_BOLGANONE);
                ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 361, 60, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -0.6, 0.0, 8, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
            }
        }
        else {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 115, 100, 40, 0, 1.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
            }
        }
    }
    frame(agent.lua_state_agent, 120.0);
    if WorkModule::is_flag(agent.module_accessor, *WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_IS_GOEITA) {
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 160, 50, 0, 50, 15.0, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        }
    }
    frame(agent.lua_state_agent, 129.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_IS_GOEITA);
    }
}

//Arcfire Shoot Effect
unsafe extern "C" fn ssbexo_reflet_arcfire_shoot_effect(agent: &mut L2CAgentBase) {
    let owner_boma = sv_battle_object::module_accessor((WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let shoot_angle = WorkModule::get_param_float(agent.module_accessor, hash40("param_gigafire"), hash40("shoot_angle"));
    let rot_x = shoot_angle.to_radians().cos();
    if WorkModule::is_flag(owner_boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_ENHANCED_MAGIC_ACTIVE) {
        if WorkModule::is_flag(agent.module_accessor, *WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_IS_GOEITA) {
            if is_excute(agent) {
                EFFECT_OFF_KIND(agent, Hash40::new("sys_explosion_sign"), false, false);
                EFFECT_OFF_KIND(agent, Hash40::new("sys_metamon_aura"), false, false);
                EFFECT_OFF_KIND(agent, Hash40::new("sys_explosion_flash"), false, false);
                EFFECT(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0.0, 0.0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
                LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 1.0);
            }
        }
        else {
            if is_excute(agent) {
                let effect_id_bullet = EffectModule::req_follow(agent.module_accessor, Hash40::new("finreflet_magic_fire_loop"), Hash40::new("top"), &Vector3f::zero(), &Vector3f{x: rot_x, y: 0.0, z: 0.0}, 1.0, false, 0, 0, -1, 0, 0, false, false) as u32;
                WorkModule::set_int(agent.module_accessor, effect_id_bullet as i32, *WEAPON_REFLET_THUNDER_INSTANCE_WORK_ID_INT_EFFECT_ID_BULLET);
            }
        }
    }
    else {
        if is_excute(agent) {
            let effect_id_bullet = EffectModule::req_follow(agent.module_accessor, Hash40::new("reflet_gigafire_bullet"), Hash40::new("top"), &Vector3f::zero(), &Vector3f{x: rot_x, y: 0.0, z: 0.0}, 1.0, false, 0, 0, -1, 0, 0, false, false) as u32;
            WorkModule::set_int(agent.module_accessor, effect_id_bullet as i32, *WEAPON_REFLET_THUNDER_INSTANCE_WORK_ID_INT_EFFECT_ID_BULLET);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if WorkModule::is_flag(agent.module_accessor, *WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_IS_GOEITA) {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_metamon_aura"), Hash40::new("top"), 0.0, 0.0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("reflet_rizaia"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(agent.lua_state_agent, 60.0);
    if WorkModule::is_flag(agent.module_accessor, *WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_IS_GOEITA) {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("reflet_rizaia"), false, true);
            EFFECT(agent, Hash40::new("sys_metamon_aura"), Hash40::new("top"), 0.0, 0.0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("reflet_rizaia_capture"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT(agent, Hash40::new("sys_explosion_sign"), Hash40::new("top"), 0.0, 0.0, 0.5, 0, 0, 0, 0.85, 5, 5, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 1.0);
            LAST_EFFECT_SET_RATE(agent, 1.1);
        }
    }
    frame(agent.lua_state_agent, 80.0);
    if WorkModule::is_flag(agent.module_accessor, *WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_IS_GOEITA) {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("sys_explosion_sign"), false, false);
            EFFECT_OFF_KIND(agent, Hash40::new("sys_metamon_aura"), false, false);
        }
    }
    frame(agent.lua_state_agent, 90.0);
    if WorkModule::is_flag(agent.module_accessor, *WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_IS_GOEITA) {
        if is_excute(agent) {
            EFFECT(agent, Hash40::new("sys_explosion_flash"), Hash40::new("top"), 0.0, 0.0, 0, 0, 0, 0, 0.8, 5, 5, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 1.0);
        }
    }
    frame(agent.lua_state_agent, 120.0);
    if WorkModule::is_flag(agent.module_accessor, *WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_IS_GOEITA) {
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("sys_explosion_flash"), false, false);
            EFFECT(agent, Hash40::new("sys_hit_purple"), Hash40::new("top"), 0.0, 0.0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            EFFECT(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0.0, 0.0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("reflet_rizaia"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("reflet_rizaia_capture"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT(agent, Hash40::new("sys_dead_dark"), Hash40::new("top"), 0.0, 0.0, 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, false);
            LAST_EFFECT_SET_RATE(agent, 1.85);
        }
    }
}

//Arcfire Shoot Sound
unsafe extern "C" fn ssbexo_reflet_arcfire_shoot_sound(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_IS_GOEITA) {
        if is_excute(agent) {
            let start = SoundModule::play_se(agent.module_accessor, Hash40::new("se_reflet_special_l01"), true, false, false, false, smash::app::enSEType(0));
            SoundModule::set_se_vol(agent.module_accessor, start as i32, 1.1, 0);
        }
    }
    else {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_reflet_special_s02"));
        }
    }
    frame(agent.lua_state_agent, 30.0);
    if WorkModule::is_flag(agent.module_accessor, *WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_IS_GOEITA) {
        if is_excute(agent) {
            let vortex = SoundModule::play_se(agent.module_accessor, Hash40::new("se_reflet_special_n01"), true, false, false, false, smash::app::enSEType(0));
            SoundModule::set_se_vol(agent.module_accessor, vortex as i32, 0.8, 0);
        }   
    }
    frame(agent.lua_state_agent, 90.0);
    if WorkModule::is_flag(agent.module_accessor, *WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_IS_GOEITA) {
        if is_excute(agent) {
            SoundModule::stop_se(agent.module_accessor, Hash40::new("se_reflet_special_n01"), 0);
            let implode = SoundModule::play_se(agent.module_accessor, Hash40::new("se_reflet_final14"), true, false, false, false, smash::app::enSEType(0));
            SoundModule::set_se_vol(agent.module_accessor, implode as i32, 0.8, 0);
        }   
    }
    frame(agent.lua_state_agent, 120.0);
    if WorkModule::is_flag(agent.module_accessor, *WEAPON_REFLET_GIGAFIRE_INSTANCE_WORK_ID_FLAG_IS_GOEITA) {
        if is_excute(agent) {
            SoundModule::stop_se(agent.module_accessor, Hash40::new("se_reflet_final14"), 0);
            let bomb = SoundModule::play_se(agent.module_accessor, Hash40::new("se_common_bomb_ll"), true, false, false, false, smash::app::enSEType(0));
            SoundModule::set_se_vol(agent.module_accessor, bomb as i32, 1.5, 0);
        }   
    }
}

//Up Special ACMD
unsafe extern "C" fn ssbexo_reflet_up_special_acmd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, -1);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
    }
    frame(agent.lua_state_agent, 17.0);
    if is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
    }
    frame(agent.lua_state_agent, 30.0);
    if is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

//Down Special ACMD
unsafe extern "C" fn ssbexo_reflet_down_special_acmd(agent: &mut L2CAgentBase) {
    FT_MOTION_RATE(agent, 0.7);
    wait(agent.lua_state_agent, 20.0);
    FT_MOTION_RATE(agent, 2.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE) {
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_ENHANCED_MAGIC_ACTIVE) {
            if is_excute(agent) {
                WorkModule::sub_int(agent.module_accessor, 1, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT);
                if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT) <= 0 {
                    FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
                }
                ArticleModule::generate_article(agent.module_accessor, *FIGHTER_REFLET_GENERATE_ARTICLE_GIGAFIRE, false, -1);
            }
        }
        else {
            if is_excute(agent) {
                WorkModule::sub_int(agent.module_accessor, 1, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT);
                if WorkModule::get_int(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT) <= 0 {
                    FighterSpecializer_Reflet::set_flag_to_table(agent.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
                }
                CATCH(agent, 0, Hash40::new("top"), 7.0, 0.0, 10.0, 14.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_REFLET, *COLLISION_SITUATION_MASK_GA);
            }
        }
    }
}

//Down Special Effect
unsafe extern "C" fn ssbexo_reflet_down_special_effect(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if is_excute(agent) {
        FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE) {
        frame(agent.lua_state_agent, 14.0);
        if is_excute(agent) {
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_flash"), Hash40::new("havel"), -1, 1, 0, 0, 0, 0, 0.45, true);
        }
    }
    frame(agent.lua_state_agent, 15.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_ENHANCED_MAGIC_ACTIVE) {
        if is_excute(agent) {
            EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("reflet_rizaia"), Hash40::new("top"), 0, 5, 14, 0, 0, 0, 1, true);
        }
    }
}

//Down Special Sound
unsafe extern "C" fn ssbexo_reflet_down_special_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE) {
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_reflet_special_l01"));
        }
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_ENHANCED_MAGIC_ACTIVE) {
        agent.clear_lua_stack();
        lua_args!(agent, 2);
        sv_animcmd::IS_RANDOM(agent.lua_state_agent);
        if agent.pop_lua_stack(1).get_bool() {
            if is_excute(agent) {
                WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_VC_IS_PLAYED);
                PLAY_SE(agent, Hash40::new("vc_reflet_special_l01"));
            }
        }
        else {
            if is_excute(agent) {
                PLAY_SE(agent, Hash40::new("se_reflet_mp_empty"));
                PLAY_SEQUENCE(agent, Hash40::new("seq_reflet_rnd_special_empty"));
            }
        }
    }
}

pub fn install() {
    Agent::new("reflet")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_specials", ssbexo_reflet_side_special_acmd, Low)
    .game_acmd("game_specialairs", ssbexo_reflet_side_special_acmd, Low)
    .effect_acmd("effect_specials", ssbexo_reflet_side_special_effect, Low)
    .effect_acmd("effect_specialairs", ssbexo_reflet_side_special_effect, Low)
    .sound_acmd("sound_specials", ssbexo_reflet_side_special_sound, Low)
    .sound_acmd("sound_specialairs", ssbexo_reflet_side_special_sound, Low)
    .game_acmd("game_specialhi", ssbexo_reflet_up_special_acmd, Low)
    .game_acmd("game_specialairhi", ssbexo_reflet_up_special_acmd, Low)
    .game_acmd("game_speciallwstart", ssbexo_reflet_down_special_acmd, Low)
    .game_acmd("game_specialairlwstart", ssbexo_reflet_down_special_acmd, Low)
    .effect_acmd("effect_speciallwstart", ssbexo_reflet_down_special_effect, Low)
    .effect_acmd("effect_specialairlwstart", ssbexo_reflet_down_special_effect, Low)
    .sound_acmd("sound_speciallwstart", ssbexo_reflet_down_special_sound, Low)
    .sound_acmd("sound_specialairlwstart", ssbexo_reflet_down_special_sound, Low)
    .install()
    ;
    Agent::new("reflet_gigafire")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_shoot0", ssbexo_reflet_arcfire_shoot_acmd, Low)
    .effect_acmd("effect_shoot0", ssbexo_reflet_arcfire_shoot_effect, Low)
    .sound_acmd("sound_shoot0", ssbexo_reflet_arcfire_shoot_sound, Low)
    .install()
    ;
}