use super::*;

const SHEIK_VTABLE_START_INITIALIZATION_OFFSET: usize = 0x1120c60; //Sheik only
const SHEIK_VTABLE_RESET_INITIALIZATION_OFFSET: usize = 0x1120910; //Sheik only
const SHEIK_VTABLE_DEATH_INITIALIZATION_OFFSET: usize = 0x1120c70; //Sheik only
const SHEIK_VTABLE_ONCE_PER_FIGHTER_FRAME: usize = 0x1120e10; //Sheik only

unsafe extern "C" fn sheik_var(boma: &mut BattleObjectModuleAccessor) {
    WorkModule::off_flag(boma, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HAS_VANISHED);
    WorkModule::off_flag(boma, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_VANISH_ATTACK);
    WorkModule::off_flag(boma, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BACK_HIT);
    WorkModule::set_int(boma, 0, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_VANISH_TIMER);
    WorkModule::set_int(boma, 0, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_SHEIKAH_EYE_TIMER);
    ModelModule::set_mesh_visibility(boma, Hash40::new("gamemodel"), true);
    ModelModule::set_mesh_visibility(boma, Hash40::new("hair"), true);
    ModelModule::set_mesh_visibility(boma, Hash40::new("sheik_eye"), true);
    ModelModule::set_mesh_visibility(boma, Hash40::new("sheik_facen"), true);
    ModelModule::set_mesh_visibility(boma, Hash40::new("sheik_openblink"), true);
    CameraModule::set_status(boma, CameraStatus{ _address: *CAMERA_STATUS_NORMAL as u8 }, 0);
}

unsafe extern "C" fn sheik_end_control(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR || WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BOUNCE);
    }
    0.into()
}

unsafe extern "C" fn sheik_should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW) {
        return 0.into();
    }
    1.into()
}

//Sheik Startup Initialization
#[skyline::hook(offset = SHEIK_VTABLE_START_INITIALIZATION_OFFSET)]
unsafe extern "C" fn sheik_start_initialization(_vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let agent = get_fighter_common_from_accessor(&mut *boma);
    common_initialization_variable_reset(&mut *boma);
    sheik_var(&mut *boma);
    agent.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(sheik_should_use_special_s_callback as *const () as _));
    agent.global_table[STATUS_END_CONTROL].assign(&L2CValue::Ptr(sheik_end_control as *const () as _));
}

//Sheik Reset Initialization
#[skyline::hook(offset = SHEIK_VTABLE_RESET_INITIALIZATION_OFFSET)]
unsafe extern "C" fn sheik_reset_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_reset_variable_reset(&mut *boma);
    sheik_var(&mut *boma);
    original!()(vtable, fighter)
}

//Sheik Death Initialization
#[skyline::hook(offset = SHEIK_VTABLE_DEATH_INITIALIZATION_OFFSET)]
unsafe extern "C" fn sheik_death_initialization(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    common_death_variable_reset(&mut *boma);
    sheik_var(&mut *boma);
    original!()(vtable, fighter)
}

