use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

use anathema::core::Nodes;
use anathema::runtime::events::{
    DefaultEventProvider, DefaultEvents, Event, KeyCode, KeyModifiers,
};
use anathema::runtime::Runtime;
use anathema::values::{Collection, List, NodeId, Path, State, StateValue, ValueRef};
use anathema::vm::*;
use anathema::widgets::{Text, VStack};

struct MyState {
    name: StateValue<String>,
    counter: StateValue<usize>,
    data: List<usize>,
}

impl MyState {
    pub fn new() -> Self {
        Self {
            name: StateValue::new("Fiddlestick".to_string()),
            counter: StateValue::new(0),
            data: List::new(0..5),
        }
    }
}

impl State for MyState {
    fn get(&self, key: &Path, node_id: Option<&NodeId>) -> Option<ValueRef<'_>> {
        match key {
            Path::Key(s) if s == "name" => {
                if let Some(node_id) = node_id.cloned() {
                    self.name.subscribe(node_id);
                }
                Some((&self.name).into())
            }
            Path::Key(s) if s == "counter" => {
                if let Some(node_id) = node_id.cloned() {
                    self.counter.subscribe(node_id);
                }
                Some((&self.counter).into())
            }
            Path::Composite(left, right) => {
                let Path::Key(key) = left.deref() else {
                    return None;
                };
                if key == "data" {
                    self.data.lookup(right, node_id).map(Into::into)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn get_collection(&self, key: &Path, node_id: Option<&NodeId>) -> Option<usize> {
        match key {
            Path::Key(s) if s == "data" => {
                if let Some(node_id) = node_id.cloned() {
                    self.data.subscribe(node_id);
                }
                Some(self.data.len())
            }
            _ => None,
        }
    }

    //    fn get_collection(&self, key: &Path, node_id: Option<&NodeId>) -> Option<Collection> {
    //        match key {
    //            Path::Key(s) => {
    //                //    //
    //                match s.as_str() {
    //                    "data" => {
    //                        //
    //                        if let Some(node_id) = node_id.cloned() {
    //                            self.data.subscribe(node_id);
    //                        }
    //                        Some(Collection::State {
    //                            path: key.clone(),
    //                            len: self.data.len(),
    //                        })
    //                    }
    //                    _ => None,
    //                }
    //            }
    //            _ => None,
    //        }
    //    }
}

fn main() {
    // tinylog::init_logger(true).unwrap();

    let main_templates = std::fs::read_to_string("templates/template.tiny").unwrap();

    let main_expr = templates(&main_templates).unwrap();

    let mut runtime = Runtime::new(
        &main_expr,
        MyState::new(),
        DefaultEvents::<_, MyState>(
            |ev, nodes, state| {
                *state.counter += 1;
                // *state.counter = *meta.count;

                if let Event::KeyPress(KeyCode::Char(' '), ..) = ev {
                    state.data.pop();
                }

                if let Event::KeyPress(KeyCode::Char('='), ..) = ev {
                    state.data.push(state.data.len());
                }

                // let val: &mut usize = state.counter.deref_mut();
                // state.counter += 1;
                // *ctx.get_mut_or::<i64>("count_things", 0) += 1;

                if let Event::MouseMove(x, y, ..) = ev {
                    // *ctx.get_mut_or::<i64>("x", 0i64) = x as i64;
                    // *ctx.get_mut_or::<i64>("y", 0i64) = y as i64;
                }

                if let Event::CtrlC = ev {
                    return Event::Quit;
                }

                ev
            },
            Default::default(),
        ),
        DefaultEventProvider::with_timeout(100),
    )
    .unwrap();

    // runtime.register_view(
    //     "stats",
    //     StatsView {
    //         templates: stats_templates,
    //     },
    // );
    runtime.enable_meta = true;
    // runtime.enable_mouse = true;
    runtime.run().unwrap();
}
