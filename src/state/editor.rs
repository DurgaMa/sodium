use edit::buffer::{Buffer, SplitBuffer};
use state::cursor::Cursor;
use io::graphics::StatusBar;
use io::key::{Key, Cmd};
use io::key_state::KeyState;
use state::options::Options;
use io::parse::Inst;
use io::redraw::RedrawTask;

#[cfg(feature = "orbital")]
use orbclient::Window;

use std::env::args;

/// The current state of the editor, including the file, the cursor, the scrolling info, etc.
pub struct Editor {
    /// The current cursor
    pub current_cursor: u8,
    /// The cursors
    pub cursors: Vec<Cursor>,
    /// The buffers (documents)
    pub buffers: Vec<SplitBuffer>,
    /// The current buffer index
    pub current_buffer_index: usize,
    /// The x coordinate of the scroll
    pub scroll_x: usize,
    /// The y coordinate of the scroll
    pub scroll_y: usize,
    /// The window
    #[cfg(feature = "orbital")]
    pub window: Window,
    /// The status bar
    pub status_bar: StatusBar,
    /// The prompt
    pub prompt: String,
    /// The settings
    pub options: Options,
    /// The key state
    pub key_state: KeyState,
    /// Redraw
    pub redraw_task: RedrawTask,
    /// The previous instruction
    pub previous_instruction: Option<Inst>,
}

impl Editor {
    /// Create new default state editor
    pub fn init() {

        #[cfg(feature = "orbital")]
        let window = Window::new(-1, -1, 700, 500, &"Sodium").unwrap();

        #[cfg(feature = "orbital")]
        let mut editor = Editor {
            current_cursor: 0,
            cursors: vec![Cursor::new()],
            buffers: vec![SplitBuffer::new()],
            current_buffer_index: 0,
            scroll_x: 0,
            scroll_y: 0,
            window: *window, // ORBITAL SPECIFIC!
            status_bar: StatusBar::new(),
            prompt: String::new(),
            options: Options::new(),
            key_state: KeyState::new(),
            redraw_task: RedrawTask::None,
            previous_instruction: None,
        };

        #[cfg(not(feature = "orbital"))]
        let mut editor = Editor {
            current_cursor: 0,
            cursors: vec![Cursor::new()],
            buffers: vec![SplitBuffer::new()],
            current_buffer_index: 0,
            scroll_x: 0,
            scroll_y: 0,
            status_bar: StatusBar::new(),
            prompt: String::new(),
            options: Options::new(),
            key_state: KeyState::new(),
            redraw_task: RedrawTask::None,
            previous_instruction: None,
        };

        if let Some(x) = args().skip(1).next() {
            editor.open(&x);
        }

        debugln!(editor, "Starting Sodium");

        editor.redraw();

        debugln!(editor, "First redraw of the screen");

        loop {
            let inp = editor.get_inst();
            if let Inst(_, Cmd { key: Key::Quit }) = inp {
                debugln!(editor, "C'ya");
                break;
            }
            editor.exec(inp);
            editor.status_bar.mode = editor.cursor().mode.to_string();
            editor.redraw();
        }
    }

    /// Hint the buffer about the cursor position.
    pub fn hint(&mut self) {
        let x = self.cursor().x;
        let y = self.cursor().y;

        self.current_buffer_mut().focus_hint_y(y);
        self.current_buffer_mut().focus_hint_x(x);
    }

    #[inline]
    /// Get a reference to the currently open buffer.
    pub fn current_buffer(&self) -> &SplitBuffer {
        &self.buffers[self.current_buffer_index]
    }

    #[inline]
    /// Get a mutable reference to the currently open buffer.
    pub fn current_buffer_mut(&mut self) -> &mut SplitBuffer {
        &mut self.buffers[self.current_buffer_index]
    }
}
