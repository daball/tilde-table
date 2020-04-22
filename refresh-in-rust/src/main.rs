mod shell;
mod app;
mod features;

// use std::io::{stdout};
// use app::state::AppState;
// use app::state::AppTerminal;

#[cfg(feature="tui")] use tokio::io::{Result, Box};
#[cfg(feature="tui")] use tokio::sync::Mutex;
#[cfg(feature="tui")] use std::sync::Arc;

#[cfg(feature="tui")]
fn close_application() -> Result<()> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    // execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

#[cfg(feature="tui")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create app state
    let mut app_state = Arc::new(Mutex::new(AppState::new()));
    let cloned_app_state = Arc::clone(&app_state);
    // process command line arguments
    // do repl
    start_ui(&cloned_app_state).await?;
    // do web server
    Ok(())
}

#[cfg(not(feature="tui"))]
fn main() {
    // create app state
    // process command line arguments
    // do repl
    app::repl::repl();
}

#[cfg(feature="tui")]
async fn start_ui(app_state: &Arc<Mutex<AppState>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut app_terminal = AppTerminal::create();
    let mut terminal = app_terminal.terminal;
    terminal.hide_cursor()?;
    // let events = event::Events::new(user_config.behavior.tick_rate_milliseconds);

    let mut is_first_render = true;
    // let cloned_app_state = Arc::clone(&app_state);
    // repl(&cloned_app_state).await?;

    // terminal.draw(|mut f| {
    //     let chunks = Layout::default()
    //         .direction(Direction::Vertical)
    //         .margin(1)
    //         .constraints(
    //             [
    //                 Constraint::Percentage(10),
    //                 Constraint::Percentage(80),
    //                 Constraint::Percentage(10)
    //             ].as_ref()
    //         )
    //         .split(f.size());
    //     let block = Block::default()
    //          .title("Command")
    //          .borders(Borders::ALL);
    //     f.render_widget(block, chunks[0]);
    //     let block = Block::default()
    //          .title("Result")
    //          .borders(Borders::ALL);
    //     f.render_widget(block, chunks[1]);
    // });
    // print_version();
    loop {
        let mut app_state = app_state.lock().await;
        // read
        // read();
        // let mut cmdLine = CommandLine::new(PS1);
        // let cmd = cmdLine.read();
        break; // broke!
        // eval/print
        // if { let e = DispatchHandler.execute; e(&cmd) } {
        //     continue; // loop
        // } else {
        //     break; // exit loop
        // }
    }
    terminal.show_cursor()?;
    Ok(())
}

// async fn repl(app_state: &Arc<Mutex<AppState>>) -> Result<(), Box<dyn std::error::Error>> {
//     // terminal.draw(|mut f| {
//     //     let chunks = Layout::default()
//     //         .direction(Direction::Vertical)
//     //         .margin(1)
//     //         .constraints(
//     //             [
//     //                 Constraint::Percentage(10),
//     //                 Constraint::Percentage(80),
//     //                 Constraint::Percentage(10)
//     //             ].as_ref()
//     //         )
//     //         .split(f.size());
//     //     let block = Block::default()
//     //          .title("Command")
//     //          .borders(Borders::ALL);
//     //     f.render_widget(block, chunks[0]);
//     //     let block = Block::default()
//     //          .title("Result")
//     //          .borders(Borders::ALL);
//     //     f.render_widget(block, chunks[1]);
//     // });
//     // print_version();
//     loop {
//         let mut app_state = app_state.lock().await;
//         // read
//         // read();
//         // let mut cmdLine = CommandLine::new(PS1);
//         // let cmd = cmdLine.read();
//         break;
//         // eval/print
//         // if { let e = DispatchHandler.execute; e(&cmd) } {
//         //     continue; // loop
//         // } else {
//         //     break; // exit loop
//         // }
//     }
//     // terminal.show_cursor()?;
//     Ok(())
// }
