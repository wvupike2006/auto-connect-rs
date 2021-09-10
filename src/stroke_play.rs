extern crate inputbot;

use autopilot::geometry::{Point, Rect};
use autopilot::mouse::{move_to, click,Button,};
use std::time::Duration;
use std::thread::sleep;
use autopilot::bitmap::{Bitmap, capture_screen, capture_screen_portion};
use std::path::Path;
use rand::distributions::Open01;
use autopilot::key::{KeyCode, Flag};
use crate::menu::{MainMenu, StrokePlayMenu, PracticeMenu, GamesMenu};
use autopilot::screen::get_color;
use std::collections::hash_map::Keys;
use self::inputbot::KeybdKey;
use rand::{thread_rng, Rng};

pub fn final_results_shown() -> bool
{
    let final_results_letter_t = is_pixel_white(512.0, 180.0);
    let final_results_letter_e = is_pixel_white(650.0, 210.0);
    let final_results_letter_s = is_pixel_white(444.0, 216.0);
    let final_results_letter_l = is_pixel_white(616.0, 212.0);
    let score_card_par = is_visible(Path::new("Scorecard_Par.png"));

    if final_results_letter_e && final_results_letter_l && final_results_letter_s && final_results_letter_t
        && (score_card_par != Point::new(0.0, 0.0)) {

        return true;
    }
    false
}

pub fn stroke_game_loaded() -> bool
{
    let hole_num_pixel_color_a = get_color(Point::new(1100.0, 40.0));
    let hole_num_pixel_color_b = get_color(Point::new(1098.0, 76.0));

    let rgb_a = hole_num_pixel_color_a.unwrap().0;
    let rgb_b = hole_num_pixel_color_b.unwrap().0;

    let is_pixel_red_a = (rgb_a[0] > rgb_a[1]) && (rgb_a[0] > rgb_a[2]);
    let is_pixel_red_b = (rgb_b[0] > rgb_b[1]) && (rgb_b[0] > rgb_b[2]);

    if is_pixel_red_a && is_pixel_red_b {
        return true;
    }
    false
}

pub fn double_click(point: &Point) {
    move_to(*point);
    click(Button::Left, None);
    std::thread::sleep(std::time::Duration::from_millis(1200));
    click(Button::Left, None);
}

pub fn click_swing_gauge(wait_millis_one: Duration, wait_millis_two: Duration) -> () {
    let game_loop_instance = RandomPlayGameLoop::new();

    button_click(game_loop_instance.swing_gauge_shoot, 1);
    std::thread::sleep(wait_millis_one);
    button_click(game_loop_instance.swing_gauge_shoot, 1);
    std::thread::sleep(wait_millis_two);
    button_click(game_loop_instance.swing_gauge_shoot, 1);
}
pub fn is_shunt_shot_open() -> bool {
    let color = get_color(Point::new(672.0, 822.0));

    let r = color.unwrap().0;
    let low: u8 = 55;
    let high: u8 = 65;

    for i in 0..r.len() {
        if r[i] <= low || r[i] >= high {
            return false;
        }
    }
    return true;
}

pub fn make_screen_active()
{
    move_to(Point::new(896.0, 24.0));
    click(Button::Left, None);

    sleep_secs(2);
}

pub fn state_ready_shoot() -> bool {
    for x in 1100..1132 {
        if get_color(Point::new(x as f64, 42.0)).unwrap()
            != get_color(Point::new(1104.0, 42.0)).unwrap()
        {
            return false;
        }
    }
    return true;
}
pub fn capture_current_screen() ->()
{
    sleep(Duration::new(2, 0));
    move_to(Point::new(10.0, 10.0));
    click(Button::Left, None);
    let bmp = capture_screen().expect("capture fullscreen");

    let rand_int = thread_rng().gen_range(0..400);
    let unique_path = format!("screenshot__{}.png", rand_int);
    let bmp_path = Path::new(file!())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(unique_path);

    let bmp_image = bmp.image.save(&bmp_path)
        .expect("Unable to save full screen image");
}

pub fn speed_sim_on() -> bool {
    for px in 0..60  {
        let mut start_x = 1040.0;
        let y = 772.0;

        if is_pixel_white(start_x, y) || is_pixel_white(1079.0, 772.0)
            || is_pixel_white(1059.0, 776.0){
            return true;
        }
        start_x += 1.0;
    }
    false
}

