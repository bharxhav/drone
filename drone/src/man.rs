use std::{io::IsTerminal, sync::Arc, time::Duration};

use clap::Args;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
        KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
    },
    execute,
};
use doc::{Documentation, NavItem, Page, Scope};
use nucleo_matcher::{
    Config, Matcher, Utf32Str,
    pattern::{AtomKind, CaseMatching, Normalization, Pattern},
};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Clear, List, ListItem, Padding, Paragraph, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Wrap,
    },
};
use sysexits::ExitCode;
use tui_markdown::{AlertKind, Options as MarkdownOptions, StyleSheet};

const TEXT: Color = Color::Reset;
const MUTED: Color = Color::Gray;
const SUBTLE: Color = Color::DarkGray;

#[derive(Clone)]
struct ManualStyle;

impl StyleSheet for ManualStyle {
    fn heading(&self, level: u8) -> Style {
        if level <= 2 {
            Style::default().fg(TEXT).bold()
        } else {
            Style::default().fg(MUTED).bold().italic()
        }
    }

    fn heading_marker(&self, _: u8) -> &str {
        ""
    }

    fn code(&self) -> Style {
        Style::default().fg(TEXT).bold()
    }

    fn code_block_fence(&self) -> &str {
        ""
    }

    fn link(&self) -> Style {
        Style::default().fg(TEXT).underlined()
    }

    fn blockquote(&self) -> Style {
        Style::default().fg(MUTED).italic()
    }

    fn metadata_block(&self) -> Style {
        Style::default().fg(MUTED)
    }

    fn math_inline(&self) -> Style {
        Style::default().fg(TEXT).italic()
    }

    fn math_display(&self) -> Style {
        Style::default().fg(TEXT)
    }

    fn alert(&self, _: AlertKind) -> Style {
        Style::default().fg(MUTED)
    }

    fn alert_icon(&self, _: AlertKind) -> &str {
        ""
    }

    fn table_header(&self) -> Style {
        Style::default().fg(TEXT).bold()
    }

    fn table_border(&self) -> Style {
        Style::default().fg(SUBTLE)
    }
}

#[derive(Args)]
pub struct Man {
    /// Emit structured JSON instead of interactive or human-readable output.
    #[arg(long)]
    json: bool,

    /// Documentation scope, such as `product getting-started overview`.
    scope: Vec<String>,
}

impl Man {
    pub async fn run(self) -> ExitCode {
        let client = doc::Client::new();
        let interactive =
            !self.json && std::io::stdin().is_terminal() && std::io::stdout().is_terminal();

        match client.get(Scope::new(self.scope)).await {
            Ok(documentation) if self.json => print_json(&documentation_json(&documentation)),
            Ok(documentation) if interactive => run_tui(&client, documentation).await,
            Ok(Documentation::Index(items)) => print_output(render_items(&items)),
            Ok(Documentation::Page(page)) => print_output(page.content.clone()),
            Err(error) => report(error),
        }
    }
}

#[derive(Clone)]
struct Entry {
    title: String,
    scope: Scope,
    context: String,
    hierarchy: Vec<String>,
    depth: usize,
}

impl Entry {
    fn searchable(&self) -> String {
        format!(
            "{} {} {} {}",
            self.title,
            self.scope,
            self.context,
            self.hierarchy.join(" ")
        )
    }
}

enum Row {
    Header { text: String, minimal: bool },
    Group { title: String, depth: usize },
    Page(usize),
    Divider,
}

enum View {
    Index,
    Page(Arc<Page>),
}

struct App {
    view: View,
    entries: Vec<Entry>,
    rows: Vec<Row>,
    visible: Vec<usize>,
    query: String,
    selected: usize,
    page_scroll: u16,
    page_height: usize,
    list_area: Rect,
    content_area: Rect,
    matcher: Matcher,
    quit: bool,
}

impl App {
    fn new(documentation: Documentation, navigation: Option<Vec<NavItem>>) -> Self {
        let (view, mut entries, mut rows) = match documentation {
            Documentation::Index(items) => {
                let (entries, rows) = collect_navigation(&items);
                (View::Index, entries, rows)
            }
            Documentation::Page(page) => (View::Page(page), Vec::new(), Vec::new()),
        };
        if let Some(items) = navigation {
            (entries, rows) = collect_navigation(&items);
        }
        let visible = (0..entries.len()).collect();

        Self {
            view,
            entries,
            rows,
            visible,
            query: String::new(),
            selected: 0,
            page_scroll: 0,
            page_height: 0,
            list_area: Rect::default(),
            content_area: Rect::default(),
            matcher: Matcher::new(Config::DEFAULT.match_paths()),
            quit: false,
        }
    }

