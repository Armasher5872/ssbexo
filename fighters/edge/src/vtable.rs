use super::*;

const EDGE_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x9d9e10; //Sephiroth only
const EDGE_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x9da390; //Sephiroth only
const EDGE_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x9da970; //Sephiroth only
const EDGE_VTABLE_ONCE_PER_FIGHTER_FRAME: usize = 0x9db070; //Sephiroth only

unsafe extern "C" fn edge_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_RUSH_CANCEL);
    WorkModule::set_int(boma, 0, *FIGHTER_EDGE_INSTANCE_WORK_ID_INT_ONE_WINGED_SPECIAL_N_DIRECTION);
}

//Sephiroth Startup Initialization
#[skyline::hook(offset = EDGE_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn edge_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    edge_var(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Sephiroth Reset Initialization
#[skyline::hook(offset = EDGE_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn edge_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    edge_var(&mut *boma);
    original!()(vtable, fighter)
}

//Sephiroth Death Initialization
#[skyline::hook(offset = EDGE_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn edge_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    edge_var(&mut *boma);
    original!()(vtable, fighter)
}

//Sephiroth Once Per Fighter Frame
#[skyline::hook(offset = EDGE_VTABLE_ONCE_PER_FIGHTER_FRAME)]
unsafe extern "C" fn edge_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let frame = MotionModule::frame(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let status_kind = StatusModule::status_kind(boma);
    //Sephiroth Taunt Holding
    if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
        if [hash40("appeal_s_r"), hash40("appeal_s_l")].contains(&motion_kind)
        && frame >= 67.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) 
            || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R)
            || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                MotionModule::set_frame_sync_anim_cmd(boma, 67.0, true, true, false);
            }
        }
        if [hash40("appeal_lw_r"), hash40("appeal_lw_l")].contains(&motion_kind)
        && frame >= 80.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
            || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                MotionModule::set_frame_sync_anim_cmd(boma, 40.0, true, true, false);
            }
        }
    }
    //Up Special Early Cancel
    if status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            WorkModule::on_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_RUSH_CANCEL);
        }
    }
    if status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING
    && WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_RUSH_CANCEL) {
        if frame >= 9.0 {
            CancelModule::enable_cancel(boma);
        }
    }
    if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_ONE_WINGED_ACTIVATED) {
        if [*FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_LANDING, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END].contains(&status_kind) {
            if WorkModule::is_flag(boma, *FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_RUSH_CANCEL) {
                CancelModule::enable_cancel(boma);
            }
        }
        //Early Detonation
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if (7.0..27.0).contains(&frame)
            && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_HIT, false);
            }
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        edge_start_initialization,
        edge_reset_initialization,
        edge_death_initialization,
        edge_opff
    );
}