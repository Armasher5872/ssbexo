use {
    crate::functions::{
        FIRE_PUNCH_TURN_COUNT,
        KIRBY_FIRE_PUNCH_TURN_COUNT,
        SHIELD_SPECIAL
    },
    smash::{
        app::lua_bind::*,
        hash40,
        lua2cpp::L2CFighterCommon,
        lib::{
            L2CValueType,
            lua_const::*,
        },
        phx::Hash40,
    },
    smashline::*,
};

#[fighter_frame( agent = FIGHTER_KIND_GANON )]
fn armstrong_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(module_accessor);
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let mut globals = fighter.globals_mut().clone();
        //Shield Special
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3
        && SHIELD_SPECIAL[entry_id] == true {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_shield"), 1.0, 1.0, false, 0.0, false, false);
        }
        if MotionModule::motion_kind(module_accessor) == smash::hash40("special_shield") {
            SHIELD_SPECIAL[entry_id] = false;
        };
        if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_OFF].contains(&status_kind) 
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            SHIELD_SPECIAL[entry_id] = true;
        }
        if SHIELD_SPECIAL[entry_id] == true {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
        };
        //Neutral Special
        if status_kind == *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN 
        && (25.0..40.0).contains(&frame) 
        && (ControlModule::get_stick_x(module_accessor)*PostureModule::lr(module_accessor)) < -0.5
        && FIRE_PUNCH_TURN_COUNT[entry_id] <= 15.0 {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN, true);
        };
        if status_kind != *FIGHTER_GANON_STATUS_KIND_SPECIAL_N_TURN {
            FIRE_PUNCH_TURN_COUNT[entry_id] = 0.0;
        }
        if FIRE_PUNCH_TURN_COUNT[entry_id] == 0.0 {
            AttackModule::set_power_up(module_accessor, 1.0);
        }
        //Down Special
        if let L2CValueType::Void = globals["reverse"].val_type {
            globals["reverse"] = false.into();
        }
        if motion_kind == hash40("special_air_lw") {
            if frame < 6.0 {
                if ControlModule::get_stick_x(module_accessor) * PostureModule::lr(module_accessor) <= -0.66 {
                    globals["reverse"] = true.into();
                }
            }
            else if (6.0..=7.0).contains(&frame)
            && globals["reverse"].get_bool() {
                PostureModule::reverse_lr(module_accessor);
                PostureModule::update_rot_y_lr(module_accessor);
                KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            }
        }
        else {
            globals["reverse"] = false.into();
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn kirby_armstrong_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = StatusModule::status_kind(module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        if status_kind == *FIGHTER_KIRBY_STATUS_KIND_GANON_SPECIAL_N_TURN 
        && (25.0..40.0).contains(&frame) 
        && (ControlModule::get_stick_x(module_accessor)*PostureModule::lr(module_accessor)) < -0.5
        && KIRBY_FIRE_PUNCH_TURN_COUNT[entry_id] <= 15.0 {
            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_KIRBY_STATUS_KIND_GANON_SPECIAL_N_TURN, true);
        };
        if status_kind != *FIGHTER_KIRBY_STATUS_KIND_GANON_SPECIAL_N_TURN {
            KIRBY_FIRE_PUNCH_TURN_COUNT[entry_id] = 0.0;
        }
        if KIRBY_FIRE_PUNCH_TURN_COUNT[entry_id] == 0.0 {
            AttackModule::set_power_up(module_accessor, 1.0);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        armstrong_frame,
        kirby_armstrong_frame
    );
}