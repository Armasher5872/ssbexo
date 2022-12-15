#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod armstrong;
mod captain;
mod custom;
mod dedede;
mod donkey;
mod edge;
mod falco;
mod fox;
mod functions;
mod gamewatch;
mod koopa;
mod krool;
mod lucario;
mod lucina;
mod marth;
mod metaknight;
mod mewtwo;
mod miifighter;
mod ness;
mod packun;
mod pacman;
mod pfushigisou;
mod pichu;
mod pikachu;
mod pikmin;
mod plizardon;
mod pzenigame;
mod ridley;
mod robot;
mod rosetta;
mod roy;
mod samusd;
mod snake;
mod sonic;

#[skyline::main(name = "Super Smash Bros Ultimate: EXO")]
pub fn main() {
  armstrong::install();
  captain::install();
  custom::install();
  dedede::install();
  donkey::install();
  edge::install();
  falco::install();
  fox::install();
  functions::install();
  gamewatch::install();
  koopa::install();
  krool::install();
  lucario::install();
  lucina::install();
  marth::install();
  metaknight::install();
  mewtwo::install();
  miifighter::install();
  ness::install();
  packun::install();
  pacman::install();
  pfushigisou::install();
  pichu::install();
  pikachu::install();
  pikmin::install();
  plizardon::install();
  pzenigame::install();
  ridley::install();
  robot::install();
  rosetta::install();
  roy::install();
  samusd::install();
  snake::install();
  sonic::install();
}