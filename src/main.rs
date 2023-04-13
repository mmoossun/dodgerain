extern crate ncurses;
use rand::Rng;
use ncurses::*;

use std::time::Duration;
use std::thread;
use std::cmp::{min, max};

const WIDTH: i32 = 50;
const HEIGHT: i32 = 20;

const USER_CHAR: char = 'U';
const RAIN_CHAR: char = 'R';

const MIN_RAIN_COUNT: i32 = 1;
const MAX_RAIN_COUNT: i32 = 3;
const MIN_RAIN_SPEED: i32 = 1;
const MAX_RAIN_SPEED: i32 = 10;

fn main() {
    // ncurses 초기화
    initscr();
    raw();
    keypad(stdscr(), true);
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // 색상 초기화
    start_color();
    init_pair(1, COLOR_CYAN, COLOR_BLACK);
    attron(COLOR_PAIR(1));

    // 게임 변수 초기화
    let mut user_x = WIDTH / 2;
    let mut score: i32 = 0;
    let mut rains = Vec::new();

    // 게임 루프
    loop {
        // 비 생성
        let rain_count = rand::thread_rng().gen_range(MIN_RAIN_COUNT, MAX_RAIN_COUNT + 1);
        for _ in 0..rain_count {
            let rain_x = rand::thread_rng().gen_range(1, WIDTH + 1);
            let rain_speed = rand::thread_rng().gen_range(MIN_RAIN_SPEED, MAX_RAIN_SPEED + 1);
            rains.push((rain_x, 1, rain_speed));
        }

        // 비 이동
        rains = rains.iter().map(|&(x, y, speed)| (x, y + speed, speed)).collect();

        // 사용자 입력 처리
        let ch = getch();
        match ch {
            KEY_LEFT => {
                user_x = max(1, user_x - 1);
            }
            KEY_RIGHT => {
                user_x = min(WIDTH, user_x + 1);
            }
            _ => {}
        }

        // 화면 클리어
        clear();

        // 사용자 출력
        
        mvaddch(HEIGHT, user_x, USER_CHAR as chtype);

        // 비 출력 및 충돌 검사
        rains.retain(|&(x, y, speed)| {
            if y > HEIGHT {
                false
            } else {
                for c in RAIN_CHAR.to_string().chars() {
                    mvaddch(y, x, c as u32);
                }
                if x == user_x && y == HEIGHT {
                    score -= 1;
                } else {
                    score += 1;
                }
                true
            }
        });

        // 점수에 따라 비의 이동 속도 조절
        let sleep_time = Duration::from_millis(1000 / (score.abs() + 1) as u64);

        // 점수 출력
        mvprintw(0, 0, &format!("Score: {}", score));

        // 게임 종료 검사
        if score <= -10 {
            mvprintw(HEIGHT/2, (WIDTH-16)/2, "GAME OVER");
            refresh();
            thread::sleep(Duration::from_secs(2));
            break;
        }

        // 화면 업데이트
        refresh();
        thread::sleep(sleep_time);
    }

    // ncurses 종료
    endwin();

    // 최종 점수 출력
    println!("Final Score: {}", score);
}
