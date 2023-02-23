//All of this is credited to the HDR Code Repository
#![allow(unused_macros)]
use {
    crate::functions::{
        BARREL_ACTIVE,
        BARREL_TIMER,
        SITUATION_KIND,
        PREV_STATUS_KIND
    },
    smash::{
        app::{
            lua_bind::*,
            *
        },
        hash40,
        lib::{
            L2CValue,
            lua_const::*,
        },
        lua2cpp::L2CFighterCommon,
        phx::Hash40,
    },
    smash_script::*,
    smashline::*,
};

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn donkey_special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion;
    let kinetic;
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        motion = Hash40::new("special_air_lw");
        kinetic = *FIGHTER_KINETIC_TYPE_AIR_STOP;
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
    }
    else {
        if BARREL_ACTIVE[entry_id] == true
        && BARREL_TIMER[entry_id] > 0 {
            kinetic = *FIGHTER_KINETIC_TYPE_NONE;
            if PostureModule::lr(fighter.module_accessor) == 1.0 {
                motion = Hash40::new("appeal_lw_r");
            }
            else {
                motion = Hash40::new("appeal_lw_l");
            }
        }
        else {
            kinetic = *FIGHTER_KINETIC_TYPE_GROUND_STOP;
            motion = Hash40::new("special_lw_start");
        }
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    }
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(donkey_special_lw_main_loop as *const () as _))
}

pub unsafe extern "C" fn donkey_special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_air = MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw");
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if is_air {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_transition_group_check_air_landing().get_bool() 
            || fighter.sub_wait_ground_check_common(false.into()).get_bool() 
            || fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        let status = if situation != *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_FALL
        }
        else {
            WorkModule::set_float(fighter.module_accessor, 20.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL
        };
        fighter.change_status(status.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if is_air {
            FIGHTER_STATUS_KIND_FALL
        }
        else {
            FIGHTER_STATUS_KIND_WAIT
        };
        fighter.change_status(status.into(), false.into());
    }
    1.into()
}

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_CATCH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn donkey_catch_pull_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        sv_kinetic_energy!(clear_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, *ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
    }
    fighter.status_CatchPull_common(hash40("catch_wait").into());
    fighter.sub_shift_status_main(L2CValue::Ptr(donkey_catch_pull_main_loop as *const () as _))
}

unsafe extern "C" fn donkey_catch_pull_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_SPECIAL_LW
    && CancelModule::is_enable_cancel(fighter.module_accessor) {
        let status = fighter.global_table[0x45].get_i32();
        fighter.change_status(status.into(), false.into());
        1.into()
    }
    else {
        fighter.status_CatchPull_Main()
    }
}

//Link Events. Enables proper transition into grabs
#[skyline::hook(offset = 0x993ec0)]
pub unsafe extern "C" fn donkey_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> u64 {
    if event.link_event_kind.0 == hash40("capture") {
        let capture_event : &mut smash2::app::LinkEventCapture = std::mem::transmute(event);
        let module_accessor = fighter.battle_object.module_accessor;
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            capture_event.result = true;
            capture_event.node = smash2::phx::Hash40::new("throw");
            StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_CATCH_PULL, false);
            return 0;
        }
        return 1;
    }
    original!()(vtable, fighter, event)
}

pub fn install() {
    install_status_scripts!(
        donkey_special_lw_main,
        donkey_catch_pull_main
    );
    skyline::install_hooks!(donkey_link_event);
}