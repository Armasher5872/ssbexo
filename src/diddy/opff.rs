use super::*;

#[fighter_frame( agent = FIGHTER_KIND_DIDDY )]
fn diddy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let prev_status_kind = fighter.global_table[PREV_STATUS_KIND].get_i32();
        let boma_opponent1 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(1));
        let status_kind_opponent1 = StatusModule::status_kind(boma_opponent1);
        let boma_opponent2 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(2));
        let status_kind_opponent2 = StatusModule::status_kind(boma_opponent2);
        let boma_opponent3 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(3));
        let status_kind_opponent3 = StatusModule::status_kind(boma_opponent3);
        let boma_opponent4 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(4));
        let status_kind_opponent4 = StatusModule::status_kind(boma_opponent4);
        let boma_opponent5 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(5));
        let status_kind_opponent5 = StatusModule::status_kind(boma_opponent5);
        let boma_opponent6 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(6));
        let status_kind_opponent6 = StatusModule::status_kind(boma_opponent6);
        let boma_opponent7 = smash::app::sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(7));
        let status_kind_opponent7 = StatusModule::status_kind(boma_opponent7);
        let banana_id = WorkModule::get_int(boma, *FIGHTER_DIDDY_STATUS_SPECIAL_LW_WORK_INT_BANANA_ID);
        if prev_status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW
        && BANANA_EXIST[entry_id] == false {
            BANANA_EXIST[entry_id] = true;
        }
        if status_kind_opponent1 == *FIGHTER_STATUS_KIND_SLIP || status_kind_opponent2 == *FIGHTER_STATUS_KIND_SLIP || status_kind_opponent3 == *FIGHTER_STATUS_KIND_SLIP 
        || status_kind_opponent4 == *FIGHTER_STATUS_KIND_SLIP || status_kind_opponent5 == *FIGHTER_STATUS_KIND_SLIP || status_kind_opponent6 == *FIGHTER_STATUS_KIND_SLIP 
        || status_kind_opponent7 == *FIGHTER_STATUS_KIND_SLIP && BANANA_EXIST[entry_id] == true {
            WorkModule::set_int(boma, banana_id, *FIGHTER_DIDDY_STATUS_SPECIAL_LW_WORK_INT_BANANA_ID);
            BANANA_EXIST[entry_id] = false;
            StatusModule::change_status_request_from_script(boma, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_LW_LAUGH, false);
        }
        println!("Is Banana Exist {:?}", BANANA_EXIST[entry_id]);
    }
}

pub fn install() {
    install_agent_frames!(diddy_frame);
}