pub fn button_click(pt: Point, clicks: u32)
{
    move_to(pt);
    if clicks > 1
    {
        click(Button::Left, Some(20));
        click(Button::Left, None);
    }
    else {  click(Button::Left, Some(20)); }
}

pub fn sleep_secs(seconds: u64,) -> ()
{
    sleep(Duration::new(seconds, 0));
}

pub fn find_and_click(path: &Path) -> ()
{
    let img = image::open(path).expect("open path as image");
    let bmp = Bitmap::new(img, Some(1 as f64));
    let fullscreen_bmp = capture_screen()
        .expect("capture fullscreen");

   let found_pt = fullscreen_bmp
       .find_bitmap(&bmp, None, None, None,)
        .unwrap_or(Point::new(56.0, 988.0));

    button_click(found_pt, 2);
}

pub fn dist_one_to_two_hundred() -> bool {
    let x = 1444;
    for y in 126..154 {
        if !is_pixel_white(x as f64, y as f64) && is_pixel_white(1429.0, 146.0) {
            return false;
        }
    }
    return true;
}

pub fn dist_four_to_five_hundreds() -> bool {
    let x = 1444;
    for y in 126..154 {
        if !is_pixel_white(x as f64, y as f64) && !is_pixel_white(1429.0, 146.0) {
            return false;
        }
    }
    return true;
}

pub fn is_pixel_white(x: f64, y: f64) -> bool {
    let color_vec = get_color(Point::new(x, y)).unwrap().0;
    let mut i = 0;
    while i < color_vec.len() {
        if color_vec[i] < 200 {
            return false
        }
        i += 1;
    }
    return true
}

pub fn find_or_manual_click(path: &Path, x: f64, y: f64){
    let img = image::open(path).expect("open path as image");
    let bmp = Bitmap::new(img, Some(1 as f64));
    let fullscreen_bmp = capture_screen()
        .expect("capture fullscreen");

    let found_pt = fullscreen_bmp
        .find_bitmap(&bmp, None, None, None,)
        .unwrap_or(Point::new(x, y));

    move_to(found_pt);
    click(Button::Left, None);
}

pub fn is_visible(path: &Path) -> Point {
    let img = image::open(path).expect("open path as image");
    let bmp = Bitmap::new(img, Some(1 as f64));
    let fullscreen_bmp = capture_screen()
        .expect("capture fullscreen");

    fullscreen_bmp
        .find_bitmap(&bmp, None, None, None,)
        .unwrap_or(Point::new(0.0, 0.0))
}


pub struct GameDecisions
{
    shot_limit_keep_hitting: Point,
    shot_limit_pickup: Point,
    out_bounds_mulligan: Point,
    out_bounds_pickup: Point,
    out_bounds_rehit: Point,
}

impl GameDecisions {
    pub fn new() -> GameDecisions
    {
        GameDecisions {
            shot_limit_keep_hitting: Point::new(644.0, 628.0),
            shot_limit_pickup: Point::new(1128.0, 628.0),
            out_bounds_mulligan: Point::new(608.0, 488.0),
            out_bounds_pickup: Point::new(848.0, 504.0),
            out_bounds_rehit: Point::new(1048.0, 512.0),
        }
    }
    pub fn choose_rehit(&self,) -> ()
    {
        button_click(self.out_bounds_rehit, 1);
    }
    pub fn handle_oob_decisions(&self,) -> ()
    {
        let out_bounds_mulligan_path = Path::new("OobMulligan.png");
        let out_bounds_pickup_path = Path::new("OobPickup.png");
        let out_bounds_rehit_path = Path::new("OobRehit.png");
        let shot_limit_keep_hitting_path = Path::new("ShotLimitKeepHitting.png");
        let shot_limit_pick_up_path = Path::new("ShotLimitPickUp.png");

        let oob_options = vec![
            out_bounds_mulligan_path,
            out_bounds_pickup_path,
            out_bounds_rehit_path,
            shot_limit_keep_hitting_path,
            shot_limit_pick_up_path
        ];

        for x in oob_options.iter() {
            if is_visible(x) == Point::new(0.0, 0.0)
            {
                find_and_click(x);
            }
            else { break; }
        }
        sleep_secs(4);

        if is_visible(oob_options[0]) == Point::new(0.0, 0.0) ||
            is_visible(oob_options[oob_options.len() -1]) == Point::new(0.0, 0.0)
        {
            button_click(self.out_bounds_mulligan, 2);
            button_click(self.shot_limit_pickup, 2);
        }
        sleep_secs(4);
    }
}

