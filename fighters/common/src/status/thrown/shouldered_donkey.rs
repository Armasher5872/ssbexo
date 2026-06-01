use super::*;

//Status Pre Shouldered Donkey Start
#[skyline::hook(replace = L2CFighterCommon_status_pre_ShoulderedDonkeyStart)]
unsafe extern "C" fn status_pre_shouldered_donkey_start(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, true, false, 0, (*FIGHTER_STATUS_ATTR_NO_DROP_ITEM | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE) as u32, 0, 0);
    0.into()
}

//Status Pre Shouldered Donkey
#[skyline::hook(replace = L2CFighterCommon_status_pre_ShoulderedDonkey)]
unsafe extern "C" fn status_pre_shouldered_donkey(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    StatusModule::init_settings(boma, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(boma, false, *FIGHTER_TREADED_KIND_NO_REAC, false, true, false, 0, (*FIGHTER_STATUS_ATTR_NO_DROP_ITEM | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE) as u32, 0, 0);
    0.into()
}

//Uniq Process Shouldered Donkey Init Status
#[skyline::hook(replace = L2CFighterCommon_uniq_process_ShoulderedDonkey_init_status)]
unsafe extern "C" fn uniq_process_shouldered_donkey_init_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let status_kind_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    if [*FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_START, *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY].contains(&status_kind_interrupt) {
        let shouldered_frame_add = WorkModule::get_param_float(boma, hash40("common"), hash40("shouldered_frame_add"));
        let shouldered_frame_mul = WorkModule::get_param_float(boma, hash40("common"), hash40("shouldered_frame_mul"));
        let shouldered_cratter_frame /*LStack_a0*/ = WorkModule::get_param_float(boma, hash40("common"), hash40("shouldered_cratter_frame"));
        let damage = DamageModule::damage(boma, 0);
        let get_clatter_time = ControlModule::get_clatter_time(boma, 0);
        ControlModule::start_clatter(boma, ((damage*shouldered_frame_mul)+shouldered_frame_add)+get_clatter_time, 0.0, shouldered_cratter_frame, 127, 0, false, false);
        fighter.clear_lua_stack();
        lua_args!(fighter, *MA_MSC_CMD_CAPTURE_THROWN);
        sv_module_access::capture(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
        LinkModule::set_attribute(boma, *LINK_NO_CAPTURE, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_COLOR_BLEND as u8}, false);
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_shouldered_donkey_start,
            status_pre_shouldered_donkey,
            uniq_process_shouldered_donkey_init_status
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}