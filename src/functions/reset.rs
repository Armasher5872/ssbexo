use super::*;

unsafe extern "C" fn reset_frame(fighter: &mut L2CFighterCommon) {
	let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
	let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	let fighter_kind = fighter.global_table[FIGHTER_KIND].get_i32();
	let status_kind = fighter.global_table[STATUS_KIND].get_i32();
	let team_no = TeamModule::team_no(boma) as i32;
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
		WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
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
		if fighter_kind == *FIGHTER_KIND_GANON {
			WorkModule::set_flag(boma, false, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_S);
			WorkModule::set_flag(boma, false, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_HI);
			WorkModule::set_int(boma, 0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_INT_CHARGE_FRAME);
			WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_ARMOR_CHARGE_MULTIPLIER);
			WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_END);
			WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CHARGE_START);
			WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_CURRENT_DAMAGE);
			WorkModule::set_float(boma, 0.0, FIGHTER_ARMSTRONG_INSTANCE_WORK_ID_FLOAT_DAMAGE_CHARGE_MULTIPLIER);	
		}
		//Bayonetta
		if fighter_kind == *FIGHTER_KIND_BAYONETTA {
			WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
		}
		//Captain
		if fighter_kind == *FIGHTER_KIND_CAPTAIN {
			WorkModule::set_flag(boma, false, FIGHTER_FALCON_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT);
			WorkModule::set_int(boma, 0, FIGHTER_FALCON_INSTANCE_WORK_ID_INT_FALCON_PUNCH_TURN_COUNT);
		}
		//Dedede
		if fighter_kind == *FIGHTER_KIND_DEDEDE {
			WorkModule::set_flag(boma, false, FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_LINK_ITEM_FUSE_BACK);
			WorkModule::set_flag(boma, false, FIGHTER_DEDEDE_INSTANCE_WORK_ID_FLAG_HAS_JET_CHARGE);
			WorkModule::set_int(boma, *ITEM_KIND_NONE, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
			WorkModule::set_int(boma, 0, FIGHTER_DEDEDE_INSTANCE_WORK_ID_INT_JET_CHARGE_PROGRESS);
		}
		//Dolly
		if fighter_kind == *FIGHTER_KIND_DOLLY {
			WorkModule::set_flag(boma, false, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FEINTED);
		}
		//Edge
		if fighter_kind == *FIGHTER_KIND_EDGE {
			WorkModule::set_flag(boma, false, FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_RUSH_CANCEL);
			WorkModule::set_int(boma, 0, FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_SPECIAL_N_DIRECTION);
		}
		//Ike
		if fighter_kind == *FIGHTER_KIND_IKE {
			WorkModule::set_flag(boma, false, FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_CAN_BOUND);
			WorkModule::set_flag(boma, false, FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_N);
			WorkModule::set_float(boma, 0.0, FIGHTER_IKE_INSTANCE_WORK_ID_FLOAT_BOUND_ANGLE);
			WorkModule::set_float(boma, 0.0, FIGHTER_IKE_INSTANCE_WORK_ID_FLOAT_X_CHECK);
			WorkModule::set_float(boma, 0.0, FIGHTER_IKE_INSTANCE_WORK_ID_FLOAT_Y_CHECK);
		}
		//Kirby
		if fighter_kind == *FIGHTER_KIND_KIRBY {
			WorkModule::set_flag(boma, false, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_WHEEL_RECOIL);
			WorkModule::set_float(boma, 0.0, FIGHTER_KIRBY_INSTANCE_WORK_ID_FLOAT_WHEEL_POWER_UP);
			WHEEL_SPEED_UP[entry_id] = 0.0;
			WorkModule::set_int(boma, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_HOLD_TIMER);
			WorkModule::set_int(boma, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_JUMP_COUNT);
			WorkModule::set_int(boma, 0, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_TURN_COUNT);
			ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_armfoot"), true);
			ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_eye1"), true);
			ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("kirby_facen"), true);
		}
		//Link
		if fighter_kind == *FIGHTER_KIND_LINK {
			WorkModule::set_flag(boma, false, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
			WorkModule::set_flag(boma, false, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPIN_ATTACK_CAN_FALL);
			WorkModule::set_flag(boma, false, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM);
			WorkModule::set_flag(boma, false, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED);
			WorkModule::set_flag(boma, false, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CAN_ASCEND);
			WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
			WorkModule::set_int(boma, team_no, FIGHTER_LINK_INSTANCE_WORK_ID_INT_TEAM_NO);
			WorkModule::set_int(boma, *ITEM_KIND_NONE, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
			WorkModule::set_int(boma, *ITEM_KIND_NONE, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
			WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
			WorkModule::set_int(boma, 0, FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ASCEND_FRAME);
			WorkModule::set_float(boma, 0.0, FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_START_Y);
			WorkModule::set_float(boma, 0.0, FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y);
		}
		//Little Mac
		if fighter_kind == *FIGHTER_KIND_LITTLEMAC {
			WorkModule::set_flag(boma, false, FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_IS_KO_GAUGE_TUMBLE_REDUCTION);
			WorkModule::set_int(boma, 0, FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_INT_STAR_PUNCH_STRENGTH);
		}
		//Luigi
		if fighter_kind == *FIGHTER_KIND_LUIGI {
			WorkModule::set_flag(boma, false, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_TYPHOON_ACTIVE);
			LUIGI_CYCLONE_RNG[entry_id] = 9;
		}
		//Marth
		if fighter_kind == *FIGHTER_KIND_MARTH {
			WorkModule::set_flag(boma, false, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DID_ANGLE);
		}
		//Meta Knight
		if fighter_kind == *FIGHTER_KIND_METAKNIGHT {
			WorkModule::set_flag(boma, false, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_S);
		}
		//Mewtwo
		if fighter_kind == *FIGHTER_KIND_MEWTWO {
			WorkModule::set_float(boma, 1.0, FIGHTER_MEWTWO_INSTANCE_WORK_ID_FLOAT_PSYCHIC_GLARE_POWER);
		}
		//Miiswordsman
		if fighter_kind == *FIGHTER_KIND_MIISWORDSMAN {
			WorkModule::set_flag(boma, false, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLAG_BLURRING_SLASHES_CANCEL);
			WorkModule::set_int(boma, 0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_AIRBORNE_ASSAULT_ANGLE);
			WorkModule::set_int(boma, 0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_BLURRING_SLASHES_TIMER);
			WorkModule::set_int(boma, 0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_DIRECTION);
			WorkModule::set_int(boma, 1, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_INT_LIGHT_SHURIKEN_COUNT);
			WorkModule::set_float(boma, 1.0, FIGHTER_MIISWORDSMAN_INSTANCE_WORK_ID_FLOAT_GUARD_BREAKER_POWER);
		}
		//Murabito
		if fighter_kind == *FIGHTER_KIND_MURABITO {
			WorkModule::set_int(boma, *ITEM_KIND_NONE, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
			WorkModule::set_int(boma, *ITEM_KIND_NONE, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM);
			WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM_ID);
			WorkModule::set_int(boma, team_no, FIGHTER_MURABTIO_INSTANCE_WORK_ID_INT_TEAM_NO);
		}
		//Pfushigisou
		if fighter_kind == *FIGHTER_KIND_PFUSHIGISOU {
			PFUSHIGISOU_IS_ACTIVE_BOMB[entry_id] = false;
		}
		//Pikachu
		if fighter_kind == *FIGHTER_KIND_PIKACHU {
			WorkModule::set_flag(boma, false, FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_ATTACK_11_DASH);
			WorkModule::set_int(boma, 0, FIGHTER_PIKACHU_INSTANCE_WORK_ID_INT_ATTACK_11_COUNT);
		}
		//Purin
		if fighter_kind == *FIGHTER_KIND_PURIN {
			WorkModule::set_flag(boma, false, FIGHTER_PURIN_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_N);
		}
		if fighter_kind == *FIGHTER_KIND_ROBOT {
			WorkModule::set_float(boma, 0.0, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_SNAKE_SPEED_VALUE);
			WorkModule::set_flag(boma, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_POWER_BOOST);
			WorkModule::set_flag(boma, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_CAN_SNAKE);
			WorkModule::set_flag(boma, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_DID_POWER_BOOST);
			WorkModule::set_flag(boma, false, FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_EARLY_CANCEL);
		}
		//Rosetta
		if fighter_kind == *FIGHTER_KIND_ROSETTA {
			WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_ROSETTA_STATUS_SPECIAL_LW_INT_CAPTURE_OBJECT_ID);
		}
		//Ryu
		if fighter_kind == *FIGHTER_KIND_RYU {
			WorkModule::set_flag(boma, false, FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_IS_HASOGEKI);
		}
		//Samusd
		if fighter_kind == *FIGHTER_KIND_SAMUSD {
			HAS_FIRE_CHARGE_SHOT[entry_id] = false;
			CHARGE_SHOT_TIMER[entry_id] = 0;
		}
		//Sheik
		if fighter_kind == *FIGHTER_KIND_SHEIK {
			ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("gamemodel"), true);
			ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("hair"), true);
			ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sheik_eye"), true);
			ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sheik_facen"), true);
			ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sheik_openblink"), true);
			WorkModule::set_flag(boma, false, FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HAS_VANISHED);
			WorkModule::set_flag(boma, false, FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_VANISH_ATTACK);
			WorkModule::set_flag(boma, false, FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BACK_HIT);
			WorkModule::set_int(boma, 0, FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_VANISH_TIMER);
			WorkModule::set_int(boma, 0, FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_SHEIKAH_EYE_TIMER);
		}
		//Shizue
		if fighter_kind == *FIGHTER_KIND_SHIZUE {
			WorkModule::set_int(boma, *ITEM_KIND_NONE, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
			WorkModule::set_int(boma, *ITEM_KIND_NONE, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM);
			WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_BOOMERANG_FUSE_ITEM_ID);
			WorkModule::set_int(boma, team_no, FIGHTER_MURABTIO_INSTANCE_WORK_ID_INT_TEAM_NO);
		}
		//Sonic
		if fighter_kind == *FIGHTER_KIND_SONIC {
			WorkModule::set_flag(boma, false, FIGHTER_SONIC_INSTANCE_WORK_ID_FLAG_BOOST_GRAVITY);
			WorkModule::set_int(boma, 0, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_GAUGE);
			WorkModule::set_int(boma, 0, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_GAUGE_DECREASE);
			WorkModule::set_int(boma, 0, FIGHTER_SONIC_INSTANCE_WORK_ID_INT_BOOST_EFFECT_COUNTER);
			WorkModule::set_float(boma, 0.0, FIGHTER_SONIC_INSTANCE_WORK_ID_FLOAT_BOOST_SPEED);
		}
	};
}

//Installation
pub fn install() {
	Agent::new("fighter")
	.on_line(Main, reset_frame)
	.install()
	;
}