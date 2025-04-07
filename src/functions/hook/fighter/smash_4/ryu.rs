use super::*;

const RYU_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x68d5a0; //Shared
const RYU_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x10d4570; //Shared
const RYU_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x10d4620; //Shared
const RYU_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x10d5f30; //Shared

//Ryu Startup Initialization
#[skyline::hook(offset = RYU_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ryu_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_RYU as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_IS_HASOGEKI);
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Ryu Reset Initialization
#[skyline::hook(offset = RYU_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ryu_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_RYU as u32 {
        let boma = fighter.battle_object.module_accessor;
        let control_energy = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        *(control_energy as *mut u8).add(0xa4) = 1;
        common_reset_variable_reset(&mut *boma);
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_IS_HASOGEKI);
    }
    original!()(vtable, fighter)
}

//Ryu Death Initialization
#[skyline::hook(offset = RYU_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ryu_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_RYU as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_IS_HASOGEKI);
    }
    original!()(vtable, fighter)
}

//Ryu Once Per Fighter Frame
#[skyline::hook(offset = RYU_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn ryu_opff(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_RYU as u32 {
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
                    EffectModule::remove_screen(boma, Hash40::new("bg_ryu_final_shinsyoryu"), -1);
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
    original!()(vtable, fighter);
}

pub fn install() {
	skyline::install_hooks!(
        ryu_start_initialization,
        ryu_reset_initialization,
        ryu_death_initialization,
        ryu_opff
    );
}