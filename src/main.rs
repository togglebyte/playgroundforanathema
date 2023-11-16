use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

use anathema::core::Nodes;
use anathema::runtime::events::{
    DefaultEventProvider, DefaultEvents, Event, KeyCode, KeyModifiers,
};
use anathema::runtime::Runtime;
use anathema::values::{Collection, List, NodeId, Path, State, StateValue, ValueRef, Map};
use anathema::vm::*;
use anathema::widgets::{Alignment, Border, Text, VStack};

#[derive(Debug, State)]
struct MyState {
    name: StateValue<String>,
    counter: StateValue<usize>,
    data: List<usize>,
    names: List<String>,
    nested_data: List<List<List<usize>>>,
    mappy: Map<Map<usize>>,
    thing: Map<List<Map<&'static str>>>,
    // things: List<Map<usize>>
}

impl MyState {
    pub fn new() -> Self {
        let map = [("aaa", 1), ("bbb", 4)];
        let map = [("aaa", Map::new(map))];
        let map_2 = [("entry", List::new(vec![
                Map::new([("lol", "omg I can't believe it I've never been this far away from")]),
        ]))];
        // let map_3 = [

        Self {
            name: StateValue::new("Fiddlesticks".to_string()),
            counter: StateValue::new(2),
            data: List::new(0..4),
            names: List::new(vec!["flip".to_string(), "flop".into()]),
            nested_data: List::new(vec![List::new(vec![List::new(0..4), List::new(100..102)])]),
            mappy: Map::new(map),
            thing: Map::new(map_2),
            // things: List::new(vec![
            //     Map::new(map),
            //     Map::new(map),
            // ])
        }
    }
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
                if let Event::KeyPress(KeyCode::Char('c'), ..) = ev {
                    *state.counter += 1;
                }

                if let Event::KeyPress(KeyCode::Char('d'), ..) = ev {
                    state.data.insert(0, 555);
                    let s = format!("{:?}", state.data);

                    let y = s;
                }
                // *state.counter = *meta.count;

                if let Event::KeyPress(KeyCode::Char(' '), ..) = ev {
                    state.data.pop();
                }

                if let Event::KeyPress(KeyCode::Char('='), ..) = ev {
                    // state.data.push(state.data.len());
                }

                if let Event::KeyPress(KeyCode::Char('1'), ..) = ev {}

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
