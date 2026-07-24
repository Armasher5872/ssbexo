use super::*;

//Shadow Flare Explode ACMD
unsafe extern "C" fn ssbexo_edge_shadow_flare_explode_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let owner_boma = get_owner_boma(agent);
    println!("Owner Canceled: {}", WorkModule::is_flag(boma, *WEAPON_EDGE_FLARE2_INSTANCE_WORK_ID_FLAG_OWNER_CANCELED));
    if WorkModule::is_flag(boma, *WEAPON_EDGE_FLARE2_INSTANCE_WORK_ID_FLAG_OWNER_CANCELED) {
        if is_excute(agent) {
            ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
            ATTACK(agent, 0, 0, Hash40::new("top"), 3.5, 122, 75, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        }
    }
    else {
        if WorkModule::is_flag(owner_boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
            if is_excute(agent) {
                ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
                ATTACK(agent, 0, 0, Hash40::new("top"), 6.5, 53, 75, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
            }
        }
        else {
            if is_excute(agent) {
                ControlModule::set_rumble(boma, Hash40::new("rbkind_attackl"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
                ATTACK(agent, 0, 0, Hash40::new("top"), 4.5, 53, 75, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
            }
        }
    }
    frame(lua_state, 1.0);
    if !WorkModule::is_flag(boma, *WEAPON_EDGE_FLARE2_INSTANCE_WORK_ID_FLAG_REFLECT) {
        if is_excute(agent) {
            notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
        }
    }
}

pub fn install() {
    Agent::new("edge_flare2")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .acmd("game_exp", ssbexo_edge_shadow_flare_explode_acmd, Low)
    .install()
    ;
}