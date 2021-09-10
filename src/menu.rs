use autopilot::geometry::Point;
use crate::stroke_play::{button_click, sleep_secs};
use crate::stroke_play::make_screen_active;
use rand::{thread_rng, Rng};
use std::thread::sleep;
use autopilot::mouse::{click, move_to, Button, ScrollDirection, scroll};



pub struct MainMenu
{
    play_golf: Point,
    practice: Point,
    games: Point,
    events: Point,
    online_play: Point,
    resume: Point,
    settings: Point,
    remembered_player: Point,
    add_guest: Point,
    join: Point,
    clear_all: Point,
    next: Point,
    home: Point,
    tee_off: Point,
}

impl MainMenu {
    pub fn new() -> MainMenu
    {
        MainMenu {
            play_golf: Point::new(260.0, 342.0 ),
            practice: Point::new(260.0, 432.0),
            games: Point::new(280.0, 565.0),
            events: Point::new(274.0, 640.0),
            online_play: Point::new(280.0, 714.0),
            resume: Point::new(272.0, 802.0),
            settings: Point::new(1506.0, 878.0),
            remembered_player: Point::new(528.0, 438.0),
            add_guest: Point::new(888.0, 356.0),
            join: Point::new(910.0, 560.0),
            clear_all: Point::new(884.0, 776.0),
            next: Point::new(1538.0, 988.0),
            home: Point::new(1880.0, 30.0),
            tee_off: Point::new(1512.0, 990.0),
        }
    }

    pub fn play_golf(&self,) -> () {
        button_click(self.play_golf, 1);
        sleep_secs(2);
    }
    pub fn add_to_roster(&self,) -> () {
        move_to(self.clear_all);
        click(Button::Left, None);
        sleep_secs(2);
        click(Button::Left, Some(100));
        sleep_secs(2);

        for x in 0..8 {
            if x < 2 {
                button_click(self.add_guest, 1);
                sleep_secs(2);
                continue;
            }
            button_click(self.remembered_player, 1);
            sleep_secs(2);
            button_click(self.join, 1);
            sleep_secs(2);
        }
        button_click(self.next, 2);
    }

    pub fn click_main_menu_next(&self,) -> () {
        make_screen_active();
        button_click(self.next, 1);
    }
    pub fn click_home(&self,) -> () {
        make_screen_active();
        sleep_secs(2);
        button_click(self.home, 1);
    }
    pub fn click_tee_off(&self,) -> () {
        make_screen_active();
        sleep_secs(2);

        button_click(self.tee_off, 2);
        sleep_secs(7);
    }
}

pub struct StrokePlayMenu {
    mode_of_play : Point,
    download_update_course: Point,
    top_point_course_list: (f64, f64),
    bottom_point_course_list: (f64, f64),
    scramble: Point,
    best_ball: Point,
    main_menu_instance: MainMenu,
}

impl StrokePlayMenu {
    pub fn new() -> StrokePlayMenu {
        StrokePlayMenu {
            mode_of_play: Point::new(1132.0, 334.0),
            download_update_course: Point::new(928.0, 890.0),
            scramble: Point::new(1132.0, 634.0),
            best_ball: Point::new(1132.0, 714.0),
            top_point_course_list: (1128.0, 1564.0),
            bottom_point_course_list: (334.0, 428.0),
            main_menu_instance: MainMenu::new(),
        }
    }
    pub fn handle_mode_of_play(&self,) -> () {
        let main_menu = MainMenu::new();
        make_screen_active();
        let rand_num = thread_rng().gen_range(0..2);
        if rand_num == 0 {
            button_click(self.mode_of_play, 1);
        }
        else if rand_num == 2 {
            button_click(self.mode_of_play, 1);
            autopilot::mouse::scroll(
                ScrollDirection::Down,
                                     20);
            sleep_secs(2);
            button_click(self.scramble, 1);
            sleep_secs(2);
        }
        else {
            button_click(self.mode_of_play, 1);
            autopilot::mouse::scroll(
                ScrollDirection::Down,
                20);
            sleep_secs(2);
            button_click(self.best_ball, 1);
            sleep_secs(2);
        }
        sleep_secs(2);
        main_menu.click_main_menu_next();
    }

