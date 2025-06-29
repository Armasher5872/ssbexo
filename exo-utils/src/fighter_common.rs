use super::*;

pub fn get_fighter_common_from_accessor<'a>(boma: &'a mut BattleObjectModuleAccessor) -> &'a mut L2CFighterCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CFighterCommon))
    }
}

//Condenses the initial reseting of variables into one function
pub unsafe extern "C" fn common_initialization_variable_reset(boma: &mut BattleObjectModuleAccessor) {
    let jump_speed_x = WorkModule::get_param_float(boma, hash40("jump_speed_x"), 0);
    let jump_speed_x_max = WorkModule::get_param_float(boma, hash40("jump_speed_x_max"), 0);
    let run_speed_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
    let ratio = (jump_speed_x_max/run_speed_max)*jump_speed_x;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let entry_id_i32 = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let fighter_info = smash::app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), FighterEntryID(entry_id_i32));
    let flags = [
        *FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK, *FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED, *FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE,
        *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED,
        *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT, *FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK, *FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS
    ];
    let floats = [*FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SMASH_ATTACK_CHARGE_FRAME];
    let ints = [
        *FIGHTER_INSTANCE_WORK_ID_INT_ATTACK_ANGLE, *FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER, *FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT, *FIGHTER_INSTANCE_WORK_ID_INT_MASHING, 
        *FIGHTER_INSTANCE_WORK_ID_INT_PARRIED, *FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_EFFECT_ID, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX
    ];
    for x in 0..flags.len() {
        WorkModule::off_flag(boma, flags[x]);
    };
    for y in 0..floats.len() {
        WorkModule::set_float(boma, 0.0, floats[y]);
    }
    for z in 0..ints.len() {
        WorkModule::set_int(boma, 0, ints[z]);
    }
    WorkModule::set_flag(boma, sv_information::is_ready_go(), *FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    WorkModule::set_float(boma, ratio, *FIGHTER_INSTANCE_WORK_ID_FLOAT_JUMP_SPEED_RATIO);
    WorkModule::set_int(boma, -1, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_ATTACKER_ID);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_DEFENDER_ID);
    BALL_VICTIMS[entry_id] = 0;
    LAST_ATTACK_HITBOX_ID = 0;
    LAST_ATTACK_HITBOX_LOCATION_X = 0.0;
    LAST_ATTACK_HITBOX_LOCATION_Y = 0.0;
    LAST_ATTACK_HITBOX_LOCATION_Z = 0.0;
    LAST_DAMAGE[entry_id] = 0.0;
    LAST_TO_HIT_BALL = 9;
    READY_GO_TIMER = 0;
    SIZE0[entry_id] = 0.0;
    SIZE1[entry_id] = 0.0;
    SPAWN_SIDE[entry_id] = false;
    STOCK_COUNT[entry_id] = smash::app::lua_bind::FighterInformation::stock_count(fighter_info);
    UiManager::set_ui_state(0, true);
}

//Condenses the reset event reseting of variables into one function
pub unsafe extern "C" fn common_reset_variable_reset(boma: &mut BattleObjectModuleAccessor) {
    let jump_speed_x = WorkModule::get_param_float(boma, hash40("jump_speed_x"), 0);
    let jump_speed_x_max = WorkModule::get_param_float(boma, hash40("jump_speed_x_max"), 0);
    let run_speed_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
    let ratio = (jump_speed_x_max/run_speed_max)*jump_speed_x;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let entry_id_i32 = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let fighter_info = smash::app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), FighterEntryID(entry_id_i32));
    let flags = [
        *FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK, *FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED, *FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE,
        *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED,
        *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT, *FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK, *FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS
    ];
    let floats = [*FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SMASH_ATTACK_CHARGE_FRAME];
    let ints = [
        *FIGHTER_INSTANCE_WORK_ID_INT_ATTACK_ANGLE, *FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER, *FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT, *FIGHTER_INSTANCE_WORK_ID_INT_MASHING, 
        *FIGHTER_INSTANCE_WORK_ID_INT_PARRIED, *FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_EFFECT_ID, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX
    ];
    for x in 0..flags.len() {
        WorkModule::off_flag(boma, flags[x]);
    };
    for y in 0..floats.len() {
        WorkModule::set_float(boma, 0.0, floats[y]);
    }
    for z in 0..ints.len() {
        WorkModule::set_int(boma, 0, ints[z]);
    }
    WorkModule::set_flag(boma, sv_information::is_ready_go(), *FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    WorkModule::set_float(boma, ratio, *FIGHTER_INSTANCE_WORK_ID_FLOAT_JUMP_SPEED_RATIO);
    WorkModule::set_int(boma, -1, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_ATTACKER_ID);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_DEFENDER_ID);
    BALL_VICTIMS[entry_id] = 0;
    LAST_ATTACK_HITBOX_ID = 0;
    LAST_ATTACK_HITBOX_LOCATION_X = 0.0;
    LAST_ATTACK_HITBOX_LOCATION_Y = 0.0;
    LAST_ATTACK_HITBOX_LOCATION_Z = 0.0;
    LAST_DAMAGE[entry_id] = 0.0;
    LAST_TO_HIT_BALL = 9;
    READY_GO_TIMER = 0;
    SIZE0[entry_id] = 0.0;
    SIZE1[entry_id] = 0.0;
    SPAWN_SIDE[entry_id] = false;
    STOCK_COUNT[entry_id] = smash::app::lua_bind::FighterInformation::stock_count(fighter_info);
    UiManager::set_ui_state(0, true);
}

