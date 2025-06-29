use super::*;

const GEKKOUGA_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xadac40; //Greninja only
const GEKKOUGA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const GEKKOUGA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xadaf50; //Greninja only

unsafe extern "C" fn gekkouga_check_ground_special_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    let status_kind_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let cmd_cat1 = fighter.global_table[CMD_CAT1].get_i32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_play = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("stick_play"));
    let life = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("life"));
    let x_offset = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("x_offset"));
    let mut bool_check = false;
    let mut ret = 0;
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    if status_kind_interrupt == *FIGHTER_STATUS_KIND_REBIRTH {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_REBIRTH_FLAG_MOVE_END) {
            ret = 0;
        }
    }
    if [*FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_CLIFF_ROBBED].contains(&status_kind_interrupt) {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            ret = 0;
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_BOUND) {
        ret = 0;
    }
    if [*FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, *FIGHTER_STATUS_KIND_LANDING].contains(&status_kind_interrupt) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_DISABLE) {
            if !CancelModule::is_enable_cancel(fighter.module_accessor) {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_LANDING_FLAG_STIFF_CANCEL) {
                    if prev_status_kind == *FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON {
                        ret = 0;
                    }
                }
            }
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_DISABLE);
        }
        else {
            if !CancelModule::is_enable_cancel(fighter.module_accessor) {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_LANDING_FLAG_STIFF_CANCEL) {
                    ret = 0;
                }
                if prev_status_kind == *FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON {
                    ret = 0;
                }
            }
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_START_HOLD) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_DISABLE) {
            if situation_kind == *SITUATION_KIND_AIR {
                if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 {
                    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI) {
                        ret = 0;
                    }
                }
                if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
                    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) {
                        ret = 0;
                    }
                }
            }
            if cmd_cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S != 0 {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_START_HOLD);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_FRONT);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOLD_FRONT);
                WorkModule::set_float(fighter.module_accessor, life, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_HOLD_COUNT);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_WARP_GIMMICK);
                FighterSpecializer_Gekkouga__set_special_s_transition_term_forbid_group(module_accessor, true);
                if stick_play >= stick_x*-lr {
                    WorkModule::set_float(fighter.module_accessor, lr, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_OFFSET_LR);
                }
                else {
                    bool_check = true;
                    WorkModule::set_float(fighter.module_accessor, -lr, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_OFFSET_LR);
                }
                WorkModule::set_float(fighter.module_accessor, x_offset, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_OFFSET_X);
                if ![*FIGHTER_STATUS_KIND_RUN, *FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status_kind_interrupt) {
                    if ![*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE].contains(&status_kind_interrupt) {
                        fun_7100004280(fighter, bool_check.into(), stick_play.into());
                        ret = 0;
                    }
                    else {
                        fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), false.into());
                        ret = 1;
                    }
                }
                else {
                    fighter.change_status(FIGHTER_STATUS_KIND_RUN_BRAKE.into(), false.into());
                    ret = 1;
                }
            }
        }
    }
    ret.into()
}

unsafe extern "C" fn fun_7100004280(fighter: &mut L2CFighterCommon, param_2: L2CValue, param_3: L2CValue) {
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_START_HOLD) {
        if !param_2.get_bool() {
            if param_3.get_f32() < stick_x*lr {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_FRONT);
            }
            if param_3.get_f32() < stick_x*-lr {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_FRONT);
            }
        }
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
        WorkModule::unable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_RUN);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_SWING_DASH);
        if status_kind == *FIGHTER_STATUS_KIND_TURN_DASH {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_FLAG_DASH);
        }
        if status_kind == *FIGHTER_STATUS_KIND_DASH {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_S4);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DASH_FLAG_NO_ESCAPE);
        }
        if status_kind == *FIGHTER_STATUS_KIND_TURN {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_FLAG_NO_TURN_TO_ESCAPE);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_FLAG_DASH);
        }
        if status_kind == *FIGHTER_STATUS_KIND_RUN_BRAKE {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_RUN_BRAKE_FLAG_TURN_RUN);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_RUN_BRAKE_FLAG_ATTACK_DASH_STRANS_OFF);
        }
    }
}

unsafe extern "C" fn gekkouga_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_VERTICAL);
    WorkModule::off_flag(boma, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_FLAG_SPECIAL_S_ATTACK_HI);
}

//Greninja Startup Initialization
#[skyline::hook(offset = GEKKOUGA_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn gekkouga_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    gekkouga_var(&mut *boma);
    agent.global_table[CHECK_GROUND_SPECIAL_UNIQ].assign(&L2CValue::Ptr(gekkouga_check_ground_special_uniq as *const () as _));
    original!()(vtable, fighter)
}

//Greninja Reset Initialization
#[skyline::hook(offset = GEKKOUGA_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn gekkouga_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_GEKKOUGA as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        gekkouga_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Greninja Death Initialization
#[skyline::hook(offset = GEKKOUGA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn gekkouga_death_initialization(vtable: u64, fighter: &mut Fighter, param_3: u64) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    gekkouga_var(&mut *boma);
    original!()(vtable, fighter, param_3)
}

pub fn install() {
    skyline::install_hooks!(
        gekkouga_start_initialization,
        gekkouga_reset_initialization,
        gekkouga_death_initialization
    );
}