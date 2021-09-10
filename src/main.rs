#[macro_use] extern crate log;
extern crate image;
extern crate autopilot;
extern crate rand;


pub mod stroke_play;
pub mod common;
pub mod menu;

use autopilot::bitmap::{capture_screen, capture_screen_portion, Bitmap};
use autopilot::screen::{get_color, is_point_visible, is_rect_visible};
use autopilot::geometry::{Point, Rect, Size};
use autopilot::mouse::{move_to, click, Button, ScrollDirection};
use std::path::Path;
use rand::{thread_rng, Rng};
use std::thread::sleep;
use std::time::Duration;
use stroke_play::{ShuntShotMenu, PauseMenu, find_and_click,};
use menu::StrokePlayMenu;
use crate::stroke_play::{sleep_secs, aim_at_pin, find_or_manual_click, make_screen_active, button_click, RandomPlayGameLoop, state_ready_shoot, is_visible, GameDecisions, is_pixel_white, dist_one_to_two_hundred, dist_four_to_five_hundreds};

use crate::menu::{MainMenu, PracticeMenu, GamesMenu};
use image::{Rgba, Pixel};


fn main() {

let random_play_loop = RandomPlayGameLoop::new();
    loop {
        random_play_loop.execute_full_loop();
    }

 }