use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

use anathema::core::{Nodes, Display, Axis, Align};
use anathema::runtime::events::{
    DefaultEventProvider, DefaultEvents, Event, KeyCode, KeyModifiers,
};
use anathema::runtime::Runtime;
use anathema::values::{Collection, List, NodeId, Path, State, StateValue, ValueRef, Map};
use anathema::vm::*;
use anathema::widgets::{Alignment, Border, Text, VStack, ZStack, Position};

fn main() {
    // let main_templates = std::fs::read_to_string("templates/app/main.tiny").unwrap();
    let main_templates = std::fs::read_to_string("templates/template.tiny").unwrap();

    let main_expr = templates(&main_templates).unwrap();

    let mut runtime = Runtime::new(
        &main_expr,
        (),
        DefaultEvents::<_, ()>(
            |ev, nodes, state| {
                if let Event::KeyPress(KeyCode::Char(c), ..) = ev {
                    if c == 'x' {
                        // nodes.by_attribute("name", "boopy", state);
                        // nodes.by_attribute("name", "boopy").for_each(|n| {
                        // });
                    }
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
