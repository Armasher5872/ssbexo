#![allow(unused_macros)]
use {
    crate::functions::{
        ASDI,
        ASDI_START,
        DIR_MULT,
        ray_check_pos
    },
    smash::{
        app::{
            lua_bind::{
                PostureModule,
                *
            }
        },
        lua2cpp::L2CFighterCommon,
        lib::lua_const::*,
    },
    smashline::*,
};

#[fighter_frame_callback]
pub fn asdi(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let stickx = ControlModule::get_stick_x(module_accessor);
        let sticky = ControlModule::get_stick_y(module_accessor);
        let substickx = ControlModule::get_sub_stick_x(module_accessor);
        let substicky = ControlModule::get_sub_stick_y(module_accessor);
        if StopModule::is_damage(module_accessor) {
            ASDI_START[entry_id] = true;
        };
        if ASDI_START[entry_id] 
        && !StopModule::is_damage(module_accessor) {
            let mut stick_dir = ControlModule::get_stick_dir(module_accessor) * DIR_MULT;
            let sub_stick_dir = ControlModule::get_sub_stick_dir(module_accessor) * DIR_MULT;
            let mut hypotenuse = ASDI;
            if stickx < 0.2 && stickx > -0.2 && sticky > -0.2 && sticky < 0.2 {
                hypotenuse = 0.001;
            };
            if (substickx < 0.2 && substickx > -0.2 && substicky > -0.2 && substicky < 0.2) == false {
                stick_dir = sub_stick_dir;
            };
            let mut stick_lr = 1.0;
            if stickx < 0.0 {
                stick_lr = -1.0;
            };
            let mut stick_ud = 1.0;
            if sticky < 0.0 {
                stick_ud = -1.0;
            };
            let x_distance = stick_dir.cos()*hypotenuse*stick_lr;
            let y_distance = stick_dir.sin()*hypotenuse*stick_ud;
            if ray_check_pos(module_accessor, x_distance, y_distance, false) == 0 {
                let pos = smash::phx::Vector3f {x: PostureModule::pos_x(module_accessor)+x_distance, y: PostureModule::pos_y(module_accessor)+y_distance, z: 0.0};
                PostureModule::set_pos(module_accessor, &pos);
                PostureModule::init_pos(module_accessor, &pos, true, true);
            }
            else if ray_check_pos(module_accessor, x_distance/2.0, y_distance/2.0, false) == 0 {
                let pos = smash::phx::Vector3f {x: PostureModule::pos_x(module_accessor)+x_distance*0.5, y: PostureModule::pos_y(module_accessor)+y_distance*0.5, z: 0.0};
                PostureModule::set_pos(module_accessor, &pos);
                PostureModule::init_pos(module_accessor, &pos, true, true);
            };
            ASDI_START[entry_id] = false;
        };
    }
}

pub fn install() {
    install_agent_frame_callbacks!(
        asdi
    );
}