pub fn aim_at_pin() -> ()
{

    let pin_path = Path::new("Pin.png");

    let img = image::open(pin_path).expect("open path as image");
    let bmp = Bitmap::new(img, Some(1 as f64));
    let fullscreen_bmp = capture_screen()
        .expect("capture fullscreen");

    let pin_location = fullscreen_bmp
        .find_bitmap(&bmp, None, None, None,)
        .unwrap_or(Point::new(1732.0, 144.0));

    button_click(pin_location, 1);
}

pub struct PauseMenu {
    menu: Point,
    open_shunt_shot: Point,
    mulligan: Point,
    score_card: Point,
    pick_up: Point,
    in_round_practice: Point,
    ok: Point,
    resume: Point,
    exit: Point,
    main_menu: Point,
    restart: Point,
    new_course: Point,
    pub main_menu_instance: MainMenu,
    pub shunt_shot_menu_instance: ShuntShotMenu
}

impl PauseMenu {
    pub fn new() -> PauseMenu
    {
        PauseMenu {
            menu: Point::new(50.0, 988.0),
            open_shunt_shot: Point::new(144.0, 972.0),
            mulligan: Point::new(488.0,372.0),
            score_card: Point::new(904.0, 364.0),
            pick_up: Point::new(1108.0, 592.0),
            in_round_practice: Point::new(280.0, 380.0),
            ok: Point::new(1628.0, 980.0),
            resume: Point::new(1200.0, 876.0),
            exit: Point::new(1404.0, 868.0),
            main_menu: Point::new(612.0, 484.0),
            restart: Point::new(804.0, 486.0),
            new_course: Point::new(1024.0, 492.0),
            main_menu_instance: MainMenu::new(),
            shunt_shot_menu_instance: ShuntShotMenu::new(),
        }
    }
    pub fn open_pause_menu(&self,) -> ()
    {
        let pause_menu_path = Path::new("EnterPauseMenu.png");

        find_and_click(pause_menu_path);
        sleep_secs(2);
    }

    pub fn open_shunt_shot(&self,) ->()
    {
        make_screen_active();
        sleep_secs(2);
        KeybdKey::RControlKey.press();
        KeybdKey::SKey.press();
        sleep_secs(1);
        KeybdKey::SKey.release();
        KeybdKey::RControlKey.release();
        sleep_secs(2);
    }

    pub fn check_score(&self,) -> ()
    {
        let score_ok_path = Path::new("ScoreCardOk.png");

        self.open_pause_menu();
        sleep_secs(2);
        button_click(self.score_card,2);
        sleep_secs(2);
        find_and_click(score_ok_path);
        sleep_secs(4);
        button_click(self.ok, 2);
        button_click(self.resume, 2);
    }

    pub fn exit_final_score(&self,) -> ()
    {

        let score_ok_path = Path::new("ScoreCardOk.png");
        find_and_click(score_ok_path);
        sleep_secs(4);
        self.main_menu_instance.click_main_menu_next();
        sleep_secs(7);
    }

    pub fn exit_during_game(&self,) -> ()
    {
        make_screen_active();
        self.open_pause_menu();
        sleep_secs(2);
        button_click(self.exit, 2);
        sleep_secs(7);
        button_click(self.new_course, 1);
        sleep_secs(2);
        self.main_menu_instance.click_home();
        sleep_secs(14);
    }
}

pub struct ShuntShotMenu
{
    launch_angle_keypad: Point,
    keypad_num_seven: Point,
    keypad_num_three: Point,
    keypad_num_zero: Point,
    keypad_accept: Point,
    ball_speed: Point,
    launch_direction_increment: Point,
    shot_direction_increment: Point,
    speed_simulation_increment: Point,
    shoot: Point,
    defaults: Point,
}

impl ShuntShotMenu
{
    pub fn new() -> ShuntShotMenu
    {
        ShuntShotMenu {
            launch_angle_keypad: Point::new(972.0, 224.0),
            keypad_num_seven: Point::new(782.0, 468.0),
            keypad_num_three: Point::new(1000.0, 680.0),
            keypad_num_zero: Point::new(784.0, 790.0),
            keypad_accept: Point::new(1008.0, 768.0),
            ball_speed: Point::new(984.0, 348.0),
            launch_direction_increment: Point::new(1208.0, 276.0),
            shot_direction_increment: Point::new(1210.0, 522.0),
            speed_simulation_increment: Point::new(1210.0, 764.0),
            shoot: Point::new(1128.0, 822.0),
            defaults: Point::new(708.0, 820.0),
        }
    }

