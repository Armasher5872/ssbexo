use super::*;

const SHIZUE_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xdba810; //Shared
const SHIZUE_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xdbab30; //Shared
const SHIZUE_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xdbad80; //Shared
const SHIZUE_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xdbb940; //Shared

unsafe extern "C" fn shizue_var(boma: &mut BattleObjectModuleAccessor) {
    let team_no = TeamModule::team_no(boma) as i32;
    WorkModule::set_int(boma, team_no, *FIGHTER_MURABTIO_INSTANCE_WORK_ID_INT_TEAM_NO);
    WorkModule::set_int(boma, *ITEM_KIND_NONE, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_LINK_ARROW_FUSE_ITEM);
}

//Isabelle Startup Initialization
#[skyline::hook(offset = SHIZUE_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn shizue_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_SHIZUE as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        shizue_var(&mut *boma);
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Isabelle Reset Initialization
#[skyline::hook(offset = SHIZUE_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn shizue_reset_initialization(vtable: u64, fighter: &mut Fighter) {
    if fighter.battle_object.kind == *FIGHTER_KIND_SHIZUE as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        shizue_var(&mut *boma);
        WorkModule::set_int(boma, 0, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_OBJECT_NUM);
        WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_OBJECT_ID);
        WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, 0x100000C8);
        WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, 0x100000C9);
        WorkModule::set_int(boma, 6, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_OBJECT_CATEGORY);
        WorkModule::set_int(boma, 6, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_SPECIAL_N_OBJECT_CATEGORY_PREV);
    }
    original!()(vtable, fighter)
}

//Isabelle Death Initialization
#[skyline::hook(offset = SHIZUE_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn shizue_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_SHIZUE as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
        shizue_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Isabelle Once Per Fighter Frame
#[skyline::hook(offset = SHIZUE_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn shizue_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    ac_common(boma);
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        shizue_start_initialization,
        shizue_reset_initialization,
        shizue_death_initialization,
        shizue_opff
    );
}