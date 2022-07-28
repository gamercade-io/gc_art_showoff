mod console;

use console::*;

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

// Required to expose this.
#[no_mangle]
pub extern "C" fn init() {}

/// # Safety
/// This function calls external Gamercade Api Functions
/// This function mutates global state, however it is strictly
/// called from the host program in a single threaded manner.
#[no_mangle]
pub unsafe extern "C" fn update() {
    // Handle inputs for scrolling the view right/left and up/down.
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

    // Clamp the sprites to the edge of the screen.
    GAME_STATE.scroll_x = GAME_STATE.scroll_x.clamp(-192, 0);
    GAME_STATE.scroll_y = GAME_STATE.scroll_y.clamp(0, 100);
}

/// # Safety
/// This function calls external Gamercade Api Functions
#[no_mangle]
pub unsafe extern "C" fn draw() {
    clear_screen(0);
    let scroll_x = GAME_STATE.scroll_x;
    let scroll_y = GAME_STATE.scroll_y;

    // Iterates over each of the background sprites, which are already
    // stored in the rom at indicies 0, 1, and 2.
    GAME_STATE
        .rate
        .into_iter()
        .enumerate()
        .for_each(|(sprite_index, rate)| {
            let scroll_x = scroll_x / rate;

            // Actually draw the sprites with default parameters.
            let params = graphics_parameters(0, 0, sprite_index as i32, 0, 0, 0);
            sprite(params, 1, scroll_x, scroll_y - 100);
        })
}
