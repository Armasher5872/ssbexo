use super::*;

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_FighterStatusCapture_set_invalid_capture)]
unsafe extern "C" fn fighter_status_capture_set_invalid_capture(fighter: &mut L2CFighterCommon) {
    let parent_id = LinkModule::get_parent_object_id(fighter.module_accessor, *LINK_NO_CAPTURE);
    if parent_id as i32 != *BATTLE_OBJECT_ID_INVALID {
        let parent_boma = sv_battle_object::module_accessor(parent_id as u32);
        let parent_kind = utility::get_kind(&mut *parent_boma);
        let parent_status_kind = StatusModule::status_kind(parent_boma);
        if parent_kind == *FIGHTER_KIND_PICHU && parent_status_kind == *FIGHTER_STATUS_KIND_THROW && WorkModule::is_flag(parent_boma, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH) {
            WorkModule::set_int(fighter.module_accessor, 90, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CAPTURE_FRAME);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CHECK_CATCH);
            EffectModule::req_common(fighter.module_accessor, Hash40::new("invalid_capture"), 0.0);
        }
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(fighter_status_capture_set_invalid_capture);
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}