// Used the patterns and some code from here:
// https://github.com/Rigellute/spotify-tui/blob/master/src/ui/mod.rs
use crate::app::state::AppState as App;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Gauge, Paragraph, Row, SelectableList, Table, Text, Widget},
    Frame,
};

pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &App)
where
  B: Backend,
{
  let margin = util::get_main_layout_margin(app);
  let parent_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(
      [
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(6),
      ]
      .as_ref(),
    )
    .margin(margin)
    .split(f.size());

  // Search input and help
  draw_input_and_help_box(f, app, parent_layout[0]);

  // Nested main block with potential routes
//   draw_routes(f, app, parent_layout[1]);

  // Currently playing
//   draw_playbar(f, app, parent_layout[2]);

  // Possibly draw confirm dialog
//   draw_dialog(f, app);
}

pub fn draw_input_and_help_box<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
  B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(layout_chunk);

    let current_route = app.get_current_route();

    let highlight_state = (
        current_route.active_block == ActiveBlock::Input,
        current_route.hovered_block == ActiveBlock::Input,
    );

    let input_string: String = app.input.iter().collect();
    Paragraph::new([Text::raw(&input_string)].iter())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Command")
                .title_style(get_color(highlight_state, app.user_config.theme))
                .border_style(get_color(highlight_state, app.user_config.theme)),
        )
        .render(f, chunks[0]);

    let help_block_text = if app.is_loading {
        (app.user_config.theme.hint, "Loading...")
    } else {
        (app.user_config.theme.inactive, "Type ?")
    };

    let block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(help_block_text.0))
        .title_style(Style::default().fg(help_block_text.0));

    Paragraph::new([Text::raw(help_block_text.1)].iter())
        .block(block)
        .style(Style::default().fg(help_block_text.0))
        .render(f, chunks[1]);
}
