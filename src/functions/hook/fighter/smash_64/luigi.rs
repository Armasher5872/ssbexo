use super::*;

const LUIGI_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xca0ce0; //Luigi only
const LUIGI_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const LUIGI_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xca0cf0; //Luigi only
const LUIGI_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xca0e40; //Luigi only
const LUIGI_VTABLE_LINK_EVENT_OFFSET: usize = 0xca0e70; //Luigi only
const LUIGI_VTABLE_CHANGE_MOTION_CALLBACK_OFFSET: usize = 0xca1510; //Luigi only

unsafe extern "C" fn luigi_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::set_int(boma, 9, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_S_DISCHARGE_CHANCE);
    WorkModule::set_int(boma, 0, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_LW_THROW_DIRECTION);
}

//Luigi Startup Initialization
#[skyline::hook(offset = LUIGI_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn luigi_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    luigi_var(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Luigi Reset Initialization
#[skyline::hook(offset = LUIGI_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn luigi_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_LUIGI as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        luigi_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Luigi Death Initialization
#[skyline::hook(offset = LUIGI_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn luigi_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    luigi_var(&mut *boma);
    original!()(vtable, fighter)
}

//Luigi Once Per Fighter Frame
#[skyline::hook(offset = LUIGI_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn luigi_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
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
                EffectModule::remove_screen(boma, Hash40::new("bg_luigi_final"), -1);
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

//Link Event for Luigi
#[skyline::hook(offset = LUIGI_VTABLE_LINK_EVENT_OFFSET)]
unsafe extern "C" fn luigi_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    if event.link_event_kind.0 == hash40("capture") {
        let capture_event : &mut smash2::app::LinkEventCapture = std::mem::transmute(event);
        let offset = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET);
        let offset_lw = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET_LW);
        if StatusModule::status_kind(boma) == FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_LOOP && capture_event.status == *FIGHTER_STATUS_KIND_CAPTURE_PULLED {
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
            capture_event.node = smash2::phx::Hash40::new("throw");
            capture_event.result = true;
            capture_event.motion_offset = offset;
            capture_event.motion_offset_lw = offset_lw;
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH_PULL, false);
        }
        SoundModule::play_se(boma, Hash40::new("se_common_cliff_catch"), true, false, false, false, enSEType(0));
        return 1;
    }
    original!()(vtable, fighter, event)
}

//Fixes issues regarding Grab
#[skyline::hook(offset = LUIGI_VTABLE_CHANGE_MOTION_CALLBACK_OFFSET)]
unsafe extern "C" fn luigi_change_motion_callback(_vtable: u64, _fighter: &mut Fighter, _some_struct: u64) {}

pub fn install() {
	skyline::install_hooks!(
        luigi_start_initialization,
        luigi_reset_initialization,
        luigi_death_initialization,
        luigi_opff,
        luigi_link_event,
        luigi_change_motion_callback
    );
}