    fn filter(&mut self) {
        if self.query.is_empty() {
            self.visible = (0..self.entries.len()).collect();
        } else {
            let pattern = Pattern::new(
                &self.query,
                CaseMatching::Smart,
                Normalization::Smart,
                AtomKind::Fuzzy,
            );
            let mut utf32 = Vec::new();
            let mut matches = self
                .entries
                .iter()
                .enumerate()
                .filter_map(|(index, entry)| {
                    let searchable = entry.searchable();
                    pattern
                        .score(Utf32Str::new(&searchable, &mut utf32), &mut self.matcher)
                        .map(|score| (index, score))
                })
                .collect::<Vec<_>>();
            matches.sort_unstable_by_key(|&(index, score)| (std::cmp::Reverse(score), index));
            self.visible = matches.into_iter().map(|(index, _)| index).collect();
        }
        self.selected = 0;
    }

    fn move_selection(&mut self, amount: isize) {
        if self.visible.is_empty() {
            self.selected = 0;
            return;
        }
        self.selected = self
            .selected
            .saturating_add_signed(amount)
            .min(self.visible.len() - 1);
    }

    fn selected_scope(&self) -> Option<Scope> {
        self.visible
            .get(self.selected)
            .and_then(|&index| self.entries.get(index))
            .map(|entry| entry.scope.clone())
    }

    fn adjacent_scope(&self, scope: &Scope, amount: isize) -> Option<Scope> {
        let current = self
            .entries
            .iter()
            .position(|entry| &entry.scope == scope)?;
        let next = current.checked_add_signed(amount)?;
        self.entries.get(next).map(|entry| entry.scope.clone())
    }

    fn page_neighbour(&self, page: &Page, amount: isize) -> Option<Scope> {
        match amount {
            -1 => page
                .prev
                .clone()
                .or_else(|| self.adjacent_scope(&page.scope, -1)),
            1 => page
                .next
                .clone()
                .or_else(|| self.adjacent_scope(&page.scope, 1)),
            _ => None,
        }
    }

    fn scroll_page(&mut self, amount: i32) {
        let max = self
            .page_height
            .saturating_sub(self.content_area.height as usize) as u16;
        self.page_scroll = self
            .page_scroll
            .saturating_add_signed(amount as i16)
            .min(max);
    }

    async fn handle_key(&mut self, client: &doc::Client, key: KeyEvent) -> Result<(), doc::Error> {
        if key.kind != KeyEventKind::Press {
            return Ok(());
        }
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.quit = true;
            return Ok(());
        }

