use super::*;

const PURIN_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xfdf970; //Jigglypuff only
const PURIN_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PURIN_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xfdf980; //Jigglypuff only
const PURIN_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x68d670; //Shared

//Jigglypuff Startup Initialization
#[skyline::hook(offset = PURIN_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn purin_start_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_PURIN_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_N);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
}

//Jigglypuff Reset Initialization
#[skyline::hook(offset = PURIN_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn purin_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_PURIN as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        WorkModule::off_flag(boma, *FIGHTER_PURIN_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_N);
    }
    original!()(vtable, fighter)
}

//Jigglypuff Death Initialization
#[skyline::hook(offset = PURIN_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn purin_death_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_PURIN_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_N);
}

//Jigglypuff Once Per Fighter Frame
#[skyline::hook(offset = PURIN_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn purin_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_PURIN as u32 {
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
                    EffectModule::remove_screen(boma, Hash40::new("bg_purin_final"), -1);
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

pub fn install() {
	skyline::install_hooks!(
        purin_start_initialization,
        purin_reset_initialization,
        purin_death_initialization,
        purin_opff
    );
}