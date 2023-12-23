use std::any::Any;
use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

use anathema::core::views::View;
use anathema::core::{Align, Axis, Display, Event, KeyCode, KeyModifiers, Nodes};
use anathema::render::Color;
use anathema::runtime::{Meta, Runtime};
use anathema::values::{
    Collection, DynValue, List, Map, NodeId, Path, State, StateValue, ValueRef, ValueExpr
};
use anathema::vm::*;
use anathema::compiler;
use anathema::widgets::{
    Alignment, Border, HStack, Position, Spacer, Text, VStack, Viewport, ZStack,
};
use flume::{Receiver, Sender};

#[derive(Debug)]
struct A(AState);

impl View for A {
    type State = AState;

    fn get_state(&self) -> &dyn State {
        &self.0
    }

    fn on_event(&mut self, event: Event, nodes: &mut Nodes<'_>) {
        if let Event::KeyPress(code, ..) = event {
            *self.0.is_true = !*self.0.is_true;

            match code {
                KeyCode::Char(c) => self.0.text.push(c),
                KeyCode::Backspace => {
                    self.0.text.pop();
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, State)]
struct TinyState {
    ext: StateValue<u32>,
}

#[derive(Debug, State)]
struct AState {
    offset: StateValue<u32>,
    things: List<usize>,
    text: StateValue<String>,
    is_true: StateValue<bool>,
    numbers: List<TinyState>,
}

impl AState {
    pub fn new() -> Self {
        Self {
            offset: 6.into(),
            things: List::new(0..25_000),
            text: String::from("hello").into(),
            is_true: true.into(),
                numbers: List::new((0..11).map(|n| TinyState { ext: n.into() }))
            }
        }
    }

    #[derive(Debug, State)]
    struct BState {
        color: StateValue<Color>,
        background: StateValue<Color>,
        counter: StateValue<u32>,
        input: StateValue<String>,
    }

    #[derive(Debug)]
    struct B(BState);

    impl View for B {
        type State = BState;

        fn get_state(&self) -> &dyn State {
            &self.0
        }

        fn on_event(&mut self, event: Event, nodes: &mut Nodes<'_>) {
            if let Event::KeyPress(code, ..) = event {
                if let (KeyCode::Tab | KeyCode::BackTab) = code {
                    return;
                }
                *self.0.counter += 1;

            match code {
                KeyCode::Char(c) => self.0.input.push(c),
                KeyCode::Backspace => drop(self.0.input.pop()),
                KeyCode::Enter => self.0.input.clear(),
                _ => {}
            }
        }
    }

    fn blur(&mut self) {
        *self.0.color = Color::Black;
        *self.0.background = Color::Reset;
    }

    fn focus(&mut self) {
        *self.0.color = Color::White;
        *self.0.background = Color::White;
    }
}

#[derive(Default)]
struct MetaView(MetaState);

impl View for MetaView {
    type State = MetaState;

    fn get_state(&self) -> &dyn State {
        &self.0
    }

    // fn tick(&mut self, meta: &dyn Any) {
    //     let meta: &Meta = meta.downcast_ref().unwrap();
    //     *self.0.layout = format!("{:?}", meta.timings.layout);
    //     *self.0.position = format!("{:?}", meta.timings.position);
    //     *self.0.paint = format!("{:?}", meta.timings.paint);
    //     *self.0.render = format!("{:?}", meta.timings.render);
    //     *self.0.total = format!("{:?}", meta.timings.total);
    //     *self.0.count = meta.count;
    // }
}

#[derive(Debug, State, Default)]
struct MetaState {
    layout: StateValue<String>,
    position: StateValue<String>,
    paint: StateValue<String>,
    render: StateValue<String>,
    total: StateValue<String>,
    count: StateValue<usize>,
}

fn main() {
    let root = std::fs::read_to_string("templates/template.tiny").unwrap();
    let bbb = std::fs::read_to_string("templates/bbb.tiny").unwrap();
    let meta = std::fs::read_to_string("templates/meta.tiny").unwrap();
    let ext = std::fs::read_to_string("templates/external.tiny").unwrap();
    let mut templates = Templates::new(root, A(AState::new()));

    templates.add_prototype("bbb", bbb.clone(), || {
        B(BState {
            color: Color::Black.into(),
            background: Color::Reset.into(),
            counter: 0.into(),
            input: String::new().into(), 
        })
    });

    templates.add_view("meta", meta, MetaView::default());
    templates.add_prototype("external", ext, || ());
    templates.compile().unwrap();

    let mut runtime = Runtime::new(&templates.expressions()).unwrap();
    runtime.enable_tabindex = false;
    // runtime.fps = 4;
    runtime.run().unwrap();
}
