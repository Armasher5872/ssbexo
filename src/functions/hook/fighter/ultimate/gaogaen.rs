use super::*;

const GAOGAEN_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xab7cb0; //Incineroar only
const GAOGAEN_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xab7cc0; //Incineroar only
const GAOGAEN_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xab8390; //Incineroar only
const GAOGAEN_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xab8a20; //Incineroar only

unsafe extern "C" fn gaogaen_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_CAN_ANGLE_CATCH);
    WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_THROW_CRITICAL_ZOOM);
    WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_S_CANCEL);
    WorkModule::off_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_SPECIAL_S_FAKE);
}

//Incineroar Startup Initialization
#[skyline::hook(offset = GAOGAEN_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn gaogaen_start_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    gaogaen_var(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
}

//Incineroar Reset Initialization
#[skyline::hook(offset = GAOGAEN_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn gaogaen_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    gaogaen_var(&mut *boma);
    original!()(vtable, fighter)
}

//Incineroar Death Initialization
#[skyline::hook(offset = GAOGAEN_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn gaogaen_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    gaogaen_var(&mut *boma);
    original!()(vtable, fighter)
}

//Incineroar Once Per Fighter Frame
#[skyline::hook(offset = GAOGAEN_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn gaogaen_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
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
                EffectModule::remove_screen(boma, Hash40::new("bg_gaogaen_final"), -1);
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

pub fn install() {
    skyline::install_hooks!(
        gaogaen_start_initialization,
        gaogaen_reset_initialization,
        gaogaen_death_initialization,
        gaogaen_opff
    );
}