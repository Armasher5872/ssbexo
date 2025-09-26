use super::*;

//On Flag Hook (Credit to Chrispo)
#[skyline::hook(replace = smash::app::lua_bind::WorkModule::on_flag)]
unsafe extern "C" fn on_flag_hook(boma: &mut smash::app::BattleObjectModuleAccessor, int: c_int) -> () {
	let category = utility::get_category(boma);
	if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
		if int == *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI {
			if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FULL_HOP_ENABLE_DELAY) <= 0 {
				original!()(boma, int)
			}
			else {
				return;
			}
		}
		else {
			original!()(boma, int)
		}
	}
	else {
		original!()(boma, int)
	}
}

#[skyline::hook(replace=smash::app::lua_bind::WorkModule::get_int)]
unsafe extern "C" fn get_int_replace(module_accessor: &mut smash::app::BattleObjectModuleAccessor, int: i32) -> u64 {
	let fighter_kind = smash::app::utility::get_kind(module_accessor);
	if SPECIAL_SMASH_BODY == 3 
    && fighter_kind == *ITEM_KIND_SOCCERBALL {
		let pos = Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor)};
		if pos.x < camera_range().x + 10.0 
        || pos.x > camera_range().y - 10.0 
        || pos.y < camera_range().w + 10.0 {
			if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED) {
				BALL_BOUNCED = Vector3f{x: pos.x, y: 0.0, z: 0.0};
			}
			else {
				BALL_BOUNCED = Vector3f{x: pos.x, y: 1.0, z: 0.0};
			}
		}
		if GroundModule::get_touch_flag(module_accessor) == *GROUND_TOUCH_FLAG_DOWN as u64 {
			if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED)
            || (WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE) && ((SPAWN_SIDE[LAST_TO_HIT_BALL] && pos.x > 3.0) 
            || (!SPAWN_SIDE[LAST_TO_HIT_BALL] && pos.x < -3.0))) { //If either we already bounced, or we hit the ball but it was still on our side, KO
				BALL_BOUNCED = Vector3f{x: pos.x, y: 0.0, z: 0.0};
                WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
			}
			else { 
				WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
			}
            WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE);
		}
	}
	original!()(module_accessor, int)
}

//Installation
pub fn install() {
	skyline::install_hooks!(
		get_int_replace
	);
}