//Credit to HDR
use super::*;

//Triggers a match reset by loading into the same state that classic mode uses when you retry a game. Note: Calling this function outside of a match shouldn't crash but it has undefined behavior. If you do that, don't
pub fn trigger_match_reset() {
    unsafe {
        let p_game_state = get_game_state();
        if p_game_state.is_null() {
            return;
        }
        //Finally call the vtable function on the game state
        let vtable_func: extern "C" fn(*const u64) = std::mem::transmute(*(*p_game_state as *const u64).add(0x5));
        vtable_func(p_game_state);
    }
}

//Triggers a match exit (all the way back to the stage select screen) by entering into the `StateExit` game state. Note: Calling this function otuside of a match shouldn't crash but it has undefined behavior. If you do that, don't
pub fn trigger_match_exit() {
    unsafe {
        let p_game_state = get_game_state();
        if p_game_state.is_null() {
            return;
        }
        //Finally call the vtable function on the game state
        let vtable_func: extern "C" fn(*const u64) = std::mem::transmute(*(*p_game_state as *const u64).add(0x3));
        vtable_func(p_game_state);
    }
}