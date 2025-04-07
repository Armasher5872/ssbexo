use super::*;

#[skyline::hook(offset = 0x64192c, inline)]
unsafe extern "C" fn shield_damage_unbroken_change_status(ctx: &mut skyline::hooks::InlineCtx) {
    let defender_boma = *(*ctx.registers[0].x.as_ref() as *const u64).add(1) as *mut BattleObjectModuleAccessor;
    let defender_status_kind = StatusModule::status_kind(defender_boma);
    let attack_power = *(*ctx.registers[19].x.as_ref() as *const f32).add(0xf730/0x4);
    let ptr = get_module_vtable_func(defender_boma, 0x100, 0x1E0);
    let get_attacker_info: extern "C" fn(shield_module: *mut u64) -> *const AttackerInfo = std::mem::transmute(ptr);
    let shield_module = *(defender_boma as *mut *mut u64).add(0x100/0x8);
    let shield_module_attacker_info = get_attacker_info(shield_module);
    let attacker_id = (*shield_module_attacker_info).attacker_id;
    let attacker_battle_object = get_battle_object_from_id(attacker_id);
    let attacker_boma = (*attacker_battle_object).module_accessor;
    let attacker_motion_rate = if attack_power == 0.0 {1.0} else {(1.0-(0.02*attack_power)).clamp(0.5, 1.0)};
    if FighterUtil::is_valid_just_shield(defender_boma) && defender_status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF {
        if StatusModule::situation_kind(attacker_boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(attacker_boma, *FIGHTER_STATUS_KIND_TREAD_DAMAGE_AIR, false);
        }
        else {
            WorkModule::set_float(attacker_boma, attacker_motion_rate, *FIGHTER_STATUS_WORK_ID_FLOAT_REBOUND_MOTION_RATE);
            StatusModule::change_status_request_from_script(attacker_boma, *FIGHTER_STATUS_KIND_REBOUND, false);
        }
    }
    StatusModule::change_status_request(defender_boma, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, false);
}

#[skyline::hook(offset = 0x6416f8, inline)]
unsafe extern "C" fn shield_damage_broken_change_status(ctx: &mut skyline::hooks::InlineCtx) {
    let defender_boma = *(*ctx.registers[0].x.as_ref() as *const u64).add(1) as *mut BattleObjectModuleAccessor;
    let ptr = get_module_vtable_func(defender_boma, 0x100, 0x1E0);
    let get_attacker_info: extern "C" fn(shield_module: *mut u64) -> *const AttackerInfo = std::mem::transmute(ptr);
    let shield_module = *(defender_boma as *mut *mut u64).add(0x100/0x8);
    let shield_module_attacker_info = get_attacker_info(shield_module);
    let attacker_id = (*shield_module_attacker_info).attacker_id;
    let attacker_battle_object = get_battle_object_from_id(attacker_id);
    let attacker_boma = (*attacker_battle_object).module_accessor;
    if !WorkModule::is_flag(attacker_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED) {
        EffectModule::req_follow(defender_boma, Hash40::new("sys_hit_ice_s"), Hash40::new("top"), &Vector3f{x: 0.0, y: 9.0, z: 0.0}, &Vector3f::zero(), 1.5, false, 0, 0, 0, 0, 0, false, false);
        StatusModule::change_status_request(defender_boma, *FIGHTER_STATUS_KIND_FURAFURA_END, false);
        CancelModule::enable_cancel(attacker_boma);
    }
    else {
        EffectModule::req_follow(defender_boma, Hash40::new("sys_hit_ice_s"), Hash40::new("top"), &Vector3f{x: 0.0, y: 9.0, z: 0.0}, &Vector3f::zero(), 1.5, false, 0, 0, 0, 0, 0, false, false);
        StatusModule::change_status_request(defender_boma, *FIGHTER_STATUS_KIND_FURAFURA, false);
    }
}

//Installation
pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x64192c).nop(); //Removes the vanilla status change in the greater than or equal to 0 shield hp check for the updated one
    let _ = skyline::patching::Patch::in_text(0x6416f8).nop(); //Removes the vanilla status change in the less than 0 shield hp check for the updated one
	skyline::install_hooks!(
        shield_damage_unbroken_change_status,
        shield_damage_broken_change_status
    );
}