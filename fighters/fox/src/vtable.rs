use super::*;

const FOX_WOLF_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0xa617c0; //Shared
const FOX_WOLF_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET: usize = 0xa62480; //Shared
const FOX_WOLF_VTABLE_ON_ATTACK_OFFSET: usize = 0xa64ce0; //Shared

//Fox & Wolf Reset Initialization
#[skyline::hook(offset = FOX_WOLF_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn fox_wolf_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let kind = fighter.battle_object.kind as i32;
    let boma = fighter.battle_object.module_accessor;
    if kind == *FIGHTER_KIND_FOX {
        common_reset_variable_reset(&mut *boma);
        WorkModule::off_flag(boma, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_HIT);
    }
    common_reset_variable_reset(&mut *boma);
    original!()(vtable, fighter)
}

//Fox & Wolf Death Initialization
unsafe extern "C" fn fox_wolf_death_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    WorkModule::set_float(boma, 0.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_POSTURE_LOG_POS_X_START);
    WorkModule::set_float(boma, 0.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_POSTURE_LOG_POS_X_0);
    WorkModule::set_float(boma, 0.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_POSTURE_LOG_POS_Y_START);
    WorkModule::set_float(boma, 0.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_POSTURE_LOG_POS_Y_0);
    WorkModule::set_float(boma, 0.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_POSTURE_LOG_POS_Z_START);
    WorkModule::set_float(boma, 0.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_POSTURE_LOG_POS_Z_0);
    WorkModule::set_float(boma, 0.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_POSTURE_LOG_ROT_X_START);
    WorkModule::set_float(boma, 0.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_POSTURE_LOG_ROT_X_0);
    WorkModule::set_float(boma, 0.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_POSTURE_LOG_POS_X_1);
    WorkModule::set_float(boma, 0.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_POSTURE_LOG_POS_Y_1);
    WorkModule::set_float(boma, 0.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_POSTURE_LOG_POS_Z_1);
    WorkModule::set_float(boma, 0.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_POSTURE_LOG_ROT_X_1);
    WorkModule::set_float(boma, 0.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_POSTURE_LOG_POS_X_2);
    WorkModule::set_float(boma, 0.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_POSTURE_LOG_POS_Y_2);
    WorkModule::set_float(boma, 0.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_POSTURE_LOG_POS_Z_2);
    WorkModule::set_float(boma, 0.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_POSTURE_LOG_ROT_X_2);
    WorkModule::set_float(boma, 0.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_POSTURE_LOG_POS_X_3);
    WorkModule::set_float(boma, 0.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_POSTURE_LOG_POS_Y_3);
    WorkModule::set_float(boma, 0.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_POSTURE_LOG_POS_Z_3);
    WorkModule::set_float(boma, 0.0, *FIGHTER_FOX_INSTANCE_WORK_ID_FLOAT_POSTURE_LOG_ROT_X_3);
    WorkModule::off_flag(boma, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR);
    WorkModule::off_flag(boma, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_BREAK);
    WorkModule::off_flag(boma, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_ILLUSION_LANDING);
    WorkModule::off_flag(boma, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_LANDING);
    WorkModule::off_flag(boma, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_HIT);
}

//Fox Wolf Once Per Fighter Frame
#[skyline::hook(offset = FOX_WOLF_VTABLE_ONCE_PER_FIGHTER_FRAME_OFFSET)]
unsafe extern "C" fn fox_wolf_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
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

//Fox & Wolf On Attack
#[skyline::hook(offset = FOX_WOLF_VTABLE_ON_ATTACK_OFFSET)]
unsafe extern "C" fn fox_wolf_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) -> u64 {
    let kind = fighter.battle_object.kind as i32;
    let boma = fighter.battle_object.module_accessor;
    if kind == *FIGHTER_KIND_FOX {
        let status_kind = StatusModule::status_kind(boma);
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            WorkModule::on_flag(boma, *FIGHTER_FOX_INSTANCE_WORK_ID_FLAG_REFLECTOR_HIT);
        }
    }
    original!()(vtable, fighter, log)
}

pub fn install() {
    let _ = skyline::patching::Patch::in_text(0x4fb6740).data(fox_wolf_death_initialization as *const () as u64);
    skyline::install_hooks!(
        fox_wolf_reset_initialization,
        fox_wolf_opff,
        fox_wolf_on_attack
    );
}