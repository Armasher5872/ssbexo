use super::*;

const SAMUSD_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x68d5a0; //Shared
const SAMUSD_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x10f3630; //Shared
const SAMUSD_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x10f3650; //Shared
const SAMUSD_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0x10f37a0; //Shared

unsafe extern "C" fn samusd_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    }
    0.into()
}

unsafe extern "C" fn samusd_var(boma: &mut BattleObjectModuleAccessor) {
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    HAS_FIRE_CHARGE_SHOT[entry_id] = false;
    CHARGE_SHOT_TIMER[entry_id] = 0;
}

//Dark Samus Startup Initialization
#[skyline::hook(offset = SAMUSD_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn samusd_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_SAMUSD as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        samusd_var(&mut *boma);
        agent.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(samusd_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Dark Samus Reset Initialization
#[skyline::hook(offset = SAMUSD_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn samusd_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_SAMUSD as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        samusd_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Dark Samus Death Initialization
#[skyline::hook(offset = SAMUSD_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn samusd_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_SAMUSD as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Dark Samus Once Per Fighter Frame
#[skyline::hook(offset = SAMUSD_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn samusd_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let frame = MotionModule::frame(boma);
    let end_frame = MotionModule::end_frame(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    if fighter.battle_object.kind == *FIGHTER_KIND_SAMUSD as u32 {
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N 
        && motion_kind == hash40("damage_n_2") {
            if StatusModule::is_situation_changed(boma) {
                if situation_kind != SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                }
                else {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                }
            }
            if end_frame - frame <= 2.0 {
                if situation_kind != *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
                }
                else {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                }
            }
        };
        if CHARGE_SHOT_TIMER[entry_id] > 0 {
            CHARGE_SHOT_TIMER[entry_id] -= 1;
        }
        if CHARGE_SHOT_TIMER[entry_id] <= 0 && HAS_FIRE_CHARGE_SHOT[entry_id] {
            HAS_FIRE_CHARGE_SHOT[entry_id] = false;
            fighter.battle_object.gimmick_flash();
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        samusd_start_initialization,
        samusd_reset_initialization,
        samusd_death_initialization,
        samusd_opff
    );
}