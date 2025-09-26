use super::*;

const BAYONETTA_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x819430; //Bayonetta only
const BAYONETTA_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x819440; //Bayonetta only
const BAYONETTA_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x81a050; //Bayonetta only

unsafe extern "C" fn bayonetta_check_special_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let cat4 = fighter.global_table[CMD_CAT4].get_i32();
    if situation_kind == *SITUATION_KIND_AIR {
        if cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6N6 != 0
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND) {
            fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F_DASH.into(), false.into());
            return true.into();
        }
    }
    false.into()
}

//Bayonetta Startup Initialization
#[skyline::hook(offset = BAYONETTA_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn bayonetta_start_initialization(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
    set_command_input_button(boma, *FIGHTER_PAD_CMD_CAT4_COMMAND_6N6 as usize, 0);
    agent.global_table[CHECK_SPECIAL_COMMAND].assign(&L2CValue::Ptr(bayonetta_check_special_command as *const () as _));
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Bayonetta Reset Initialization
#[skyline::hook(offset = BAYONETTA_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn bayonetta_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
    set_command_input_button(boma, *FIGHTER_PAD_CMD_CAT4_COMMAND_6N6 as usize, 0);
    original!()(vtable, fighter)
}

//Bayonetta Death Initialization
#[skyline::hook(offset = BAYONETTA_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn bayonetta_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_COMMAND);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        bayonetta_start_initialization,
        bayonetta_reset_initialization,
        bayonetta_death_initialization
    );
}