        match &self.view {
            View::Index => match key.code {
                KeyCode::Esc => self.quit = true,
                KeyCode::Up => self.move_selection(-1),
                KeyCode::Down => self.move_selection(1),
                KeyCode::PageUp => self.move_selection(-(self.list_area.height as isize / 2)),
                KeyCode::PageDown => self.move_selection(self.list_area.height as isize / 2),
                KeyCode::Home => self.selected = 0,
                KeyCode::End => self.selected = self.visible.len().saturating_sub(1),
                KeyCode::Backspace => {
                    self.query.pop();
                    self.filter();
                }
                KeyCode::Char(character)
                    if !key
                        .modifiers
                        .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
                {
                    self.query.push(character);
                    self.filter();
                }
                KeyCode::Enter => {
                    if let Some(scope) = self.selected_scope() {
                        match client.get(scope).await? {
                            Documentation::Page(page) => {
                                self.view = View::Page(page);
                                self.page_scroll = 0;
                            }
                            Documentation::Index(items) => {
                                (self.entries, self.rows) = collect_navigation(&items);
                                self.query.clear();
                                self.filter();
                            }
                        }
                    }
                }
                _ => {}
            },
            View::Page(page) => match key.code {
                KeyCode::Esc | KeyCode::Left | KeyCode::Backspace => {
                    if self.entries.is_empty() {
                        self.quit = true;
                    } else {
                        self.view = View::Index;
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => self.scroll_page(-1),
                KeyCode::Down | KeyCode::Char('j') => self.scroll_page(1),
                KeyCode::PageUp => self.scroll_page(-(self.content_area.height as i32)),
                KeyCode::PageDown | KeyCode::Char(' ') => {
                    self.scroll_page(self.content_area.height as i32)
                }
                KeyCode::Home | KeyCode::Char('g') => self.page_scroll = 0,
                KeyCode::End | KeyCode::Char('G') => self.scroll_page(i32::MAX),
                KeyCode::Char('n') | KeyCode::Char('p') => {
                    let amount = if key.code == KeyCode::Char('n') {
                        1
                    } else {
                        -1
                    };
                    let Some(scope) = self.page_neighbour(page, amount) else {
                        return Ok(());
                    };
                    if let Documentation::Page(page) = client.get(scope).await? {
                        self.view = View::Page(page);
                        self.page_scroll = 0;
                    }
                }
                _ => {}
            },
        }
        Ok(())
    }

    async fn handle_mouse(
        &mut self,
        client: &doc::Client,
        mouse: MouseEvent,
    ) -> Result<(), doc::Error> {
        match mouse.kind {
            MouseEventKind::ScrollUp => match self.view {
                View::Index => self.move_selection(-3),
                View::Page(_) => self.scroll_page(-3),
            },
            MouseEventKind::ScrollDown => match self.view {
                View::Index => self.move_selection(3),
                View::Page(_) => self.scroll_page(3),
            },
            MouseEventKind::Down(MouseButton::Left)
                if matches!(self.view, View::Index)
                    && self.list_area.contains((mouse.column, mouse.row).into()) =>
            {
                let row = usize::from(mouse.row.saturating_sub(self.list_area.y));
                let offset =
                    centered_offset(self.selected, self.visible.len(), self.list_area.height);
                let selected = offset + row;
                if selected < self.visible.len() {
                    let open = selected == self.selected;
                    self.selected = selected;
                    if open
                        && let Some(scope) = self.selected_scope()
                        && let Documentation::Page(page) = client.get(scope).await?
                    {
                        self.view = View::Page(page);
                        self.page_scroll = 0;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}

async fn run_tui(client: &doc::Client, documentation: Documentation) -> ExitCode {
    // The alternate screen fills the terminal and restores the shell exactly
    // as it was on exit, matching full-screen pickers such as fzf.
    let mut terminal = match ratatui::try_init() {
        Ok(terminal) => terminal,
        Err(_) => return ExitCode::IoErr,
    };
    if execute!(std::io::stdout(), EnableMouseCapture).is_err() {
        let _ = ratatui::try_restore();
        return ExitCode::IoErr;
    }

    let result = run_event_loop(&mut terminal, client, documentation).await;
    if result.is_err() {
        let _ = terminal.clear();
    }
    let mouse_result = execute!(std::io::stdout(), DisableMouseCapture);
    let restore_result = ratatui::try_restore();

    match result {
        Err(TuiError::Doc(error)) => {
            if mouse_result.is_err() || restore_result.is_err() {
                ExitCode::IoErr
            } else {
                report(error)
            }
        }
        Err(TuiError::Io) | Ok(()) if mouse_result.is_err() || restore_result.is_err() => {
            ExitCode::IoErr
        }
        Err(TuiError::Io) => ExitCode::IoErr,
        Ok(()) => ExitCode::Ok,
    }
}

enum TuiError {
    Io,
    Doc(doc::Error),
}

impl From<std::io::Error> for TuiError {
    fn from(_: std::io::Error) -> Self {
        Self::Io
    }
}

impl From<doc::Error> for TuiError {
    fn from(error: doc::Error) -> Self {
        Self::Doc(error)
    }
}

async fn run_event_loop(
    terminal: &mut DefaultTerminal,
    client: &doc::Client,
    documentation: Documentation,
) -> Result<(), TuiError> {
    let navigation = if let Documentation::Page(page) = &documentation {
        load_navigation(client, &page.scope).await
    } else {
        None
    };
    let mut app = App::new(documentation, navigation);

    while !app.quit {
        terminal.draw(|frame| draw(frame, &mut app))?;
        if !event::poll(Duration::from_millis(250))? {
            continue;
        }
        match event::read()? {
            Event::Key(key) => app.handle_key(client, key).await?,
            Event::Mouse(mouse) => app.handle_mouse(client, mouse).await?,
            Event::Resize(_, _) => {}
            _ => {}
        }
    }
    Ok(())
}

async fn load_navigation(client: &doc::Client, scope: &Scope) -> Option<Vec<NavItem>> {
    let segments = scope.to_string();
    let mut segments = segments.split('/').collect::<Vec<_>>();
    segments.pop();

    while !segments.is_empty() {
        if let Ok(Documentation::Index(items)) = client.get(Scope::new(&segments)).await {
            return Some(items);
        }
        segments.pop();
    }
    None
}

fn draw(frame: &mut Frame<'_>, app: &mut App) {
    frame.render_widget(Clear, frame.area());
    if let View::Page(page) = &app.view {
        let page = Arc::clone(page);
        draw_page(frame, app, &page);
    } else {
        draw_index(frame, app);
    }
}

fn draw_index(frame: &mut Frame<'_>, app: &mut App) {
    let area = frame.area();
    let [title, search_area, list, footer] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Min(5),
        Constraint::Length(1),
    ])
    .areas(area);

    frame.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled("  Drone manual", Style::default().fg(TEXT).bold()),
            Span::styled("  esc", Style::default().fg(MUTED)),
        ])),
        title,
    );

