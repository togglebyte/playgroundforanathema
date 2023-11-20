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

#[derive(Debug)]
struct MyState {
    name: StateValue<String>,
    counter: StateValue<usize>,
    data: List<usize>,
    names: List<String>,
    nested_list: List<List<usize>>,
    nested_map: Map<Map<usize>>,
    thing: Map<List<Map<String>>>,
    // things: List<Map<usize>>
}

impl MyState {
    pub fn new() -> Self {
        let map = [("aaa", 1), ("bbb", 4)];
        let map = [("aaa", Map::new(map))];
        let map_2 = [("entry", List::new(vec![
                Map::new([("flap", "fiddle sticks".to_string())]),
                Map::new([("lol", "omg I can't believe it I've never been this far away from".to_string())]),
                Map::new([("flap", "fiddle sticks".to_string())]),
        ]))];
        // let map_3 = [

        Self {
            name: StateValue::new("Fiddlesticks".to_string()),
            counter: StateValue::new(0),
            data: List::new(0..4),
            names: List::new(vec!["flip".to_string(), "flop".into()]),
            nested_list: List::new(vec![List::new(0..4), List::new(100..102)]),
            nested_map: Map::new(map),
            thing: Map::new(map_2),
            // things: List::new(vec![
            //     Map::new(map),
            //     Map::new(map),
            // ])
        }
    }
}

impl ::anathema::values::state::State for MyState {
    fn get(
        &self,
        key: &::anathema::values::Path,
        node_id: ::core::option::Option<&::anathema::values::NodeId>,
    ) -> ::anathema::values::ValueRef<'_> {
        use ::anathema::values::{ValueRef, Path};
        use ::anathema::values::state::BlanketGet;
        match key {
            Path::Key(s) => {
                match s.as_str() {
                    "name" => (&self.name).__anathema_get_value(node_id),
                    "counter" => (&self.counter).__anathema_get_value(node_id),
                    "data" => (&self.data).__anathema_get_value(node_id),
                    "names" => (&self.names).__anathema_get_value(node_id),
                    "nested_list" => (&self.nested_list).__anathema_get_value(node_id),
                    "nested_map" => (&self.nested_map).__anathema_get_value(node_id),
                    "thing" => (&self.thing).__anathema_get_value(node_id),
                    _ => ValueRef::Empty,
                }
            }
            Path::Composite(lhs, rhs) => {
                let Path::Key(ref key) = &**lhs else {
                    return ValueRef::Empty;
                };
                match key.as_str() {
                    "name" => (&self.name).__anathema_get(rhs, node_id),
                    "counter" => (&self.counter).__anathema_get(rhs, node_id),
                    "data" => (&self.data).__anathema_get(rhs, node_id),
                    "names" => (&self.names).__anathema_get(rhs, node_id),
                    "nested_list" => (&self.nested_list).__anathema_get(rhs, node_id),
                    "nested_map" => (&self.nested_map).__anathema_get(rhs, node_id),
                    "thing" => (&self.thing).__anathema_get(rhs, node_id),
                    _ => ValueRef::Empty,
                }
            }
            _ => ValueRef::Empty,
        }
    }
}


fn main() {
    // // tinylog::init_logger(true).unwrap();

    // let list = List::new(vec!["lol".to_string()]);
    // let lol = list.get(&Path::Index(0), None);
    // eprintln!("{lol:?}");

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
                    state.data.push(state.data.len());
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
