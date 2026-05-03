use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Widget,
};

/// Braille animation frames for the spinner.
#[allow(dead_code)]
const SPINNER_FRAMES: &[char] = &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

/// An animated spinner widget that cycles through braille/dot frames based on
/// the application tick counter. Displays a message alongside the spinner.
#[allow(dead_code)]
pub struct Spinner<'a> {
    /// Current tick counter from the app state (used to select animation frame).
    pub tick: u64,
    /// Message to display alongside the spinner.
    pub message: &'a str,
}

impl<'a> Spinner<'a> {
    #[allow(dead_code)]
    pub fn new(tick: u64, message: &'a str) -> Self {
        Self { tick, message }
    }

    /// Get the current animation frame character based on the tick.
    #[allow(dead_code)]
    pub fn current_frame(&self) -> char {
        let index = (self.tick as usize) % SPINNER_FRAMES.len();
        SPINNER_FRAMES[index]
    }
}

impl<'a> Widget for Spinner<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let frame_char = self.current_frame();
        let spinner_style = Style::default().fg(Color::Cyan);
        let message_style = Style::default().fg(Color::Gray);

        let line = Line::from(vec![
            Span::styled(format!("{} ", frame_char), spinner_style),
            Span::styled(self.message, message_style),
        ]);

        line.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spinner_cycles_through_frames() {
        for i in 0..20u64 {
            let spinner = Spinner::new(i, "Loading...");
            let expected_index = (i as usize) % SPINNER_FRAMES.len();
            assert_eq!(spinner.current_frame(), SPINNER_FRAMES[expected_index]);
        }
    }

    #[test]
    fn spinner_renders_without_panic() {
        let spinner = Spinner::new(3, "Fetching quests...");
        let area = Rect::new(0, 0, 40, 1);
        let mut buf = Buffer::empty(area);
        spinner.render(area, &mut buf);

        let content: String = buf
            .content()
            .iter()
            .map(|cell| cell.symbol().chars().next().unwrap_or(' '))
            .collect();
        assert!(content.contains("Fetching quests..."));
    }

    #[test]
    fn spinner_zero_area_does_not_panic() {
        let spinner = Spinner::new(0, "test");
        let area = Rect::new(0, 0, 0, 0);
        let mut buf = Buffer::empty(area);
        spinner.render(area, &mut buf);
    }

    #[test]
    fn spinner_frame_wraps_correctly() {
        let spinner = Spinner::new(10, "msg");
        // 10 % 10 == 0, should wrap back to first frame.
        assert_eq!(spinner.current_frame(), SPINNER_FRAMES[0]);
    }
}
