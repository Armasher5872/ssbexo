use super::*;

unsafe extern "C" fn sheik_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let vanish_timer = WorkModule::get_int(boma, FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_VANISH_TIMER);
    let sheikah_eye_timer = WorkModule::get_int(boma, FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_SHEIKAH_EYE_TIMER);
    let transition_terms = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_HOLD, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_HOLD, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_HOLD, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_STAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_HEAVY_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_PICKUP_LIGHT_DASH, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N, 
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S
    ];
    let transition_group_check = [
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK, 
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL
    ];
    let opponent_boma_1 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
    let opponent_boma_2 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(2));
    let opponent_boma_3 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(3));
    let opponent_boma_4 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(4));
    let opponent_boma_5 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(5));
    let opponent_boma_6 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(6));
    let opponent_boma_7 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(7));
    let pos_x = PostureModule::pos_x(boma);
    let pos_y = PostureModule::pos_y(boma);
    let pos_x_1 = PostureModule::pos_x(opponent_boma_1);
    let pos_y_1 = PostureModule::pos_y(opponent_boma_1);
    let pos_x_2 = PostureModule::pos_x(opponent_boma_2);
    let pos_y_2 = PostureModule::pos_y(opponent_boma_2);
    let pos_x_3 = PostureModule::pos_x(opponent_boma_3);
    let pos_y_3 = PostureModule::pos_y(opponent_boma_3);
    let pos_x_4 = PostureModule::pos_x(opponent_boma_4);
    let pos_y_4 = PostureModule::pos_y(opponent_boma_4);
    let pos_x_5 = PostureModule::pos_x(opponent_boma_5);
    let pos_y_5 = PostureModule::pos_y(opponent_boma_5);
    let pos_x_6 = PostureModule::pos_x(opponent_boma_6);
    let pos_y_6 = PostureModule::pos_y(opponent_boma_6);
    let pos_x_7 = PostureModule::pos_x(opponent_boma_7);
    let pos_y_7 = PostureModule::pos_y(opponent_boma_7);
    let alpha_1 = ((1.0/(((pos_x-pos_x_1).powf(2.0)+(pos_y-pos_y_1).powf(2.0)).sqrt()))*9.0).clamp(0.05, 1.0);
    let alpha_2 = ((1.0/(((pos_x-pos_x_2).powf(2.0)+(pos_y-pos_y_2).powf(2.0)).sqrt()))*9.0).clamp(0.05, 1.0);
    let alpha_3 = ((1.0/(((pos_x-pos_x_3).powf(2.0)+(pos_y-pos_y_3).powf(2.0)).sqrt()))*9.0).clamp(0.05, 1.0);
    let alpha_4 = ((1.0/(((pos_x-pos_x_4).powf(2.0)+(pos_y-pos_y_4).powf(2.0)).sqrt()))*9.0).clamp(0.05, 1.0);
    let alpha_5 = ((1.0/(((pos_x-pos_x_5).powf(2.0)+(pos_y-pos_y_5).powf(2.0)).sqrt()))*9.0).clamp(0.05, 1.0);
    let alpha_6 = ((1.0/(((pos_x-pos_x_6).powf(2.0)+(pos_y-pos_y_6).powf(2.0)).sqrt()))*9.0).clamp(0.05, 1.0);
    let alpha_7 = ((1.0/(((pos_x-pos_x_7).powf(2.0)+(pos_y-pos_y_7).powf(2.0)).sqrt()))*9.0).clamp(0.05, 1.0);
    if WorkModule::is_flag(boma, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW)
    && WorkModule::is_flag(boma, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::set_flag(boma, true, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
        WorkModule::off_flag(boma, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW);
    }
    if WorkModule::is_flag(boma, FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HAS_VANISHED) {
        if vanish_timer > 0 {
            WorkModule::dec_int(boma, FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_VANISH_TIMER);
            for x in 0..transition_terms.len() {
                WorkModule::unable_transition_term(boma, transition_terms[x]);
                WorkModule::enable_transition_term_forbid(boma, transition_terms[x]);
            }
            for x in 0..transition_group_check.len() {
                WorkModule::unable_transition_term_group(boma, transition_group_check[x]);
            }
            ModelModule::set_mesh_visibility(boma, Hash40::new("gamemodel"), false);
            ModelModule::set_mesh_visibility(boma, Hash40::new("hair"), false);
            ModelModule::set_mesh_visibility(boma, Hash40::new("sheik_blink"), false);
            ModelModule::set_mesh_visibility(boma, Hash40::new("sheik_eye"), false);
            ModelModule::set_mesh_visibility(boma, Hash40::new("sheik_facen"), false);
            ModelModule::set_mesh_visibility(boma, Hash40::new("sheik_final"), false);
            ModelModule::set_mesh_visibility(boma, Hash40::new("sheik_furafura"), false);
            ModelModule::set_mesh_visibility(boma, Hash40::new("sheik_halfblink1"), false);
            ModelModule::set_mesh_visibility(boma, Hash40::new("sheik_halfblink2"), false);
            ModelModule::set_mesh_visibility(boma, Hash40::new("sheik_heavyattack"), false);
            ModelModule::set_mesh_visibility(boma, Hash40::new("sheik_openblink"), false);
            ModelModule::set_mesh_visibility(boma, Hash40::new("sheik_ouch"), false);
            ModelModule::set_mesh_visibility(boma, Hash40::new("sheik_talk"), false);
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
            WorkModule::inc_int(boma, FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_SHEIKAH_EYE_TIMER);
            if sheikah_eye_timer == 10 {
                WorkModule::set_int(boma, 0, FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_SHEIKAH_EYE_TIMER);
                EffectModule::kill_kind(opponent_boma_1, Hash40::new("sys_aura_light"), false, false);
                EffectModule::kill_kind(opponent_boma_2, Hash40::new("sys_aura_light"), false, false);
                EffectModule::kill_kind(opponent_boma_3, Hash40::new("sys_aura_light"), false, false);
                EffectModule::kill_kind(opponent_boma_4, Hash40::new("sys_aura_light"), false, false);
                EffectModule::kill_kind(opponent_boma_5, Hash40::new("sys_aura_light"), false, false);
                EffectModule::kill_kind(opponent_boma_6, Hash40::new("sys_aura_light"), false, false);
                EffectModule::kill_kind(opponent_boma_7, Hash40::new("sys_aura_light"), false, false);
                let effect_1 = EffectModule::req_follow(opponent_boma_1, Hash40::new("sys_aura_light"), Hash40::new("top"), &Vector3f{x: 0.0, y: 12.0, z: 0.0}, &NONE_VECTOR, 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                let effect_2 = EffectModule::req_follow(opponent_boma_2, Hash40::new("sys_aura_light"), Hash40::new("top"), &Vector3f{x: 0.0, y: 12.0, z: 0.0}, &NONE_VECTOR, 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                let effect_3 = EffectModule::req_follow(opponent_boma_3, Hash40::new("sys_aura_light"), Hash40::new("top"), &Vector3f{x: 0.0, y: 12.0, z: 0.0}, &NONE_VECTOR, 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                let effect_4 = EffectModule::req_follow(opponent_boma_4, Hash40::new("sys_aura_light"), Hash40::new("top"), &Vector3f{x: 0.0, y: 12.0, z: 0.0}, &NONE_VECTOR, 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                let effect_5 = EffectModule::req_follow(opponent_boma_5, Hash40::new("sys_aura_light"), Hash40::new("top"), &Vector3f{x: 0.0, y: 12.0, z: 0.0}, &NONE_VECTOR, 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                let effect_6 = EffectModule::req_follow(opponent_boma_6, Hash40::new("sys_aura_light"), Hash40::new("top"), &Vector3f{x: 0.0, y: 12.0, z: 0.0}, &NONE_VECTOR, 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                let effect_7 = EffectModule::req_follow(opponent_boma_7, Hash40::new("sys_aura_light"), Hash40::new("top"), &Vector3f{x: 0.0, y: 12.0, z: 0.0}, &NONE_VECTOR, 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                EffectModule::set_alpha(opponent_boma_1, effect_1, alpha_1);
                EffectModule::set_alpha(opponent_boma_2, effect_2, alpha_2);
                EffectModule::set_alpha(opponent_boma_3, effect_3, alpha_3);
                EffectModule::set_alpha(opponent_boma_4, effect_4, alpha_4);
                EffectModule::set_alpha(opponent_boma_5, effect_5, alpha_5);
                EffectModule::set_alpha(opponent_boma_6, effect_6, alpha_6);
                EffectModule::set_alpha(opponent_boma_7, effect_7, alpha_7);
            }
        }
        if vanish_timer <= 0 {
            WorkModule::set_int(boma, 0, FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_SHEIKAH_EYE_TIMER);
            WorkModule::set_flag(boma, true, FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_VANISH_ATTACK);
            EffectModule::kill_kind(opponent_boma_1, Hash40::new("sys_aura_light"), false, false);
            EffectModule::kill_kind(opponent_boma_2, Hash40::new("sys_aura_light"), false, false);
            EffectModule::kill_kind(opponent_boma_3, Hash40::new("sys_aura_light"), false, false);
            EffectModule::kill_kind(opponent_boma_4, Hash40::new("sys_aura_light"), false, false);
            EffectModule::kill_kind(opponent_boma_5, Hash40::new("sys_aura_light"), false, false);
            EffectModule::kill_kind(opponent_boma_6, Hash40::new("sys_aura_light"), false, false);
            EffectModule::kill_kind(opponent_boma_7, Hash40::new("sys_aura_light"), false, false);
        }
        if WorkModule::is_flag(boma, FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_VANISH_ATTACK)
        && fighter.global_table[STATUS_KIND].get_i32() != FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_VANISH_CANCEL {
            StatusModule::change_status_request_from_script(boma, FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_VANISH_ATTACK, false);
        }
    }
}

unsafe extern "C" fn sheik_init(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    //Universal
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALL_LAST_STOCK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ALREADY_BOUNCED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ASDI_START);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_CONTINUE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_FALL);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_AIR_LANDING);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_ENABLE_GRAVITY);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_DASH_GRAVITY_ENABLED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_AUTO_COUNTER);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_B_CHECK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_CAN_ADD);
    COUNTERHIT_CHECK[entry_id] = false;
    COUNTERHIT_SUCCESS[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_DID_MAX_JUMP_COUNT);
    FIGHTER_BOOL_1[entry_id] = false;
    FIGHTER_BOOL_2[entry_id] = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIGHTER_SPECIAL_STATE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FIRST_BOUNCE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HAS_CATCH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HITFLOW);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_MOVE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_IS_CC);
    WorkModule::set_flag(boma, smash::app::sv_information::is_ready_go(), FIGHTER_INSTANCE_WORK_ID_FLAG_READY_GO);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_SPECIAL);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_N_DISABLE);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_SPECIAL_S_DISABLE);
    SPECIAL_WALL_JUMP = false;
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_WAVEDASH);
    WorkModule::set_flag(boma, false, FIGHTER_INSTANCE_WORK_ID_FLAG_USED_FS);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_ATTACK_DASH_FALL_SPEED_Y_MUL);
    WorkModule::set_float(boma, 0.0, FIGHTER_INSTANCE_WORK_ID_FLOAT_DASH_GRAB_SPEED);
    LAST_DAMAGE[entry_id] = 0.0;
    SIZE0[entry_id] = 0.0;
    SIZE1[entry_id] = 0.0;
    SIZE2[entry_id] = 0.0;
    SIZE3[entry_id] = 0.0;
    FULL_HOP_ENABLE_DELAY[entry_id] = 0;
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_GOT_HIT);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_MASHING);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRIED);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_PARRY_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_BREAK_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SHIELD_DAMAGE);
    WorkModule::set_int(boma, 0, FIGHTER_INSTANCE_WORK_ID_INT_SPECIAL_ZOOM_GFX);
    //Sheik
    WorkModule::set_flag(boma, false, FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HAS_VANISHED);
    WorkModule::set_flag(boma, false, FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_VANISH_ATTACK);
    WorkModule::set_flag(boma, false, FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BACK_HIT);
    WorkModule::set_int(boma, 0, FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_VANISH_TIMER);
    WorkModule::set_int(boma, 0, FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_SHEIKAH_EYE_TIMER);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("gamemodel"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("hair"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sheik_eye"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sheik_facen"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sheik_openblink"), true);
}

pub fn install() {
    Agent::new("sheik")
    .on_start(sheik_init)
    .on_line(Main, sheik_frame)
    .install()
    ;
}