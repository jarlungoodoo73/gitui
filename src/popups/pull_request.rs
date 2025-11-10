use crate::components::{
	visibility_blocking, CommandBlocking, CommandInfo, Component,
	DrawableComponent, EventState,
};
use crate::{
	app::Environment,
	keys::{key_match, SharedKeyConfig},
	strings, ui,
};
use anyhow::Result;
use crossterm::event::Event;
use ratatui::{
	layout::{Alignment, Rect},
	text::Span,
	widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
	Frame,
};
use ui::style::SharedTheme;

pub struct PullRequestPopup {
	visible: bool,
	theme: SharedTheme,
	key_config: SharedKeyConfig,
}

impl DrawableComponent for PullRequestPopup {
	fn draw(&self, f: &mut Frame, _rect: Rect) -> Result<()> {
		if !self.visible {
			return Ok(());
		}

		let area = ui::centered_rect_absolute(50, 10, f.area());

		f.render_widget(Clear, area);
		f.render_widget(
			Paragraph::new(vec![
				ratatui::text::Line::from(""),
				ratatui::text::Line::from(
					"Pull Request functionality is not yet",
				),
				ratatui::text::Line::from(
					"implemented. This feature would allow",
				),
				ratatui::text::Line::from(
					"creating pull requests to GitHub/GitLab",
				),
				ratatui::text::Line::from("directly from gitui."),
				ratatui::text::Line::from(""),
				ratatui::text::Line::from(
					"Press Esc to close this dialog.",
				),
			])
			.block(
				Block::default()
					.title(Span::styled(
						"Pull Request (Coming Soon)",
						self.theme.title(true),
					))
					.borders(Borders::ALL)
					.border_type(BorderType::Thick)
					.border_style(self.theme.block(true)),
			)
			.alignment(Alignment::Center)
			.wrap(Wrap { trim: true }),
			area,
		);

		Ok(())
	}
}

impl Component for PullRequestPopup {
	fn commands(
		&self,
		out: &mut Vec<CommandInfo>,
		_force_all: bool,
	) -> CommandBlocking {
		out.push(CommandInfo::new(
			strings::commands::close_popup(&self.key_config),
			true,
			self.visible,
		));

		visibility_blocking(self)
	}

	fn event(&mut self, ev: &Event) -> Result<EventState> {
		if self.visible {
			if let Event::Key(e) = ev {
				if key_match(e, self.key_config.keys.exit_popup) {
					self.hide();
				}
			}
			return Ok(EventState::Consumed);
		}
		Ok(EventState::NotConsumed)
	}

	fn is_visible(&self) -> bool {
		self.visible
	}

	fn hide(&mut self) {
		self.visible = false;
	}

	fn show(&mut self) -> Result<()> {
		self.visible = true;
		Ok(())
	}
}

impl PullRequestPopup {
	pub fn new(env: &Environment) -> Self {
		Self {
			visible: false,
			theme: env.theme.clone(),
			key_config: env.key_config.clone(),
		}
	}

	pub fn open(&mut self) -> Result<()> {
		self.show()
	}
}
