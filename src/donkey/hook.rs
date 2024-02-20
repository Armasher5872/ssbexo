use super::*;

//Link Event for Donkey Kong
#[skyline::hook(offset = 0x993ec0)]
unsafe extern "C" fn donkey_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    if event.link_event_kind.0 == hash40("capture") {
        let capture_event : &mut smash2::app::LinkEventCapture = std::mem::transmute(event);
        let offset = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET);
        let offset_lw = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET_LW);
        let direction = PostureModule::lr(boma) as i32;
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_AIR_LASSO && capture_event.status == *FIGHTER_STATUS_KIND_CAPTURE_PULLED {
            WorkModule::set_int(boma, direction, *FIGHTER_STATUS_SHOULDERED_DONKEY_WORK_INT_START_LR);
            capture_event.node = smash2::phx::Hash40::new("throw");
            capture_event.result = true;
            capture_event.motion_offset = offset;
            capture_event.motion_offset_lw = offset_lw;
            fighter.battle_object.change_status_req(*FIGHTER_STATUS_KIND_CATCH_PULL, false);
            return 0;
        }
        return 1;
    }
    original!()(vtable, fighter, event)
}

//Installation
pub fn install() {
	skyline::install_hooks!(donkey_link_event);
}