#![allow(unused_macros)]
use {
    crate::functions::ARMSTRONG_IS_SPECIAL_HI,
    smash::{
        app::{
            lua_bind::*,
            *
        },
        lua2cpp::L2CFighterCommon,
        lib::{
            L2CValue,
            lua_const::*,
        },
        phx::Hash40
    },
    smashline::*,
};

//dissables side-special kinetic stuff
#[status_script(agent = "ganon", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn armstrong_side_special_status_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if ARMSTRONG_IS_SPECIAL_HI[entry_id] {
        0.into()
    }
    else {
        original!(fighter)
    }
}

pub unsafe fn armstrong_side_special_status_loop(fighter: &mut L2CFighterCommon) -> bool {
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), true.into());
        }
        return true.into()
    }
    else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        GroundModule::set_attach_ground(fighter.module_accessor, false);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GANON_SPECIAL_HI);
    }
    return false.into()
}

//resets flag when status ends
#[status_script(agent = "ganon", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn armstrong_side_special_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    ARMSTRONG_IS_SPECIAL_HI[entry_id] = false;
    original!(fighter)
}

//switch status before anything happens in LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN
#[status_script(agent = "ganon", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn armstrong_up_special_status_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    ARMSTRONG_IS_SPECIAL_HI[entry_id] = true;
    fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), true.into());
    0.into()
}

//new up-special script in side-special status
#[status_script(agent = "ganon", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn armstrong_up_special_cling_status_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if ARMSTRONG_IS_SPECIAL_HI[entry_id] {
        KineticModule::clear_speed_all(fighter.module_accessor);
        if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        fighter.sub_shift_status_main(L2CValue::Ptr(armstrong_side_special_status_loop as *const () as _))
    }
    else {
        original!(fighter)
    }
}

pub fn install() {
    install_status_scripts!(
        armstrong_side_special_status_exec,
        armstrong_side_special_status_end,
        armstrong_up_special_status_pre,
        armstrong_up_special_cling_status_exit
    );
}