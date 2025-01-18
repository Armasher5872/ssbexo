use super::*;

#[skyline::hook(replace=FighterUtil::is_valid_auto_catch_item)]
pub unsafe fn is_valid_auto_catch_item_hook(module_accessor: &mut BattleObjectModuleAccessor, is_possible: bool) -> bool {
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    if fighter_kind == *FIGHTER_KIND_LINK {
        if WorkModule::is_flag(module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_PICK_ITEM) {
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

//Credited to HDR, retains your previously selected character when returning from a match exit to the CSS
#[skyline::hook(offset = 0x1798ac8, inline)]
unsafe fn fix_chara_replace(ctx: &skyline::hooks::InlineCtx) {
    let ptr1 = *ctx.registers[0].x.as_ref() as *mut u64;
    let ptr2 = *ctx.registers[1].x.as_ref() as *mut u64;
    *ptr2.add(0x2) = *ptr1.add(0x2);
    *ptr2.add(0x3) = *ptr1.add(0x3);
    *ptr2.add(0x4) = *ptr1.add(0x4);
}

pub fn install() {
    skyline::install_hooks!(
        is_valid_just_shield_replace,
        is_valid_auto_catch_item_hook,
        gravity_replace,
        fix_chara_replace
    );
}