use super::*;

const PFUSHIGISOU_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xea03d0; //Ivysaur only
const PFUSHIGISOU_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PFUSHIGISOU_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xea03e0; //Ivysaur only
const PFUSHIGISOU_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xf95c90; //Shared
const PFUSHIGISOU_VTABLE_RESPAWN_INITIALIZATION_OFFSET: usize = 0xf96330; //Shared

//Ivysaur Startup Initialization
#[skyline::hook(offset = PFUSHIGISOU_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pfushigisou_start_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
}

//Ivysaur Reset Initialization
#[skyline::hook(offset = PFUSHIGISOU_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pfushigisou_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_PFUSHIGISOU as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Ivysaur Death Initialization
#[skyline::hook(offset = PFUSHIGISOU_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pfushigisou_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Ivysaur Once Per Fighter Frame
#[skyline::hook(offset = PFUSHIGISOU_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn pfushigisou_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_PFUSHIGISOU as u32 {
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
                    EffectModule::remove_screen(boma, Hash40::new("bg_ptrainer_final"), -1);
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
    }
    original!()(vtable, fighter)
}

//Ivysaur Respawn Initialization
#[skyline::hook(offset = PFUSHIGISOU_VTABLE_RESPAWN_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pfushigisou_respawn_initialization(_vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_PFUSHIGISOU as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
}

pub fn install() {
    skyline::install_hooks!(
        pfushigisou_start_initialization,
        pfushigisou_reset_initialization,
        pfushigisou_death_initialization,
        pfushigisou_opff,
        pfushigisou_respawn_initialization
    );
}