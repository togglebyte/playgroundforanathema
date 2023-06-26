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

fn ctx() -> DataCtx {
    let mut ctx = DataCtx::default();
    let values = (0..40_000).map(|i| Value::from(i)).collect::<Vec<_>>();
    ctx.insert("data".to_string(), Value::List(values));

    let values = (0..40_000).map(|i| Value::from(i)).collect::<Vec<_>>();
    ctx.insert("cols".to_string(), Value::List(values));
    ctx
}

fn main() {
    let template = "
    vstack
        hstack
            for _ in {{ cols }}
                vstack
                    for val in {{ data }}
                        text 'X'
    ";

    let templates = templates(template).unwrap();

    let mut runtime = Runtime::new(&templates, ctx()).unwrap();
    runtime.run().unwrap();
}
