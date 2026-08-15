use super::*;

const MARIOD_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xcc8f20; //Dr. Mario only
const MARIOD_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xcc96c0; //Dr. Mario only

//Dr. Mario Reset Initialization
unsafe extern "C" fn mariod_reset_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    common_reset_variable_reset(&mut *boma);
    UiManager::set_mariod_meter_info(entry_id, 0);
    WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
}

//Dr. Mario Death Initialization
#[skyline::hook(offset = MARIOD_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn mariod_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    common_death_variable_reset(&mut *boma);
    UiManager::set_mariod_meter_info(entry_id, 0);
    WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    original!()(vtable, fighter)
}

//Dr. Mario Once Per Fighter Frame
#[skyline::hook(offset = MARIOD_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn mariod_opff(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    UiManager::set_mariod_meter_enable(entry_id, true);
}

//Dr. Mario On Attack
unsafe extern "C" fn mariod_on_attack(_vtable: u64, fighter: &mut Fighter, log: u64) {
    let boma = fighter.battle_object.module_accessor;
    let collision_log = log as *mut CollisionLogScuffed;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32;
    let status_kind = StatusModule::status_kind(boma);
    if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_LW3, *FIGHTER_STATUS_KIND_ATTACK_DASH].contains(&status_kind) {
        UiManager::set_mariod_meter_info(entry_id, 1);
        WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    }
    if [*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
        UiManager::set_mariod_meter_info(entry_id, 2);
        WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    }
    if [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_CATCH_ATTACK, *FIGHTER_STATUS_KIND_THROW].contains(&status_kind) {
        UiManager::set_mariod_meter_info(entry_id, 3);
        WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
        UiManager::set_mariod_meter_info(entry_id, 0);
        WorkModule::set_int(boma, UiManager::get_mariod_pill_id(entry_id), *FIGHTER_MARIOD_INSTANCE_WORK_ID_INT_PILL_ID);
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            println!("Is Side B");
            let opponent_object_id = (*collision_log).opponent_object_id;
            if opponent_object_id != *BATTLE_OBJECT_ID_INVALID as u32 {
                println!("Isn't an invalid id");
                let opponent_battle_object = get_battle_object_from_id(opponent_object_id);
                let opponent_battle_object_vtable: extern "C" fn(*mut BattleObject) -> bool = std::mem::transmute(**(opponent_battle_object as *const *const u64));
                if !opponent_battle_object_vtable(opponent_battle_object) && 3 < *(opponent_battle_object as *const u8).add(0x34) {
                    println!("Battle Object Methods Work");
                    let opponent_battle_object_id = (*opponent_battle_object).battle_object_id;
                    if opponent_battle_object_id >> 0x1C == 0 {
                        println!("Is a fighter");
                        let opponent_boma = (*opponent_battle_object).module_accessor;
                        StopModule::set_hit_stop_frame_fix(opponent_boma, 50);
                        WorkModule::set_int(opponent_boma, 50, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_SLOW_FRAME);
                        WorkModule::set_int(opponent_boma, 50, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_SLOW_MAG);
                    }
                }
            }
        }
    }
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x4fe4fa0).data(mariod_reset_initialization as *const () as u64);
    let _ = skyline::patching::Patch::in_text(0x4fe50a0).data(mariod_on_attack as *const () as u64);
    let _ = skyline::patching::Patch::in_text(0x34414dc).nop(); //The following removes the life initialization so it can be assigned dynamically in a weapon init status
    let _ = skyline::patching::Patch::in_text(0x34417a8).nop(); //The following removes the horizontal speed initialization so it can be assigned dynamically in a weapon init status
    let _ = skyline::patching::Patch::in_text(0x34417cc).nop(); //The following removes the initial gravity acceleration so it can be assigned dynamically in a weapon init status
    skyline::install_hooks!(
        mariod_death_initialization,
        mariod_opff
    );
}