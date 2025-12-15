use super::*;

const LINK_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xc280f0; //Shared
const LINK_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xc28280; //Shared
const LINK_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xc28860; //Shared
const LINK_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xc289e0; //Shared

unsafe extern "C" fn link_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_URBOSA_FURY);
    WorkModule::off_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPIN_ATTACK_CAN_FALL);
    WorkModule::off_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_CHARGE);
    WorkModule::off_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_INIT_FUSE);
    WorkModule::off_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_CAN_ASCEND);
    WorkModule::off_flag(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_NO_GAIN);
    WorkModule::set_float(boma, 0.0, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_SPECIAL_N_DEGREE);
    WorkModule::set_float(boma, 0.0, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_START_Y);
    WorkModule::set_float(boma, 0.0, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_ASCEND_TARGET_Y);
    WorkModule::set_float(boma, 0.0, *FIGHTER_LINK_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_DEGREE);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID);
    WorkModule::set_int(boma, TeamModule::team_no(boma) as i32, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_TEAM_NO);
    WorkModule::set_int(boma, *ITEM_KIND_NONE, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ARROW_FUSE);
    WorkModule::set_int(boma, 0, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_ASCEND_FRAME);
    WorkModule::set_int(boma, *ITEM_KIND_NONE, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE);
    WorkModule::set_int(boma, *BATTLE_OBJECT_ID_INVALID, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_CURRENT_BOOMERANG_FUSE_ID);
    WorkModule::set_int(boma, 300, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_STAMINA);
}

unsafe extern "C" fn link_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::set_int(fighter.module_accessor, 300, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_STAMINA);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_NO_GAIN);
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
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
        common_death_variable_reset(&mut *boma);
        link_var(&mut *boma);
        UiManager::set_link_wheel_info(entry_id, 0);
    }
    original!()(vtable, fighter)
}

//Link Once Per Fighter Frame
#[skyline::hook(offset = LINK_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn link_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_LINK as u32 {
        let boma = fighter.battle_object.module_accessor;
        let agent = get_fighter_common_from_accessor(&mut *boma);
        let status_kind = agent.global_table[STATUS_KIND].get_i32();
        let frame = agent.global_table[CURRENT_FRAME].get_f32();
        let motion_kind = MotionModule::motion_kind(boma);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
        let stamina = WorkModule::get_int(boma, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_STAMINA);
        let wheel_value = if stamina >= 283 {0} else if stamina >= 264 {1} else if stamina >= 246 {2} else if stamina >= 228 {3} else if stamina >= 210 {4} else if stamina >= 192 {5} else if stamina >= 174 {6} else if stamina >= 156 {7} 
        else if stamina >= 138 {8} else if stamina >= 120 {9} else if stamina >= 102 {10} else if stamina >= 84 {11} else if stamina >= 66 {12} else if stamina >= 48 {13} else if stamina >= 30 {14} else if stamina >= 1 {15} else {16};
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            if [hash40("appeal_hi_r"), hash40("appeal_hi_l")].contains(&motion_kind) && (80.0..=95.0).contains(&frame) {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_LINK_STATUS_KIND_MORTAL_DRAW_LOOP, false);
                }
            }
        }
        if [*FIGHTER_STATUS_KIND_ATTACH_WALL, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_GLIDE].contains(&status_kind) {
            UiManager::set_link_wheel_enable(entry_id, true);
            UiManager::set_link_wheel_info(entry_id, wheel_value);
        }
        else {
            UiManager::set_link_wheel_enable(entry_id, false);
            UiManager::set_link_wheel_info(entry_id, 0);
        }
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