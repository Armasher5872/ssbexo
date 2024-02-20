use super::*;

unsafe extern "C" fn ssbexo_link_bowarrow_fly_acmd(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_SPECIAL_FLAG) == FuseType::ELEMENTAL {
        let fuse_item_kind = WorkModule::get_int(agent.module_accessor,WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_KIND);
        if fuse_item_kind == *ITEM_KIND_FIREFLOWER {
            if macros::is_excute(agent) {
                macros::ATTACK(agent,0,0,Hash40::new("top"), 10.0, 361, 71, 0, 10, 1.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0.0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
                AttackModule::set_power_mul(agent.module_accessor,1.15);
                AttackModule::enable_safe_pos(agent.module_accessor);
            }
        }
        else if fuse_item_kind == *ITEM_KIND_FREEZER {
            if macros::is_excute(agent) {
                macros::ATTACK(agent,0,0,Hash40::new("top"), 5.0, 361, 71, 0, 10, 1.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0.0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
                AttackModule::set_power_mul(agent.module_accessor, 0.4);
                AttackModule::enable_safe_pos(agent.module_accessor);
            }
        }
        else {
            if macros::is_excute(agent) {
                macros::ATTACK(agent,0,0,Hash40::new("top"), 5.0, 361, 71, 0, 10, 1.75, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 0.0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralysis"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
                AttackModule::set_power_mul(agent.module_accessor, 1.5);
                AttackModule::enable_safe_pos(agent.module_accessor);
                macros::QUAKE(agent,*CAMERA_QUAKE_KIND_S);
            }
        }
    }
    else {
        if WorkModule::get_int(agent.module_accessor, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_SHOOT_NUM) <= 0 {
            if macros::is_excute(agent) {
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 71, 0, 10, 1.35, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            }
        }
        else {
            if macros::is_excute(agent) {
                macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 100, 0, 75, 1.35, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
                AttackModule::enable_safe_pos(agent.module_accessor);
            }
        }
    }
}

unsafe extern "C" fn ssbexo_link_boomerang_turn_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 60, 100, 60, 0, 5.5, 0.0, -6.0, 0.0, Some(0.0), Some(6.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -1.5, 0.0, 0, true, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

pub fn install() {
    Agent::new("link_bowarrow")
    .game_acmd("game_fly", ssbexo_link_bowarrow_fly_acmd)
    .install()
    ;
    Agent::new("link_boomerang")
    .game_acmd("game_turn", ssbexo_link_boomerang_turn_acmd)
    .install()
    ;
}