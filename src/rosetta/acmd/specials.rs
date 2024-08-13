use super::*;

//Up Special ACMD
unsafe extern "C" fn ssbexo_rosetta_up_special_acmd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

//Down Special ACMD
unsafe extern "C" fn ssbexo_rosetta_down_special_acmd(agent: &mut L2CAgentBase) {
    let prev_status_kind = StatusModule::prev_status_kind(agent.module_accessor, 0);
    let transit_timer = WorkModule::get_int(agent.module_accessor, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_WORMHOLE_TRANSIT_TIMER);
    if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&prev_status_kind) {
        if transit_timer > 1 {
            frame(agent.lua_state_agent, 2.0);
            if macros::is_excute(agent) {
                macros::WHOLE_HIT(agent, *HIT_STATUS_OFF);
                JostleModule::set_status(agent.module_accessor, false);
            }
            frame(agent.lua_state_agent, 16.0);
            if macros::is_excute(agent) {
                macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
                JostleModule::set_status(agent.module_accessor, true);
            }
        }
        else {
            frame(agent.lua_state_agent, 2.0);
            if macros::is_excute(agent) {
                macros::WHOLE_HIT(agent, *HIT_STATUS_OFF);
                JostleModule::set_status(agent.module_accessor, false);
            }
            frame(agent.lua_state_agent, 9.0);
            if macros::is_excute(agent) {
                macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
                JostleModule::set_status(agent.module_accessor, true);
            }
        }
    }
    else {
        frame(agent.lua_state_agent, 3.0);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_ROSETTA_STATUS_SPECIAL_LW_FLAG_ENABLE_SEARCH);
        }
        for _ in 0..27 {
            if macros::is_excute(agent) {
                ItemModule::pickup_item(agent.module_accessor, ItemSize{ _address: *ITEM_SIZE_LIGHT as u8 }, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK, QuickItemTreatType{ _address: 0 }, ItemPickupSearchMode{ _address: 0 });
                ItemModule::use_item(agent.module_accessor, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, false);
                ItemModule::pickup_item(agent.module_accessor, ItemSize{ _address: *ITEM_SIZE_LIGHT as u8 }, 0, -1, QuickItemTreatType{ _address: 0 }, ItemPickupSearchMode{ _address: 0 });
            }
            wait(agent.lua_state_agent, 1.0);
        }
        wait(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_ROSETTA_STATUS_SPECIAL_LW_FLAG_ENABLE_SEARCH);
            AttackModule::clear_all(agent.module_accessor);
        }
        if macros::is_excute(agent) {
            ItemModule::pickup_item(agent.module_accessor, ItemSize{ _address: *ITEM_SIZE_LIGHT as u8 }, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, *ITEM_TRAIT_FLAG_QUICK, QuickItemTreatType{ _address: 0 }, ItemPickupSearchMode{ _address: 0 });
            ItemModule::use_item(agent.module_accessor, *FIGHTER_HAVE_ITEM_WORK_TEMPORARY, false);
            ItemModule::pickup_item(agent.module_accessor, ItemSize{ _address: *ITEM_SIZE_LIGHT as u8 }, 0, -1, QuickItemTreatType{ _address: 0 }, ItemPickupSearchMode{ _address: 0 });
        }
    }
}

//Down Special Effect
unsafe extern "C" fn ssbexo_rosetta_down_special_effect(agent: &mut L2CAgentBase) {
    let prev_status_kind = StatusModule::prev_status_kind(agent.module_accessor, 0);
    let transit_timer = WorkModule::get_int(agent.module_accessor, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_WORMHOLE_TRANSIT_TIMER);
    if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&prev_status_kind) {
        if transit_timer > 0 {
            frame(agent.lua_state_agent, 2.0);
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("rosetta_escape"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
            frame(agent.lua_state_agent, 9.0);
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("rosetta_escape_end"), Hash40::new("top"), 0, 0, -1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
        }
        else {
            frame(agent.lua_state_agent, 2.0);
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("rosetta_escape"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
            frame(agent.lua_state_agent, 16.0);
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("rosetta_escape_end"), Hash40::new("top"), 0, 0, -1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
        }
    }
}

//Down Special Sound
unsafe extern "C" fn ssbexo_rosetta_down_special_sound(agent: &mut L2CAgentBase) {
    let prev_status_kind = StatusModule::prev_status_kind(agent.module_accessor, 0);
    let transit_timer = WorkModule::get_int(agent.module_accessor, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_WORMHOLE_TRANSIT_TIMER);
    if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&prev_status_kind) {
        if transit_timer > 0 {
            frame(agent.lua_state_agent, 2.0);
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("rosetta_escape"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
            frame(agent.lua_state_agent, 9.0);
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("rosetta_escape_end"), Hash40::new("top"), 0, 0, -1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
        }
        else {
            frame(agent.lua_state_agent, 2.0);
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("rosetta_escape"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
            frame(agent.lua_state_agent, 16.0);
            if macros::is_excute(agent) {
                macros::EFFECT(agent, Hash40::new("rosetta_escape_end"), Hash40::new("top"), 0, 0, -1.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            }
        }
    }
    else {
        frame(agent.lua_state_agent, 1.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_rosetta_special_l01"));
        }
    }
}

//Down Special Expression
unsafe extern "C" fn ssbexo_rosetta_down_special_expression(agent: &mut L2CAgentBase) {
    let prev_status_kind = StatusModule::prev_status_kind(agent.module_accessor, 0);
    let transit_timer = WorkModule::get_int(agent.module_accessor, FIGHTER_ROSETTA_INSTANCE_WORK_ID_INT_WORMHOLE_TRANSIT_TIMER);
    if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&prev_status_kind) {
        if transit_timer > 0 {
            frame(agent.lua_state_agent, 2.0);
            if macros::is_excute(agent) {
                VisibilityModule::set_whole(agent.module_accessor, false);
            }
            frame(agent.lua_state_agent, 9.0);
            if macros::is_excute(agent) {
                VisibilityModule::set_whole(agent.module_accessor, true);
            }
        }
        else {
            frame(agent.lua_state_agent, 2.0);
            if macros::is_excute(agent) {
                VisibilityModule::set_whole(agent.module_accessor, false);
            }
            frame(agent.lua_state_agent, 16.0);
            if macros::is_excute(agent) {
                VisibilityModule::set_whole(agent.module_accessor, true);
            }
        }
    }
    else {
        if macros::is_excute(agent) {
            slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
            ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        }
        frame(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 14.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 26.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 35.0);
        if macros::is_excute(agent) {
            ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
        }
        frame(agent.lua_state_agent, 38.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

pub fn install() {
    Agent::new("rosetta")
    .game_acmd("game_specialhi", ssbexo_rosetta_up_special_acmd, Priority::Low)
    .game_acmd("game_speciallw", ssbexo_rosetta_down_special_acmd, Priority::Low)
    .effect_acmd("effect_speciallw", ssbexo_rosetta_down_special_effect, Priority::Low)
    .sound_acmd("sound_speciallw", ssbexo_rosetta_down_special_sound, Priority::Low)
    .expression_acmd("expression_speciallw", ssbexo_rosetta_down_special_expression, Priority::Low)
    .install()
    ;
}