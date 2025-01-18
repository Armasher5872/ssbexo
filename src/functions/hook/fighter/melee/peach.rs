use super::*;

const PEACH_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xe88ca0; //Shared
const PEACH_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PEACH_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xe89090; //Shared
const PEACH_VTABLE_LINK_EVENT_OFFSET: usize = 0x68d880; //Shared

unsafe extern "C" fn peach_check_special_lw_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ItemModule::is_have_item(fighter.module_accessor, 0) {
        return 0.into();
    }
    1.into()
}

//Peach Startup Initialization
#[skyline::hook(offset = PEACH_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn peach_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_PEACH as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        agent.global_table[CHECK_AIR_SPECIAL_UNIQ].assign(&false.into());
        agent.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(peach_check_special_lw_uniq as *const () as _));
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Peach Reset Initialization
#[skyline::hook(offset = PEACH_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn peach_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_PEACH as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Peach Death Initialization
#[skyline::hook(offset = PEACH_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn peach_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_PEACH as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Link Event for Peach
#[skyline::hook(offset = PEACH_VTABLE_LINK_EVENT_OFFSET)]
unsafe extern "C" fn peach_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    if fighter.battle_object.kind == *FIGHTER_KIND_PEACH as u32 {
        if event.link_event_kind.0 == hash40("capture") {
            let capture_event : &mut smash2::app::LinkEventCapture = std::mem::transmute(event);
            let offset = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET);
            let offset_lw = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET_LW);
            if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW && capture_event.status == *FIGHTER_STATUS_KIND_CAPTURE_PULLED {
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
                capture_event.node = smash2::phx::Hash40::new("throw");
                capture_event.result = true;
                capture_event.motion_offset = offset;
                capture_event.motion_offset_lw = offset_lw;
                fighter.battle_object.change_status_req(*FIGHTER_STATUS_KIND_THROW, false);
                return 0;
            }
            return 1;
        }
    }
    original!()(vtable, fighter, event)
}

pub fn install() {
    skyline::install_hooks!(
        peach_start_initialization,
        peach_reset_initialization,
        peach_death_initialization,
        peach_link_event
    );
}