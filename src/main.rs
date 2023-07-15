use anathema::core::contexts::DataCtx;
use anathema::core::template::Template;
use anathema::core::views::View;
use anathema::core::{Value, WidgetContainer};
use anathema::runtime::events::{
    DefaultEventProvider, DefaultEvents, Event, KeyCode, KeyModifiers,
};
use anathema::runtime::Runtime;
use anathema::vm::*;
use anathema::widgets::{Border, register_default_widgets};

#[derive(Debug)]
struct StatsView {
    templates: Vec<Template>,
}

impl View for StatsView {
    fn templates(&self) -> &[Template] {
        &self.templates
    }
}

fn ctx() -> DataCtx {
    let mut ctx = DataCtx::default();

    let values = (0..40_000).map(|i| Value::from(i)).collect::<Vec<_>>();
    ctx.insert("data".to_string(), Value::List(values));

    let values = (0..40_000).map(|i| Value::from(i)).collect::<Vec<_>>();
    ctx.insert("cols".to_string(), Value::List(values));

    let words = vec!["abra".to_string().into(), "crabdabra".to_string().into()];
    ctx.insert("words".to_string(), Value::List(words));

    ctx
}

fn main() {
    register_default_widgets();
    let main_templates = std::fs::read_to_string("templates/template.tiny").unwrap();
    let stats_templates = std::fs::read_to_string("templates/stats.tiny").unwrap();

    let main_templates = templates(&main_templates).unwrap();
    let stats_templates = templates(&stats_templates).unwrap();
    let stats_view = StatsView { templates: stats_templates };

    let mut runtime = Runtime::new(
        main_templates,
        ctx(),
        DefaultEvents(|ev, ctx, _| {
            match ctx.get_mut::<i64>("counter") {
                Some(counter) => *counter += 1,
                None => ctx.insert("counter", 1),
            }

            if let Event::KeyPress(KeyCode::Char('c'), KeyModifiers::CONTROL, ..) = ev {
                return Event::Quit;
            }

            ev
        }),
        DefaultEventProvider::with_timeout(200),
    )
    .unwrap();
    runtime.register_view("stats", stats_view);
    runtime.enable_meta = true;
    runtime.run().unwrap();
}

// Login screen
// Chat screen
// Settings screen
