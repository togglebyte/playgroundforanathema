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
    ctx.insert("data".to_string(), values);

    let values = (0..6).map(|i| Value::from(i)).collect::<Vec<_>>();
    ctx.insert("cols".to_string(), values);

    let words = vec!["abra", "crabdabra"];
    ctx.insert("words", words);

    ctx
}

fn main() {
    tinylog::init_logger(true).unwrap();

    register_default_widgets();
    let main_templates = std::fs::read_to_string("templates/template.tiny").unwrap();
    let stats_templates = std::fs::read_to_string("templates/stats.tiny").unwrap();

    let main_templates = templates(&main_templates).unwrap();
    let stats_templates = templates(&stats_templates).unwrap();

    let mut runtime = Runtime::new(
        main_templates,
        ctx(),
        DefaultEvents(|ev, ctx, nodes| {
            *ctx.get_mut_or::<i64>("count_things", 0) += 1;

            if let Event::CtrlC = ev {
                use std::fs::write;
                write("/tmp/a", format!("{:#?}", nodes));
                return Event::Quit;
            }

            ev
        }),
        DefaultEventProvider::with_timeout(200),
    )
    .unwrap();

    runtime.register_view("stats", StatsView { templates: stats_templates });
    runtime.enable_meta = true;
    runtime.run().unwrap();
}