    pub fn adjust_launch_angle(&self,) ->()
    {
        button_click(self.launch_angle_keypad, 1);
        sleep_secs(2);
        button_click(self.keypad_num_three, 1);
        sleep_secs(2);
        button_click(self.keypad_num_zero, 1);
        sleep_secs(2);
        button_click(self.keypad_accept, 1);
        sleep_secs(2);
    }
    pub fn change_ball_speed(&self,) -> ()
    {
        button_click(self.ball_speed, 1);
        sleep_secs(2);
        button_click(self.keypad_num_seven, 1);
        sleep_secs(2);
        button_click(self.keypad_num_zero, 1);
        sleep_secs(2);
        button_click(self.keypad_accept, 1);
        sleep_secs(2);
    }
    pub fn increase_launch_direction(&self,) ->()
    {
        button_click(self.launch_direction_increment, 1);

    }

    pub fn select_ball_to_hole(&self, ) ->()
    {
        button_click(self.shot_direction_increment, 1);
        
    }

    pub fn increase_speed_simulation(&self,) ->()
    {
        for _ in 0..3 {
            move_to(self.speed_simulation_increment);
            click(Button::Left, Some(100));
            sleep_secs(2);
        }
    }

    pub fn increase_shot_direction(&self,) -> ()
    {
        move_to(self.shot_direction_increment);
        click(Button::Left, None);
        move_to(Point::new(100.0, 500.0));
        sleep_secs(2);

    }

    pub fn accept(&self,) -> ()
    {
        button_click(self.keypad_accept, 1);


    }

    pub fn shoot(&self,)->()
    {
        button_click(self.shoot, 2);
    }
}

pub trait GameLoop {
    fn take_shot_routine();
}

pub struct RandomPlayGameLoop {
    main_menu_controller: MainMenu,
    stroke_menu_controller: StrokePlayMenu,
    pause_menu_controller: PauseMenu,
    shot_controller: ShuntShotMenu,
    practice_menu_controller: PracticeMenu,
    games_controller: GamesMenu,
    swing_gauge_shoot: Point,
}

impl RandomPlayGameLoop {
    pub fn new() -> RandomPlayGameLoop {
        RandomPlayGameLoop  {
            main_menu_controller: MainMenu::new(),
            stroke_menu_controller: StrokePlayMenu::new(),
            practice_menu_controller: PracticeMenu::new(),
            pause_menu_controller: PauseMenu::new(),
            shot_controller: ShuntShotMenu::new(),
            games_controller: GamesMenu::new(),
            swing_gauge_shoot: Point::new(1124.0, 890.0),
        }
    }
    pub fn take_swing_gauge_shot(&self, count: i32) -> () {
        sleep_secs(4);

        let driver_club_img = Path::new("DriverClub.png");
        let four_hybrid_img = Path::new("FourHybridClub.png");
        let sand_wedge_img = Path::new("SandWedge.png");

        if is_visible(driver_club_img) != Point::new(0.0, 0.0) {
            click_swing_gauge(std::time::Duration::from_millis(1800),
            std::time::Duration::from_millis(1000));

            sleep_secs(2);
        }
        else if is_visible(four_hybrid_img) != Point::new(0.0, 0.0) {
            click_swing_gauge(std::time::Duration::from_millis(1600),
                              std::time::Duration::from_millis(900));
            sleep_secs(2);
        }

        else if is_visible(sand_wedge_img) != Point::new(0.0, 0.0)  {
            click_swing_gauge(std::time::Duration::from_millis(1500),
                              std::time::Duration::from_millis(750));
            sleep_secs(2);
        }


        else {
            if count == 0 || count % 1 == 0 {
                click_swing_gauge(std::time::Duration::from_millis(1500),
                std::time::Duration::from_millis(750));
            } else if count % 3 == 0 {
                click_swing_gauge(std::time::Duration::from_millis(1800),
                                  std::time::Duration::from_millis(900));
            } else if count % 4 == 0 {
                click_swing_gauge(std::time::Duration::from_millis(1800),
                                  std::time::Duration::from_millis(1005));
            } else {
                self.pause_menu_controller.open_shunt_shot();
                sleep_secs(2);
                self.shot_controller.shoot();
            }
        }
    }