    let search = if app.query.is_empty() {
        Line::styled(
            "      Search documentation",
            Style::default().fg(MUTED).italic(),
        )
    } else {
        Line::from(vec![
            Span::raw("      "),
            Span::styled(&app.query, Style::default().fg(TEXT)),
            Span::styled("▏", Style::default().fg(MUTED)),
        ])
    };
    frame.render_widget(Paragraph::new(search), search_area);

    let list_block = Block::new().padding(Padding::new(1, 3, 0, 0));
    let inner = list_block.inner(list);
    app.list_area = inner;

    if app.visible.is_empty() {
        frame.render_widget(
            Paragraph::new("No documentation matches your search.")
                .fg(MUTED)
                .block(list_block),
            list,
        );
    } else {
        let height = inner.height as usize;
        let selected_entry = app.visible[app.selected];
        let (items, scroll_position) = structured_rows(app, selected_entry, height);
        frame.render_widget(List::new(items).block(list_block), list);

        let scroll_length = visible_rows(app).len();
        let mut scrollbar = ScrollbarState::new(scroll_length).position(scroll_position);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None)
                .track_symbol(Some("│"))
                .thumb_symbol("┃"),
            list.inner(Margin::new(0, 1)),
            &mut scrollbar,
        );
    }

    let count = format!("{} of {}", app.visible.len(), app.entries.len());
    frame.render_widget(
        Paragraph::new(help_line(&[
            ("↑↓/jk", "navigate"),
            ("enter", "open"),
            ("esc", "close"),
            (&count, ""),
        ]))
        .alignment(Alignment::Center),
        footer,
    );
}

fn draw_page(frame: &mut Frame<'_>, app: &mut App, page: &Arc<Page>) {
    let area = frame.area();
    let block = Block::bordered()
        .title(format!(" {} ", page.scope))
        .title_style(Style::default().fg(TEXT).bold())
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(SUBTLE))
        .padding(Padding::horizontal(2));
    let inner = block.inner(area);
    let [content, footer] =
        Layout::vertical([Constraint::Min(3), Constraint::Length(1)]).areas(inner);
    app.content_area = content;

    let markdown_options = MarkdownOptions::new(ManualStyle);
    let markdown = tui_markdown::from_str_with_options(&page.content, &markdown_options);
    app.page_height = markdown.height();
    app.scroll_page(0);
    frame.render_widget(
        Paragraph::new(markdown)
            .wrap(Wrap { trim: false })
            .scroll((app.page_scroll, 0)),
        content,
    );

    let previous = app.page_neighbour(page, -1).is_some();
    let next = app.page_neighbour(page, 1).is_some();
    let controls = page_controls(app.entries.is_empty(), previous, next);
    frame.render_widget(
        Paragraph::new(help_line(&controls)).alignment(Alignment::Center),
        footer,
    );

    let max_scroll = app.page_height.saturating_sub(content.height as usize);
    let mut scrollbar =
        ScrollbarState::new(max_scroll.saturating_add(1)).position(usize::from(app.page_scroll));
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None)
            .track_symbol(Some("│"))
            .thumb_symbol("┃"),
        content,
        &mut scrollbar,
    );
    frame.render_widget(block, area);
}

