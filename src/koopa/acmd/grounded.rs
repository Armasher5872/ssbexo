use super::*;

//Wait 2 Expression
#[acmd_script( agent = "koopa", script = "expression_wait2", category = ACMD_EXPRESSION )]
unsafe fn ssbuexo_koopa_wait_2_expression(fighter : &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 56.0);
    for _ in 0..10 {
        if macros::is_excute(fighter) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        }
        wait(fighter.lua_state_agent, 5.0);
    }
}

//Wait 3 Expression
#[acmd_script( agent = "koopa", script = "expression_wait3", category = ACMD_EXPRESSION )]
unsafe fn ssbuexo_koopa_wait_3_expression(fighter : &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
    frame(fighter.lua_state_agent, 63.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
    frame(fighter.lua_state_agent, 106.0);
    for _ in 0..14 {
        if macros::is_excute(fighter) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        }
        wait(fighter.lua_state_agent, 5.0);
    }
}

//Up Taunt Effect
#[acmd_script( agent = "koopa", scripts = ["effect_appealhir", "effect_appealhil"], category = ACMD_EFFECT)]
unsafe fn ssbuexo_koopa_up_taunt_effect(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("koopa_wait_breath"), Hash40::new("head"), 0, 4, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 90.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("koopa_wait_breath"), false, true);
    }
}

//Up Taunt Sound
#[acmd_script( agent = "koopa", scripts = ["sound_appealhir", "sound_appealhil"], category = ACMD_SOUND )]
unsafe fn ssbuexo_koopa_up_taunt_sound(fighter : &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("vc_koopa_attack07"));
    }
}

//Up Taunt Expression
#[acmd_script( agent = "koopa", scripts = ["expression_appealhir", "expression_appealhil"], category = ACMD_EXPRESSION )]
unsafe fn ssbuexo_koopa_up_taunt_expression(fighter : &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 45.0);
    for _ in 0..8 {
        if macros::is_excute(fighter) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        }
        wait(fighter.lua_state_agent, 5.0);
    }
}

//Side Taunt ACMD
#[acmd_script( agent = "koopa", scripts = ["game_appealsr", "game_appealsl"], category = ACMD_GAME)]
unsafe fn ssbuexo_koopa_side_taunt_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 6.0, 40, 60, 0, 80, 6.0, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 56.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 6.0, 40, 60, 0, 80, 6.0, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    frame(fighter.lua_state_agent, 60.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 73.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 6.0, 40, 60, 0, 80, 6.0, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    frame(fighter.lua_state_agent, 77.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Side Taunt Expression
#[acmd_script( agent = "koopa", scripts = ["expression_appealsr", "expression_appealsl"], category = ACMD_EXPRESSION )]
unsafe fn ssbuexo_koopa_side_taunt_expression(fighter : &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, 0);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
    frame(fighter.lua_state_agent, 56.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, 0);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
    frame(fighter.lua_state_agent, 73.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, 0);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
}

//Jab 1 ACMD
#[acmd_script( agent = "koopa", script = "game_attack11", category = ACMD_GAME)]
unsafe fn ssbuexo_koopa_jab_1_acmd(fighter: &mut L2CAgentBase) 
{
    MotionModule::set_rate(fighter.module_accessor, 1.4);
    frame(fighter.lua_state_agent, 7.0);
    MotionModule::set_rate(fighter.module_accessor, 1.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 18, 0, 40, 3.2, 0.0, 11.0, 6.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 361, 18, 0, 35, 3.2, 0.0, 8.6, 8.5, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 180, 18, 0, 30, 4.5, 0.0, 8.0, 13.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 4.0, 361, 18, 0, 30, 4.5, 0.0, 8.0, 13.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

//Dash Attack ACMD
#[acmd_script( agent = "koopa", script = "game_attackdash", category = ACMD_GAME)]
unsafe fn ssbuexo_koopa_dash_attack_acmd(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 11.0);
    for _ in 0..4 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 30, 80, 60, 0, 10.0, 0.0, 8.0, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }  
        wait(fighter.lua_state_agent, 1.0);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 30, 100, 0, 70, 10.0, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//Dash Attack Effect
#[acmd_script( agent = "koopa", script = "effect_attackdash", category = ACMD_EFFECT)]
unsafe fn ssbuexo_koopa_dash_attack_effect(fighter: &mut L2CAgentBase) 
{
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ssbuexo_koopa_wait_2_expression,
        ssbuexo_koopa_wait_3_expression,
        ssbuexo_koopa_up_taunt_effect,
        ssbuexo_koopa_up_taunt_sound,
        ssbuexo_koopa_up_taunt_expression,
        ssbuexo_koopa_side_taunt_acmd,
        ssbuexo_koopa_side_taunt_expression,
        ssbuexo_koopa_jab_1_acmd,
        ssbuexo_koopa_dash_attack_acmd,
        ssbuexo_koopa_dash_attack_effect
    );
}