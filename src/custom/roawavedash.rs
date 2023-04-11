#![allow(unused_macros)]
use {
	crate::functions::variables::*,
    smash::{
        app::{
            lua_bind::{
                PostureModule,
                *
            },
			utility::*,
        },
        lua2cpp::L2CFighterCommon,
        lib::{
			L2CValue,
			lua_const::*,
		},
        phx::Vector2f
    },
    smashline::*,
};

//Credit to Chrispo
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

//Changes your situation kind if you're wavedashing
#[skyline::hook(replace = smash::app::lua_bind::StatusModule::change_status_request)]
pub unsafe fn change_status_request_hook(module_accessor: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, arg3: bool) -> u64 {
	let next_status = status_kind;
	if smash::app::utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
		if [*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&next_status) {
			if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI) {
				original!()(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false)
			} 
			else {
				original!()(module_accessor, status_kind, arg3)
			}
		} 
		else if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&next_status) {
			let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
			if IS_WAVEDASH[entry_id] == true {
				StatusModule::set_situation_kind(module_accessor, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
			}
			original!()(module_accessor, status_kind, arg3)
		}
		else {
			original!()(module_accessor, status_kind, arg3)
		}
	} 
	else {
		original!()(module_accessor, status_kind, arg3)
	}
}

//Automatically forces you to the ground if you're buffering Wavedashes during the startup of Airdodge
#[common_status_script(status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE, symbol = "_ZN7lua2cpp16L2CFighterCommon20status_pre_EscapeAirEv")]
pub unsafe fn status_pre_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
	let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	let y = ControlModule::get_stick_y(boma);
    //Handles wavedash
    if IS_WAVEDASH[entry_id] == true && y < 0.5 && (ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI)) {
        GroundModule::attach_ground(fighter.module_accessor, true);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }
    call_original!(fighter)
}

pub fn install() {
    install_agent_frame_callbacks!(
        roawavedash
    );
	skyline::install_hooks!(
		change_status_request_hook
    );
	install_status_scripts!(status_pre_EscapeAir);
}