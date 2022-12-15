#![allow(unused_macros)]
use {
    smash::{
        app::{
            lua_bind::*,
            sv_animcmd::*,
        },
        lua2cpp::L2CAgentBase,
        lib::lua_const::*,
        phx::Hash40,
    },
    smashline::*,
    smash_script::*,
};

//Flare Charge ACMD
#[acmd_script( agent = "edge", scripts = ["game_specialnstart", "game_specialairnstart"], category = ACMD_GAME )]
unsafe fn ssbuexo_sephiroth_flare_charge_acmd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_EDGE_SPECIAL_N_S, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_EDGE_SPECIAL_N_M, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(fighter.lua_state_agent, 79.0);
    MotionModule::set_rate(fighter.module_accessor, 0.8);
    frame(fighter.lua_state_agent, 99.0);
    MotionModule::set_rate(fighter.module_accessor, 1.0);
    frame(fighter.lua_state_agent, 100.0);
    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_EDGE_SPECIAL_N_L, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CHARGE_KIND);
    }
    frame(fighter.lua_state_agent, 105.0);
    MotionModule::set_rate(fighter.module_accessor, 0.625);
    frame(fighter.lua_state_agent, 115.0);
    MotionModule::set_rate(fighter.module_accessor, 1.0);
    frame(fighter.lua_state_agent, 120.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_EDGE_GENERATE_ARTICLE_FIRE, false, 0);
    }
    frame(fighter.lua_state_agent, 140.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_N_FLAG_ENABLE_FALL_SPEED);
    }
}

//Flare ACMD
#[acmd_script( agent = "edge_fire", script = "game_specialn1", category = ACMD_GAME )]
unsafe fn ssbuexo_sephiroth_flare_acmd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::disable_tip(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 82, 15, 0, 55, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, -2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::set_size(fighter.module_accessor, 0, 4.5);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::set_size(fighter.module_accessor, 0, 5.0);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        AttackModule::set_size(fighter.module_accessor, 0, 5.5);
    }
}

