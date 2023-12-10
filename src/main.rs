use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

use anathema::core::views::{TabIndex, View};
use anathema::core::{Align, Axis, Display, Event, KeyCode, KeyModifiers, Nodes};
use anathema::runtime::Runtime;
use anathema::values::{Collection, List, Map, NodeId, Path, State, StateValue, ValueRef};
use anathema::vm::*;
use anathema::widgets::{Alignment, Border, Position, Text, VStack, ZStack};

// -----------------------------------------------------------------------------
//   - Message -
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct MessageView;

impl View for MessageView {
    type State = Message;

    fn make() -> Self {
        Self
    }
}

#[derive(Debug, State)]
struct Message {
    sender: StateValue<String>,
    text: StateValue<String>,
    count: StateValue<usize>,
}

// -----------------------------------------------------------------------------
//   - Messages -
// -----------------------------------------------------------------------------
#[derive(Debug)]
struct MessagesView {
    messages: Messages,
}

impl View for MessagesView {
    type State = Messages;

    fn make() -> Self {
        Self {
            messages: Self::State::new(),
        }
    }

    fn on_event(&mut self, event: Event, nodes: &mut Nodes<'_>) {
        match event {
            Event::KeyPress(KeyCode::Char('b'), ..) => {
                // nodes.query().by_attrib("hi", 3).for_each(|node| {});
            }
            Event::KeyPress(KeyCode::Char('x'), ..) => {
                self.messages.messages.pop();
            }
            Event::KeyPress(KeyCode::Char('a'), ..) => {
                let next = self.messages.messages.len();
                self.messages.messages.push(Message {
                    sender: StateValue::new(format!("Floppy")),
                    text: StateValue::new("This is the text".into()),
                    count: StateValue::new(next),
                });
            }
            _ => {}
        }
    }

    fn get_state(&self) -> &dyn State {
        &self.messages
    }
}

#[derive(Debug, State)]
struct Messages {
    messages: List<Message>,
}

impl Messages {
    pub fn new() -> Self {
        Self {
            messages: List::new(vec![Message {
                sender: StateValue::new(format!("ToggleBoggleBlorp")),
                text: StateValue::new("you are all wonderful".into()),
                count: StateValue::new(0),
            }]),
        }
    }
}

#[derive(Debug)]
struct InputView(InputState);

impl View for InputView {
    type State = StateValue<String>;

    fn make() -> Self {
        Self(InputState {
            input: StateValue::new(String::new()),
        })
    }

    fn on_event(&mut self, event: Event, nodes: &mut Nodes<'_>) {
        match event {
            Event::KeyPress(keycode, ..) => {
                match keycode {
                    KeyCode::Char(c) => self.0.input.push(c),
                    KeyCode::Backspace => drop(self.0.input.pop()),
                    KeyCode::Enter => {
                        let msg = self.0.input.drain(..).collect::<String>();
                        self.0.input.clear()
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn get_state(&self) -> &dyn State {
        &self.0
    }
}

#[derive(Debug, State)]
struct InputState {
    input: StateValue<String>,
}

fn main() {
    let main_templates = std::fs::read_to_string("templates/app/main.tiny").unwrap();
    let input_templates = std::fs::read_to_string("templates/app/input.tiny").unwrap();
    let messages_templates = std::fs::read_to_string("templates/app/messages.tiny").unwrap();
    let message_templates = std::fs::read_to_string("templates/app/message.tiny").unwrap();

    let mut templates = Templates::new(main_templates);
    templates.add_view("input".into(), input_templates, InputView::make);
    templates.add_view("messages".into(), messages_templates, MessagesView::make);
    templates.add_view("message".into(), message_templates, || MessageView);
    templates.compile().unwrap();

    let mut runtime = Runtime::new(&templates.expressions()).unwrap();

    runtime.enable_meta = true;
    // runtime.enable_mouse = true;
    runtime.run().expect("this is weird");
}
