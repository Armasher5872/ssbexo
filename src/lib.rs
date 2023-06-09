#![feature(
    concat_idents, 
    proc_macro_hygiene, 
    repr_simd, 
    simd_ffi
)]
#![allow(
    clippy::absurd_extreme_comparisons,
    clippy::bool_comparison,
    clippy::borrow_interior_mutable_const,
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::excessive_precision,
    clippy::from_over_into,
    clippy::if_same_then_else,
    clippy::let_and_return,
    clippy::module_inception,
    clippy::needless_lifetimes,
    clippy::needless_range_loop,
    clippy::needless_return,
    clippy::ptr_offset_with_cast,
    clippy::redundant_field_names,
    clippy::suspicious_else_formatting,
    clippy::transmute_ptr_to_ref,
    clippy::unnecessary_cast,
    clippy::unnecessary_operation,
    clippy::useless_conversion,
    clippy::useless_transmute,
    dead_code,
    illegal_floating_point_literal_pattern,
    improper_ctypes_definitions,
    non_upper_case_globals,
    unused_assignments,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_unsafe,
    unused_variables
)]

mod armstrong;
mod bayonetta;
mod brave;
mod buddy;
mod captain;
mod chrom;
mod cloud;
mod common;
mod daisy;
mod dedede;
mod demon;
mod diddy;
mod dolly;
mod donkey;
mod duckhunt;
mod edge;
mod eflame;
mod elight;
mod falco;
mod fox;
mod functions;
mod gamewatch;
mod gaogaen;
mod gekkouga;
mod ike;
mod inkling;
mod jack;
mod kamui;
mod ken;
mod kirby;
mod koopa;
mod koopajr;
mod krool;
mod link;
mod littlemac;
mod lucario;
mod lucas;
mod lucina;
mod luigi;
mod mario;
mod mariod;
mod marth;
mod master;
mod metaknight;
mod mewtwo;
mod miifighter;
mod miigunner;
mod miiswordsman;
mod nana;
mod ness;
mod packun;
mod pacman;
mod palutena;
mod peach;
mod pfushigisou;
mod pichu;
mod pickel;
mod pikachu;
mod pikmin;
mod pit;
mod pitb;
mod plizardon;
mod popo;
mod purin;
mod pzenigame;
mod reflet;
mod richter;
mod ridley;
mod robot;
mod rockman;
mod rosetta;
mod roy;
mod ryu;
mod samus;
mod samusd;
mod sheik;
mod shulk;
mod simon;
mod snake;
mod sonic;
mod szerosuit;
mod toonlink;
mod trail;
mod wario;
mod wiifit;
mod wolf;
mod yoshi;
mod younglink;
mod zelda;

#[skyline::main(name = "plugin")]
pub fn main() {
    armstrong::install();
    bayonetta::install();
    brave::install();
    buddy::install();
    captain::install();
    chrom::install();
    cloud::install();
    common::install();
    daisy::install();
    dedede::install();
    demon::install();
    diddy::install();
    dolly::install();
    donkey::install();
    duckhunt::install();
    edge::install();
    eflame::install();
    elight::install();
    falco::install();
    fox::install();
    functions::install();
    gamewatch::install();
    gaogaen::install();
    gekkouga::install();
    ike::install();
    inkling::install();
    jack::install();
    kamui::install();
    ken::install();
    kirby::install();
    koopa::install();
    koopajr::install();
    krool::install();
    link::install();
    littlemac::install();
    lucario::install();
    lucas::install();
    lucina::install();
    luigi::install();
    mario::install();
    mariod::install();
    marth::install();
    master::install();
    metaknight::install();
    mewtwo::install();
    miifighter::install();
    miigunner::install();
    miiswordsman::install();
    nana::install();
    ness::install();
    packun::install();
    pacman::install();
    palutena::install();
    peach::install();
    pfushigisou::install();
    pichu::install();
    pickel::install();
    pikachu::install();
    pikmin::install();
    pit::install();
    pitb::install();
    plizardon::install();
    popo::install();
    purin::install();
    pzenigame::install();
    reflet::install();
    richter::install();
    ridley::install();
    robot::install();
    rockman::install();
    rosetta::install();
    roy::install();
    ryu::install();
    samus::install();
    samusd::install();
    sheik::install();
    shulk::install();
    simon::install();
    snake::install();
    sonic::install();
    szerosuit::install();
    toonlink::install();
    trail::install();
    wario::install();
    wiifit::install();
    wolf::install();
    yoshi::install();
    younglink::install();
    zelda::install();
}
//cargo skyline build --no-default-features --release -- --offline