/*
//Octaslash
#[acmd_script( agent = "edge", script = "game_specialhi2", category = ACMD_GAME )]
unsafe fn ssbuexo_sephiroth_octaslash(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        notify_event_msc_cmd(0x2127e37c07, *GROUND_CLIFF_CHECK_KIND_NONE);
        battle_object_module_accessor();
        methodlib::L2CValue::L2CValue(void*)();
    }
    else{;
        methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
    }
    methodlib::L2CValue::as_pointer()const();
    set_special_hi_jostle_area();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 2.3, 361, 100, 0, 0, 8.0, 0.0, 2.5, 1.0, 0.0, -2.5, 1.0, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("rot"), 2.3, 368, 100, 0, 0, 8.0, 0.0, 2.5, 1.0, 0.0, -2.5, 1.0, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("rot"), 2.3, 368, 100, 0, 0, 6.5, 0.0, 3.0, 10.0, 0.0, -3.0, 10.0, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("rot"), 2.3, 368, 100, 0, 0, 6.5, 0.0, 3.0, 20.5, 0.0, -3.0, 20.5, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("rot"), 2.3, 368, 100, 0, 0, 8.0, 0.0, 2.5, 1.0, 0.0, -2.5, 1.0, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 5, 0, Hash40::new("rot"), 2.3, 368, 100, 0, 0, 6.5, 0.0, 3.0, 10.0, 0.0, -3.0, 10.0, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 6, 0, Hash40::new("rot"), 2.3, 368, 100, 0, 0, 6.5, 0.0, 3.0, 20.5, 0.0, -3.0, 20.5, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        battle_object_module_accessor();
        methodlib::L2CValue::L2CValue(void*)();
    }
    else{;
        methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
    }
    methodlib::L2CValue::as_pointer()const(1, Hash40::new("rot"), 30, -5, 3);
    methodlib::L2CValue::as_hash()const();
    set_vec_target_pos();
    battle_object_module_accessor();
    methodlib::L2CValue::L2CValue(void*)();
    methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
    methodlib::L2CValue::as_pointer()const(2, Hash40::new("rot"), 26, -5, 2);
    methodlib::L2CValue::as_hash()const();
    set_vec_target_pos();
    battle_object_module_accessor();
    methodlib::L2CValue::L2CValue(void*)();
    methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
    methodlib::L2CValue::as_pointer()const(3, Hash40::new("rot"), 20, -5, 2);
    methodlib::L2CValue::as_hash()const();
    set_vec_target_pos();
    battle_object_module_accessor();
    methodlib::L2CValue::L2CValue(void*)();
    methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
    methodlib::L2CValue::as_pointer()const(4, Hash40::new("rot"), 5, 1, 4);
    methodlib::L2CValue::as_hash()const();
    set_vec_target_pos();
    battle_object_module_accessor();
    methodlib::L2CValue::L2CValue(void*)();
    methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
    methodlib::L2CValue::as_pointer()const(5, Hash40::new("rot"), 15, 1, 4);
    methodlib::L2CValue::as_hash()const();
    set_vec_target_pos();
    battle_object_module_accessor();
    methodlib::L2CValue::L2CValue(void*)();
    methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
    methodlib::L2CValue::as_pointer()const(6, Hash40::new("rot"), 20, 1, 4);
    methodlib::L2CValue::as_hash()const();
    set_vec_target_pos();
    AttackModule::set_no_finish_camera_ex(boma, 1, true, false);
    AttackModule::set_no_finish_camera_ex(boma, 2, true, false);
    AttackModule::set_no_finish_camera_ex(boma, 3, true, false);
    AttackModule::set_no_finish_camera_ex(boma, 4, true, false);
    AttackModule::set_no_finish_camera_ex(boma, 5, true, false);
    AttackModule::set_no_finish_camera_ex(boma, 6, true, false);
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 4.0);
    for(6 Iterations){;
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 2.3, 368, 100, 0, 0, 5.0, 0.0, 2.5, 0.0, 0.0, -2.5, 0.0, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 1, 0, Hash40::new("rot"), 2.3, 368, 100, 0, 0, 5.0, 0.0, 3.0, 10.0, 0.0, -3.0, 10.0, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 2, 0, Hash40::new("rot"), 2.3, 368, 100, 0, 0, 6.5, 0.0, 3.0, 20.5, 0.0, -3.0, 20.5, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 3, 0, Hash40::new("rot"), 2.3, 368, 100, 0, 0, 5.0, 0.0, 2.5, 0.0, 0.0, -2.5, 0.0, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 4, 0, Hash40::new("rot"), 2.3, 368, 100, 0, 0, 5.0, 0.0, 3.0, 10.0, 0.0, -3.0, 10.0, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 5, 0, Hash40::new("rot"), 2.3, 368, 100, 0, 0, 6.5, 0.0, 3.0, 20.5, 0.0, -3.0, 20.5, 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, -1.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 6, 0, Hash40::new("rot"), 2.3, 60, 60, 0, 90, 6.5, 0.0, 3.0, 0.0, 0.0, -3.0, 0.0, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, -1, -1.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        ATTACK(fighter, 7, 0, Hash40::new("rot"), 2.3, 60, 60, 0, 90, 6.5, 0.0, 3.0, 10.0, 0.0, -3.0, 10.0, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, -1, -1.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        battle_object_module_accessor();
        methodlib::L2CValue::L2CValue(void*)();
    }
    else{
        methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
    }
    methodlib::L2CValue::as_pointer()const(0, Hash40::new("rot"), 30, 100, 368);
    methodlib::L2CValue::as_hash()const();
    set_vec_target_pos();
    battle_object_module_accessor();
    methodlib::L2CValue::L2CValue(void*)();
    methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
    methodlib::L2CValue::as_pointer()const(1, Hash40::new("rot"), 26, -5, 2);
    methodlib::L2CValue::as_hash()const();
    set_vec_target_pos();
    battle_object_module_accessor();
    methodlib::L2CValue::L2CValue(void*)();
    methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
    methodlib::L2CValue::as_pointer()const(2, Hash40::new("rot"), 20, -5, 2);
    methodlib::L2CValue::as_hash()const();
    set_vec_target_pos();
    battle_object_module_accessor();
    methodlib::L2CValue::L2CValue(void*)();
    methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
    methodlib::L2CValue::as_pointer()const(3, Hash40::new("rot"), 5, 1, 4);
    methodlib::L2CValue::as_hash()const();
    set_vec_target_pos();
    battle_object_module_accessor();
    methodlib::L2CValue::L2CValue(void*)();
    methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
    methodlib::L2CValue::as_pointer()const(4, Hash40::new("rot"), 15, 1, 4);
    methodlib::L2CValue::as_hash()const();
    set_vec_target_pos();
    battle_object_module_accessor();
    methodlib::L2CValue::L2CValue(void*)();
    methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
    methodlib::L2CValue::as_pointer()const(5, Hash40::new("rot"), 20, 1, 4);
    methodlib::L2CValue::as_hash()const();
    set_vec_target_pos();
    AttackModule::set_no_finish_camera_ex(boma, 0, true, false);
    AttackModule::set_no_finish_camera_ex(boma, 1, true, false);
    AttackModule::set_no_finish_camera_ex(boma, 2, true, false);
    AttackModule::set_no_finish_camera_ex(boma, 3, true, false);
    AttackModule::set_no_finish_camera_ex(boma, 4, true, false);
    AttackModule::set_no_finish_camera_ex(boma, 5, true, false);
    AttackModule::set_no_finish_camera(boma, 6, true, false);
    AttackModule::set_no_finish_camera(boma, 7, true, false);
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    wait(lua_state, 1.0);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        battle_object_module_accessor();
        methodlib::L2CValue::L2CValue(void*)();
    }
    else{;
        methodlib::L2CValue::L2CValue(lib::L2CValueconst&)();
    }
    methodlib::L2CValue::as_pointer()const();
    clear_special_hi_jostle_area();
}
*/

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_sephiroth_flare_acmd,
        ssbuexo_sephiroth_flare_charge_acmd,
        //ssbuexo_sephiroth_octaslash,
    );
}