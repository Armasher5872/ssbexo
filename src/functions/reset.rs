use super::*;

//Universal Fighter Reset
#[fighter_reset]
fn fighter_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
		let fighter_kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_PLIZARDON {
            return;
        }
        fighter.global_table[THROW_B_STATUS_KIND].assign(&FIGHTER_STATUS_KIND_THROW_KIRBY.into());
	}
}

#[fighter_frame_callback]
pub fn reset_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let status_kind = StatusModule::status_kind(fighter.module_accessor);
		//Death Reset
		if [*FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) {
			//Universal
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
            WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_GFX_TIMER);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH_DONE);
			//Donkey Kong
			WorkModule::set_int(boma, 0, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_DIRECTION);
			WorkModule::set_flag(boma, false, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_POWER_DOWN);
			WorkModule::set_float(boma, 0.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_X);
			WorkModule::set_float(boma, 0.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_Y);
			//Dark Samus
		};
		//Out of Game Reset
        if !smash::app::sv_information::is_ready_go() {
			//Universal
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
            WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_GFX_TIMER);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH_DONE);
			//Donkey Kong
			WorkModule::set_flag(boma, false, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE);
			WorkModule::set_int(boma, 0, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_DIRECTION);
			WorkModule::set_int(boma, 0, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_TIMER);
			WorkModule::set_flag(boma, false, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_POWER_DOWN);
			WorkModule::set_float(boma, 0.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_X);
			WorkModule::set_float(boma, 0.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_Y);
			//Dark Samus
			CHARGE_SHOT_TIMER[entry_id] = 0;
			HAS_FIRE_CHARGE_SHOT[entry_id] = false;
		};
		//Training Mode Reset
		if is_training_mode() && !smash::app::sv_information::is_ready_go() {
			//Universal
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
            WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_GFX_TIMER);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH_DONE);
			//Donkey Kong
			WorkModule::set_flag(boma, false, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE);
			WorkModule::set_int(boma, 0, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_DIRECTION);
			WorkModule::set_int(boma, 0, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_TIMER);
			WorkModule::set_flag(boma, false, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_POWER_DOWN);
			WorkModule::set_float(boma, 0.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_X);
			WorkModule::set_float(boma, 0.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_Y);
			//Dark Samus
			CHARGE_SHOT_TIMER[entry_id] = 0;
			HAS_FIRE_CHARGE_SHOT[entry_id] = false;
		}
    };
}

//Installation
pub fn install() {
	install_agent_frame_callbacks!(reset_frame);
	install_agent_resets!(fighter_reset);
}