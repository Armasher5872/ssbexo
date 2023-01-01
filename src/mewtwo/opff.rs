#![allow(unused_macros)]
use {
    crate::functions::{
        CAN_ADD,
        CAN_CHLOEDASH,
        CHLOEDASH_OFF_GFX_TIMER,
        CHLOEDASH_ON_GFX_TIMER,
        CHLOEDASH_TIMER,
        CHLOEDASHING_ENABLED,
        SHIELD_SPECIAL,
        SITUATION_KIND,
        SPEED_ADD
    },
    smash::{
        app::{
            lua_bind::*,
            sv_information
        },
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
    },
    smashline::*,
    smash_script::*,
};

#[fighter_frame( agent = FIGHTER_KIND_MEWTWO )]
fn mewtwo_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(module_accessor);
        let frame = MotionModule::frame(module_accessor);
        if !sv_information::is_ready_go() {
            CHLOEDASHING_ENABLED[entry_id] = 0;
            CAN_ADD[entry_id] = false;
        }
        if status_kind == *FIGHTER_STATUS_KIND_DEAD {
            CHLOEDASHING_ENABLED[entry_id] = 0;
            CAN_ADD[entry_id] = false;
        }
        if frame < 2.0 { 
			CAN_ADD[entry_id] = true;
		};
        if CAN_ADD[entry_id] == true 
        && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			CAN_ADD[entry_id] = false;
			CHLOEDASH_TIMER[entry_id] -= 180;
		};
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL
        && SHIELD_SPECIAL[entry_id] == true {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_shield"), 1.0, 1.0, false, 0.0, false, false);
        }
        if MotionModule::motion_kind(module_accessor) == smash::hash40("special_shield") {
            SHIELD_SPECIAL[entry_id] = false;
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
            && CHLOEDASHING_ENABLED[entry_id] != 1 {
                CHLOEDASHING_ENABLED[entry_id] = 1;
                CHLOEDASH_TIMER[entry_id] = 3600;
                CAN_CHLOEDASH[entry_id] = false;
            }
        };
        if [*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind) {
            if (ControlModule::get_command_flag_cat(module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
        }
        if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_OFF].contains(&status_kind) 
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            SHIELD_SPECIAL[entry_id] = true;
        }
        if SHIELD_SPECIAL[entry_id] == true {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_APPEAL, true);
        };
        if CHLOEDASH_TIMER[entry_id] > 0 {
            CHLOEDASH_TIMER[entry_id] -= 1;
        }
        if CHLOEDASH_TIMER[entry_id] <= 0
        && CHLOEDASHING_ENABLED[entry_id] == 1 {
            CAN_CHLOEDASH[entry_id] = true;
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            if CAN_CHLOEDASH[entry_id] != false
            && frame >= 4.0 {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
            if CancelModule::is_enable_cancel(fighter.module_accessor) {
                fighter.sub_wait_ground_check_common(false.into());
                CHLOEDASH_TIMER[entry_id] = 3600;
                CAN_CHLOEDASH[entry_id] = false;
                SPEED_ADD[entry_id] = true;
            }
        }
        if SPEED_ADD[entry_id] == true
        && fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR
        && ![*FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, *FIGHTER_STATUS_KIND_WAIT].contains(&status_kind) {
            WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            macros::SET_SPEED_EX(fighter, 7.5, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
            WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            SPEED_ADD[entry_id] = false;
        }
        if CHLOEDASHING_ENABLED[entry_id] == 1 {
            CHLOEDASH_OFF_GFX_TIMER[entry_id] += 1;
            CHLOEDASH_ON_GFX_TIMER[entry_id] += 1;
            if CAN_CHLOEDASH[entry_id] == false {
                if CHLOEDASH_OFF_GFX_TIMER[entry_id] == 10 {
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                }
                if CHLOEDASH_OFF_GFX_TIMER[entry_id] >= 20 {
                    macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
                    macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_aura"), false, false);
                    macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_bullet"), false, false);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 0.0);
                    CHLOEDASH_OFF_GFX_TIMER[entry_id] = 0;
                }
            }
            else {
                if CHLOEDASH_ON_GFX_TIMER[entry_id] == 10 {
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 1.0);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 1.0);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 1.0);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 1.0);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 1.0);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 1.0);
                }
                if CHLOEDASH_ON_GFX_TIMER[entry_id] >= 20 {
                    macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
                    macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_aura"), false, false);
                    macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_bullet"), false, false);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 1.0);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 1.0);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 1.0);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 1.0);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 1.0);
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_revenge_bullet"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.0, true, 1.0);
                    macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.0, 1.0);
                    CHLOEDASH_ON_GFX_TIMER[entry_id] = 0;
                }
            }
        }
        else {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_aura"), false, false);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_revenge_bullet"), false, false);
            CHLOEDASH_OFF_GFX_TIMER[entry_id] = 0;
            CHLOEDASH_ON_GFX_TIMER[entry_id] = 0;
        };
    }
}

pub fn install() {
    install_agent_frames!(
        mewtwo_frame
    );
}