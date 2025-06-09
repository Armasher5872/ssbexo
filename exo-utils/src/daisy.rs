use super::*;

//Spawns the glove on either the left or right arm depending on the lr param, with 0 being the left, and 1 being the right
pub unsafe extern "C" fn spawn_glove(boma: *mut BattleObjectModuleAccessor, lr: i32) {
    ArticleModule::generate_article(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, false, 0);
    if ArticleModule::is_exist(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR) {
        let crystal_fist_boma = get_article_boma(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR);
        if lr == 0 {
            LinkModule::set_model_constraint_pos_ort(crystal_fist_boma, *LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("arml"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_OFFSET_ROT) as u32, true);
            LinkModule::set_constraint_rot_offset(crystal_fist_boma, &Vector3f{x: 90.0, y: 0.0, z: -90.0});
            LinkModule::set_constraint_translate_offset(crystal_fist_boma, &Vector3f{x: 0.0, y: 2.5, z: 0.0});
        }
        else {
            LinkModule::set_model_constraint_pos_ort(crystal_fist_boma, *LINK_NO_CONSTRAINT, Hash40::new("have"), Hash40::new("armr"), (*CONSTRAINT_FLAG_ORIENTATION | *CONSTRAINT_FLAG_POSITION | *CONSTRAINT_FLAG_OFFSET_TRANSLATE | *CONSTRAINT_FLAG_OFFSET_ROT) as u32, true);
            LinkModule::set_constraint_rot_offset(crystal_fist_boma, &Vector3f{x: 0.0, y: 0.0, z: -90.0});
            LinkModule::set_constraint_translate_offset(crystal_fist_boma, &Vector3f{x: 0.0, y: 2.5, z: 0.0});
        }
    }
}

pub unsafe extern "C" fn swap_glove_lr(boma: *mut BattleObjectModuleAccessor) {
    let lr = PostureModule::lr(boma);
    if ArticleModule::is_exist(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR) {
        let crystal_fist_boma = get_article_boma(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR);
        if lr == 1.0 {
            ModelModule::set_mesh_visibility(crystal_fist_boma, Hash40::new("daisy_glove"), false);
            ModelModule::set_mesh_visibility(crystal_fist_boma, Hash40::new("daisy_gloveleft"), true);
        }
        else {
            ModelModule::set_mesh_visibility(crystal_fist_boma, Hash40::new("daisy_glove"), true);
            ModelModule::set_mesh_visibility(crystal_fist_boma, Hash40::new("daisy_gloveleft"), false);
        }
    }
}

pub unsafe extern "C" fn despawn_glove(boma: *mut BattleObjectModuleAccessor) {
    let get_active_num = ArticleModule::get_active_num(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR);
    for _ in 0..get_active_num {
        ArticleModule::remove_exist(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}