use super::*;

const PIKACHU_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xf2a520; //Shared
const PIKACHU_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x68d5e0; //Shared
const PIKACHU_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xf2a530; //Shared
const PIKACHU_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xf2a630; //Shared

unsafe extern "C" fn pikachu_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        WorkModule::set_flag(fighter.module_accessor, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    }
    0.into()
}

unsafe extern "C" fn pikachu_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_ATTACK_11_DASH);
    WorkModule::set_int(boma, 0, FIGHTER_PIKACHU_INSTANCE_WORK_ID_INT_ATTACK_11_COUNT);
}

//Pikachu Startup Initialization
#[skyline::hook(offset = PIKACHU_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pikachu_start_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_PIKACHU as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        pikachu_var(&mut *boma);
        agent.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(pikachu_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Pikachu Reset Initialization
#[skyline::hook(offset = PIKACHU_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pikachu_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_PIKACHU as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        pikachu_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Pikachu Death Initialization
#[skyline::hook(offset = PIKACHU_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn pikachu_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_PIKACHU as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
        pikachu_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Pikachu Once Per Fighter Frame
#[skyline::hook(offset = PIKACHU_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn pikachu_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let frame = MotionModule::frame(boma);
    if fighter.battle_object.kind == *FIGHTER_KIND_PIKACHU as u32 {
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK {
            if frame < 1.0 {
                WorkModule::on_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
            }
            if WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD) && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                WorkModule::off_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
                WorkModule::inc_int(boma, FIGHTER_PIKACHU_INSTANCE_WORK_ID_INT_ATTACK_11_COUNT);
            }
            if WorkModule::get_int(boma, FIGHTER_PIKACHU_INSTANCE_WORK_ID_INT_ATTACK_11_COUNT) > 1 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
                WorkModule::on_flag(boma, FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_ATTACK_11_DASH);
            }
        };
        if status_kind != *FIGHTER_STATUS_KIND_ATTACK {
            WorkModule::set_int(boma, 0, FIGHTER_PIKACHU_INSTANCE_WORK_ID_INT_ATTACK_11_COUNT);
            WorkModule::off_flag(boma, FIGHTER_PIKACHU_INSTANCE_WORK_ID_FLAG_ATTACK_11_DASH);
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        pikachu_start_initialization,
        pikachu_reset_initialization,
        pikachu_death_initialization,
        pikachu_opff
    );
}