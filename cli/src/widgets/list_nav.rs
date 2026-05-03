use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};

/// Trait for items that can be displayed in the navigable list.
#[allow(dead_code)]
pub trait ListItem {
    /// Return the display text for this item.
    fn display_text(&self) -> &str;
}

/// Blanket implementation for String.
impl ListItem for String {
    fn display_text(&self) -> &str {
        self.as_str()
    }
}

/// Blanket implementation for &str via a newtype isn't needed since we use
/// generics. Instead, provide a simple wrapper for convenience.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TextItem(pub String);

impl ListItem for TextItem {
    fn display_text(&self) -> &str {
        &self.0
    }
}

/// State for the navigable list, tracking selection and scroll position.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NavListState {
    /// Currently selected item index.
    pub selected: usize,
    /// Scroll offset (index of the first visible item).
    pub offset: usize,
}

impl NavListState {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            selected: 0,
            offset: 0,
        }
    }

    /// Move selection down, clamping to the last item.
    #[allow(dead_code)]
    pub fn move_down(&mut self, item_count: usize) {
        if item_count == 0 {
            return;
        }
        self.selected = (self.selected + 1).min(item_count - 1);
    }

    /// Move selection up, clamping to the first item.
    #[allow(dead_code)]
    pub fn move_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    /// Adjust the scroll offset so the selected item is visible within the
    /// given viewport height.
    #[allow(dead_code)]
    pub fn adjust_scroll(&mut self, viewport_height: usize) {
        if viewport_height == 0 {
            return;
        }
        // If selected is above the viewport, scroll up.
        if self.selected < self.offset {
            self.offset = self.selected;
        }
        // If selected is below the viewport, scroll down.
        if self.selected >= self.offset + viewport_height {
            self.offset = self.selected - viewport_height + 1;
        }
    }
}

impl Default for NavListState {
    fn default() -> Self {
        Self::new()
    }
}

/// A reusable navigable list widget with vim-style scrolling.
///
/// Renders a list of items with:
/// - `>` indicator on the selected item
/// - Background highlight on the selected item
/// - Viewport-based scrolling (only visible items are rendered)
#[allow(dead_code)]
pub struct NavList<'a, T: ListItem> {
    /// The items to display.
    pub items: &'a [T],
    /// Current list state (selection + scroll offset).
    pub state: &'a NavListState,
}

impl<'a, T: ListItem> NavList<'a, T> {
    #[allow(dead_code)]
    pub fn new(items: &'a [T], state: &'a NavListState) -> Self {
        Self { items, state }
    }
}

impl<'a, T: ListItem> Widget for NavList<'a, T> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 || self.items.is_empty() {
            return;
        }

        let viewport_height = area.height as usize;
        let start = self.state.offset;
        let end = (start + viewport_height).min(self.items.len());

        let normal_style = Style::default().fg(Color::White);
        let selected_style = Style::default()
            .fg(Color::White)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::BOLD);

        for (row_idx, item_idx) in (start..end).enumerate() {
            let y = area.y + row_idx as u16;
            if y >= area.y + area.height {
                break;
            }

            let is_selected = item_idx == self.state.selected;
            let style = if is_selected {
                selected_style
            } else {
                normal_style
            };

            // Build the line: "> item text" or "  item text"
            let indicator = if is_selected { "> " } else { "  " };
            let text = self.items[item_idx].display_text();

            let line = Line::from(vec![
                Span::styled(indicator, style),
                Span::styled(text, style),
            ]);

            let row_area = Rect::new(area.x, y, area.width, 1);

            // Fill background for selected row.
            if is_selected {
                for x in area.x..area.x + area.width {
                    buf[(x, y)].set_style(selected_style);
                }
            }

            line.render(row_area, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nav_list_state_move_down_clamps() {
        let mut state = NavListState::new();
        state.move_down(3); // 0 -> 1
        assert_eq!(state.selected, 1);
        state.move_down(3); // 1 -> 2
        assert_eq!(state.selected, 2);
        state.move_down(3); // 2 -> 2 (clamped)
        assert_eq!(state.selected, 2);
    }

    #[test]
    fn nav_list_state_move_up_clamps() {
        let mut state = NavListState::new();
        state.move_up(); // 0 -> 0 (clamped)
        assert_eq!(state.selected, 0);
        state.selected = 2;
        state.move_up(); // 2 -> 1
        assert_eq!(state.selected, 1);
    }

    #[test]
    fn nav_list_state_move_down_empty_list() {
        let mut state = NavListState::new();
        state.move_down(0); // No items, should not change.
        assert_eq!(state.selected, 0);
    }

    #[test]
    fn nav_list_state_adjust_scroll_down() {
        let mut state = NavListState::new();
        state.selected = 5;
        state.offset = 0;
        state.adjust_scroll(3); // viewport shows 3 items
        // selected=5 should be visible: offset = 5 - 3 + 1 = 3
        assert_eq!(state.offset, 3);
    }

    #[test]
    fn nav_list_state_adjust_scroll_up() {
        let mut state = NavListState::new();
        state.selected = 1;
        state.offset = 3;
        state.adjust_scroll(5);
        // selected=1 is above offset=3, so offset should become 1.
        assert_eq!(state.offset, 1);
    }

    #[test]
    fn nav_list_state_adjust_scroll_zero_viewport() {
        let mut state = NavListState::new();
        state.selected = 2;
        state.offset = 0;
        state.adjust_scroll(0); // Should not panic.
        assert_eq!(state.offset, 0);
    }

    #[test]
    fn nav_list_renders_with_selection() {
        let items: Vec<String> = vec!["Alpha".into(), "Beta".into(), "Gamma".into()];
        let state = NavListState {
            selected: 1,
            offset: 0,
        };
        let list = NavList::new(&items, &state);
        let area = Rect::new(0, 0, 20, 5);
        let mut buf = Buffer::empty(area);
        list.render(area, &mut buf);

        // Check that the second row has the ">" indicator.
        let row1: String = (0..20)
            .map(|x| buf[(x, 1)].symbol().chars().next().unwrap_or(' '))
            .collect();
        assert!(row1.contains('>'));
        assert!(row1.contains("Beta"));
    }

    #[test]
    fn nav_list_renders_empty_without_panic() {
        let items: Vec<String> = vec![];
        let state = NavListState::new();
        let list = NavList::new(&items, &state);
        let area = Rect::new(0, 0, 20, 5);
        let mut buf = Buffer::empty(area);
        list.render(area, &mut buf);
    }

    #[test]
    fn nav_list_scrolled_renders_correct_items() {
        let items: Vec<String> = (0..10).map(|i| format!("Item {}", i)).collect();
        let state = NavListState {
            selected: 7,
            offset: 5,
        };
        let list = NavList::new(&items, &state);
        let area = Rect::new(0, 0, 20, 3);
        let mut buf = Buffer::empty(area);
        list.render(area, &mut buf);

        // Should render items 5, 6, 7 (offset=5, viewport=3).
        let row0: String = (0..20)
            .map(|x| buf[(x, 0)].symbol().chars().next().unwrap_or(' '))
            .collect();
        assert!(row0.contains("Item 5"));

        let row2: String = (0..20)
            .map(|x| buf[(x, 2)].symbol().chars().next().unwrap_or(' '))
            .collect();
        assert!(row2.contains("Item 7"));
        assert!(row2.contains('>'));
    }
}
