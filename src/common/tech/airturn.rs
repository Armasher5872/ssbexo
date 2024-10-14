use super::*;

pub unsafe extern "C" fn flip_air(fighter : &mut L2CFighterCommon) {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);    
	let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
	let status_kind = smash::app::lua_bind::StatusModule::status_kind(boma);
	let stick_x = ControlModule::get_stick_x(boma) * PostureModule::lr(boma);
	let fall_status = [*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_FALL_AERIAL, *FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL];
	let attack_status = [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_AIR_LASSO];
	if TAP_WAIT[entry_id] > 1 {
		TAP_WAIT[entry_id] -= 1;
	};
	if TAP_WAIT[entry_id] == 1 {
		TAP_NUM[entry_id] = 0;
	};
	if situation_kind != *SITUATION_KIND_AIR {
		TAP_WAIT[entry_id] = 0;
	};
	if situation_kind == *SITUATION_KIND_AIR && (fall_status.contains(&status_kind) || (attack_status.contains(&status_kind) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT))) {
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
			CancelModule::enable_cancel(boma);
			TAP_WAIT[entry_id] = 1;
			TAP_NUM[entry_id] = 0;
		};
	} 
	else {
		TAP_NUM[entry_id] = 0;
	};
}

pub fn install() {
	Agent::new("fighter")
	.on_line(Main, flip_air)
	.install()
	;
}