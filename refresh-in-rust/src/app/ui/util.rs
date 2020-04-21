// From:
// https://github.com/Rigellute/spotify-tui/blob/master/src/ui/util.rs

// Make better use of space on small terminals
pub fn get_main_layout_margin(app: &App) -> u16 {
    if app.size.height > SMALL_TERMINAL_HEIGHT {
        1
    } else {
        0
    }
}
