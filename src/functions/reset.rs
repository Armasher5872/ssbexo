use super::*;

//Universal Fighter Reset
#[fighter_reset]
fn fighter_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
		let fighter_kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
		let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if fighter_kind == *FIGHTER_KIND_SAMUSD {
			fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
            fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
		}
		if fighter_kind == *FIGHTER_KIND_PIKACHU {
			fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
            fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
		}
        if fighter_kind == *FIGHTER_KIND_PLIZARDON {
            fighter.global_table[THROW_B_STATUS_KIND].assign(&FIGHTER_STATUS_KIND_THROW_KIRBY.into());
        }
	}
}

#[fighter_frame_callback]
pub fn reset_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
		let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		let fighter_kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
		let status_kind = fighter.global_table[STATUS_KIND].get_i32();
		let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
		//Death Reset
		if [*FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) {
			//Universal
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
			COUNTERHIT_CHECK[entry_id] = false;
			COUNTERHIT_SUCCESS[entry_id] = false;
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH_DONE);
			WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
			LAST_DAMAGE[entry_id] = 0.0;
			WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SHIELD_COLOR_R);
			WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SHIELD_COLOR_G);
			WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SHIELD_COLOR_B);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
			//Donkey Kong
			WorkModule::set_flag(boma, false, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_POWER_DOWN);
			WorkModule::set_float(boma, 0.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_X);
			WorkModule::set_float(boma, 0.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_Y);
			WorkModule::set_int(boma, 0, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_DIRECTION);
			//Dark Samus
			SAMUSD_X[entry_id] = 0.0;
			SAMUSD_Y[entry_id] = 0.0;
			//Kirby
			WorkModule::set_int(boma, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_HOLD_TIMER);
			WorkModule::set_int(boma, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_JUMP_COUNT);
			WorkModule::set_float(boma, 0.0, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLOAT_WHEEL_POWER_UP);
			WorkModule::set_flag(boma, false, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_WHEEL_RECOIL);
			WorkModule::set_int(boma, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_TURN_COUNT);
			WHEEL_SPEED_UP[entry_id] = 0.0;
			if fighter_kind == *FIGHTER_KIND_KIRBY
			&& [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WAIT, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK].contains(&prev_status_kind) {
				ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), true);
				ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_eye1"), true);
				ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_facen"), true);
			}
			//Pikachu
			WorkModule::set_int(boma, 0, FIGHTER_PIKACHU_INSTANCE_WORK_ID_INT_ATTACK_11_COUNT);
			WorkModule::set_flag(boma, false, FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_ATTACK_11_DASH);
			//Luigi
			DOWN_B_CATCH[entry_id] = false;
			LUIGI_CYCLONE_RNG[entry_id] = 9;
			WorkModule::set_flag(boma, false, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE);
			//Little Mac
			MAC_HITSTUN[get_player_number(boma)] = 0;
		};
		//Out of Game Reset
        if !smash::app::sv_information::is_ready_go() {
			//Universal
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
			COUNTERHIT_CHECK[entry_id] = false;
			COUNTERHIT_SUCCESS[entry_id] = false;
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
			FEATURES = false;
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH_DONE);
			WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
			LAST_DAMAGE[entry_id] = 0.0;
			WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SHIELD_COLOR_R);
			WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SHIELD_COLOR_G);
			WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SHIELD_COLOR_B);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
			//Donkey Kong
			WorkModule::set_flag(boma, false, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_POWER_DOWN);
			WorkModule::set_flag(boma, false, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE);
			WorkModule::set_float(boma, 0.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_X);
			WorkModule::set_float(boma, 0.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_Y);
			WorkModule::set_int(boma, 0, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_DIRECTION);
			WorkModule::set_int(boma, 0, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_TIMER);
			//Dark Samus
			CHARGE_SHOT_TIMER[entry_id] = 0;
			HAS_FIRE_CHARGE_SHOT[entry_id] = false;
			SAMUSD_X[entry_id] = 0.0;
			SAMUSD_Y[entry_id] = 0.0;
			//Kirby
			WorkModule::set_int(boma, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_HOLD_TIMER);
			WorkModule::set_int(boma, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_JUMP_COUNT);
			WorkModule::set_float(boma, 0.0, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLOAT_WHEEL_POWER_UP);
			WorkModule::set_flag(boma, false, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_WHEEL_RECOIL);
			WorkModule::set_int(boma, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_TURN_COUNT);
			WHEEL_SPEED_UP[entry_id] = 0.0;
			if fighter_kind == *FIGHTER_KIND_KIRBY
			&& [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WAIT, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK].contains(&prev_status_kind) {
				ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), true);
				ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_eye1"), true);
				ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_facen"), true);
			}
			//Pikachu
			WorkModule::set_int(boma, 0, FIGHTER_PIKACHU_INSTANCE_WORK_ID_INT_ATTACK_11_COUNT);
			WorkModule::set_flag(boma, false, FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_ATTACK_11_DASH);
			//Luigi
			DOWN_B_CATCH[entry_id] = false;
			LUIGI_CYCLONE_RNG[entry_id] = 9;
			WorkModule::set_flag(boma, false, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE);
			//Diddy
			BANANA_EXIST[entry_id] = false;
			//Little Mac
			MAC_HITSTUN[get_player_number(boma)] = 0;
		};
		//Training Mode Reset
		if is_training_mode() && !smash::app::sv_information::is_ready_go() {
			//Universal
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
			COUNTERHIT_CHECK[entry_id] = false;
			COUNTERHIT_SUCCESS[entry_id] = false;
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
			FEATURES = false;
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_WAVEDASH);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
			WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH_DONE);
			WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
			LAST_DAMAGE[entry_id] = 0.0;
			WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SHIELD_COLOR_R);
			WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SHIELD_COLOR_G);
			WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_SHIELD_COLOR_B);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
			WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
			//Donkey Kong
			WorkModule::set_flag(boma, false, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_POWER_DOWN);
			WorkModule::set_flag(boma, false, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLAG_BARREL_ACTIVE);
			WorkModule::set_float(boma, 0.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_X);
			WorkModule::set_float(boma, 0.0, FIGHTER_DONKEY_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_SPEED_Y);
			WorkModule::set_int(boma, 0, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_DIRECTION);
			WorkModule::set_int(boma, 0, FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_BARREL_TIMER);
			//Dark Samus
			CHARGE_SHOT_TIMER[entry_id] = 0;
			HAS_FIRE_CHARGE_SHOT[entry_id] = false;
			SAMUSD_X[entry_id] = 0.0;
			SAMUSD_Y[entry_id] = 0.0;
			//Kirby
			WorkModule::set_int(boma, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_HOLD_TIMER);
			WorkModule::set_int(boma, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_JUMP_COUNT);
			WorkModule::set_float(boma, 0.0, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLOAT_WHEEL_POWER_UP);
			WorkModule::set_flag(boma, false, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_WHEEL_RECOIL);
			WorkModule::set_int(boma, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_TURN_COUNT);
			WHEEL_SPEED_UP[entry_id] = 0.0;
			if fighter_kind == *FIGHTER_KIND_KIRBY
			&& [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WAIT, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK].contains(&prev_status_kind) {
				ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), true);
				ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_eye1"), true);
				ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_facen"), true);
			}
			//Pikachu
			WorkModule::set_int(boma, 0, FIGHTER_PIKACHU_INSTANCE_WORK_ID_INT_ATTACK_11_COUNT);
			WorkModule::set_flag(boma, false, FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_ATTACK_11_DASH);
			//Luigi
			DOWN_B_CATCH[entry_id] = false;
			LUIGI_CYCLONE_RNG[entry_id] = 9;
			WorkModule::set_flag(boma, false, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE);
			//Diddy
			BANANA_EXIST[entry_id] = false;
			//Little Mac
			MAC_HITSTUN[get_player_number(boma)] = 0;
		}
    };
}

//Installation
pub fn install() {
	install_agent_frame_callbacks!(reset_frame);
	install_agent_resets!(fighter_reset);
}