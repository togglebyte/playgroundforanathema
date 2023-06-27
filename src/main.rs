use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::stdout;
use std::time::Instant;

use anathema::compiler;
use anathema::render::*;
use anathema::runtime::Runtime;
use anathema::vm::*;
use anathema::widgets::template::Template;
use anathema::widgets::{Border, HStack, Spacer, VStack, Viewport, *};
use anathema::widgets::Many;

struct Event {
    view: &mut View,
    event: Ev,
    target: &mut WidgetContainer<'_>
}

struct View {
    templates: Vec<Template>,
    ctx: DataCtx,
}

trait ViewT {
    fn event(&self, ctx: &mut DataCtx, event: Event, widgets: &mut Vec<WidgetContainer<'_>>);
}

fn ctx() -> DataCtx {
    let mut ctx = DataCtx::default();
    let values = (0..40_000).map(|i| Value::from(i)).collect::<Vec<_>>();
    ctx.insert("data".to_string(), Value::List(values));

    let values = (0..40_000).map(|i| Value::from(i)).collect::<Vec<_>>();
    ctx.insert("cols".to_string(), Value::List(values));
    ctx
}

fn main() {
    let main_template = "
        zstack
            vstack
                border
                    hstack
                        border [sides: right, onclick=bob]
                            text [onclick=bob] 'login '
                        border [sides: right, onclick=bob]
                            text ' chat '
                        text ' settings '
                        input [keypress=bob, text: 'lol']
                        spacer
                expand
                    border
                        expand
                            text 'hello world'
    ";

    let rubbish = "
        position [display: exclude]
            alignment [align: centre]
                border [width: 20, height: 5]
                    text 'well this is something'
    ";

    let t = templates(main_template).unwrap();
    let c = ctx();
    let mut main_view = View { templates: t, ctx: c };
    // main_view.register_event("bob", |ev: Event, cont: &mut WidgetContainer<'_>, ctx: &mut Ctx| {});
    main_view.register_event("bob", |ev, widget, ctx| {});

    let t = templates(rubbish).unwrap();
    let c = ctx();
    let rubbish_view = View { templates: t, ctx: c };

    // let mut runtime = Runtime::new(&templates, ctx()).unwrap();
    // runtime.run().unwrap();
}


// Login screen
// Chat screen
// Settings screen
