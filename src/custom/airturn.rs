#![allow(unused_macros)]
use {
    crate::functions::{
        TAP_MAX,
        TAP_NUM,
        TAP_WAIT
    },
    smash::{
        app::lua_bind::*,
        lib::lua_const::*,
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
    },
    smashline::*,
};

#[fighter_frame_callback]
pub fn flip_air(fighter : &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
		let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
		let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma, Hash40::new_raw(MotionModule::motion_kind(boma)), false) as f32;
		let frame = MotionModule::frame(boma);
		let situation_kind = StatusModule::situation_kind(boma);
		if TAP_WAIT[entry_id] > 1 {
			TAP_WAIT[entry_id] -= 1;
		};
		if TAP_WAIT[entry_id] == 1 {
			TAP_NUM[entry_id] = 0;
		};
		if situation_kind != *SITUATION_KIND_AIR {
			TAP_WAIT[entry_id] = 0;
		};
		if situation_kind == *SITUATION_KIND_AIR 
        && ([*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_FALL_AERIAL, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL].contains(&status_kind) 
        || (frame >= cancel_frame && cancel_frame > 0.0)) {
			if TAP_NUM[entry_id] == 0 && stick_x < -0.5 {
				TAP_NUM[entry_id] = 1;
				if TAP_WAIT[entry_id] == 0 {
					TAP_WAIT[entry_id] = TAP_MAX;
				};
			} 
            else if TAP_NUM[entry_id] == 1 && stick_x > -0.2 {
				TAP_NUM[entry_id] = 2;
			}
            else if TAP_NUM[entry_id] == 2 && stick_x < -0.5 {
				PostureModule::reverse_lr(boma);
				PostureModule::update_rot_y_lr(boma);
				TAP_WAIT[entry_id] = 1;
				TAP_NUM[entry_id] = 0;
			};
		} 
        else {
			TAP_NUM[entry_id] = 0;
		};
		println!("wait {}, num {}, lr {}", TAP_WAIT[entry_id], TAP_NUM[entry_id], PostureModule::lr(boma));
    }
}

pub fn install() {
    install_agent_frame_callbacks!(flip_air);
}