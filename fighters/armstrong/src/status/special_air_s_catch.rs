use super::*;

unsafe extern "C" fn armstrong_special_air_s_catch_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::set_rate(fighter.module_accessor, 0.0);
    WorkModule::set_float(fighter.module_accessor, 1.5, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_EXPLOSION_AIR_SPEED_X);
    WorkModule::set_float(fighter.module_accessor, 2.0, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_EXPLOSION_AIR_SPEED_Y);
    fighter.sub_shift_status_main(L2CValue::Ptr(armstrong_special_air_s_catch_main_loop as *const () as _))
}

unsafe extern "C" fn armstrong_special_air_s_catch_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let explosion_air_speed_x = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_EXPLOSION_AIR_SPEED_X);
    let explosion_air_speed_y = WorkModule::get_float(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_FLOAT_EXPLOSION_AIR_SPEED_Y);
    let lr = PostureModule::lr(fighter.module_accessor);
    if current_frame == 1.0 {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_catch"), 0.0, 1.0, false, 0.0, false, false);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, explosion_air_speed_y);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, explosion_air_speed_x*lr, 0.0);
    }
    if current_frame > 1.0 {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL.into(), false.into());
        }
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END.into(), false.into());
        }
        return 1.into();
    }
    0.into()
}

pub fn install() {
    Agent::new("ganon")
    .set_costume([8, 9, 10, 11, 12, 13, 14, 15].to_vec())
    .status(Main, *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_CATCH, armstrong_special_air_s_catch_main_status)
    .install()
    ;
}