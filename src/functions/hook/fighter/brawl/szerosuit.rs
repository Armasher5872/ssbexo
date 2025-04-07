use super::*;

const SZEROSUIT_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x11f4ba0; //Zero Suit Samus only
const SZEROSUIT_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x11f4c40; //Zero Suit Samus only
const SZEROSUIT_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x11f4c70; //Zero Suit Samus only
const SZEROSUIT_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x11f6d30; //Zero Suit Samus only

//Zero Suit Samus Startup Initialization
#[skyline::hook(offset = SZEROSUIT_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn szerosuit_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Zero Suit Samus Reset Initialization
#[skyline::hook(offset = SZEROSUIT_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn szerosuit_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    WorkModule::set_int(boma, 1, *FIGHTER_INSTANCE_WORK_ID_INT_AIR_LASSO_HANG_DATA);
}

//Zero Suit Samus Death Initialization
#[skyline::hook(offset = SZEROSUIT_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn szerosuit_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Zero Suit Samus Once Per Fighter Frame
#[skyline::hook(offset = SZEROSUIT_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn szerosuit_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
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
                EffectModule::remove_screen(boma, Hash40::new("bg_szerosuit_final"), -1);
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
        szerosuit_start_initialization,
        szerosuit_reset_initialization,
        szerosuit_death_initialization,
        szerosuit_opff
    );
}