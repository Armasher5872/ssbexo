use super::*;

//Related to Param Edits
fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

//Related to Param Edits
#[skyline::hook(offset=0x3f0028, inline)]
pub unsafe fn offset_dump(ctx: &InlineCtx) {
	let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
	println!("Function Offset: {:#X}", ctx.registers[8].x.as_ref() - text);
}

//Param Adjustments (mainly used in things like Bowsers Fireballs and Ness's PSIOU PK Fire)
#[skyline::hook(offset = INT_OFFSET)]
pub unsafe fn get_param_int_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
	let mut boma = *((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
	let boma_reference = &mut *boma;
	let fighter_kind = boma_reference.kind();
	let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if boma_reference.is_fighter() {
		if fighter_kind == *FIGHTER_KIND_PICHU {
			if param_type == hash40("param_special_hi") {
				if param_hash == hash40("special_hi_warp2_angle_") {
					if DISCHARGE_ACTIVE[entry_id] == true {
						return 35;
					}
					else {
						return 360;
					}
				}
			}
		}
	}
	else if boma_reference.is_weapon() {
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		let entry_id = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if fighter_kind == *WEAPON_KIND_NESS_PK_FIRE {
			if param_type == hash40("param_pkfire") {
				if param_hash == hash40("life") {
					if OFFENSE_UP_ACTIVE[entry_id] == true {
						return 60;
					}
					else {
						return 20;
					}
				}
				if param_hash == hash40("pillar_life") {
					if OFFENSE_UP_ACTIVE[entry_id] == true {
						return 0;
					}
					else {
						return 100;
					}
				}
			}
		}
		if fighter_kind == *WEAPON_KIND_MEWTWO_SHADOWBALL {
			if param_type == hash40("param_shadowball") {
				if param_hash == hash40("life") {
					if STORED_POWER_ENABLED[entry_id] == 1 {
						return 120;
					}
					else {
						return 80;
					}
				}
			}
		}
	}
	original!()(module_accessor, param_type, param_hash)
}

#[skyline::hook(offset=FLOAT_OFFSET)]
pub unsafe fn get_param_float_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
	let mut boma = *((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
	let boma_reference = &mut *boma;
	let fighter_kind = boma_reference.kind();
	let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	let sticky = ControlModule::get_stick_y(boma);
	if boma_reference.is_fighter() {
		if fighter_kind == *FIGHTER_KIND_PICHU {
			if param_type == hash40("param_special_hi") {
				if param_hash == hash40("special_hi_warp_spd_add") {
					if DISCHARGE_ACTIVE[entry_id] == true {
						return 6.5;
					}
					else {
						return 9.0;
					}
				}
			}
		}
		if fighter_kind == *FIGHTER_KIND_GANON {
			if param_type == hash40("air_accel_y") {
				if WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) == true {
					return 0.33;
				}
				else {
					return 0.11;
				}
			}
		}
		if fighter_kind == *FIGHTER_KIND_MEWTWO {
			if param_type == hash40("ground_brake") {
				if STORED_POWER_ENABLED[entry_id] == 1 {
					return 0.04;
				}
				else {
					return 0.0754;
				}
			}
			if param_type == hash40("dash_speed") {
				if STORED_POWER_ENABLED[entry_id] == 1 {
					return 1.4;
				}
				else {
					return 1.65;
				}
			}
			if param_type == hash40("run_speed_max") {
				if STORED_POWER_ENABLED[entry_id] == 1 {
					return 1.6;
				}
				else {
					return 1.95;
				}
			}
			if param_type == hash40("jump_initial_y") {
				if STORED_POWER_ENABLED[entry_id] == 1 {
					return 20.0;
				}
				else {
					return 14.0;
				}
			}
			if param_type == hash40("air_accel_y") {
				if STORED_POWER_ENABLED[entry_id] == 1 {
					return 0.2;
				}
				else {
					return 0.08;
				}
			}
			if param_type == hash40("air_speed_y_stable") {
				if STORED_POWER_ENABLED[entry_id] == 1 {
					return 2.0;
				}
				else {
					return 1.5;
				}
			}
			if param_type == hash40("dive_speed_y") {
				if STORED_POWER_ENABLED[entry_id] == 1 {
					return 3.0;
				}
				else {
					return 2.4;
				}
			}
			if param_type == hash40("weight") {
				if STORED_POWER_ENABLED[entry_id] == 1 {
					return 115.0;
				}
				else {
					return 79.0;
				}
			}
		}
		if fighter_kind == *FIGHTER_KIND_PZENIGAME {
			if param_type == hash40("ground_brake") {
				if IN_RAIN_DANCE[entry_id] == true {
					return 0.0143;
				}
				else {
					return 0.0572;
				}
			}
		}
		if fighter_kind == *FIGHTER_KIND_PFUSHIGISOU {
			if param_type == hash40("ground_brake") {
				if IN_RAIN_DANCE[entry_id] == true {
					return 0.0197;
				}
				else {
					return 0.07865;
				}
			}
		}
		if fighter_kind == *FIGHTER_KIND_PLIZARDON {
			if param_type == hash40("ground_brake") {
				if IN_RAIN_DANCE[entry_id] == true {
					return 0.0179;
				}
				else {
					return 0.0715;
				}
			}
		}
	}
	else if boma_reference.is_weapon() {
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		let entry_id = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if fighter_kind == *WEAPON_KIND_KOOPA_BREATH {
			if param_type == hash40("param_special_n") {
				if param_hash == hash40("fire_speed_mul_max") {
					if CAN_FIREBALL[entry_id] == true {
						return 1.5;
					}
					else {
						return 1.2;
					}
				}
				if param_hash == hash40("fire_speed_mul_min") {
					if CAN_FIREBALL[entry_id] == true {
						return 1.5;
					}
					else {
						return 0.15;
					}
				}
				if param_hash == hash40("fire_speed_min") {
					if CAN_FIREBALL[entry_id] == true {
						return 0.57;
					}
					else {
						return 0.2;
					}
				}
			}
			if param_type == hash40("param_breath") {
				if param_hash == hash40("life") {
					if CAN_FIREBALL[entry_id] == true {
						return 70.0;
					}
					else {
						return 12.0;
					}
				}
				if param_hash == hash40("hit_frames") {
					if CAN_FIREBALL[entry_id] == true {
						return 70.0;
					}
					else {
						return 12.0;
					}
				}
				if param_hash == hash40("min_speed") {
					if CAN_FIREBALL[entry_id] == true {
						return 0.8;
					}
					else {
						return 0.5;
					}
				}
				if param_hash == hash40("max_speed") {
					if CAN_FIREBALL[entry_id] == true {
						return 0.8;
					}
					else {
						return 3.0;
					}
				}
			}
		}
		if fighter_kind == *WEAPON_KIND_PICHU_DENGEKI {
			if param_type == hash40("param_degeki") {
				if param_hash == hash40("move_life_") {
					if DISCHARGE_ACTIVE[entry_id] == true {
						return 180.0;
					}
					else {
						return 0.0;
					}
				}
			}
		}
		if fighter_kind == *WEAPON_KIND_PICHU_DENGEKIDAMA {
			if param_type == hash40("param_degekidama") {
				if param_hash == hash40("life_") {
					if DISCHARGE_ACTIVE[entry_id] == true {
						return 180.0;
					}
					else {
						return 0.0;
					}
				}
			}
		}
		if fighter_kind == *WEAPON_KIND_PICHU_KAMINARI {
			if param_type == hash40("param_kaminari") {
				if param_hash == hash40("speed_") {
					if DISCHARGE_ACTIVE[entry_id] == true {
						return 0.0;
					}
					else {
						return -4.9;
					}
				}
				if param_hash == hash40("flying_dist_") {
					if DISCHARGE_ACTIVE[entry_id] == true {
						return 0.0;
					}
					else {
						return 30.0;
					}
				}
				if param_hash == hash40("pass_fall_dist_") {
					if DISCHARGE_ACTIVE[entry_id] == true {
						return 0.0;
					}
					else {
						return 7.5;
					}
				}
				if param_hash == hash40("width_") {
					if DISCHARGE_ACTIVE[entry_id] == true {
						return 0.0;
					}
					else {
						return 1.7;
					}
				}
			}
		}
		if fighter_kind == *WEAPON_KIND_PICHU_CLOUD {
			if param_type == hash40("param_cloud") {
				if param_hash == hash40("speed_") {
					if DISCHARGE_ACTIVE[entry_id] == true {
						return 0.0;
					}
					else {
						return -4.9;
					}
				}
				if param_hash == hash40("width_") {
					if DISCHARGE_ACTIVE[entry_id] == true {
						return 0.0;
					}
					else {
						return 1.7;
					}
				}
			}
		}
		if fighter_kind == *WEAPON_KIND_MEWTWO_SHADOWBALL {
			if param_type == hash40("param_shadowball") {
				if param_hash == hash40("angle") {
					if STORED_POWER_ENABLED[entry_id] == 1 {
						if sticky > 0.5 {
							return 45.0;
						}
						else if sticky < -0.5 {
							return -45.0;
						}
					}
					else {
						return 0.0;
					}
				}
			}
		}
		if fighter_kind == *WEAPON_KIND_SNAKE_TRENCHMORTAR_BULLET {
            if param_type == hash40("param_trenchmortarbullet") {
				if param_hash == hash40("speed_x") {
					return ControlModule::get_stick_x(boma) / 1.5 * PostureModule::lr(boma);
				}
            }
        }
    }
	original!()(module_accessor, param_type, param_hash)
}

pub fn install() {
    unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            INT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
    }
	skyline::install_hook!(get_param_int_replace);
	skyline::install_hook!(get_param_float_replace);
    skyline::install_hook!(offset_dump);
}