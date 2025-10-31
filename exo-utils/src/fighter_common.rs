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
    let dash_speed = WorkModule::get_param_float(boma, hash40("dash_speed"), 0);
    let run_speed_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
    let speed_max = if dash_speed > run_speed_max {dash_speed} else {run_speed_max};
    let ratio = (jump_speed_x_max/speed_max)*jump_speed_x;
    let flags = [
        *FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED, *FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH, *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE
    ];
    let ints = [
        *FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER, *FIGHTER_INSTANCE_WORK_ID_INT_GLIDE_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_EFFECT_ID, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX
    ];
    for x in 0..flags.len() {
        WorkModule::off_flag(boma, flags[x]);
    };
    for z in 0..ints.len() {
        WorkModule::set_int(boma, 0, ints[z]);
    }
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    WorkModule::set_float(boma, ratio, *FIGHTER_INSTANCE_WORK_ID_FLOAT_JUMP_SPEED_RATIO);
    WorkModule::set_float(boma, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
    WorkModule::set_int(boma, -1, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_ATTACKER_ID);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_DEFENDER_ID);
}

//Condenses the reset event reseting of variables into one function
pub unsafe extern "C" fn common_reset_variable_reset(boma: &mut BattleObjectModuleAccessor) {
    let jump_speed_x = WorkModule::get_param_float(boma, hash40("jump_speed_x"), 0);
    let jump_speed_x_max = WorkModule::get_param_float(boma, hash40("jump_speed_x_max"), 0);
    let dash_speed = WorkModule::get_param_float(boma, hash40("dash_speed"), 0);
    let run_speed_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
    let speed_max = if dash_speed > run_speed_max {dash_speed} else {run_speed_max};
    let ratio = (jump_speed_x_max/speed_max)*jump_speed_x;
    let flags = [
        *FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED, *FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH, *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE
    ];
    let ints = [
        *FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER, *FIGHTER_INSTANCE_WORK_ID_INT_GLIDE_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_EFFECT_ID, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX
    ];
    for x in 0..flags.len() {
        WorkModule::off_flag(boma, flags[x]);
    };
    for z in 0..ints.len() {
        WorkModule::set_int(boma, 0, ints[z]);
    }
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    WorkModule::set_float(boma, ratio, *FIGHTER_INSTANCE_WORK_ID_FLOAT_JUMP_SPEED_RATIO);
    WorkModule::set_float(boma, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
    WorkModule::set_int(boma, -1, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_ATTACKER_ID);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_DEFENDER_ID);
}

//Condenses the reseting of variables on death into one function
pub unsafe extern "C" fn common_death_variable_reset(boma: &mut BattleObjectModuleAccessor) {
    let jump_speed_x = WorkModule::get_param_float(boma, hash40("jump_speed_x"), 0);
    let jump_speed_x_max = WorkModule::get_param_float(boma, hash40("jump_speed_x_max"), 0);
    let dash_speed = WorkModule::get_param_float(boma, hash40("dash_speed"), 0);
    let run_speed_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
    let speed_max = if dash_speed > run_speed_max {dash_speed} else {run_speed_max};
    let ratio = (jump_speed_x_max/speed_max)*jump_speed_x;
    let flags = [
        *FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED, *FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE
    ];
    let ints = [
        *FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER, *FIGHTER_INSTANCE_WORK_ID_INT_GLIDE_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_EFFECT_ID, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX
    ];
    for x in 0..flags.len() {
        WorkModule::off_flag(boma, flags[x]);
    };
    for z in 0..ints.len() {
        WorkModule::set_int(boma, 0, ints[z]);
    }
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_GATLING);
    WorkModule::set_float(boma, ratio, *FIGHTER_INSTANCE_WORK_ID_FLOAT_JUMP_SPEED_RATIO);
    WorkModule::set_float(boma, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
    WorkModule::set_int(boma, -1, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_ID);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_ATTACKER_ID);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_DEFENDER_ID);
}

//Gets Article Boma
pub unsafe fn get_article_boma(boma: *mut BattleObjectModuleAccessor, article_type: skyline::libc::c_int) -> *mut BattleObjectModuleAccessor {
    let article = ArticleModule::get_article(boma, article_type);
    let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
    return sv_battle_object::module_accessor(object_id);
}

//Gets the article owner boma
pub unsafe fn get_owner_boma(weapon: &mut L2CAgentBase) -> *mut BattleObjectModuleAccessor {
    return &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
}