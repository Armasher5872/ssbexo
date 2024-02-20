use super::*;

//Fixes issues regarding Grab
#[skyline::hook(offset = 0xca14f0)]
unsafe extern "C" fn luigi_change_motion_callback(_vtable: u64, _fighter: &mut Fighter, _some_struct: u64) {}

//Link Event for Luigi
#[skyline::hook(offset = 0xca0e50)]
unsafe extern "C" fn luigi_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    if event.link_event_kind.0 == hash40("capture") {
        let capture_event : &mut smash2::app::LinkEventCapture = std::mem::transmute(event);
        if StatusModule::status_kind(boma) == FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_LOOP && capture_event.status == *FIGHTER_STATUS_KIND_CAPTURE_PULLED {
            WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
            capture_event.node = smash2::phx::Hash40::new("throw");
            capture_event.result = true;
            fighter.battle_object.change_status_req(*FIGHTER_STATUS_KIND_THROW, false);
            return 0;
        }
        SoundModule::play_se(boma, Hash40::new("se_common_cliff_catch"), true, false, false, false, enSEType(0));
        return 1;
    }
    original!()(vtable, fighter, event)
}

pub fn install() {
	skyline::install_hooks!(
        luigi_change_motion_callback,
        luigi_link_event
    );
}