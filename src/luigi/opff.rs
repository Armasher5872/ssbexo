use super::*;

#[fighter_frame( agent = FIGHTER_KIND_LUIGI )]
fn luigi_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status_kind = fighter.global_table[STATUS_KIND].get_i32();
        println!("Is Catch:{:?}", DOWN_B_CATCH[entry_id]);
        println!("Typhoon RNG:{}", LUIGI_CYCLONE_RNG[entry_id]);
        if status_kind == *FIGHTER_STATUS_KIND_CATCH_PULL && DOWN_B_CATCH[entry_id] == true {
            let status = fighter.global_table[THROW_F_STATUS_KIND].get_i32();
            StatusModule::change_status_force(fighter.module_accessor, status, false);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        luigi_frame
    );
}