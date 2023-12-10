use super::*;

//Universal Fighter Reset
unsafe extern "C" fn fighter_reset(fighter: &mut L2CFighterCommon) {
	let fighter_kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
	if fighter_kind == *FIGHTER_KIND_DONKEY {
		fighter.global_table[THROW_F_STATUS_KIND].assign(&FIGHTER_STATUS_KIND_THROW.into());
		fighter.global_table[THROW_HI_STATUS_KIND].assign(&FIGHTER_DONKEY_STATUS_KIND_SHOULDER_START.into());
	}
	if fighter_kind == *FIGHTER_KIND_SAMUSD {
		fighter.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
		fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
	}
	if [*FIGHTER_KIND_PIKACHU, *FIGHTER_KIND_GANON].contains(&fighter_kind) {
		fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
		fighter.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
	}
	if fighter_kind == *FIGHTER_KIND_PLIZARDON {
		fighter.global_table[THROW_HI_STATUS_KIND].assign(&FIGHTER_STATUS_KIND_THROW_KIRBY.into());
	}
}

unsafe extern "C" fn reset_frame(fighter: &mut L2CFighterCommon) {
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
		FIGHTER_BOOL_1[entry_id] = false;
		FIGHTER_BOOL_2[entry_id] = false;
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
		HAS_CATCH[entry_id] = false;
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_LAG_REDUCTION);
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
	};
	//Out of Game Reset
	if !smash::app::sv_information::is_ready_go() {
		//Universal
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
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
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
		HAS_CATCH[entry_id] = false;
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_LAG_REDUCTION);
		WorkModule::set_flag(boma, sv_information::is_ready_go(), FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
		SPECIAL_WALL_JUMP = false;
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_TRAINING_MODE_FEATURES);
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
	};
	//Training Mode Reset
	if is_training_mode() && !smash::app::sv_information::is_ready_go() {
		//Universal
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
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
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
		HAS_CATCH[entry_id] = false;
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_LAG_REDUCTION);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
		SPECIAL_WALL_JUMP = false;
		WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_TRAINING_MODE_FEATURES);
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
	}
}

//Installation
pub fn install() {
	Agent::new("mario").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("donkey").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("link").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("samus").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("samusd").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("yoshi").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("kirby").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("fox").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("pikachu").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("luigi").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("ness").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("captain").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("purin").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("peach").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("daisy").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("koopa").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("popo").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("nana").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("sheik").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("zelda").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("mariod").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("pichu").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("falco").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("marth").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("lucina").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("younglink").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("ganon").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("mewtwo").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("roy").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("chrom").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("gamewatch").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("metaknight").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("pit").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("pitb").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("szerosuit").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("wario").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("snake").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("ike").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("pzenigame").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("pfushigisou").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("plizardon").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("diddy").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("lucas").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("sonic").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("dedede").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("pikmin").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("lucario").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("robot").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("toonlink").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("wolf").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("murabito").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("rockman").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("wiifit").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("rosetta").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("littlemac").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("gekkouga").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("miifighter").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("miiswordsman").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("miigunner").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("palutena").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("pacman").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("reflet").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("shulk").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("koopajr").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("duckhunt").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("ryu").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("ken").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("cloud").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("kamui").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("bayonetta").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("inkling").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("ridley").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("simon").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("richter").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("krool").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("shizue").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("gaogaen").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("packun").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("jack").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("brave").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("buddy").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("dolly").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("master").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("tantan").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("pickel").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("edge").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("elight").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("eflame").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("demon").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("trail").on_start(fighter_reset).on_line(Main, reset_frame).install();
	Agent::new("koopag").on_start(fighter_reset).on_line(Main, reset_frame).install();
}