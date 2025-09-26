#![feature(concat_idents, proc_macro_hygiene, repr_simd, simd_ffi, seek_stream_len)]

use skyline_web::dialog_ok::DialogOk;
use exo_utils::extern_func::is_on_ryujinx;
use crate::fighters::mods_mounted;
use std::path::Path;
use smash::hash40;

#[cfg(feature = "skyline-web")]
extern crate skyline_web;

fn quick_validate_install() -> bool {
    let mut passed = true;
    let has_param_config = Path::new("rom:/skyline/plugins/libparam_config.nro").is_file();
    let has_css_redirector = Path::new("rom:/skyline/plugins/libthe_csk_collection.nro").is_file();
    let has_arcropolis = Path::new("rom:/skyline/plugins/libarcropolis.nro").is_file();
    let has_nro_hook = Path::new("rom:/skyline/plugins/libnro_hook.nro").is_file();
    let has_smashline = Path::new("rom:/skyline/plugins/libsmashline_plugin.nro").is_file();
    if has_param_config {
        println!("libparam_config.nro is present");
    } 
    else {
        if is_on_ryujinx() {
            println!("libparam_config.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        } 
        else {
            DialogOk::ok("libparam_config.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        }
        passed = false;
    }
    if has_css_redirector {
        println!("libthe_csk_collection.nro is present");
    } 
    else {
        if is_on_ryujinx() {
            println!("libthe_csk_collection.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        } 
        else {
            DialogOk::ok("libthe_csk_collection.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        }
        passed = false;
    }
    if has_arcropolis {
        println!("libarcropolis.nro is present");
    } 
    else {
        if is_on_ryujinx() {
            println!("libarcropolis.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        } 
        else {
            DialogOk::ok("libarcropolis.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        }
        passed = false;
    }
    if has_nro_hook {
        println!("libnro_hook.nro is present");
    } 
    else {
        if is_on_ryujinx() {
            println!("libnro_hook.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        } 
        else {
            DialogOk::ok("libnro_hook.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        }
        passed = false;
    }
    if has_smashline {
        println!("libsmashline_plugin.nro is present");
    } 
    else {
        if is_on_ryujinx() {
            println!("libsmashline_plugin.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        } 
        else {
            DialogOk::ok("libsmashline_plugin.nro not found! This installation is incomplete. Please download all dependencies listed in the README file.");
        }
        passed = false;
    }
    passed
}

mod fighters;

#[no_mangle]
pub fn smashline_install() {
    fighters::install();
}

#[skyline::main(name = "ssbexo")]
pub fn main() {
    if !quick_validate_install() {
        return; // don't do anything else since they don't have all dependencies
    }
    unsafe {
        //allows online play
        extern "C" {
            fn allow_ui_chara_hash_online(ui_chara_hash: u64);
        }
        allow_ui_chara_hash_online(0x12540231f0); //ui_chara_armstrong
    }
    unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                ty: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, mods_mounted);
    }
    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_armstrong");
    the_csk_collection_api::add_bgm_db_entry_info(&the_csk_collection_api::BgmDatabaseRootEntry {
        ui_bgm_id: hash40("ui_bgm_zz09_f_armstrong"),
        clone_from_ui_bgm_id: Some(hash40("ui_bgm_z20_f_ganon")),
        stream_set_id: the_csk_collection_api::Hash40Type::Overwrite(hash40("set_zz09_f_armstrong")),
        ..Default::default()
    });
    the_csk_collection_api::add_stream_set_entry_info(&the_csk_collection_api::StreamSetEntry { 
        stream_set_id: hash40("set_zz09_f_armstrong"),
        info0: the_csk_collection_api::Hash40Type::Overwrite(hash40("info_zz09_f_armstrong")),
        ..Default::default()
    });
    the_csk_collection_api::add_assigned_info_entry_info(&the_csk_collection_api::AssignedInfoEntry { 
        info_id: hash40("info_zz09_f_armstrong"),
        stream_id: the_csk_collection_api::Hash40Type::Overwrite(hash40("stream_zz09_f_armstrong")),
        condition: the_csk_collection_api::Hash40Type::Overwrite(hash40("sound_condition_none")),
        condition_process: the_csk_collection_api::Hash40Type::Overwrite(hash40("sound_condition_process_add")),
        change_fadeout_frame: the_csk_collection_api::IntType::Overwrite(60),
        menu_change_fadeout_frame: the_csk_collection_api::IntType::Overwrite(60),
        ..Default::default()
    });
    the_csk_collection_api::add_stream_property_entry_info(&the_csk_collection_api::StreamPropertyEntry {
        stream_id: hash40("stream_zz09_f_armstrong"),
        data_name0: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("zz09_f_armstrong")),
        ..Default::default()
    });
    the_csk_collection_api::add_new_bgm_property_entry(&smash_bgm_property::BgmPropertyEntry {
        stream_name: hash40::Hash40::new("zz09_f_armstrong"),
        loop_start_ms: 0,
        loop_start_sample: 0,
        loop_end_ms: 0,
        loop_end_sample: 0,
        duration_ms: 7659,
        duration_sample: 359424 
    });
    the_csk_collection_api::set_fighter_jingle(hash40("ui_chara_armstrong"), "zz09_f_armstrong");    
    fighters::install();
    std::panic::set_hook(Box::new(|info: &std::panic::PanicHookInfo<'_>| {
        let location: &std::panic::Location<'_> = info.location().unwrap();
        let message: &str = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>"
            }
        };
        skyline::error::show_error(
            69,
            "Super Smash Bros: EXO has panicked! Please open the details and send a screenshot to PhazoGanon on Discord, and explain what you were doing.\0",
            &format!("Super Smash Bros: EXO has panicked with \"{message}\"\n\n{location}\0")
        );
    }));
}