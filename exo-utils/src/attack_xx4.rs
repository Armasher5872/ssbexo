use super::*;

//Full Smash Attack Check
pub unsafe fn attack_4_hold(fighter: &mut L2CFighterCommon) {
    let frame = fighter.global_table[CURRENT_FRAME].get_f32();
    if frame >= 59.0 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FULL_SMASH_ATTACK);
    }
    physics!(fighter, MA_MSC_CMD_PHYSICS_STOP_CHARGE);
    fighter.pop_lua_stack(1);
}