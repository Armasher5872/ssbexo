use super::*;

//Status Shield Break Fall Main, makes soft breaks go into untechable knockdown instead of Shield Break Downed
#[skyline::hook(replace = L2CFighterCommon_status_ShieldBreakFall)]
unsafe extern "C" fn status_shield_break_fall(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.module_accessor;
    let gravity_accel = {fighter.clear_lua_stack(); lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY); sv_kinetic_energy::get_accel_y(lua_state)};
    let gravity_limit_speed_y = {fighter.clear_lua_stack(); lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY); sv_kinetic_energy::get_limit_speed_y(lua_state)};
    if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK) {
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, gravity_accel*0.2);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, gravity_limit_speed_y*0.3);
    }
    MotionModule::change_motion(boma, Hash40::new("fall_damage"), 0.0, 1.0, false, 0.0, false, false);
    ControlModule::set_rumble(boma, Hash40::new_raw(0x1126cc38f), 0, true, 0x50000000);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_ShieldBreakFall_Main as *const () as _))
}

//Status Shield Break Fall Main, makes soft breaks go into untechable knockdown instead of Shield Break Downed
#[skyline::hook(replace = L2CFighterCommon_status_ShieldBreakFall_Main)]
unsafe extern "C" fn status_shield_break_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let boma = fighter.module_accessor;
    if situation_kind == *SITUATION_KIND_GROUND {
        if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK) {
            fighter.change_status(FIGHTER_STATUS_KIND_DOWN.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_SHIELD_BREAK_DOWN.into(), false.into());
        }
    }
    0.into()
}

//Status End Shield Break Fall
#[skyline::hook(replace = L2CFighterCommon_status_end_ShieldBreakFall)]
unsafe extern "C" fn status_end_shield_break_fall(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    let boma = fighter.module_accessor;
    if status_kind != *FIGHTER_STATUS_KIND_SHIELD_BREAK_FALL {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HARD_BREAK);
    }
    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_shield_break_fall,
            status_shield_break_fall_main,
            status_end_shield_break_fall
        );
    }
}

pub fn install() {
    let _ = skyline::nro::add_hook(nro_hook);
}