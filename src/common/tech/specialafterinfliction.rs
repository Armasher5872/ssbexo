//Code Here comes from Championship Edition
use super::*;

unsafe extern "C" fn air_specials(module_accessor: &mut smash::app::BattleObjectModuleAccessor, fighter_kind: i32, motion_kind: u64, status_kind: i32, situation_kind: i32) {
	if (fighter_kind == FIGHTER_KIND_SHEIK && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW)
	|| (fighter_kind == FIGHTER_KIND_MEWTWO && (status_kind == *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2 || motion_kind == hash40("special_hi") || motion_kind == hash40("special_air_hi"))) 
	|| (fighter_kind == FIGHTER_KIND_PITB && status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_RUSH)
	|| (fighter_kind == FIGHTER_KIND_SZEROSUIT && status_kind == *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_LW_START)
	|| (fighter_kind == FIGHTER_KIND_LITTLEMAC && status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP)
	|| (fighter_kind == FIGHTER_KIND_MIIFIGHTER && status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_START)
	|| (fighter_kind == FIGHTER_KIND_KOOPAJR && status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_SHOOT) {
		WorkModule::set_flag(module_accessor, true, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
	}
	if situation_kind != SITUATION_KIND_AIR {
		WorkModule::set_flag(module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
	}
	if !WorkModule::is_flag(module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK) {
		if fighter_kind == FIGHTER_KIND_LITTLEMAC {
			if WorkModule::is_flag(module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S) {
				WorkModule::off_flag(module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S);
			}
		}
	}
}

unsafe extern "C" fn special_after_infliction_install(fighter: &mut L2CFighterCommon) {
	let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
	let motion_kind = MotionModule::motion_kind(module_accessor);
	let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
	let situation_kind = StatusModule::situation_kind(module_accessor);
	let fighter_kind = smash::app::utility::get_kind(module_accessor) as i32;
	if smash::app::utility::get_category(module_accessor) == BATTLE_OBJECT_CATEGORY_FIGHTER {
		air_specials(module_accessor, fighter_kind, motion_kind, status_kind, situation_kind);
	}
}

pub fn install() {
	Agent::new("mario").on_line(Main, special_after_infliction_install).install();
	Agent::new("donkey").on_line(Main, special_after_infliction_install).install();
	Agent::new("link").on_line(Main, special_after_infliction_install).install();
	Agent::new("samus").on_line(Main, special_after_infliction_install).install();
	Agent::new("samusd").on_line(Main, special_after_infliction_install).install();
	Agent::new("yoshi").on_line(Main, special_after_infliction_install).install();
	Agent::new("kirby").on_line(Main, special_after_infliction_install).install();
	Agent::new("fox").on_line(Main, special_after_infliction_install).install();
	Agent::new("pikachu").on_line(Main, special_after_infliction_install).install();
	Agent::new("luigi").on_line(Main, special_after_infliction_install).install();
	Agent::new("ness").on_line(Main, special_after_infliction_install).install();
	Agent::new("captain").on_line(Main, special_after_infliction_install).install();
	Agent::new("purin").on_line(Main, special_after_infliction_install).install();
	Agent::new("peach").on_line(Main, special_after_infliction_install).install();
	Agent::new("daisy").on_line(Main, special_after_infliction_install).install();
	Agent::new("koopa").on_line(Main, special_after_infliction_install).install();
	Agent::new("popo").on_line(Main, special_after_infliction_install).install();
	Agent::new("nana").on_line(Main, special_after_infliction_install).install();
	Agent::new("sheik").on_line(Main, special_after_infliction_install).install();
	Agent::new("zelda").on_line(Main, special_after_infliction_install).install();
	Agent::new("mariod").on_line(Main, special_after_infliction_install).install();
	Agent::new("pichu").on_line(Main, special_after_infliction_install).install();
	Agent::new("falco").on_line(Main, special_after_infliction_install).install();
	Agent::new("marth").on_line(Main, special_after_infliction_install).install();
	Agent::new("lucina").on_line(Main, special_after_infliction_install).install();
	Agent::new("younglink").on_line(Main, special_after_infliction_install).install();
	Agent::new("ganon").on_line(Main, special_after_infliction_install).install();
	Agent::new("mewtwo").on_line(Main, special_after_infliction_install).install();
	Agent::new("roy").on_line(Main, special_after_infliction_install).install();
	Agent::new("chrom").on_line(Main, special_after_infliction_install).install();
	Agent::new("gamewatch").on_line(Main, special_after_infliction_install).install();
	Agent::new("metaknight").on_line(Main, special_after_infliction_install).install();
	Agent::new("pit").on_line(Main, special_after_infliction_install).install();
	Agent::new("pitb").on_line(Main, special_after_infliction_install).install();
	Agent::new("szerosuit").on_line(Main, special_after_infliction_install).install();
	Agent::new("wario").on_line(Main, special_after_infliction_install).install();
	Agent::new("snake").on_line(Main, special_after_infliction_install).install();
	Agent::new("ike").on_line(Main, special_after_infliction_install).install();
	Agent::new("pzenigame").on_line(Main, special_after_infliction_install).install();
	Agent::new("pfushigisou").on_line(Main, special_after_infliction_install).install();
	Agent::new("plizardon").on_line(Main, special_after_infliction_install).install();
	Agent::new("diddy").on_line(Main, special_after_infliction_install).install();
	Agent::new("lucas").on_line(Main, special_after_infliction_install).install();
	Agent::new("sonic").on_line(Main, special_after_infliction_install).install();
	Agent::new("dedede").on_line(Main, special_after_infliction_install).install();
	Agent::new("pikmin").on_line(Main, special_after_infliction_install).install();
	Agent::new("lucario").on_line(Main, special_after_infliction_install).install();
	Agent::new("robot").on_line(Main, special_after_infliction_install).install();
	Agent::new("toonlink").on_line(Main, special_after_infliction_install).install();
	Agent::new("wolf").on_line(Main, special_after_infliction_install).install();
	Agent::new("murabito").on_line(Main, special_after_infliction_install).install();
	Agent::new("rockman").on_line(Main, special_after_infliction_install).install();
	Agent::new("wiifit").on_line(Main, special_after_infliction_install).install();
	Agent::new("rosetta").on_line(Main, special_after_infliction_install).install();
	Agent::new("littlemac").on_line(Main, special_after_infliction_install).install();
	Agent::new("gekkouga").on_line(Main, special_after_infliction_install).install();
	Agent::new("miifighter").on_line(Main, special_after_infliction_install).install();
	Agent::new("miiswordsman").on_line(Main, special_after_infliction_install).install();
	Agent::new("miigunner").on_line(Main, special_after_infliction_install).install();
	Agent::new("palutena").on_line(Main, special_after_infliction_install).install();
	Agent::new("pacman").on_line(Main, special_after_infliction_install).install();
	Agent::new("reflet").on_line(Main, special_after_infliction_install).install();
	Agent::new("shulk").on_line(Main, special_after_infliction_install).install();
	Agent::new("koopajr").on_line(Main, special_after_infliction_install).install();
	Agent::new("duckhunt").on_line(Main, special_after_infliction_install).install();
	Agent::new("ryu").on_line(Main, special_after_infliction_install).install();
	Agent::new("ken").on_line(Main, special_after_infliction_install).install();
	Agent::new("cloud").on_line(Main, special_after_infliction_install).install();
	Agent::new("kamui").on_line(Main, special_after_infliction_install).install();
	Agent::new("bayonetta").on_line(Main, special_after_infliction_install).install();
	Agent::new("inkling").on_line(Main, special_after_infliction_install).install();
	Agent::new("ridley").on_line(Main, special_after_infliction_install).install();
	Agent::new("simon").on_line(Main, special_after_infliction_install).install();
	Agent::new("richter").on_line(Main, special_after_infliction_install).install();
	Agent::new("krool").on_line(Main, special_after_infliction_install).install();
	Agent::new("shizue").on_line(Main, special_after_infliction_install).install();
	Agent::new("gaogaen").on_line(Main, special_after_infliction_install).install();
	Agent::new("packun").on_line(Main, special_after_infliction_install).install();
	Agent::new("jack").on_line(Main, special_after_infliction_install).install();
	Agent::new("brave").on_line(Main, special_after_infliction_install).install();
	Agent::new("buddy").on_line(Main, special_after_infliction_install).install();
	Agent::new("dolly").on_line(Main, special_after_infliction_install).install();
	Agent::new("master").on_line(Main, special_after_infliction_install).install();
	Agent::new("tantan").on_line(Main, special_after_infliction_install).install();
	Agent::new("pickel").on_line(Main, special_after_infliction_install).install();
	Agent::new("edge").on_line(Main, special_after_infliction_install).install();
	Agent::new("elight").on_line(Main, special_after_infliction_install).install();
	Agent::new("eflame").on_line(Main, special_after_infliction_install).install();
	Agent::new("demon").on_line(Main, special_after_infliction_install).install();
	Agent::new("trail").on_line(Main, special_after_infliction_install).install();
	Agent::new("koopag").on_line(Main, special_after_infliction_install).install();
}