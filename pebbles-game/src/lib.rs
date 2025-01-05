#![no_std]
use gstd::{msg, prelude::*, exec};
use pebbles_game_io::*;

static mut GAME_STATE: Option<GameState> = None;



