use super::*;

//Arrow Fly ACMD
unsafe extern "C" fn ssbexo_link_bowarrow_fly_acmd(agent: &mut L2CAgentBase) {
    let weapon = get_weapon_common_from_accessor(&mut *(agent.module_accessor));
    let owner_boma = get_owner_boma(weapon);
    let shoot_num = WorkModule::get_int(agent.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_SHOOT_NUM);
    let arrow_type = WorkModule::get_int(agent.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
    let shot_angle = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_FLOAT_SHOT_ANGLE);
    let angle = if shot_angle < 0.0 {361 as u64} else {shot_angle as u64};
    if shoot_num > 0 {
        if arrow_type == *WN_LINK_BOWARROW_NORMAL_ARROW {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, angle, 160, 0, 10, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        }
        if arrow_type == *WN_LINK_BOWARROW_FIRE_ARROW {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, angle, 50, 0, 35, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            }
        }
        if arrow_type == *WN_LINK_BOWARROW_SHOCK_ARROW {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, angle, 70, 0, 25, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
            }
        }
        if arrow_type == *WN_LINK_BOWARROW_ICE_ARROW {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, angle, 110, 0, 25, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
            }
        }
    }
    else {
        if arrow_type == *WN_LINK_BOWARROW_NORMAL_ARROW {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, angle, 30, 0, 10, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
                AttackModule::enable_safe_pos(agent.module_accessor);
            }
        }
        if arrow_type == *WN_LINK_BOWARROW_FIRE_ARROW {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, angle, 50, 0, 35, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
                AttackModule::enable_safe_pos(agent.module_accessor);
            }
        }
        if arrow_type == *WN_LINK_BOWARROW_SHOCK_ARROW {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, angle, 70, 0, 25, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
                AttackModule::enable_safe_pos(agent.module_accessor);
            }
        }
        if arrow_type == *WN_LINK_BOWARROW_ICE_ARROW {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, angle, 30, 0, 25, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
                AttackModule::enable_safe_pos(agent.module_accessor);
            }
        }
        if arrow_type == *WN_LINK_BOWARROW_LIGHT_ARROW {
            if WorkModule::is_flag(owner_boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_CHARGE) {
                if is_excute(agent) {
                    ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, angle, 110, 0, 25, 1.35, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
                    AttackModule::enable_safe_pos(agent.module_accessor);
                }
            }
        }
    }
}

//Arrow Fly Effect
unsafe extern "C" fn ssbexo_link_bowarrow_fly_effect(agent: &mut L2CAgentBase) {
    let arrow_type = WorkModule::get_int(agent.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_ARROW_TYPE);
    if arrow_type == *WN_LINK_BOWARROW_NORMAL_ARROW {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("link_arrow_trace"), Hash40::new("arrow"), 0, 0, 1, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("link_arrow"), Hash40::new("arrow"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    if arrow_type == *WN_LINK_BOWARROW_FIRE_ARROW {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("link_arrow_trace"), Hash40::new("arrow"), 0, 0, 1, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("link_arrow"), Hash40::new("arrow"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("link_shock_arrow_aura"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.3, true);
            LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 0.0);
            EFFECT_FOLLOW(agent, Hash40::new("link_fire_arrow_aura"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.6, true);
        }
    }
    if arrow_type == *WN_LINK_BOWARROW_SHOCK_ARROW {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("link_arrow_trace"), Hash40::new("arrow"), 0, 0, 1, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("link_arrow"), Hash40::new("arrow"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("link_shock_arrow_aura"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.3, true);
        }
    }
    if arrow_type == *WN_LINK_BOWARROW_ICE_ARROW {
        if is_excute(agent) {
            EFFECT_FOLLOW(agent, Hash40::new("link_arrow_trace"), Hash40::new("arrow"), 0, 0, 1, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("link_arrow"), Hash40::new("arrow"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(agent, Hash40::new("link_shock_arrow_aura"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.3, true);
            LAST_EFFECT_SET_COLOR(agent, 0.0, 0.0, 1.0);
            EFFECT_FOLLOW(agent, Hash40::new("link_ice_arrow_aura"), Hash40::new("arrow"), 0, 0, 13, 0, 0, 0, 0.3, true);
        }
    }
}

pub fn install() {
    Agent::new("link_bowarrow")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_fly", ssbexo_link_bowarrow_fly_acmd, Low)
    .effect_acmd("effect_fly", ssbexo_link_bowarrow_fly_effect, Low)
    .install()
    ;
}