fn centered_offset(selected: usize, length: usize, height: u16) -> usize {
    let height = usize::from(height);
    selected
        .saturating_sub(height / 2)
        .min(length.saturating_sub(height))
}

fn structured_rows(
    app: &App,
    selected_entry: usize,
    height: usize,
) -> (Vec<ListItem<'static>>, usize) {
    let rows = visible_rows(app);
    let selected_row = rows
        .iter()
        .position(|row| matches!(row, Row::Page(index) if *index == selected_entry))
        .unwrap_or_default();
    let row_offset = centered_offset(selected_row, rows.len(), height as u16);

    (
        rows.into_iter()
            .skip(row_offset)
            .take(height)
            .map(|row| match row {
                Row::Header { text, minimal } => ListItem::new(Line::styled(
                    if *minimal {
                        format!("  {text}")
                    } else {
                        format!("  {text}").to_uppercase()
                    },
                    Style::default().fg(TEXT).bold(),
                )),
                Row::Group { title, depth } => ListItem::new(Line::styled(
                    format!("{}{}", "  ".repeat(*depth), title),
                    Style::default().fg(MUTED),
                )),
                Row::Divider => ListItem::new(""),
                Row::Page(index) => entry_row(&app.entries[*index], *index == selected_entry),
            })
            .collect(),
        selected_row,
    )
}

fn visible_rows(app: &App) -> Vec<&Row> {
    if app.query.is_empty() {
        return app.rows.iter().collect();
    }

    let visible = app
        .visible
        .iter()
        .copied()
        .collect::<std::collections::HashSet<_>>();
    let mut include = vec![false; app.rows.len()];
    let mut ancestors = Vec::new();

    for (row_index, row) in app.rows.iter().enumerate() {
        match row {
            Row::Header { .. } => {
                ancestors.clear();
                ancestors.push(row_index);
            }
            Row::Group { depth, .. } => {
                ancestors.retain(|&index| {
                    matches!(&app.rows[index], Row::Header { .. })
                        || matches!(&app.rows[index], Row::Group { depth: parent, .. } if parent < depth)
                });
                ancestors.push(row_index);
            }
            Row::Page(index) if visible.contains(index) => {
                include[row_index] = true;
                for &ancestor in &ancestors {
                    include[ancestor] = true;
                }
            }
            Row::Divider => ancestors.clear(),
            Row::Page(_) => {}
        }
    }

    app.rows
        .iter()
        .enumerate()
        .filter_map(|(index, row)| include[index].then_some(row))
        .collect()
}

fn entry_row(entry: &Entry, selected: bool) -> ListItem<'static> {
    let indent = "  ".repeat(entry.depth);
    let spans = vec![Span::styled(
        format!("   {indent}{}", entry.title),
        Style::default().fg(if selected { TEXT } else { Color::Reset }),
    )];
    ListItem::new(Line::from(spans)).style(if selected {
        Style::default()
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    })
}

fn help_line(items: &[(&str, &str)]) -> Line<'static> {
    let mut spans = Vec::new();
    for (index, (key, description)) in items.iter().enumerate() {
        if index > 0 {
            spans.push(Span::styled("  •  ", Style::default().fg(SUBTLE)));
        }
        spans.push(Span::styled(
            (*key).to_owned(),
            Style::default().fg(TEXT).bold(),
        ));
        if !description.is_empty() {
            spans.push(Span::styled(
                format!(" {description}"),
                Style::default().fg(MUTED),
            ));
        }
    }
    Line::from(spans)
}

fn page_controls(root_page: bool, previous: bool, next: bool) -> Vec<(&'static str, &'static str)> {
    let mut controls = vec![
        if root_page {
            ("esc", "close")
        } else {
            ("←/esc", "back")
        },
        ("wheel/jk", "scroll"),
    ];
    if previous {
        controls.push(("p", "previous"));
    }
    if next {
        controls.push(("n", "next"));
    }
    controls
}

fn collect_navigation(items: &[NavItem]) -> (Vec<Entry>, Vec<Row>) {
    let mut entries = Vec::new();
    let mut rows = Vec::new();
    collect_level(items, 0, &mut Vec::new(), &mut entries, &mut rows);
    if !entries.is_empty() && entries.iter().all(|entry| entry.hierarchy.is_empty()) {
        infer_hierarchy(&mut entries, &mut rows);
    }
    (entries, rows)
}