    pub fn execute_full_loop(&self,) -> () {

        let mut shot_count = 0;
        let mut set_aim_marker_setting = false;

        make_screen_active();
        sleep_secs(2);

        if is_visible(Path::new("Ok.png")) != Point::new(0.0, 0.0) {
            self.pause_menu_controller.exit_final_score();
        }

        if stroke_game_loaded() {

            self.pause_menu_controller.open_shunt_shot();
            sleep_secs(2);

            if !speed_sim_on() {
                self.shot_controller.increase_speed_simulation();
                sleep_secs(2);
            }

            self.shot_controller.shoot();
            sleep_secs(4);
            set_aim_marker_setting = true;
        }
        else {

            sleep_secs(7);

            if !stroke_game_loaded() {

                self.main_menu_controller.click_main_menu_next();
                aim_at_pin();
                sleep_secs(4);
            }
        }

        if !set_aim_marker_setting {

            while !stroke_game_loaded() {
                self.main_menu_controller.click_main_menu_next();
                sleep_secs(2);
            }

            self.pause_menu_controller.open_shunt_shot();
            sleep_secs(2);

            if !speed_sim_on() {

                self.shot_controller.increase_speed_simulation();
                sleep_secs(2);
            }
            self.shot_controller.shoot();
            sleep_secs(4);
            set_aim_marker_setting = true;
        }

        while final_results_shown() == false {

            let rehit_path = Path::new("OobLetter.png");
            if !RandomPlayGameLoop::entered_round_state()
                || is_visible(rehit_path) == Point::new(0.0, 0.0)
            {
                let oob_handler = GameDecisions::new();

                button_click(oob_handler.out_bounds_rehit, 1);
                sleep_secs(2);
            }
            let tee_off_button = Path::new("TeeOff.png");
            if is_visible(tee_off_button) != Point::new(0.0, 0.0)
            {
                make_screen_active();
                sleep_secs(2);
                button_click(is_visible(tee_off_button), 1);
                sleep_secs(14);
            }
            sleep_secs(2);
            let shot_limiting_path = Path::new("ShotLimitKeepHitting.png");
            if is_visible(shot_limiting_path) != Point::new(0.0, 0.0)
            {
                let game_decision = GameDecisions::new();

                make_screen_active();
                sleep_secs(2);
                button_click(game_decision.shot_limit_keep_hitting, 1);

                sleep_secs(4);
            }
            let low_quality_drop_img = Path::new("SmallDropHand.jpg");
            let high_quality_drop_img = Path::new("DropHand.png");

            if is_visible(low_quality_drop_img ) != Point::new(0.0, 0.0)
                || is_visible(high_quality_drop_img ) != Point::new(0.0, 0.0)
            {
                let drop_rehit = Point::new(1612.0, 878.0);
                button_click(drop_rehit, 1);
                sleep_secs(2);
            }

            let aim_at_tee = Point::new(1772.0, 158.0);
            button_click(aim_at_tee, 1);
            sleep_secs(2);
            let links_e6_logo = Path::new("LinksE6.png");
            if is_visible(links_e6_logo) != Point::new(0.0, 0.0) {

                if shot_count % 2 == 0 {
                    self.take_swing_gauge_shot(shot_count);
                    sleep_secs(14);
                    shot_count += 1;
                }
                else {
                    self.pause_menu_controller.open_shunt_shot();
                    sleep_secs(2);
                    self.shot_controller.shoot();
                }
            }
            else {
                self.pause_menu_controller.open_shunt_shot();
                sleep_secs(2);
                self.shot_controller.shoot();
            }
            sleep_secs(14);

            shot_count += 1;
        } // end game loop due to final results screen being visible

        if !stroke_game_loaded() {

            self.main_menu_controller.click_main_menu_next();
            sleep_secs(2);
        }
        sleep_secs(7);

        shot_count = 0;
    }

    pub fn entered_round_state() -> bool {
        let mut visible = true;
        let hole_one_path = Path::new("HoleOne.png");
        let hole_one_visible = is_visible(hole_one_path);

        if hole_one_visible == Point::new(0.0, 0.0) {
            visible = false;
        }
        visible
    }
}