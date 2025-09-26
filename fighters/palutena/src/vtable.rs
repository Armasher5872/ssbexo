use super::*;

const PALUTENA_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xe5e6a0; //Palutena only
const PALUTENA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PALUTENA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xe5f350; //Palutena only
const PALUTENA_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xe5f3e0; //Palutena only

unsafe extern "C" fn palutena_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT);
    WorkModule::off_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT);
    WorkModule::off_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_JUMP_GLIDE_ACTIVE);
    WorkModule::off_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_PARAM_CHANGE);
    WorkModule::set_float(boma, 100.0, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_METER_TIMER);
    WorkModule::set_float(boma, 100.0, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_BURNOUT_METER_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_EFFECT_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_EFFECT_TIMER);
}

unsafe extern "C" fn palutena_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    }
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_REFLECTIONBOARD) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    }
    0.into()
}

//Palutena Startup Initialization
#[skyline::hook(offset = PALUTENA_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn palutena_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    common_initialization_variable_reset(&mut *boma);
    palutena_var(&mut *boma);
    UiManager::set_palutena_meter_info(entry_id, 100.0, 100.0, 50.0);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(palutena_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Palutena Reset Initialization
#[skyline::hook(offset = PALUTENA_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn palutena_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_PALUTENA as u32 {
        let boma = fighter.battle_object.module_accessor;
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
        common_reset_variable_reset(&mut *boma);
        palutena_var(&mut *boma);
        update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("walk_speed_max"), 0, 1.271));
        update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("dash_speed"), 0, 1.768));
        update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("run_accel_mul"), 0, 0.12991));
        update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("run_accel_add"), 0, 0.044));
        update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("run_speed_max"), 0, 2.077));
        update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("jump_speed_x"), 0, 0.67));
        update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("jump_speed_x_max"), 0, 2.077));
        update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("jump_y"), 0, 35.0));
        update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("mini_jump_y"), 0, 17.3));
        update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("jump_aerial_y"), 0, 35.9));
        update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("air_accel_x_mul"), 0, 0.105));
        update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("air_accel_x_add"), 0, 0.01));
        update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("air_speed_x_stable"), 0, 1.0));
        update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("air_speed_y_stable"), 0, 1.55));
        update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("damage_fly_top_speed_y_stable"), 0, 1.55));
        update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("dive_speed_y"), 0, 2.48));
        update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("weight"), 0, 91.0));
        UiManager::set_palutena_meter_info(entry_id, 100.0, 100.0, 50.0);
    }
    original!()(vtable, fighter)
}

//Palutena Death Initialization
#[skyline::hook(offset = PALUTENA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn palutena_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    common_death_variable_reset(&mut *boma);
    palutena_var(&mut *boma);
    UiManager::set_palutena_meter_info(entry_id, 100.0, 100.0, 50.0);
    original!()(vtable, fighter)
}

