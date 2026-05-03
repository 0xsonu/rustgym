use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Widget,
};

/// A status bar widget that renders screen name (left), username + XP (center),
/// and key hints (right) on a distinct background.
pub struct StatusBar<'a> {
    /// Name of the current screen (displayed left-aligned).
    pub screen_name: &'a str,
    /// Authenticated username (displayed center).
    pub username: &'a str,
    /// User's current XP (displayed center alongside username).
    pub xp: i32,
    /// Key hints from the current screen's `key_hints()` method.
    /// Each tuple is (key, description), e.g. ("j/k", "navigate").
    pub key_hints: Vec<(&'a str, &'a str)>,
}

impl<'a> StatusBar<'a> {
    pub fn new(
        screen_name: &'a str,
        username: &'a str,
        xp: i32,
        key_hints: Vec<(&'a str, &'a str)>,
    ) -> Self {
        Self {
            screen_name,
            username,
            xp,
            key_hints,
        }
    }

    /// Format the key hints into a single string like "j/k: nav  Enter: select  q: quit".
    fn format_hints(&self) -> String {
        self.key_hints
            .iter()
            .map(|(key, desc)| format!("{}: {}", key, desc))
            .collect::<Vec<_>>()
            .join("  ")
    }

    /// Format the center user info section.
    fn format_user_info(&self) -> String {
        if self.username.is_empty() {
            String::new()
        } else {
            format!("{} · {} XP", self.username, self.xp)
        }
    }
}

impl<'a> Widget for StatusBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let bg_style = Style::default().bg(Color::DarkGray).fg(Color::White);

        // Fill the entire area with the background color.
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                buf[(x, y)].set_style(bg_style);
            }
        }

        let width = area.width as usize;
        if width == 0 || area.height == 0 {
            return;
        }

        let left = format!(" {}", self.screen_name);
        let center = self.format_user_info();
        let right = format!("{} ", self.format_hints());

        // Render left-aligned screen name.
        let left_line = Line::from(Span::styled(&left, bg_style)).alignment(Alignment::Left);
        let left_area = Rect::new(area.x, area.y, area.width, 1);
        left_line.render(left_area, buf);

        // Render center user info.
        if !center.is_empty() {
            let center_line =
                Line::from(Span::styled(&center, bg_style)).alignment(Alignment::Center);
            let center_area = Rect::new(area.x, area.y, area.width, 1);
            center_line.render(center_area, buf);
        }

        // Render right-aligned key hints.
        if !right.trim().is_empty() {
            let right_line =
                Line::from(Span::styled(&right, bg_style)).alignment(Alignment::Right);
            let right_area = Rect::new(area.x, area.y, area.width, 1);
            right_line.render(right_area, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_bar_renders_without_panic() {
        let bar = StatusBar::new("Quests", "alice", 1500, vec![("j/k", "nav"), ("q", "quit")]);
        let area = Rect::new(0, 0, 80, 1);
        let mut buf = Buffer::empty(area);
        bar.render(area, &mut buf);

        // Verify the buffer contains the screen name.
        let content: String = buf
            .content()
            .iter()
            .map(|cell| cell.symbol().chars().next().unwrap_or(' '))
            .collect();
        assert!(content.contains("Quests"));
    }

    #[test]
    fn status_bar_empty_username_skips_center() {
        let bar = StatusBar::new("Login", "", 0, vec![("Tab", "switch"), ("Enter", "submit")]);
        let area = Rect::new(0, 0, 60, 1);
        let mut buf = Buffer::empty(area);
        bar.render(area, &mut buf);

        let content: String = buf
            .content()
            .iter()
            .map(|cell| cell.symbol().chars().next().unwrap_or(' '))
            .collect();
        assert!(content.contains("Login"));
        // Should not contain XP info.
        assert!(!content.contains("XP"));
    }

    #[test]
    fn status_bar_zero_area_does_not_panic() {
        let bar = StatusBar::new("Test", "user", 100, vec![]);
        let area = Rect::new(0, 0, 0, 0);
        let mut buf = Buffer::empty(area);
        bar.render(area, &mut buf);
    }
}
