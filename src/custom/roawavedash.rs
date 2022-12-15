#![allow(unused_macros)]
use {
	crate::functions::{
		IS_WAVEDASH,
		WAVEDASH_DONE
    },
    smash::{
        app::{
            lua_bind::{
                PostureModule,
                *
            },
			utility::*,
        },
        lua2cpp::L2CFighterCommon,
        lib::lua_const::*,
        phx::Vector2f
    },
    smashline::*,
};

#[fighter_frame_callback]
pub fn roawavedash(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
		let fighter_kind = get_kind(boma);
		let status_kind = StatusModule::status_kind(boma);
		let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if ![*FIGHTER_KIND_POPO, *FIGHTER_KIND_NANA].contains(&fighter_kind) {
			if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
				IS_WAVEDASH[entry_id] = true;
			};
			if [*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind) == false {
				IS_WAVEDASH[entry_id] = false;
				WAVEDASH_DONE[entry_id] = false;
			};
			if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind) 
            && IS_WAVEDASH[entry_id] == true {
				let y = ControlModule::get_stick_y(boma);
				let x = ControlModule::get_stick_x(boma);
				if y < 0.3 
                && y > -0.3 {
					let stop_rise  = smash::phx::Vector3f {x: 1.0, y: 0.0, z: 1.0};
					KineticModule::mul_speed(boma, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					while GroundModule::ray_check(boma, &Vector2f{x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)}, &Vector2f{x: 0.0, y: -0.5}, true) == 0 {
						let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma)-0.5, z: 0.0 };
						PostureModule::set_pos(boma, &pos);
						PostureModule::init_pos(boma, &pos, true, true);
					};
					if x > -0.2 
                    && x < 0.2 {
						let stop_rise  = smash::phx::Vector3f { x: 0.0, y: 1.0, z: 1.0 };
						KineticModule::mul_speed(boma, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
						WAVEDASH_DONE[entry_id] = true;
					};
					StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
				};
				IS_WAVEDASH[entry_id] = false;
			};
			if WAVEDASH_DONE[entry_id] == true 
            && status_kind == *FIGHTER_STATUS_KIND_LANDING {
				let stop_rise = smash::phx::Vector3f {x: 0.0, y: 1.0, z: 1.0};
				KineticModule::mul_speed(boma, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
				let speed = smash::phx::Vector3f {x: 0.1, y: 0.0, z: 0.0};
				KineticModule::add_speed(boma, &speed);
				WAVEDASH_DONE[entry_id] = false;
			};
		} 
        else {
			if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
				IS_WAVEDASH[entry_id] = true;
			};
			if [*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind) == false {
				IS_WAVEDASH[entry_id] = false;
				WAVEDASH_DONE[entry_id] = false;
			};
			if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status_kind) 
            && IS_WAVEDASH[entry_id] == true {
				let y = ControlModule::get_stick_y(boma);
				if y < 0.3 
                && y > -0.3 {
					let speed = smash::phx::Vector3f { x: 0.0, y: -3.0, z: 0.0 };
					KineticModule::add_speed(boma, &speed);
					WAVEDASH_DONE[entry_id] = true;
				};
				IS_WAVEDASH[entry_id] = false;
			};
			if WAVEDASH_DONE[entry_id] == true {
				if MotionModule::frame(boma) > 3.0 
                && MotionModule::frame(boma) < 6.0 {
					let stop_rise  = smash::phx::Vector3f { x: 1.0, y: 0.0, z: 1.0 };
					KineticModule::mul_speed(boma, &stop_rise, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
					WAVEDASH_DONE[entry_id] = false;
				};
			};
		};
    };
}

pub fn install() {
    install_agent_frame_callbacks!(
        roawavedash
    );
}