use super::*;

const LINK_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xc280f0; //Shared
const LINK_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc28280; //Shared
const LINK_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xc28860; //Shared
const LINK_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xc289e0; //Shared

unsafe extern "C" fn link_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
    WorkModule::off_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPIN_ATTACK_CAN_FALL);
    WorkModule::off_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_CHARGE);
    WorkModule::off_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CAN_ASCEND);
    WorkModule::set_float(boma, 0.0, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_DEGREE);
    WorkModule::set_float(boma, 0.0, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_START_Y);
    WorkModule::set_float(boma, 0.0, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y);
    WorkModule::set_int(boma, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ASCEND_FRAME);
}

unsafe extern "C" fn link_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    }
    0.into()
}

//Link Startup Initialization
#[skyline::hook(offset = LINK_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn link_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LINK as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        common_initialization_variable_reset(&mut *boma);
        link_var(&mut *boma);
        agent.global_table[CHECK_SPECIAL_HI_UNIQ].assign(&L2CValue::Ptr(should_use_special_hi_callback as *const () as _));
        agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(link_end_control as *const () as _));
    }
    original!()(vtable, fighter)
}

//Link Reset Initialization
#[skyline::hook(offset = LINK_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn link_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LINK as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
        link_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Link Death Initialization
#[skyline::hook(offset = LINK_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn link_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LINK as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
        link_var(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Link Once Per Fighter Frame
#[skyline::hook(offset = LINK_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn link_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LINK as u32 {
        let boma = fighter.battle_object.module_accessor;
        if WorkModule::is_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED) {
            let item_id = WorkModule::get_int(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32;
            if !sv_battle_object::is_active(item_id) {
                ModelModule::set_mesh_visibility(boma, Hash40::new("link_ken"), true);
                WorkModule::off_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_BOMB_FUSED);
            }
        }
        println!("Current Status Kind: {}", StatusModule::status_kind(boma));
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        link_start_initialization,
        link_reset_initialization,
        link_death_initialization,
        link_opff
    );
}