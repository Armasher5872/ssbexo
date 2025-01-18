use super::*;

const PALUTENA_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xe5e6a0; //Palutena only
const PALUTENA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PALUTENA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xe5f350; //Palutena only
const PALUTENA_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xe5f3e0; //Palutena only

unsafe extern "C" fn palutena_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT);
    WorkModule::off_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT);
    WorkModule::off_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_JUMP_GLIDE_ACTIVE);
    WorkModule::set_float(boma, 100.0, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_METER_TIMER);
    WorkModule::set_float(boma, 100.0, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_BURNOUT_METER_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_EFFECT_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_EFFECT_TIMER);
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
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
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
        WorkModule::set_int(boma, 720, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_TIMER);
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
        UiManager::change_palutena_meter_color_green(entry_id);
        UiManager::set_palutena_meter_info(entry_id, lightweight_meter_timer, 100.0, 50.0);
    }
    if WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
        WorkModule::off_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT);
        WorkModule::set_float(boma, 100.0, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_METER_TIMER);
        //Timer
        if lightweight_burnout_timer > 0 {
            WorkModule::dec_int(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_INT_LIGHTWEIGHT_BURNOUT_TIMER);
            WorkModule::sub_float(boma, 0.138, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_BURNOUT_METER_TIMER);
            if lightweight_burnout_timer <= 10 {
                EffectModule::kill_kind(boma, Hash40::new("sys_status_down"), false, false);
            }
        }
        else {
            WorkModule::set_float(boma, 100.0, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLOAT_LIGHTWEIGHT_BURNOUT_METER_TIMER);
            WorkModule::off_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT);
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
        UiManager::change_palutena_meter_color_purple(entry_id);
        UiManager::set_palutena_meter_info(entry_id, lightweight_burnout_meter_timer, 100.0, 50.0);
    }
    if !WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT) && !WorkModule::is_flag(boma, *FIGHTER_PALUTENA_INSTANCE_WORK_ID_FLAG_IS_LIGHTWEIGHT_BURNOUT) {
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