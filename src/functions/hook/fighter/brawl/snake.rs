use super::*;

const SNAKE_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x11b5710; //Snake only
const SNAKE_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x11b5720; //Snake only
const SNAKE_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x11b5890; //Snake only
const SNAKE_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x11b59f0; //Snake only
pub static mut SNAKE_GRENADE_STATUS_FALL_STATUS: usize = 0x7c9ae0;
pub static mut SNAKE_GRENADE_STATUS_LANDING_STATUS: usize = 0x7c9d10;
pub static mut SNAKE_GRENADE_STATUS_THROWN_STATUS: usize = 0x7c9fc0;

//Snake Startup Initialization
#[skyline::hook(offset = SNAKE_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn snake_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    WorkModule::set_int(boma, 0, *FIGHTER_SNAKE_INSTANCE_WORK_ID_INT_ATTACK_S4_COUNT);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Snake Reset Initialization
#[skyline::hook(offset = SNAKE_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn snake_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    WorkModule::set_int(boma, 0, *FIGHTER_SNAKE_INSTANCE_WORK_ID_INT_ATTACK_S4_COUNT);
    original!()(vtable, fighter)
}

//Snake Death Initialization
#[skyline::hook(offset = SNAKE_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn snake_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_initialization_variable_reset(&mut *boma);
    WorkModule::set_int(boma, 0, *FIGHTER_SNAKE_INSTANCE_WORK_ID_INT_ATTACK_S4_COUNT);
    original!()(vtable, fighter)
}

//Snake Once Per Fighter Frame
#[skyline::hook(offset = SNAKE_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn snake_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let counter = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
    let handle = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
    //Final Zoom Effect Clearing
    if counter > 0 {
        if counter == 20 {
            if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK) {
                EffectModule::remove_screen(boma, Hash40::new("bg_finishhit"), -1);
                set_stage_visibility(boma, 1);
                set_vis_hud(true);
            }
            else {
                EffectModule::remove_screen(boma, Hash40::new("bg_snake_final"), -1);
                EffectModule::set_rate(boma, handle as u32, 1.0);
            }
            macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_bg_black"), false, false);
            macros::CAM_ZOOM_OUT(agent);
        }
        if counter == 10 {
            SlowModule::clear_whole(boma);
        }
        WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
    }
    else {
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
    }
    original!()(vtable, fighter)
}

//Credited to WuBoyTH
#[skyline::hook(replace = SNAKE_GRENADE_STATUS_FALL_STATUS)]
unsafe extern "C" fn snake_grenade_status_fall_status(item: &mut L2CAgent) -> L2CValue {
    TeamModule::set_hit_team(item.module_accessor, -1);
    HitModule::set_hit_stop_mul(item.module_accessor, 0.25, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_ALL as u8}, 0.0);
    original!()(item)
}

#[skyline::hook(replace = SNAKE_GRENADE_STATUS_LANDING_STATUS)]
unsafe extern "C" fn snake_grenade_status_landing_status(item: &mut L2CAgent) -> L2CValue {
    TeamModule::set_hit_team(item.module_accessor, -1);
    HitModule::set_hit_stop_mul(item.module_accessor, 0.25, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_ALL as u8}, 0.0);
    original!()(item)
}

#[skyline::hook(replace = SNAKE_GRENADE_STATUS_THROWN_STATUS)]
unsafe extern "C" fn snake_grenade_status_thrown_status(item: &mut L2CAgent) -> L2CValue {
    TeamModule::set_hit_team(item.module_accessor, -1);
    HitModule::set_hit_stop_mul(item.module_accessor, 0.25, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_ALL as u8}, 0.0);
    original!()(item)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "item" {
        unsafe {
            let item_offset = (*info.module.ModuleObject).module_base as usize;
            let _ = skyline::patching::patch_pointer((0x7ca48c+item_offset) as *const u8, &0x529FE608u32);
            SNAKE_GRENADE_STATUS_FALL_STATUS += item_offset;
            SNAKE_GRENADE_STATUS_LANDING_STATUS += item_offset;
            SNAKE_GRENADE_STATUS_THROWN_STATUS += item_offset;
            skyline::install_hooks!(
                snake_grenade_status_fall_status,
                snake_grenade_status_landing_status,
                snake_grenade_status_thrown_status
            );
        }
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
    skyline::install_hooks!(
        snake_start_initialization,
        snake_reset_initialization,
        snake_death_initialization,
        snake_opff
    );
}