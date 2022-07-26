mod console;

use console::*;

// const SCREEN_WIDTH: i32 = 320;
// const SCREEN_HEIGHT: i32 = 180;

struct GameState {
    scroll_x: i32,
    scroll_y: i32,
    rate: [i32; 3],
}


static mut GAME_STATE: GameState = GameState {
    scroll_x: 0,
    scroll_y: 0,
    rate: [3, 2, 1],
};

#[no_mangle]
pub unsafe extern "C" fn init() {

}

#[no_mangle]
pub unsafe extern "C" fn update() {

    if button_right_held(0) != 0 {
        GAME_STATE.scroll_x -= 1;
    } else if button_left_held(0) != 0 {
        GAME_STATE.scroll_x += 1
    }

    if button_up_held(0) != 0 {
        GAME_STATE.scroll_y += 1;
    } else if button_down_held(0) != 0 {
        GAME_STATE.scroll_y -= 1
    }

    GAME_STATE.scroll_x = GAME_STATE.scroll_x.clamp(-192, 0);
    GAME_STATE.scroll_y = GAME_STATE.scroll_y.clamp(0, 100);

}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    clear_screen(0);
    let scroll_x = GAME_STATE.scroll_x;
    let scroll_y = GAME_STATE.scroll_y;

    GAME_STATE.rate.into_iter().enumerate().for_each(|(sprite_index, rate)| {
        let scroll_x = scroll_x / rate;

        let params = graphics_parameters(0, 0, sprite_index as i32, 0, 0, 0);
        sprite(params, 1, scroll_x, scroll_y -100);
    })
}
