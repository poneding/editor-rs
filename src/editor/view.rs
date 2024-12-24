mod buffer;
mod line;
mod location;
use super::{
    editorcommand::{Direction, EditorCommand},
    terminal::{Size, Terminal},
    Position,
};
use buffer::Buffer;
use location::Location;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    location: Location,
    scroll_offset: Location,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            location: Location::default(),
            scroll_offset: Location::default(),
        }
    }
}

impl View {
    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }

        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }

        #[allow(clippy::as_conversions)]
        let vertical_center = height / 3;
        let top = self.scroll_offset.y;
        for cur_row in 0..height {
            if let Some(line) = self.buffer.lines.get(cur_row.saturating_add(top)) {
                let left = self.scroll_offset.x;
                let right = self.scroll_offset.x.saturating_add(width);
                Self::render_line(cur_row, &line.get(left..right));
            } else if cur_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(cur_row, &Self::build_welcome_message(width));
            } else {
                Self::render_line(cur_row, "~");
            }
        }

        self.needs_redraw = false;
    }

    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Quit => {}
        }
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }

    pub fn get_position(&self) -> Position {
        self.location.subtract(&self.scroll_offset).into()
    }

    fn move_text_location(&mut self, direction: &Direction) {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size().unwrap_or_default();
        match direction {
            Direction::Up => y = y.saturating_sub(1),
            Direction::Down => y = y.saturating_add(1),
            Direction::Left => x = x.saturating_sub(1),
            Direction::Right => x = x.saturating_add(1),
            Direction::PageUp => y = 0,
            Direction::PageDown => y = height.saturating_sub(1),
            Direction::Home => x = 0,
            Direction::End => x = width.saturating_sub(1),
        }
        self.location = Location { x, y };
        self.scroll_location_into_view();
    }

    fn resize(&mut self, to: Size) {
        self.size = to;
        self.scroll_location_into_view();
        self.needs_redraw = true;
    }

    fn scroll_location_into_view(&mut self) {
        let Location { x, y } = self.location;
        let Size { height, width } = self.size;
        let mut offset_changed = false;

        // Scroll vertically
        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            offset_changed = true;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = y.saturating_sub(height).saturating_add(1);
            offset_changed = true;
        }

        // Scroll horizontally
        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changed = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
            offset_changed = true;
        }

        self.needs_redraw = offset_changed;
    }

    fn render_line(row: usize, line: &str) {
        Terminal::print_row(row, line).expect("failed to render line");
    }

    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return String::new();
        }
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        if width <= len {
            return String::from("~");
        }

        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len).saturating_sub(1)) / 2;
        let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
        full_message.truncate(width);
        full_message
    }
}
