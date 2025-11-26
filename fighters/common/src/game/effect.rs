use super::*;

//Credit to Lily Lambda
#[skyline::hook(replace = sv_animcmd::AFTER_IMAGE4_ON_arg29)]
unsafe extern "C" fn after_image4_on_arg29_replace(lua_state: u64) {
	let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
	let fighter_kind = smash::app::utility::get_kind(boma);
	if fighter_kind == FIGHTER_KIND_SHULK 
    && WorkModule::is_flag(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE) {
		let mut l2c_agent = L2CAgent::new(lua_state);
		let hitbox_params: Vec<L2CValue> = (0..29).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
		l2c_agent.clear_lua_stack();
		let mut new_sword_hash: u64 = hitbox_params[0].get_int();
		let monado_type = WorkModule::get_int(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE);
		for current_tex in ["1", "3", "5", "pink1", "red1"] {
			if hitbox_params[0].get_int() == L2CValue::new_int(hash40(format!("tex_shulk_sword{}", current_tex).as_str())).get_int() { 
				for (i, art) in (0i32..).zip(["jump", "speed", "shield", "buster", "smash"]) {
					if monado_type == i {
						new_sword_hash = hash40(format!("tex_shulk_sword{}_{}", current_tex, art).as_str());
					}
				}
			}
		}
		l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_sword_hash)); 
		for i in 1..29 {
			l2c_agent.push_lua_stack(&mut hitbox_params[i].clone());
		}
    }
    original!()(lua_state);
}

#[skyline::hook(replace = sv_animcmd::EFFECT_FOLLOW)]
unsafe extern "C" fn effect_follow_replace(lua_state: u64) {
	let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
	let fighter_kind = smash::app::utility::get_kind(boma);
	if fighter_kind == FIGHTER_KIND_SHULK 
    && WorkModule::is_flag(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE) {
		let mut l2c_agent = L2CAgent::new(lua_state);
		let mut hitbox_params: Vec<L2CValue> = (0..10).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
		if [
            hash40("shulk_airslash"), hash40("shulk_backslash_trace"), hash40("shulk_counter_success"), hash40("shulk_counter"), hash40("shulk_monad_circle_red"), hash40("shulk_monad_circle"), hash40("shulk_monad_sword"), hash40("shulk_monad_sword2_arc_2"),
            hash40("shulk_monad_sword2_arc"), hash40("shulk_monad_sword2_end"), hash40("shulk_monad_sword2_lightning"), hash40("shulk_monad_sword2"), hash40("shulk_monad_sword3_2"), hash40("shulk_monad_sword3_3"), hash40("shulk_monad_sword3_4"),
            hash40("shulk_monad_sword3_end"), hash40("shulk_monad_sword3_pink_end"), hash40("shulk_monad_sword3_pink"), hash40("shulk_monad_sword3_red_end"), hash40("shulk_monad_sword3_red"), hash40("shulk_monad_sword3"), hash40("shulk_vision_attack")
        ].contains(&hitbox_params[0].get_int()) {
			let monado_type = WorkModule::get_int(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE);
			let (mut r, mut g, mut b) = (0.0, 0.0, 1.0);
            match monado_type {
                0 => {
                    r = 0.1; g = 1.0; b = 0.1; // Jump
                }
                1 => {
                    r = 0.0; g = 0.62; b = 1.0; // Speed
                }
                2 => {
                    r = 1.0; g = 0.9; b = 0.0; // Shield
                }
                3 => {
                    r = 0.4; g = 0.0; b = 1.0; // Buster
                }
                4 => {
                    r = 1.0; g = 0.0; b = 0.0; // Smash
                }
                _ => {}
            }
            l2c_agent.clear_lua_stack();
			for i in 0..10 {
				l2c_agent.push_lua_stack(&mut hitbox_params[i]); 
			}
			l2c_agent.push_lua_stack(&mut L2CValue::new_num(r)); 
			l2c_agent.push_lua_stack(&mut L2CValue::new_num(g)); 
			l2c_agent.push_lua_stack(&mut L2CValue::new_num(b)); 
			sv_animcmd::EFFECT_FOLLOW_COLOR(lua_state);	
		} 
        else {
		    original!()(lua_state);
	    }
    } 
    else {
		original!()(lua_state);
	}
}

//EffectModule::req_follow
#[skyline::hook(offset = 0x44f880)]
unsafe extern "C" fn effectmodule_req_follow(effect_module: u64, effect_hash: Hash40, bone_hash: Hash40, pos: *mut Vector3f, rot: *mut Vector3f, size: f32, arg7: bool, arg8: u32, arg9: i32, arg10: i32, arg11: i32, arg12: i32, arg13: bool, arg14: bool) -> u64 {
    let boma = *(effect_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let mut effect_size = size;
	let mut new_effect_hash = effect_hash;
    if effect_hash.hash == hash40("sys_falling_smoke") {
        effect_size = size*1.5;
		new_effect_hash = Hash40::new("sys_steam2");
    }
    let ret = original!()(effect_module, new_effect_hash, bone_hash, pos, rot, effect_size, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14);
    if effect_hash.hash == hash40("sys_falling_smoke") {
        EffectModule::set_alpha(boma, ret as u32, 1.0);
    }
    ret
}

pub fn install() {
	skyline::install_hooks!(
        after_image4_on_arg29_replace,
		effect_follow_replace,
		effectmodule_req_follow
    );
}