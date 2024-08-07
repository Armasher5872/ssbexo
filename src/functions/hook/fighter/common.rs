use super::*;

const COMMON_WEAPON_SHIELD_ATTACK_CALLBACK: usize = 0x346c8b0; //Used for when generic weapons hit something elses shield.

#[skyline::hook(replace=FighterUtil::is_valid_auto_catch_item)]
pub unsafe fn is_valid_auto_catch_item_hook(module_accessor: &mut BattleObjectModuleAccessor, is_possible: bool) -> bool {
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    if fighter_kind == *FIGHTER_KIND_LINK {
        if WorkModule::is_flag(module_accessor, FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM) {
            return true;
        }
        else {
            original!()(module_accessor, is_possible)
        }
    }
    else {
        original!()(module_accessor, is_possible)
    }
}

//Gravity, used in Custom Gamemodes
#[skyline::hook(replace=smash::app::lua_bind::FighterInformation::gravity)]
unsafe fn gravity_replace(fighter_information: &mut smash::app::FighterInformation) -> f32 {
	let ret = original!()(fighter_information);
	if ret == 1.33 {
		SPECIAL_SMASH_GRAVITY = 1;
	}
	else if ret == 0.66 {
		SPECIAL_SMASH_GRAVITY = 2;
	}
	else {
		SPECIAL_SMASH_GRAVITY = 0;
	}
	return 1.0;
}

//Special Smash
#[skyline::hook(replace=smash::app::FighterUtil::is_valid_just_shield)]
unsafe fn is_valid_just_shield_replace(boma: &mut BattleObjectModuleAccessor) -> bool {
	if SPECIAL_SMASH_STATUS == 2 {
		return false;
	}
	else {
		original!()(boma)
	}
}

#[skyline::hook(offset = COMMON_WEAPON_SHIELD_ATTACK_CALLBACK)]
unsafe extern "C" fn common_weapon_shield_attack_callback(vtable: u64, weapon: *mut smash::app::Weapon, arg: u32) {
    let boma = (*weapon).battle_object.module_accessor;
    let owner_object_id = WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
    let owner_boma = smash::app::sv_battle_object::module_accessor(owner_object_id as u32);
    let status_kind = StatusModule::status_kind(boma);
    if (*weapon).battle_object.kind == *WEAPON_KIND_NESS_PK_FIRE as u32
    && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD)
    && status_kind != *WEAPON_NESS_PK_FIRE_STATUS_KIND_PILLAR {
        if WorkModule::is_flag(owner_boma, FIGHTER_NESS_INSTANCE_WORK_ID_FLAG_OFFENSE_UP) {
            *(weapon as *mut bool).add(0x90) = true;
            (*boma).change_status_req(WEAPON_NESS_PK_FIRE_STATUS_KIND_PILLAR.into(), false.into());
        }
        else {
            *(weapon as *mut bool).add(0x90) = false;
        }
    }
    call_original!(vtable, weapon, arg)
}

pub fn install() {
    skyline::install_hook!(is_valid_just_shield_replace);
    skyline::install_hooks!(
        is_valid_auto_catch_item_hook,
        gravity_replace,
        common_weapon_shield_attack_callback
    );
}