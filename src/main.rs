use std::borrow::Cow;
use std::ops::{DerefMut, Deref};

use anathema::values::{State, Path, NodeId, Collection, Value, List};
use anathema::runtime::events::{
    DefaultEventProvider, DefaultEvents, Event, KeyCode, KeyModifiers,
};
use anathema::runtime::Runtime;
use anathema::vm::*;
use anathema::widgets::{Text, VStack};

// fn ctx() -> DataCtx {
//     let mut ctx = DataCtx::default();

//     let values = (0..60_000).map(|i| Value::from(i)).collect::<Vec<_>>();
//     ctx.insert("data".to_string(), values);

//     let values = (0..12).map(|i| Value::from(i)).collect::<Vec<_>>();
//     ctx.insert("cols".to_string(), values);

//     let words = vec!["abra", "crabdabra"];
//     ctx.insert("words", words);

//     ctx
// }


struct MyState {
    name: Value<String>,
    counter: Value<usize>,
    data: List<usize>
}

impl MyState {
    pub fn new() -> Self {
        Self {
            name: Value::new("Fin McSilverpants".to_string()),
            counter: Value::new(0),
            data: (0..60_000).collect::<Vec<_>>().into()
        }
    }
}

impl State for MyState {
    fn get(&self, key: &Path, node_id: Option<&NodeId>) -> Option<Cow<'_, str>> {
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
                let Path::Key(key) = left.deref() else { return None };
                if key == "data" {
                    self.data.lookup(right)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn get_collection(&self, key: &Path) -> Option<Collection> {
        match key { 
            Path::Key(s) if s == "data" => Some(Collection::State { path: key.clone(), len: self.data.len() }),
            _ => None,
        }
    }
}

fn main() {
    // tinylog::init_logger(true).unwrap();

    let main_templates = std::fs::read_to_string("templates/template.tiny").unwrap();

    let main_expr = templates(&main_templates).unwrap();

    let mut runtime = Runtime::new(
        main_expr,
        MyState::new(),
        DefaultEvents::<_, MyState>(|ev, nodes, state, meta| {
            *state.counter = *meta.count;

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
        }, Default::default()),
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