//Palutena Once Per Fighter Frame
#[skyline::hook(offset = PALUTENA_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn palutena_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    let lightweight_timer = WorkModule::get_int(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_TIMER);
    let lightweight_effect_timer = WorkModule::get_int(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_EFFECT_TIMER);
    let lightweight_burnout_timer = WorkModule::get_int(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_TIMER);
    let lightweight_burnout_effect_timer = WorkModule::get_int(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_EFFECT_TIMER);
    let lightweight_meter_timer = WorkModule::get_float(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_METER_TIMER);
    let lightweight_burnout_meter_timer = WorkModule::get_float(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_BURNOUT_METER_TIMER);
    if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) {
        WorkModule::set_int(boma, 1080, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_TIMER);
        //Timer
        if lightweight_timer > 0 {
            WorkModule::dec_int(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_TIMER);
            WorkModule::sub_float(boma, 0.138, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_METER_TIMER);
            if lightweight_timer <= 10 {
                EffectModule::kill_kind(boma, Hash40::new("sys_status_speed_up"), false, false);
            }
        }
        else {
            WorkModule::on_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT);
            WorkModule::on_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_PARAM_CHANGE);
        }
        //Effects
        WorkModule::inc_int(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_EFFECT_TIMER);
        if lightweight_effect_timer > 25 {
            WorkModule::set_int(boma, 0, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_EFFECT_TIMER);
        }
        if lightweight_effect_timer == 10 {
            EffectModule::req_follow(boma, Hash40::new("sys_status_speed_up"), Hash40::new("hip"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        }
        if lightweight_effect_timer >= 20 {
            EffectModule::kill_kind(boma, Hash40::new("sys_status_speed_up"), false, false);
            EffectModule::req_follow(boma, Hash40::new("sys_status_speed_up"), Hash40::new("hip"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        }
        //UI
        UiManager::change_palutena_meter_color_green(entry_id);
        UiManager::set_palutena_meter_info(entry_id, lightweight_meter_timer, 100.0, 50.0);
        //Param Changes
        if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_PARAM_CHANGE) && lightweight_timer > 0 {
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("walk_speed_max"), 0, 1.9065));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("dash_speed"), 0, 3.0525));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("run_accel_mul"), 0, 0.168883));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("run_accel_add"), 0, 0.0572));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("run_speed_max"), 0, 3.1155));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("jump_speed_x"), 0, 0.709));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("jump_speed_x_max"), 0, 3.1155));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("jump_y"), 0, 44.875));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("mini_jump_y"), 0, 21.625));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("jump_aerial_y"), 0, 44.875));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("air_accel_x_mul"), 0, 0.1365));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("air_accel_x_add"), 0, 0.013));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("air_speed_x_stable"), 0, 1.7));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("air_speed_y_stable"), 0, 1.24));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("damage_fly_top_speed_y_stable"), 0, 1.24));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("dive_speed_y"), 0, 3.72));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("weight"), 0, 50.0));
            WorkModule::off_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_PARAM_CHANGE);
        }
    }
    if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
        WorkModule::off_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT);
        WorkModule::set_float(boma, 100.0, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_METER_TIMER);
        //Timer
        if lightweight_burnout_timer > 0 {
            WorkModule::dec_int(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_TIMER);
            WorkModule::sub_float(boma, 0.09259, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_BURNOUT_METER_TIMER);
            if lightweight_burnout_timer <= 10 {
                EffectModule::kill_kind(boma, Hash40::new("sys_status_down"), false, false);
            }
        }
        else {
            WorkModule::set_float(boma, 100.0, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_BURNOUT_METER_TIMER);
            WorkModule::off_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT);
            WorkModule::on_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_PARAM_CHANGE);
        }
        //Effects
        WorkModule::inc_int(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_EFFECT_TIMER);
        if lightweight_burnout_effect_timer > 25 {
            WorkModule::set_int(boma, 0, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_EFFECT_TIMER);
        }
        if lightweight_burnout_effect_timer == 10 {
            EffectModule::req_follow(boma, Hash40::new("sys_status_down"), Hash40::new("hip"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        }
        if lightweight_burnout_effect_timer >= 20 {
            EffectModule::kill_kind(boma, Hash40::new("sys_status_down"), false, false);
            EffectModule::req_follow(boma, Hash40::new("sys_status_down"), Hash40::new("hip"), &Vector3f{x: 0.7, y: 0.0, z: 0.0}, &Vector3f::zero(), 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
        }
        //UI
        UiManager::change_palutena_meter_color_purple(entry_id);
        UiManager::set_palutena_meter_info(entry_id, lightweight_burnout_meter_timer, 100.0, 50.0);
        //Param Changes
        if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_PARAM_CHANGE) && lightweight_burnout_timer > 0 {
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("walk_speed_max"), 0, 0.847));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("dash_speed"), 0, 1.356));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("run_accel_mul"), 0, 0.099));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("run_accel_add"), 0, 0.033));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("run_speed_max"), 0, 1.384));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("jump_speed_x"), 0, 0.67));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("jump_speed_x_max"), 0, 2.077));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("jump_y"), 0, 28.0));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("mini_jump_y"), 0, 13.84));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("jump_aerial_y"), 0, 28.0));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("air_accel_x_mul"), 0, 0.0807));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("air_accel_x_add"), 0, 0.007));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("air_speed_x_stable"), 0, 0.588));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("air_speed_y_stable"), 0, 1.9375));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("damage_fly_top_speed_y_stable"), 0, 1.9375));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("dive_speed_y"), 0, 3.1));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("weight"), 0, 83.0));
            WorkModule::off_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_PARAM_CHANGE);
        }
    }
    if !WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) && !WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
        //Param Changes
        if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_PARAM_CHANGE) {
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("walk_speed_max"), 0, 1.271));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("dash_speed"), 0, 1.768));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("run_accel_mul"), 0, 0.12991));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("run_accel_add"), 0, 0.044));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("run_speed_max"), 0, 2.077));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("jump_speed_x"), 0, 0.67));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("jump_speed_x_max"), 0, 2.077));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("jump_y"), 0, 35.0));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("mini_jump_y"), 0, 17.3));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("jump_aerial_y"), 0, 35.9));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("air_accel_x_mul"), 0, 0.105));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("air_accel_x_add"), 0, 0.01));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("air_speed_x_stable"), 0, 1.0));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("air_speed_y_stable"), 0, 1.55));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("damage_fly_top_speed_y_stable"), 0, 1.55));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("dive_speed_y"), 0, 2.48));
            update_float_2(*FIGHTER_KIND_PALUTENA, vec![0, 1, 2, 3, 4, 5, 6, 7].clone(), (hash40("weight"), 0, 91.0));
            WorkModule::off_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_PARAM_CHANGE);
        }
        UiManager::change_palutena_meter_color_green(entry_id);
        UiManager::set_palutena_meter_info(entry_id, 100.0, 100.0, 50.0);
    }
    UiManager::set_palutena_meter_enable(entry_id, true);
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        palutena_start_initialization,
        palutena_reset_initialization,
        palutena_death_initialization,
        palutena_opff
    );
}