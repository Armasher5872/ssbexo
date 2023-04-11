#![allow(
	unused_macros,
	unused_mut,
    unused_parens
)]
use {
	crate::functions::variables::*,
    smash::{
        app::lua_bind::*,
        lib::lua_const::*,
		lua2cpp::L2CFighterCommon
    },
	smashline::*,
};

//Universal Fighter Reset
#[fighter_reset]
fn fighter_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		//Universal Variables
		AUTO_COUNTER[entry_id] = false;
		CAN_ADD[entry_id] = false;
		CAN_GLIDE_TOSS[entry_id] = false;
		CAN_JAB[entry_id] = 0;
		CAN_RAPID_JAB[entry_id] = 0;
		DAMAGED[entry_id] = false;
		DAMAGED_PREVENT[entry_id] = false;
		DASH_GRAB_SPEED[entry_id] = 0.0;
		DID_MAX_JUMP_COUNT[entry_id] = false;
		FIGHTER_BOOL_1[entry_id] = false;
		FIGHTER_BOOL_2[entry_id] = false;
		FULL_SMASH_ATTACK[entry_id] = false;
		HITFLOW[entry_id] = false;
		IS_WAVEDASH[entry_id] = false;
		MASHING[entry_id] = 0;
		PARRIED[entry_id] = 0;
		PARRY_TIMER[entry_id] = 0;
		SHIELD_BREAK_TIMER[entry_id] = 0;
		SHIELD_SPECIAL[entry_id] = false;
		SPECIAL_ZOOM_GFX[entry_id] = 0;
		TAP_NUM[entry_id] = 6;
		TAP_WAIT[entry_id] = 6;
		USED_FS[entry_id] = false;
		WAVEDASH_DONE[entry_id] = false;
		//Bowser Variables
		CAN_FIREBALL[entry_id] = false;
		FIREBALL_GFX[entry_id] = 0;
		FIREBALL_TIMER[entry_id] = 0;
		KOOPA_EXCELLENT_SMASH[entry_id] = false;
		KOOPA_EXCELLENT_SMASH_GFX[entry_id] = 0;
		KOOPA_GOOD_SMASH[entry_id] = false;
		KOOPA_GOOD_SMASH_GFX[entry_id] = 0;
		KOOPA_GREAT_SMASH[entry_id] = false;
		KOOPA_GREAT_SMASH_GFX[entry_id] = 0;
		KOOPA_OK_SMASH[entry_id] = false;
		KOOPA_OK_SMASH_GFX[entry_id] = 0;
		//Captain Falcon Variables
		BOOST_INSTALL_ACTIVE[entry_id] = false;
		BOOST_INSTALL_GFX_COUNTER[entry_id] = 0;
		BOOST_INSTALL_MOTION_RATE[entry_id] = 1.0;
		BOOST_INSTALL_TIMER[entry_id] = 0;
		FALCON_PUNCH_HIT[entry_id] = false;
		FALCON_PUNCH_TURN_COUNT[entry_id] = 0.0;
		HYPE_HIT[entry_id] = false;
		KIRBY_FALCON_PUNCH_TURN_COUNT[entry_id] = 0.0;
		//Dark Samus Variables
		CHARGE_SHOT_TIMER[entry_id] = 0;
		HAS_FIRE_CHARGE_SHOT[entry_id] = false;
		SAMUSD_CHECK_FLOAT[entry_id] = 0;
		SAMUSD_FLOAT[entry_id] = 0;
		SAMUSD_FLOAT_GFX_COUNTER[entry_id] = 0;
		SAMUSD_HAS_FLOAT[entry_id] = false;
		SAMUSD_START_FLOAT[entry_id] = false;
		SAMUSD_X[entry_id] = 0.0;
		SAMUSD_Y[entry_id] = 0.0;
		//DK Variables
		BARREL_ACTIVE[entry_id] = false;
		BARREL_TIMER[entry_id] = 0;
		BARREL_DIRECTION[entry_id] = 0;
		DONKEY_DASH_ATTACK_JUMP[entry_id] = 0;
		DONKEY_DASH_ATTACK_POWER_DOWN[entry_id] = false;
		DONKEY_GIANT_PUNCH_STAGE[entry_id] = 0;
		SPEED_X[entry_id] = 8.0;
		SPEED_Y[entry_id] = 8.0;
		//Falco Variables
		REFLECTOR_KNOCKBACK[entry_id] = 100;
		REFLECTOR_ANGLE[entry_id] = 60;
		//Ivysaur Variables
		IVYSAUR_IS_SPECIAL_N[entry_id] = false;
		//King K Rool Variables
		KROOL_HAS_UAIR[entry_id] = false;
		KROOL_UP_SPECIAL_CANCEL[entry_id] = false;
		//Lucario Variables
		MEGA_EVOLVE[entry_id] = false;
		//Lucina Variables
		DID_ASTRA_2_S[entry_id] = false;
		DID_ASTRA_5_HI[entry_id] = false;
		LANDING_HIT[entry_id] = false;
		LUCINA_GFX_COUNTER[entry_id] = 0;
		USE_SWORDSMAN_DASH[entry_id] = true;
		USE_UP_SPECIAL[entry_id] = true;
		//Mewtwo Variables
		FUTURESIGHT_CURRENT_FRAME[entry_id] = 0;
		FUTURESIGHT_HIT_COOLDOWN_FRAME[entry_id] = 0;
		FUTURESIGHT_LAST_STATUS[entry_id] = 0;
		FUTURESIGHT_X[entry_id] = 0.0;
		FUTURESIGHT_Y[entry_id] = 0.0;
		GHOST_DASH_ENABLED[entry_id] = false;
		GROUNDED_TELEPORT[entry_id] = false;
		HAS_FUTURESIGHT[entry_id] = false;
		SPEED_ADD[entry_id] = false;
		STORED_POWER_ENABLED[entry_id] = 0;
		STORED_POWER_GFX_TIMER[entry_id] = 0;
		STORED_POWER_POINT[entry_id] = 0;
		STORED_POWER_TIMER[entry_id] = 0;
		UP_SPECIAL_CANCEL[entry_id] = false;
		UP_SPECIAL_JUMP_REFRESH[entry_id] = false;
		//Mii Brawler Variables
		USE_ONSLAUGHT[entry_id] = true;
		//Ness Variables
		OFFENSE_UP_ACTIVE[entry_id] = false;
		OFFENSE_UP_TIMER[entry_id] = 0;
		OFFENSE_UP_GFX_COUNTER[entry_id] = 0;
		PK_FLASH_TIMER[entry_id] = 0;
		//Pichu Variables
		DISCHARGE_ACTIVE[entry_id] = false;
		DISCHARGE_DAMAGE_TIMER[entry_id] = 60;
		DISCHARGE_GFX[entry_id] = 0;
		ELECTRIC_HIT[entry_id] = 0;
		USE_TACKLE[entry_id] = true;
		//Ridley Variables
		POGO_GROUND_BOUNCE[entry_id] = false;
		POGO_OPPONENT_BOUNCE[entry_id] = false;
		RIDLEY_INT_SPECIAL_HI_REBOUNCE_COUNT[entry_id] = 0;
		//Roy Variables
		ROY_GFX_COUNTER[entry_id] = 0;
		//Senator Armstrong Variables
		ARMSTRONG_IS_SPECIAL_HI[entry_id] = false;
		USE_DROPKICK[entry_id] = true;
		//Snake Variables
		GRENADE_HOLD[entry_id] = false;
		SNAKE_FLAG_ATTACK_S4_COMBO_ENABLE[entry_id] = false;
		SNAKE_FLAG_ATTACK_S4_COMBO_IS_BUFFERED[entry_id] = false;
		SNAKE_INT_ATTACK_S4_COMBO_COUNT[entry_id] = 0;
		//Sonic Variables
		BOUNCE_BRACELET_POWER[entry_id] = 3.0;
		FAIR_HIT[entry_id] = false;
		HOMING_ATTACK_HIT[entry_id] = false;
		SONIC_BOOST[entry_id] = 0.0;
		SONIC_BOOST_GFX_COUNTER[entry_id] = 0;
		SONIC_BOOST_SPEED[entry_id] = 0.0;
		//Squirtle Variables
		PZENIGAME_WITHDRAW_JUMP[entry_id] = 0;
	}
}

//Installation
pub fn install() {
	install_agent_resets!(fighter_reset);
}