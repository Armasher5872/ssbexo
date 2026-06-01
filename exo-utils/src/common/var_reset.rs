use super::*;

//Condenses the initial reseting of variables into one function
pub unsafe extern "C" fn common_initialization_variable_reset(boma: &mut BattleObjectModuleAccessor) {
    let jump_speed_x = WorkModule::get_param_float(boma, hash40("jump_speed_x"), 0);
    let jump_speed_x_max = WorkModule::get_param_float(boma, hash40("jump_speed_x_max"), 0);
    let dash_speed = WorkModule::get_param_float(boma, hash40("dash_speed"), 0);
    let run_speed_max = WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);
    let speed_max = if dash_speed > run_speed_max {dash_speed} else {run_speed_max};
    let ratio = (jump_speed_x_max/speed_max)*jump_speed_x;
    let flags = [
        *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_STOCK, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH, *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE
    ];
    let floats = [*FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_X, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_Y, *FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL];
    let ints = [
        *FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE, *FIGHTER_INSTANCE_WORK_ID_INT_GLIDE_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_EFFECT_ID, 
        *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX
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
        *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_STOCK, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH, *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE
    ];
    let ints = [
        *FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE, *FIGHTER_INSTANCE_WORK_ID_INT_GLIDE_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_EFFECT_ID, 
        *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX
    ];
    let floats = [*FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_X, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_Y, *FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL];
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
        *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING,
        *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY, *FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH, *FIGHTER_INSTANCE_WORK_ID_FLAG_PERFECT_WAVEDASH, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE, 
        *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE
    ];
    let ints = [
        *FIGHTER_INSTANCE_WORK_ID_INT_COMMAND_INPUT_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_GLIDE_TIMER, *FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_EFFECT_ID, *FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX
    ];
    let floats = [*FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_X, *FIGHTER_INSTANCE_WORK_ID_FLOAT_CLIFF_ROBBED_SPEED_Y, *FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL];
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
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_STOCK) {
        set_stage_visibility(boma, 1);
		set_vis_hud(true);
        SlowModule::clear_whole(boma);
        CameraModule::zoom_out(boma, 0);
        ControlModule::stop_rumble(boma, true);
        for quake_kind in *CAMERA_QUAKE_KIND_NONE..=*CAMERA_QUAKE_KIND_MAX {
            CameraModule::stop_quake(boma, quake_kind);
        }
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_STOCK);
    }
}