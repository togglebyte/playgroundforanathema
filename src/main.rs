use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::stdout;
use std::time::Instant;

use anathema::compiler;
use anathema::render::*;
use anathema::runtime::Runtime;
use anathema::vm::*;
use anathema::widgets::template::Template;
use anathema::widgets::view::{DefaultView, View};
use anathema::widgets::{Border, *};

struct TheView {
    ctx: DataContext,
    templates: Vec<Template>,
}

impl TheView {
    pub fn new(templates: Vec<Template>) -> Self {
        let mut ctx = DataContext::default();
        let values = (0..2).map(|i| Value::from(i)).collect::<Vec<_>>();
        ctx.insert("data".into(), Value::List(values));
        ctx.insert("offset".into(), Value::from(-1i64));
        ctx.insert("item-offset".into(), Value::from(1i64));
        Self { templates, ctx }
    }
}

impl View for TheView {
    fn update(&mut self) {
        // let Value::List(list) = self.ctx.get_mut("data").unwrap() else { panic!() };
        // list.push("updated".into());
        // let offset = self.ctx.get("offset").unwrap().to_signed_int().unwrap();
        // self.ctx.insert("offset".into(), Value::from(offset + 1));
    }

    fn ctx(&self) -> &DataContext {
        &self.ctx
    }

    fn templates(&self) -> &[Template] {
        &self.templates
    }
}

fn main() {
    let template = "
        viewport [direction: forward, item: {{ item-offset }}, offset: {{ offset }}, source: {{ data }}, binding: x]
            text 'first'
            text 'second'
            item
                border
                    text 'number: {{ x }}'
            text 'end'
    ";

    let templates = templates(template).unwrap();

    let view = TheView::new(templates);

    let mut runtime = Runtime::new().unwrap();
    runtime.load_view(Box::new(view));
    runtime.run().unwrap();
}
