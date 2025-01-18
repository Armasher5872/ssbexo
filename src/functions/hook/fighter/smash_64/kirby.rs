use super::*;

const KIRBY_VTABLE_START_INITIALIZATION_OFFSET: usize = 0xb96100; //Kirby only
const KIRBY_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xb96350; //Kirby only
const KIRBY_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xb96f70; //Kirby only
const KIRBY_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xb97b30; //Kirby only

unsafe extern "C" fn kirby_var(boma: &mut BattleObjectModuleAccessor) {
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let team_no = TeamModule::team_no(boma) as i32;
    ModelModule::set_mesh_visibility(boma, Hash40::new("kirby_armfoot"), true);
    ModelModule::set_mesh_visibility(boma, Hash40::new("kirby_eye1"), true);
    ModelModule::set_mesh_visibility(boma, Hash40::new("kirby_facen"), true);
    WorkModule::off_flag(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_WHEEL_RECOIL);
    WorkModule::set_float(boma, 0.0, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLOAT_WHEEL_POWER_UP);
    WHEEL_SPEED_UP[entry_id] = 0.0;
    WorkModule::set_int(boma, 0, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_HOLD_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_JUMP_COUNT);
    WorkModule::set_int(boma, 0, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_TURN_COUNT);
    WorkModule::set_int(boma, team_no, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_TEAM_NO);
}

//Kirby Startup Initialization
#[skyline::hook(offset = KIRBY_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn kirby_start_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    kirby_var(&mut *boma);
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(common_end_control as *const () as _));
    original!()(vtable, fighter)
}

//Kirby Reset Initialization
#[skyline::hook(offset = KIRBY_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn kirby_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    kirby_var(&mut *boma);
    original!()(vtable, fighter)
}

//Kirby Death Initialization
#[skyline::hook(offset = KIRBY_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn kirby_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    kirby_var(&mut *boma);
    original!()(vtable, fighter)
}

