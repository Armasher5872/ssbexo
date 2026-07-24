/* The hooks and status_kind edits are credited to the HDR Code Repository and WuBoyTH's source code from the WuBor Patch */
use super::*;

//FighterStatusGuard__check_hit_stop_delay. Removes shield ASDI
#[skyline::hook(replace = L2CFighterCommon_FighterStatusGuard__check_hit_stop_delay)]
unsafe extern "C" fn fighterstatusguard_check_hit_stop_delay(_fighter: &mut L2CFighterCommon, _param_2: L2CValue) -> L2CValue {
    false.into()
}

//FighterStatusGuard__check_hit_stop_delay_flick. Removes shield SDI
#[skyline::hook(replace = L2CFighterCommon_FighterStatusGuard__check_hit_stop_delay_flick)]
unsafe extern "C" fn fighterstatusguard_check_hit_stop_delay_flick(_fighter: &mut L2CFighterCommon, _param_2: L2CValue) -> L2CValue {
    false.into()
}

/*SHIELD MAIN HOOKS*/

#[skyline::hook(offset = 0x64192c, inline)]
unsafe extern "C" fn shield_damage_unbroken_change_status(ctx: &mut skyline::hooks::InlineCtx) {
    let defender_boma = *(ctx.registers[0].x() as *const u64).add(1) as *mut BattleObjectModuleAccessor;
    let defender_status_kind = StatusModule::status_kind(defender_boma);
    let attack_power = *(ctx.registers[19].x() as *const f32).add(0xf730/0x4);
    let ptr = get_module_vtable_func(defender_boma, 0x100, 0x1E0);
    let get_attacker_info: extern "C" fn(shield_module: *mut u64) -> *const AttackerInfo = std::mem::transmute(ptr);
    let shield_module = *(defender_boma as *mut *mut u64).add(0x100/0x8);
    let shield_module_attacker_info = get_attacker_info(shield_module);
    let attacker_id = (*shield_module_attacker_info).attacker_id;
    if attacker_id != *BATTLE_OBJECT_ID_INVALID as u32 {
        let attacker_battle_object = get_battle_object_from_id(attacker_id);
        let attacker_battle_object_vtable: extern "C" fn(*mut BattleObject) -> bool = std::mem::transmute(**(attacker_battle_object as *const *const u64));
        if !attacker_battle_object_vtable(attacker_battle_object) && 3 < *(attacker_battle_object as *const u8).add(0x34) {
            let attacker_boma = (*attacker_battle_object).module_accessor;
            let attacker_motion_rate = if attack_power == 0.0 {1.0} else {(1.0-(0.02*attack_power)).clamp(0.5, 1.0)};
            if WorkModule::is_flag(defender_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ENABLE_PARRY) && defender_status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF {
                if (*shield_module_attacker_info).attacker_category == *BATTLE_OBJECT_CATEGORY_FIGHTER as u8 {
                    if StatusModule::situation_kind(attacker_boma) == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request_from_script(attacker_boma, *FIGHTER_STATUS_KIND_REBOUND_JUMP, false);
                        KineticModule::clear_speed_energy_id(attacker_boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
                    }
                    else {
                        WorkModule::set_float(attacker_boma, attacker_motion_rate, *FIGHTER_STATUS_WORK_ID_FLOAT_REBOUND_MOTION_RATE);
                        StatusModule::change_status_request_from_script(attacker_boma, *FIGHTER_STATUS_KIND_REBOUND, false);
                    }
                }
            }
            if WorkModule::is_flag(attacker_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED) {
                let hard_break = EffectModule::req(attacker_boma, Hash40::new("sys_flash"), &Vector3f{x: LAST_ATTACK_HITBOX_LOCATION_X, y: LAST_ATTACK_HITBOX_LOCATION_Y, z: 0.0}, &Vector3f::zero(), 1.5, 0, 0, false, 0) as u32;
                EffectModule::set_rgb(attacker_boma, hard_break, 1.0, 0.05, 0.05);
                EffectModule::set_rate(attacker_boma, hard_break, 0.75);
            }
        }
    }
    StatusModule::change_status_request(defender_boma, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, false);
}

#[skyline::hook(offset = 0x6416f8, inline)]
unsafe extern "C" fn shield_damage_broken_change_status(ctx: &mut skyline::hooks::InlineCtx) {
    let defender_boma = *(ctx.registers[0].x() as *const u64).add(1) as *mut BattleObjectModuleAccessor;
    let attack_power = *(ctx.registers[19].x() as *const f32).add(0xf730/0x4);
    let hp = WorkModule::get_float(defender_boma, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    let ptr = get_module_vtable_func(defender_boma, 0x100, 0x1E0);
    let get_attacker_info: extern "C" fn(shield_module: *mut u64) -> *const AttackerInfo = std::mem::transmute(ptr);
    let shield_module = *(defender_boma as *mut *mut u64).add(0x100/0x8);
    let shield_module_attacker_info = get_attacker_info(shield_module);
    let attacker_id = (*shield_module_attacker_info).attacker_id;
    if attacker_id != *BATTLE_OBJECT_ID_INVALID as u32 {
        let attacker_battle_object = get_battle_object_from_id(attacker_id);
        let attacker_battle_object_vtable: extern "C" fn(*mut BattleObject) -> bool = std::mem::transmute(**(attacker_battle_object as *const *const u64));
        if !attacker_battle_object_vtable(attacker_battle_object) && 3 < *(attacker_battle_object as *const u8).add(0x34) {
            let attacker_boma = (*attacker_battle_object).module_accessor;
            if !WorkModule::is_flag(attacker_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK_ENABLED) {
                if attack_power >= hp*2.5 {
                    EffectModule::req_follow(defender_boma, Hash40::new("sys_hit_ice_s"), Hash40::new("throw"), &Vector3f::zero(), &Vector3f::zero(), 1.5, false, 0, 0, 0, 0, 0, false, false);
                    WorkModule::on_flag(defender_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK);
                    StatusModule::change_status_request(defender_boma, *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY, false);
                }
                else {
                    EffectModule::req_follow(defender_boma, Hash40::new("sys_hit_ice_s"), Hash40::new("throw"), &Vector3f::zero(), &Vector3f::zero(), 1.5, false, 0, 0, 0, 0, 0, false, false);
                    WorkModule::off_flag(defender_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK);
                    StatusModule::change_status_request(defender_boma, *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY, false);
                }
            }
            else {
                EffectModule::req_follow(defender_boma, Hash40::new("sys_hit_ice_s"), Hash40::new("throw"), &Vector3f::zero(), &Vector3f::zero(), 1.5, false, 0, 0, 0, 0, 0, false, false);
                WorkModule::on_flag(defender_boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK);
                StatusModule::change_status_request(defender_boma, *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY, false);
            }
        }
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            fighterstatusguard_check_hit_stop_delay,
            fighterstatusguard_check_hit_stop_delay_flick
        );
    }
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x64192c).nop(); //Removes the vanilla status change in the greater than or equal to 0 shield hp check for the updated one
    let _ = skyline::patching::Patch::in_text(0x6416f8).nop(); //Removes the vanilla status change in the less than 0 shield hp check for the updated one
    let _ = skyline::nro::add_hook(nro_hook);
    skyline::install_hooks!(
        shield_damage_unbroken_change_status,
        shield_damage_broken_change_status
    );
}