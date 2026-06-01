use super::*;

//Final Smash ACMD
unsafe extern "C" fn ssbexo_mario_final_smash_acmd(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    let scale = PostureModule::scale(boma);
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_CHARGE) {
        if is_excute(agent) {
            CHECK_VALID_FINAL_START_CAMERA(agent, 0, 1, 20, 0, 0, 0);
        }
        frame(lua_state, 5.0);
        if is_excute(agent) {
            ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_HUGE_FLAME, false, -1);
        }
        frame(lua_state, 10.0);
        if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {      
            if is_excute(agent) {
                FT_SET_FINAL_FEAR_FACE(agent, 60);
                REQ_FINAL_START_CAMERA_arg3(agent, Hash40::new("d04final.nuanmb"), false, false);
                FT_START_CUTIN(agent);
            }  
        }
        else {
            if is_excute(agent) {
                CAM_ZOOM_IN_arg5(agent, 2.05, 0.0, 3.0*scale, 0.0, 0.0);
                FT_START_CUTIN(agent);
            }
        }
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            if is_excute(agent) {
                camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
            }
        }
        else {
            if is_excute(agent) {
                camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
            }
        }
        frame(lua_state, 25.0);
        if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            CAM_ZOOM_OUT(agent);
        }
    }
    else {
        if is_excute(agent) {
            WHOLE_HIT(agent, *HIT_STATUS_XLU);
            CHECK_VALID_FINAL_START_CAMERA(agent, 0, 1, 20, 0, 0, 0);
            SLOW_OPPONENT(agent, 20.0, 75.0);
        }
        frame(lua_state, 5.0);
        if is_excute(agent) {
            ArticleModule::generate_article(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_HUGE_FLAME, false, -1);
        }
        frame(lua_state, 10.0);
        if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {      
            if is_excute(agent) {
                FT_SET_FINAL_FEAR_FACE(agent, 60);
                REQ_FINAL_START_CAMERA_arg3(agent, Hash40::new("d04final.nuanmb"), false, false);
                FT_START_CUTIN(agent);
            }  
        }
        else {
            if is_excute(agent) {
                CAM_ZOOM_IN_arg5(agent, 2.05, 0.0, 3.0*scale, 0.0, 0.0);
                FT_START_CUTIN(agent);
            }
        }
        if sv_animcmd::get_value_float(lua_state, *SO_VAR_FLOAT_LR) < 0.0 {
            if is_excute(agent) {
                camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
            }
        }
        else {
            if is_excute(agent) {
                camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
            }
        }
        frame(lua_state, 25.0);
        if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
            CAM_ZOOM_OUT(agent);
        }
    }
}

pub fn install() {
    Agent::new("mario")
    .set_costume([0, 1, 2, 3, 4, 5, 6, 7].to_vec())
    .game_acmd("game_final", ssbexo_mario_final_smash_acmd, Low)
    .game_acmd("game_finalair", ssbexo_mario_final_smash_acmd, Low)
    .install()
    ;
}