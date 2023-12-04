use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

use anathema::core::views::View;
use anathema::core::{Nodes, Display, Axis, Align};
use anathema::runtime::events::{
    DefaultEventProvider, DefaultEvents, Event, KeyCode, KeyModifiers,
};
use anathema::runtime::Runtime;
use anathema::values::{Collection, List, NodeId, Path, State, StateValue, ValueRef, Map};
use anathema::vm::*;
// use anathema::widgets::{Alignment, Border, Text, VStack, ZStack, Position};
use anathema::widgets::{Text};

struct Items;

impl View for Items {
    type State = ();

    fn event(&self, event: (), state: &mut Self::State) {
        todo!()
    }

    fn make() -> Self {
        Self
    }
}

fn main() {
    let main_templates = std::fs::read_to_string("templates/app/main.tiny").unwrap();

    Views has to be compiled first, so they exists in some kind of larger structure.
    * Share consts maybe? (get rid of owned strings for views)
    * What about hot reloading templates? Maybe a "large" reload function that
      reloads everything?
    * Let's add a `Template` struct that handles all the templates (can include hot-reloading too)


    // let main_templates = std::fs::read_to_string("templates/template.tiny").unwrap();

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

    runtime.register_view("items", || Items);

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
