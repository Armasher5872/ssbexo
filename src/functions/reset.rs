use super::*;

unsafe extern "C" fn reset_frame(fighter: &mut L2CFighterCommon) {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
	let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	let status_kind = fighter.global_table[STATUS_KIND].get_i32();
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
		FIGHTER_BOOL_1[entry_id] = false;
		FIGHTER_BOOL_2[entry_id] = false;
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
		SPECIAL_WALL_JUMP = false;
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
		WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
		WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
		LAST_DAMAGE[entry_id] = 0.0;
		SIZE0[entry_id] = 0.0;
		SIZE1[entry_id] = 0.0;
		SIZE2[entry_id] = 0.0;
		SIZE3[entry_id] = 0.0;
		FULL_HOP_ENABLE_DELAY[entry_id] = 0;
		WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
		WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
		WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
		WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
		WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
		WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
		WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
		//Armstrong
		WorkModule::set_flag(boma, false, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_S);
		WorkModule::set_flag(boma, false, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_HI);
		WorkModule::set_int(boma, 0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
		WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
		WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
		WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
		WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CURRENT_DAMAGE);
		WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);
		//Kirby
		WorkModule::set_flag(boma, false, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_WHEEL_RECOIL);
		WorkModule::set_float(boma, 0.0, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLOAT_WHEEL_POWER_UP);
		WHEEL_SPEED_UP[entry_id] = 0.0;
		WorkModule::set_int(boma, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_HOLD_TIMER);
		WorkModule::set_int(boma, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_JUMP_COUNT);
		WorkModule::set_int(boma, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_TURN_COUNT);
		//Luigi
		WorkModule::set_flag(boma, false, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE);
		LUIGI_CYCLONE_RNG[entry_id] = 9;
		//Meta Knight
		WorkModule::set_flag(boma, false, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_S);
		//Mii Swordfighter
		WorkModule::set_flag(boma, false, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_BLURRING_SLASHES_CANCEL);
		WorkModule::set_int(boma, 0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_AIRBORNE_ASSAULT_ANGLE);
		WorkModule::set_int(boma, 0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_BLURRING_SLASHES_TIMER);
		WorkModule::set_int(boma, 0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_DIRECTION);
		WorkModule::set_int(boma, 1, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_LIGHT_SHURIKEN_COUNT);
		WorkModule::set_float(boma, 1.0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLOAT_GUARD_BREAKER_POWER);
		//Pikachu
		WorkModule::set_flag(boma, false, FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_ATTACK_11_DASH);
		WorkModule::set_int(boma, 0, FIGHTER_PIKACHU_INSTANCE_WORK_ID_INT_ATTACK_11_COUNT);
		//Dark Samus
		HAS_FIRE_CHARGE_SHOT[entry_id] = false;
		CHARGE_SHOT_TIMER[entry_id] = 0;
		SAMUSD_X[entry_id] = 0.0;
		SAMUSD_Y[entry_id] = 0.0;
		//Sonic
		WorkModule::set_flag(boma, false, FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_BOOST_GRAVITY);
		WorkModule::set_int(boma, 0, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_GAUGE);
		WorkModule::set_int(boma, 0, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_GAUGE_DECREASE);
		WorkModule::set_int(boma, 0, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_EFFECT_COUNTER);
		WorkModule::set_float(boma, 0.0, FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_BOOST_SPEED);
	};
}

//Installation
pub fn install() {
	smashline::api::install_line_callback(None, StatusLine::Main, reset_frame as _);
}