    pub fn select_course(&self,) -> () {

        make_screen_active();
        sleep_secs(2);

        let rand_scrolls: u32  = thread_rng().gen_range(0..80);
        scroll(ScrollDirection::Down, rand_scrolls);
        sleep_secs(2);

        let (x1, x2) = self.top_point_course_list;
        let (y1, y2) = self.bottom_point_course_list;
        let rand_point_x = thread_rng()
            .gen_range(x1..x2);
        let rand_point_y = thread_rng()
            .gen_range(y1..y2);

        button_click(Point::new(rand_point_x, rand_point_y), 2);
        sleep_secs(2);

        button_click(self.download_update_course, 1);
        // long sleep to account for long download times
        sleep_secs(40);

        self.main_menu_instance.click_main_menu_next();
    }

    pub fn full_stroke_menu_loop(&self,) -> () {
        make_screen_active();
        self.main_menu_instance.play_golf();
        sleep_secs(2);
        self.main_menu_instance.add_to_roster();
        sleep_secs(2);
        self.main_menu_instance.click_main_menu_next();
        sleep_secs(2);
        self.handle_mode_of_play();
        self.select_course();
        self.main_menu_instance.click_tee_off();
        sleep_secs(4);
        self.main_menu_instance.click_main_menu_next();
        sleep_secs(7);
    }
}

pub struct PracticeMenu {
    practice: Point,
     driving_range: Point,
     chip_and_putt: Point,
    stroke_play: StrokePlayMenu,
}

impl PracticeMenu {
    pub fn new() -> PracticeMenu {
        PracticeMenu {
            practice: Point::new(276.0, 442.0),
            driving_range: Point::new(1080.0, 338.0),
            chip_and_putt: Point::new(1080.0, 410.0),
            stroke_play: StrokePlayMenu::new(),
        }
    }
    pub fn choose_random_mode(&self,) -> () {
        let rand_num = thread_rng().gen_range(0..1);
        if rand_num == 0 {
            button_click(self.driving_range, 1);
        }
        else { button_click(self.chip_and_putt, 1); }
    }

    pub fn execute_full_practice_loop(&self,) -> () {
        make_screen_active();
        sleep_secs(2);
        button_click(self.practice, 1);
        sleep_secs(2);
        self.stroke_play.main_menu_instance.add_to_roster();
        sleep_secs(2);
        self.stroke_play.main_menu_instance.click_main_menu_next();
        sleep_secs(2);
        self.stroke_play.main_menu_instance.click_main_menu_next();
        sleep_secs(2);
        self.choose_random_mode();
        sleep_secs(2);
        self.stroke_play.handle_mode_of_play();
        sleep_secs(2);
        self.stroke_play.main_menu_instance.click_main_menu_next();
        sleep_secs(2);
        self.stroke_play.main_menu_instance.click_tee_off();
        sleep_secs(14);
    }
}

pub struct GamesMenu {
    games: Point,
    practice_menu: PracticeMenu,
    are_you_read_play: Point,
}

impl GamesMenu {
    pub fn new() -> GamesMenu {
        GamesMenu {
            games: Point::new(276.0, 562.0),
            practice_menu: PracticeMenu::new(),
            are_you_read_play: Point::new(1412.0, 490.0),
        }
    }
    pub fn are_you_read_play(&self,) -> () {
        button_click(self.are_you_read_play, 1);
    }

    pub fn full_games_loop(&self, ) -> () {
        make_screen_active();
        sleep_secs(2);
        button_click(self.games, 1);
        sleep_secs(2);
        self.practice_menu.stroke_play.full_stroke_menu_loop();
        sleep_secs(2);
        self.are_you_read_play();
    }
}