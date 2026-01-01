//Credited to WuBoyTH for the opff
use super::*;

const KOOPA_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x68d5a0; //Shared
const KOOPA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xbc1dd0; //Bowser only
const KOOPA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xbc1e00; //Bowser only
const KOOPA_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xbc2290; //Bowser only

unsafe extern "C" fn koopa_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || is_damaged(fighter.module_accessor) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KOOPA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USE_COUNT);
    }
    0.into()
}

unsafe extern "C" fn koopa_var(fighter: &mut L2CFighterCommon) {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_KOOPA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USE_COUNT);
}

//Bowser Startup Initialization
#[skyline::hook(offset = KOOPA_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn koopa_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_KOOPA as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        koopa_var(agent);
        agent.global_table[CHECK_SPECIAL_N_UNIQ].assign(&L2CValue::Ptr(should_use_special_n_callback as *const () as _));
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(koopa_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Bowser Reset Initialization
#[skyline::hook(offset = KOOPA_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn koopa_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_reset_variable_reset(&mut *boma);
    koopa_var(agent);
    original!()(vtable, fighter)
}

//Bowser Death Initialization
#[skyline::hook(offset = KOOPA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn koopa_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_death_variable_reset(&mut *boma);
    koopa_var(agent);
    original!()(vtable, fighter)
}

//Bowser OPFF
#[skyline::hook(offset = KOOPA_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn koopa_opff(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    let status_kind = StatusModule::status_kind(boma);
    if (*battle_object_slow.add(0x8) == 0 || *(battle_object_slow as *const u32) == 0) && !StopModule::is_stop(boma) {
        if (fighter.battle_object.kind == *FIGHTER_KIND_KIRBY as u32 && status_kind == *FIGHTER_KIRBY_STATUS_KIND_KOOPA_SPECIAL_N) || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            return;
        }
        let fire_speed_mul_max = WorkModule::get_param_float(boma, hash40("param_special_n"), hash40("fire_speed_mul_max"));
        let fire_speed_mul_min = WorkModule::get_param_float(boma, hash40("param_special_n"), hash40("fire_speed_mul_min"));
        let fire_scale_max = WorkModule::get_param_float(boma, hash40("param_special_n"), hash40("fire_scale_max"));
        let fire_scale_min = WorkModule::get_param_float(boma, hash40("param_special_n"), hash40("fire_scale_min"));
        let fire_speed_max_frame = WorkModule::get_param_float(boma, hash40("param_special_n"), hash40("fire_speed_max_frame"));
        let fire_scale_max_frame = WorkModule::get_param_float(boma, hash40("param_special_n"), hash40("fire_scale_max_frame"));
        let breath_speed_mul = WorkModule::get_float(boma, *FIGHTER_KOOPA_INSTANCE_WORK_ID_FLOAT_BREATH_SPEED_MUL);
        let breath_scale = WorkModule::get_float(boma, *FIGHTER_KOOPA_INSTANCE_WORK_ID_FLOAT_BREATH_SCALE);
        let speed_mul = breath_speed_mul+((fire_speed_mul_max-fire_speed_mul_min)/fire_speed_max_frame);
        let scale = breath_scale+((fire_scale_max-fire_scale_min)/fire_scale_max_frame);
        WorkModule::set_float(boma, speed_mul.clamp(fire_speed_mul_min, fire_speed_mul_max), *FIGHTER_KOOPA_INSTANCE_WORK_ID_FLOAT_BREATH_SPEED_MUL);
        WorkModule::set_float(boma, scale.clamp(fire_scale_min, fire_scale_max), *FIGHTER_KOOPA_INSTANCE_WORK_ID_FLOAT_BREATH_SCALE);
    }
}

pub fn install() {
    skyline::install_hooks!(
        koopa_start_initialization,
        koopa_reset_initialization,
        koopa_death_initialization,
        koopa_opff
    );
}