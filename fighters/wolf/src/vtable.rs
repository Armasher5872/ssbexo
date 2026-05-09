use super::*;

const WOLF_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xa617c0; //Shared
const WOLF_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0xa62210; //Shared
const WOLF_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xa62480; //Shared

//Wolf Reset Initialization
#[skyline::hook(offset = WOLF_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn wolf_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_WOLF as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_reset_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Wolf Death Initialization
#[skyline::hook(offset = WOLF_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn wolf_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_WOLF as u32 {
        let boma = fighter.battle_object.module_accessor;
        common_death_variable_reset(&mut *boma);
    }
    original!()(vtable, fighter)
}

//Wolf Once Per Fighter Frame
#[skyline::hook(offset = WOLF_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn wolf_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    if fighter.battle_object.kind == *FIGHTER_KIND_WOLF as u32 {
        let boma = fighter.battle_object.module_accessor;
        let status_kind = StatusModule::status_kind(boma);
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
            WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && LAST_ATTACK_HITBOX_ID == 0 {
                CancelModule::enable_cancel(boma);
            }
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        wolf_reset_initialization,
        wolf_death_initialization,
        wolf_opff
    );
}