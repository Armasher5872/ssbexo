//Credit to AParticularUser for Mortal Draw code
use super::*;

unsafe extern "C" fn link_mortal_draw_attack_pre_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn link_mortal_draw_attack_init_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn link_mortal_draw_attack_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("mortal_draw_attack"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(link_mortal_draw_attack_main_loop as *const () as _))
}

unsafe extern "C" fn link_mortal_draw_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if situation_kind != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn link_mortal_draw_attack_exec_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn link_mortal_draw_attack_check_attack_status(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    let opponent_id = get_table_value(table, "object_id_").try_integer().unwrap() as u32;
    let opponent_boma = sv_battle_object::module_accessor(opponent_id);
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER && collision_kind == *COLLISION_KIND_HIT {
        let opponent_pos = PostureModule::pos_2d(opponent_boma);
        if DamageModule::damage(opponent_boma, 0) >= 100.0 {
            FighterUtil::request_critical_hit_cut_in_force(fighter.module_accessor, opponent_id, &Vector2f{x: opponent_pos.x, y: opponent_pos.y}, -1, Hash40::new("param_critical"), 0, true, 0, true);
            StatusModule::change_status_request_from_script(opponent_boma, *FIGHTER_STATUS_KIND_DEAD, false);
        }
    }
    0.into()
}

unsafe extern "C" fn link_mortal_draw_attack_end_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn link_mortal_draw_attack_exit_status(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("link")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .status(Pre, *FIGHTER_LINK_STATUS_KIND_MORTAL_DRAW_ATTACK, link_mortal_draw_attack_pre_status)
    .status(Init, *FIGHTER_LINK_STATUS_KIND_MORTAL_DRAW_ATTACK, link_mortal_draw_attack_init_status)
    .status(Main, *FIGHTER_LINK_STATUS_KIND_MORTAL_DRAW_ATTACK, link_mortal_draw_attack_main_status)
    .status(Exec, *FIGHTER_LINK_STATUS_KIND_MORTAL_DRAW_ATTACK, link_mortal_draw_attack_exec_status)
    .status(CheckAttack, *FIGHTER_LINK_STATUS_KIND_MORTAL_DRAW_ATTACK, link_mortal_draw_attack_check_attack_status)
    .status(End, *FIGHTER_LINK_STATUS_KIND_MORTAL_DRAW_ATTACK, link_mortal_draw_attack_end_status)
    .status(Exit, *FIGHTER_LINK_STATUS_KIND_MORTAL_DRAW_ATTACK, link_mortal_draw_attack_exit_status)
    .install()
    ;
}