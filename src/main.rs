use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::stdout;
use std::time::Instant;

use anathema::compiler;
use anathema::render::*;
use anathema::runtime::events::{Event, DefaultEventProvider, DefaultEvents, KeyCode};
use anathema::runtime::Runtime;
use anathema::vm::*;
use anathema::widgets::template::Template;
use anathema::widgets::{Border, HStack, Many, Spacer, VStack, Viewport, *};

struct View {
    templates: Vec<Template>,
    ctx: DataCtx,
}

trait ViewT {
    fn event(&self, ctx: &mut DataCtx, widgets: &mut Vec<WidgetContainer<'_>>);
}

fn ctx() -> DataCtx {
    let mut ctx = DataCtx::default();

    let values = (0..40_000).map(|i| Value::from(i)).collect::<Vec<_>>();
    ctx.insert("data".to_string(), Value::List(values));

    let values = (0..40_000).map(|i| Value::from(i)).collect::<Vec<_>>();
    ctx.insert("cols".to_string(), Value::List(values));

    let words = vec![
        "abra".to_string().into(),
        "crabdabra".to_string().into(),
    ];
    ctx.insert("words".to_string(), Value::List(words));


    ctx
}

fn main() {
    let main_templates = std::fs::read_to_string("template.tiny").unwrap();

    let rubbish = "
        position [display: exclude, top: 3, right: 4]
            // alignment [align: centre]
                border [width: 20, height: 5]
                    text 'well this is something'
    ";

    let t = templates(&main_templates).unwrap();

    //     let c = ctx();
    //     let mut main_view = View { templates: t, ctx: c };

    //     let t = templates(rubbish).unwrap();
    //     let c = ctx();
    //     let rubbish_view = View { templates: t, ctx: c };

    let mut runtime = Runtime::new(
        &t,
        ctx(),
        DefaultEvents(|ev, ctx, _| {
            match ctx.get_mut::<i64>("counter") {
                Some(counter) => *counter += 1,
                None => ctx.insert("counter", 1),
            }

            if let Event::KeyPress(KeyCode::Esc, ..) = ev {
                return Event::Quit;
            }

            ev
        }),
        DefaultEventProvider::with_timeout(16),
    )
    .unwrap();
    runtime.enable_meta = true;
    runtime.run().unwrap();
}

// Login screen
// Chat screen
// Settings screen
