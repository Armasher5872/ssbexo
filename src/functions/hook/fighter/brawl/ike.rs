use super::*;

const IKE_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xaf8010; //Ike only
const IKE_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const IKE_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xaf80b0; //Ike only
const IKE_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xaf80d0; //Ike only

//Ike Startup Initialization
#[skyline::hook(offset = IKE_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ike_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_N);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Ike Reset Initialization
#[skyline::hook(offset = IKE_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ike_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_IKE as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        WorkModule::off_flag(boma, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_N);
    }
    original!()(vtable, fighter)
}

//Ike Death Initialization
#[skyline::hook(offset = IKE_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn ike_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    WorkModule::off_flag(boma, *FIGHTER_IKE_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_N);
    original!()(vtable, fighter)
}

//Ike Once Per Fighter Frame
#[skyline::hook(offset = IKE_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn ike_opff(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    let vtable_slow = *battle_object_slow.add(0x8) == 0 || *(battle_object_slow as *const u32) == 0;
    let counter = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_COUNTER);
    let handle = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_FINAL_ZOOM_HANDLE);
    if vtable_slow && !StopModule::is_stop(boma) {
        if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
            WorkModule::set_int(boma, 0, *FIGHTER_IKE_INSTANCE_WORK_ID_INT_SPECIAL_HI_CLIFF_NUM);
        }
    }
    //Final Zoom Effect Clearing
    if counter > 0 {
        if counter == 20 {
            if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_ZOOM_LAST_STOCK) {
                EffectModule::remove_screen(boma, Hash40::new("bg_finishhit"), -1);
                set_stage_visibility(boma, 1);
                set_vis_hud(true);
            }
            else {
                EffectModule::remove_screen(boma, Hash40::new("bg_ike_final"), -1);
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

pub fn install() {
	skyline::install_hooks!(
        ike_start_initialization,
        ike_reset_initialization,
        ike_death_initialization,
        ike_opff
    );
}