fn infer_hierarchy(entries: &mut [Entry], rows: &mut Vec<Row>) {
    rows.clear();
    let mut previous = Vec::<String>::new();

    for (index, entry) in entries.iter_mut().enumerate() {
        let scope = entry.scope.to_string();
        let segments = scope.split('/').collect::<Vec<_>>();
        let hierarchy = segments
            .get(..segments.len().saturating_sub(1))
            .unwrap_or_default()
            .iter()
            .map(|segment| humanize(segment))
            .collect::<Vec<_>>();
        let common = hierarchy
            .iter()
            .zip(&previous)
            .take_while(|(left, right)| left == right)
            .count();
        for (depth, title) in hierarchy.iter().enumerate().skip(common) {
            rows.push(Row::Group {
                title: title.clone(),
                depth,
            });
        }
        entry.depth = hierarchy.len();
        entry.hierarchy = hierarchy.clone();
        rows.push(Row::Page(index));
        previous = hierarchy;
    }
}

fn humanize(segment: &str) -> String {
    segment
        .strip_suffix("-v2-resources")
        .unwrap_or(segment)
        .split('-')
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut characters = word.chars();
            characters
                .next()
                .map(char::to_uppercase)
                .into_iter()
                .flatten()
                .chain(characters)
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn collect_level(
    items: &[NavItem],
    depth: usize,
    hierarchy: &mut Vec<String>,
    entries: &mut Vec<Entry>,
    rows: &mut Vec<Row>,
) {
    for item in items {
        match item {
            NavItem::Header { text, minimal } => rows.push(Row::Header {
                text: text.clone(),
                minimal: *minimal,
            }),
            NavItem::Section { title, items, .. } => {
                rows.push(Row::Group {
                    title: title.clone(),
                    depth,
                });
                hierarchy.push(title.clone());
                collect_level(items, depth + 1, hierarchy, entries, rows);
                hierarchy.pop();
            }
            NavItem::PageGroup { title, pages } => {
                rows.push(Row::Group {
                    title: title.clone(),
                    depth,
                });
                hierarchy.push(title.clone());
                collect_level(pages, depth + 1, hierarchy, entries, rows);
                hierarchy.pop();
            }
            NavItem::PageLink {
                text,
                scope,
                context,
                ..
            } => {
                let index = entries.len();
                entries.push(Entry {
                    title: text.clone(),
                    scope: scope.clone(),
                    context: context.clone(),
                    hierarchy: hierarchy.clone(),
                    depth,
                });
                rows.push(Row::Page(index));
            }
            NavItem::Divider => rows.push(Row::Divider),
        }
    }
}

fn render_items(items: &[NavItem]) -> String {
    collect_navigation(items)
        .0
        .into_iter()
        .map(|entry| format!("{}\t{}\n", entry.scope, entry.title))
        .collect()
}

fn documentation_json(documentation: &Documentation) -> serde_json::Value {
    match documentation {
        Documentation::Index(items) => serde_json::json!({
            "type": "index",
            "items": collect_navigation(items).0
                .into_iter()
                .map(|entry| serde_json::json!({
                    "scope": entry.scope.to_string(),
                    "text": entry.title,
                    "context": entry.context,
                    "hierarchy": entry.hierarchy,
                }))
                .collect::<Vec<_>>(),
        }),
        Documentation::Page(page) => serde_json::json!({
            "type": "page",
            "scope": page.scope.to_string(),
            "content": page.content,
            "preview": page.preview,
            "previous": page.prev.as_ref().map(ToString::to_string),
            "next": page.next.as_ref().map(ToString::to_string),
        }),
    }
}

fn print_json(value: &serde_json::Value) -> ExitCode {
    match serde_json::to_writer_pretty(std::io::stdout().lock(), value) {
        Ok(()) => {
            println!();
            ExitCode::Ok
        }
        Err(_) => ExitCode::IoErr,
    }
}

fn print_output(output: String) -> ExitCode {
    print!("{output}");
    ExitCode::Ok
}

fn report(error: doc::Error) -> ExitCode {
    eprintln!("drone: {error}");
    ExitCode::Software
}
