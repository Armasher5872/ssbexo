use super::*;

unsafe extern "C" fn luigi_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = fighter.global_table[STATUS_KIND].get_i32();
    if status_kind == *FIGHTER_STATUS_KIND_CATCH_PULL && HAS_CATCH[entry_id] == true {
        let status = fighter.global_table[THROW_F_STATUS_KIND].get_i32();
        StatusModule::change_status_force(fighter.module_accessor, status, false);
    }
}

pub fn install() {
    Agent::new("luigi")
    .on_line(Main, luigi_frame)
    .install()
    ;
}