//Condenses the reseting of variables on death into one function
pub unsafe extern "C" fn common_death_variable_reset(boma: &mut BattleObjectModuleAccessor) {
    let jump_speed_x = WorkModule::get_param_float(boma, hash40("jump_speed_x"), 0);
    let jump_speed_x_max = WorkModule::get_param_float(boma, hash40("jump_speed_x_max"), 0);
    let run_speed_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
    let ratio = (jump_speed_x_max/run_speed_max)*jump_speed_x;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let flags = [
        *FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED, *FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT, *FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT, *FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH, *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS
    ];
    let floats = [*FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL, *FIGHTER_INSTANCE_WORK_ID_FLOAT_SMASH_ATTACK_CHARGE_FRAME];
    let ints = [
        *FIGHTER_INSTANCE_WORK_ID_INT_ATTACK_ANGLE, *FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER, *FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT, *FIGHTER_INSTANCE_WORK_ID_INT_MASHING, 
        *FIGHTER_INSTANCE_WORK_ID_INT_PARRIED, *FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_EFFECT_ID, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX
    ];
    for x in 0..flags.len() {
        WorkModule::off_flag(boma, flags[x]);
    };
    for y in 0..floats.len() {
        WorkModule::set_float(boma, 0.0, floats[y]);
    }
    for z in 0..ints.len() {
        WorkModule::set_int(boma, 0, ints[z]);
    }
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    WorkModule::set_float(boma, ratio, *FIGHTER_INSTANCE_WORK_ID_FLOAT_JUMP_SPEED_RATIO);
    WorkModule::set_int(boma, -1, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_ATTACKER_ID);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_DEFENDER_ID);
    LAST_ATTACK_HITBOX_ID = 0;
    LAST_ATTACK_HITBOX_LOCATION_X = 0.0;
    LAST_ATTACK_HITBOX_LOCATION_Y = 0.0;
    LAST_ATTACK_HITBOX_LOCATION_Z = 0.0;
    LAST_DAMAGE[entry_id] = 0.0;
    READY_GO_TIMER = 0;
    SIZE0[entry_id] = 0.0;
    SIZE1[entry_id] = 0.0;
    SPAWN_SIDE[entry_id] = false;
    UiManager::set_ui_state(0, true);
}

//Checks what alt you are
pub unsafe fn get_player_number(module_accessor:  &mut smash::app::BattleObjectModuleAccessor) -> usize {
    let player_number;
    if smash::app::utility::get_kind(module_accessor) == *WEAPON_KIND_PTRAINER_PTRAINER {
        player_number = WorkModule::get_int(module_accessor, *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_INT_FIGHTER_ENTRY_ID) as usize;
    }
    else if get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        player_number = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }
    else {
        let mut owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        while get_category(owner_module_accessor) != *BATTLE_OBJECT_CATEGORY_FIGHTER { //Keep checking the owner of the boma we're working with until we've hit a boma that belongs to a fighter
            owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        }
        player_number = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    }
    return player_number;
}

//Gets attacker number
pub unsafe fn get_attacker_number(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> usize {
	let attacker_number = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ATTACKER_COLOR) as usize;
	return attacker_number;
}

//Gets the boma
pub unsafe fn get_boma(entry_id: i32) -> *mut smash::app::BattleObjectModuleAccessor {
	let boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(entry_id));
	return boma;
}

//Gets Article Boma
pub unsafe fn get_article_boma(boma: *mut BattleObjectModuleAccessor, article_type: skyline::libc::c_int) -> *mut BattleObjectModuleAccessor {
    let article = ArticleModule::get_article(boma, article_type);
    let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
    return sv_battle_object::module_accessor(object_id);
}

//Used for GGST COUNTER!
pub unsafe fn estimate_frame(module_accessor: &mut smash::app::BattleObjectModuleAccessor, frame: f32) -> bool {
    let motion_frame = MotionModule::frame(module_accessor);
	if motion_frame >= frame && motion_frame < frame + 1.0 {
		return true;
	}
	else {
		return false;
	}
}

//Gets the article owner boma
pub unsafe fn get_owner_boma(weapon: &mut L2CAgentBase) -> *mut BattleObjectModuleAccessor {
    return &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
}