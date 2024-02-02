use super::*;

//Fixes issues regarding Grab
#[skyline::hook(offset = 0xca14f0)]
unsafe extern "C" fn luigi_change_motion_callback(_vtable: u64, _fighter: &mut Fighter, _some_struct: u64) {}

//Link Event for Luigi
#[skyline::hook(offset = 0xca0e50)]
pub unsafe extern "C" fn luigi_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    if event.link_event_kind.0 == hash40("capture") {
        let capture_event: &mut smash2::app::LinkEventCapture = std::mem::transmute(event);
        if StatusModule::status_kind(boma) == FIGHTER_LUIGI_STATUS_KIND_SPECIAL_LW_LOOP && capture_event.status == *FIGHTER_STATUS_KIND_CAPTURE_PULLED {
            if LinkModule::is_link(boma, *LINK_NO_CAPTURE) {
                capture_event.result = false;
                return 0;
            }
            StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_CATCH_PULL, false);
            capture_event.result = true;
            capture_event.node = smash2::phx::Hash40::new("throw");
            capture_event.constraint = false;
            CatchModule::set_catch(boma, capture_event.sender_id);
            if !LinkModule::is_link(boma, *LINK_NO_CAPTURE) {
                return 0;
            }
            return 0;
        }
        SoundModule::play_se(boma, Hash40::new("se_common_cliff_catch"), true, false, false, false, smash::app::enSEType(0));
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