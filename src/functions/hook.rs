#![allow(
	unused_macros,
	unused_mut,
    unused_parens
)]
use {
	crate::functions::{
		ext::*,
		variables::*,
		util::*,
	},
	skyline::hooks::*,
    smash::{
        app::{
			lua_bind::*,
			utility::*,
		},
        lib::lua_const::*,
    },
	std::os::raw::c_int
};

//Prevention of Moves in Air/Wavedash Logic (Credit to Chrispo)
#[skyline::hook(replace = StatusModule::change_status_request_from_script)]
pub unsafe fn change_status_hook(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, unk: bool) -> u64 {
	let next_status = status_kind;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if get_kind(boma) == *FIGHTER_KIND_ALL {
		if [*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&next_status) {
			if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP_MINI) {
				original!()(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, false);
			} 
			else {
				original!()(boma, status_kind, unk);
			}
		}
		else if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&next_status) {
			if IS_WAVEDASH[entry_id] == true {
				StatusModule::set_situation_kind(boma, smash::app::SituationKind(*SITUATION_KIND_GROUND), true);
			}
			original!()(boma, status_kind, unk);
		}
		else {
			original!()(boma, status_kind, unk);
		}
	}
	if get_kind(boma) == *FIGHTER_KIND_PICHU
	&& [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_HOLD, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END].contains(&status_kind)
	&& !USE_TACKLE[entry_id as usize] {
		return 0;
	}
	if get_kind(boma) == *FIGHTER_KIND_LUCINA {
		if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N
		&& !USE_SWORDSMAN_DASH[entry_id as usize] {
			return 0;
		}
		if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
		&& !USE_UP_SPECIAL[entry_id as usize] {
			return 0;
		}
	}
	if get_kind(boma) == *FIGHTER_KIND_GANON
	&& status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
	&& !USE_DROPKICK[entry_id as usize] {
		return 0;
	}
	if get_kind(boma) == *FIGHTER_KIND_MIIFIGHTER
	&& status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
	&& !USE_ONSLAUGHT[entry_id as usize] {
		return 0;
	}
	return original!()(boma, status_kind, unk);
}

//On Flag Hook, mainly used to deal with Jabs
#[skyline::hook(replace = smash::app::lua_bind::WorkModule::on_flag)]
pub unsafe fn on_flag_hook(module_accessor: &mut smash::app::BattleObjectModuleAccessor, int: c_int) -> () {
	if smash::app::utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER { 
		let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
		let fighter_kind = smash::app::utility::get_kind(module_accessor);
		let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if int == *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100 {
			ATTACK_100_ON[entry_id] = true;
			if ![*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_DEMON_STATUS_KIND_ATTACK_COMBO].contains(&status_kind) || [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_DEMON].contains(&fighter_kind){
				original!()(module_accessor, int)
			};
		} 
		else if int == *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO {
			ATTACK_ENABLE_COMBO_ON[entry_id] = true;
			if status_kind != *FIGHTER_STATUS_KIND_ATTACK || [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_DEMON].contains(&fighter_kind) {
				original!()(module_accessor, int)
			};
		} 
		else if int == *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO {
			ATTACK_NO_HIT_COMBO_ON[entry_id] = true;
			if status_kind != *FIGHTER_STATUS_KIND_ATTACK || [*FIGHTER_KIND_MURABITO, *FIGHTER_KIND_DEMON].contains(&fighter_kind) {
				original!()(module_accessor, int)
			};
		}	
		else {
			original!()(module_accessor, int)
		}
	} 
	else {
		original!()(module_accessor, int)
	}
}

//Off Flag Hook, mainly used to prevent Jabs from overriding other options
#[skyline::hook(replace = smash::app::lua_bind::WorkModule::off_flag)]
pub unsafe fn off_flag_hook(module_accessor: &mut smash::app::BattleObjectModuleAccessor, int: c_int) -> () {
	let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if smash::app::utility::get_category(module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
		original!()(module_accessor, int)
	}
	if int == *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100 {
		ATTACK_100_ON[entry_id] = false;
		original!()(module_accessor, int)
	} 
	else if int == *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO {
		ATTACK_ENABLE_COMBO_ON[entry_id] = false;
		original!()(module_accessor, int)
	} 
	else {
		original!()(module_accessor, int)
	}
}

//Hit Module Handle Attack Event, determines where you hit and with what hitbox id
#[skyline::hook(offset = 0x46ae64, inline)]
unsafe fn hit_module_handle_attack_event(ctx: &InlineCtx) {
    let data = *ctx.registers[1].x.as_ref() as *mut u32;
    let attacker_id = *data;
    let collision_id = *data.add(1);
    let battle_object = &mut *get_battle_object_from_id(attacker_id);
    if !battle_object.is_fighter() && !battle_object.is_weapon() {
        return;
    }
    LAST_ATTACK_HITBOX_ID = collision_id as i32;
}

//Shield Module Send Shield Attack Collision Event, basically does the same thing as 0x46ae64, but on shield
#[skyline::hook(offset = 0x4c7060)]
unsafe fn shield_module_send_shield_attack_collision_event(shield_module: *mut u64, opp_attack_module: *mut u64, collision: *mut u8, group_index: i32, raw_power: f32, real_power: f32, pos_x: f32, lr: f32) {
    call_original!(shield_module, opp_attack_module, collision, group_index, raw_power, real_power, pos_x, lr);
    let attacker_id = *(collision.add(0x24) as *const u32);
	let battle_object = &mut *get_battle_object_from_id(attacker_id);
    if !battle_object.is_fighter() && !battle_object.is_weapon() {
        return;
    }
    let hitbox_id = *(collision.add(0x33) as *const u8);
    LAST_ATTACK_HITBOX_ID = hitbox_id as i32;
}

//Installation
pub fn install() {
	skyline::install_hook!(change_status_hook);
	skyline::install_hook!(on_flag_hook);
	skyline::install_hook!(off_flag_hook);
	skyline::install_hooks!(
        hit_module_handle_attack_event,
        shield_module_send_shield_attack_collision_event
    );
}