//Sheik Once Per Fighter Frame
#[skyline::hook(offset = SHEIK_VTABLE_ONCE_PER_FIGHTER_FRAME)]
unsafe extern "C" fn sheik_opff(vtable: u64, fighter: &mut Fighter) -> u64 {
    let boma = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let vanish_timer = WorkModule::get_int(boma, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_VANISH_TIMER);
    let sheikah_eye_timer = WorkModule::get_int(boma, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_SHEIKAH_EYE_TIMER);
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
    if WorkModule::is_flag(boma, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW) && WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGED_PREVENT);
        WorkModule::off_flag(boma, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW);
    }
    if WorkModule::is_flag(boma, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HAS_VANISHED) {
        if vanish_timer > 0 {
            WorkModule::dec_int(boma, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_VANISH_TIMER);
            for x in 0..transition_terms.len() {
                WorkModule::unable_transition_term(boma, transition_terms[x]);
                WorkModule::enable_transition_term_forbid(boma, transition_terms[x]);
            }
            for x in 0..transition_group_check.len() {
                WorkModule::unable_transition_term_group(boma, transition_group_check[x]);
            }
            AreaModule::set_whole(boma, false);
            CameraModule::set_status(boma, CameraStatus{ _address: *CAMERA_STATUS_SLEEP as u8 }, 0);
            JostleModule::set_status(boma, false);
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
            WorkModule::inc_int(boma, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_SHEIKAH_EYE_TIMER);
            if sheikah_eye_timer == 10 {
                WorkModule::set_int(boma, 0, *FIGHTER_SHEIK_INSTANCE_WORK_ID_INT_SHEIKAH_EYE_TIMER);
                EffectModule::kill_kind(opponent_boma_1, Hash40::new("sys_aura_light"), false, false);
                EffectModule::kill_kind(opponent_boma_2, Hash40::new("sys_aura_light"), false, false);
                EffectModule::kill_kind(opponent_boma_3, Hash40::new("sys_aura_light"), false, false);
                EffectModule::kill_kind(opponent_boma_4, Hash40::new("sys_aura_light"), false, false);
                EffectModule::kill_kind(opponent_boma_5, Hash40::new("sys_aura_light"), false, false);
                EffectModule::kill_kind(opponent_boma_6, Hash40::new("sys_aura_light"), false, false);
                EffectModule::kill_kind(opponent_boma_7, Hash40::new("sys_aura_light"), false, false);
                let effect_1 = EffectModule::req_follow(opponent_boma_1, Hash40::new("sys_aura_light"), Hash40::new("top"), &Vector3f{x: 0.0, y: 12.0, z: 0.0}, &Vector3f::zero(), 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                let effect_2 = EffectModule::req_follow(opponent_boma_2, Hash40::new("sys_aura_light"), Hash40::new("top"), &Vector3f{x: 0.0, y: 12.0, z: 0.0}, &Vector3f::zero(), 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                let effect_3 = EffectModule::req_follow(opponent_boma_3, Hash40::new("sys_aura_light"), Hash40::new("top"), &Vector3f{x: 0.0, y: 12.0, z: 0.0}, &Vector3f::zero(), 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                let effect_4 = EffectModule::req_follow(opponent_boma_4, Hash40::new("sys_aura_light"), Hash40::new("top"), &Vector3f{x: 0.0, y: 12.0, z: 0.0}, &Vector3f::zero(), 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                let effect_5 = EffectModule::req_follow(opponent_boma_5, Hash40::new("sys_aura_light"), Hash40::new("top"), &Vector3f{x: 0.0, y: 12.0, z: 0.0}, &Vector3f::zero(), 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                let effect_6 = EffectModule::req_follow(opponent_boma_6, Hash40::new("sys_aura_light"), Hash40::new("top"), &Vector3f{x: 0.0, y: 12.0, z: 0.0}, &Vector3f::zero(), 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                let effect_7 = EffectModule::req_follow(opponent_boma_7, Hash40::new("sys_aura_light"), Hash40::new("top"), &Vector3f{x: 0.0, y: 12.0, z: 0.0}, &Vector3f::zero(), 4.0, true, 0, 0, 0, 0, 0, true, true) as u32;
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
            WorkModule::on_flag(boma, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_VANISH_ATTACK);
            EffectModule::kill_kind(opponent_boma_1, Hash40::new("sys_aura_light"), false, false);
            EffectModule::kill_kind(opponent_boma_2, Hash40::new("sys_aura_light"), false, false);
            EffectModule::kill_kind(opponent_boma_3, Hash40::new("sys_aura_light"), false, false);
            EffectModule::kill_kind(opponent_boma_4, Hash40::new("sys_aura_light"), false, false);
            EffectModule::kill_kind(opponent_boma_5, Hash40::new("sys_aura_light"), false, false);
            EffectModule::kill_kind(opponent_boma_6, Hash40::new("sys_aura_light"), false, false);
            EffectModule::kill_kind(opponent_boma_7, Hash40::new("sys_aura_light"), false, false);
        }
        if ![*FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_VANISH_CANCEL, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_VANISH_ATTACK].contains(&status_kind) && WorkModule::is_flag(boma, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_VANISH_ATTACK) {
            WorkModule::off_flag(boma, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_VANISH_ATTACK);
            WorkModule::off_flag(boma, *FIGHTER_SHEIK_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_HAS_VANISHED);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_VANISH_ATTACK, false);
        }
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        sheik_start_initialization,
        sheik_reset_initialization,
        sheik_death_initialization,
        sheik_opff
    );
}