//Kirby Once Per Fighter Frame
#[skyline::hook(offset = KIRBY_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn kirby_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let situation_kind = StatusModule::situation_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let frame = MotionModule::frame(boma);
    let copy_chara = WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA);
    let kirby_falcon_punch_turn_count = WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_FALCON_PUNCH_TURN_COUNT);
    //Kirby Stuff
    if ![*FIGHTER_STATUS_KIND_ATTACK_HI4_START, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4].contains(&status_kind) {
        ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_HAMMER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    if ![
        *FIGHTER_STATUS_KIND_ATTACK_S4_START, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_WAIT, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_STATUS_KIND_SPECIAL_HI, 
        *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI3, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI4
    ].contains(&status_kind) {
        ArticleModule::remove_exist(boma, *FIGHTER_KIRBY_GENERATE_ARTICLE_FINALCUTTER, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    if ![*FIGHTER_STATUS_KIND_ATTACK_100, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK].contains(&status_kind) {
        SoundModule::stop_se(boma, Hash40::new("se_kirby_attack100"), 0);
    }
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_LOOP {
        let obj_id = WorkModule::get_int(boma, *FIGHTER_KIRBY_STATUS_SPECIAL_N_WORK_INT_INHALE_OBJECT_ID) as u32;
        let obj_boma = smash::app::sv_battle_object::module_accessor(obj_id);
        let obj_kind = smash::app::utility::get_kind(&mut *obj_boma);
        let item_id = if obj_kind == *WEAPON_KIND_LINK_BOWARROW {
            WorkModule::get_int64(obj_boma, *WN_LINK_BOWARROW_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32
        }
        else if obj_kind == *WEAPON_KIND_LINK_BOOMERANG {
            WorkModule::get_int64(obj_boma, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_INT_FUSE_ITEM_ID) as u32
        }
        else {
            *BATTLE_OBJECT_ID_INVALID as u32
        };
        let item_manager = *(singletons::ItemManager() as *mut *mut smash::app::ItemManager);
        smash::app::lua_bind::ItemManager::remove_item_from_id(item_manager, item_id);
    }
    if ![*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_TURN, *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK].contains(&status_kind) {
        WorkModule::set_int(boma, 0, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_WHEEL_TURN_COUNT);
        WorkModule::off_flag(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_WHEEL_RECOIL);
    }
    //Dark Samus
    if copy_chara == *FIGHTER_KIND_SAMUSD
    && status_kind == *FIGHTER_KIRBY_STATUS_KIND_SAMUSD_SPECIAL_N
    && motion_kind == hash40("damage_n_2") {
        if StatusModule::is_situation_changed(boma) {
            if situation_kind != SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
            }
            else {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
            }
        }
        if MotionModule::is_end(boma) {
            if situation_kind != *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_WAIT, true);
            }
            else {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
            }
        }
    }
    //Captain Falcon
    if copy_chara == *FIGHTER_KIND_CAPTAIN {
        if [23, 45, 67, 89, 114, 133, 152, 171, 194, 217, 240, 263, 286].contains(&fighter.battle_object.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N, true);
        };
        if [*FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN].contains(&status_kind) {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                WorkModule::on_flag(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT);
            };
            if WorkModule::is_flag(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT)
            && (54.0..57.0).contains(&frame) {
                SoundModule::play_se(boma, Hash40::new("vc_kirby_cheer"), true, false, false, false, enSEType(0));
            }
        };
        if status_kind == *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N
        && WorkModule::is_flag(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT)
        && frame > 70.0 {
            CancelModule::enable_cancel(boma);
        }
        if status_kind == *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN
        && WorkModule::is_flag(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT)
        && frame > 104.0 {
            CancelModule::enable_cancel(boma);
        }
        if ![*FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N, *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN].contains(&status_kind) {
            WorkModule::off_flag(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_FALCON_PUNCH_HIT);
        }
        if status_kind == *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN 
        && (25.0..40.0).contains(&frame)
        && (ControlModule::get_stick_x(boma)*PostureModule::lr(boma)) < -0.5
        && kirby_falcon_punch_turn_count <= 15 {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN, true);
        };
        if status_kind != *FIGHTER_KIRBY_STATUS_KIND_CAPTAIN_SPECIAL_N_TURN {
            WorkModule::set_int(boma, 0, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_FALCON_PUNCH_TURN_COUNT);
        }
    }
    //Ryu
    if copy_chara == *FIGHTER_KIND_RYU {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
        if [8, 30, 52, 74, 156, 179, 202, 225, 248, 271].contains(&fighter.battle_object.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N2_COMMAND, false);
        }
        if [9, 31, 53, 75, 157, 180, 203, 226, 249, 272].contains(&fighter.battle_object.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND, false);
        }
        if [23, 45, 67, 89, 171, 194, 217, 240, 263, 286].contains(&fighter.battle_object.magic_series()) {
            if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, false);
            }
            else {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, false);
            }
        }
        if [24, 46, 68, 90, 172, 195, 218, 241, 264, 287].contains(&fighter.battle_object.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
        }
        if [25, 47, 69, 91, 173, 196, 219, 242, 265, 288].contains(&fighter.battle_object.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
        }
        if [26, 48, 70, 92, 174, 197, 220, 243, 266, 289].contains(&fighter.battle_object.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
        }
    }
    //Ken
    if copy_chara == *FIGHTER_KIND_KEN {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
        if [9, 31, 53, 75, 157, 180, 203, 226, 249, 272].contains(&fighter.battle_object.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND, false);
        }
        if [23, 45, 67, 89, 171, 194, 217, 240, 263, 286].contains(&fighter.battle_object.magic_series()) {
            if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, false);
            }
            else {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, false);
            }
        }
        if [24, 46, 68, 90, 172, 195, 218, 241, 264, 287].contains(&fighter.battle_object.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
        }
        if [25, 47, 69, 91, 173, 196, 219, 242, 265, 288].contains(&fighter.battle_object.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
        }
        if [26, 48, 70, 92, 174, 197, 220, 243, 266, 289].contains(&fighter.battle_object.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
        }
    }
    //Terry
    if copy_chara == *FIGHTER_KIND_DOLLY {
        if [23, 45, 67, 89, 194, 263, 286].contains(&fighter.battle_object.magic_series()) {
            if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FINAL, false);
            }
            else {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, false);
            }
        }
        if [24, 46, 68, 90, 195, 264, 287].contains(&fighter.battle_object.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
        }
        if [25, 47, 69, 91, 196, 265, 288].contains(&fighter.battle_object.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
        }
        if [26, 48, 70, 92, 197, 266, 289].contains(&fighter.battle_object.magic_series()) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
    skyline::install_hooks!(
        kirby_start_initialization,
        kirby_reset_initialization,
        kirby_death_initialization,
        kirby_opff
    );
}