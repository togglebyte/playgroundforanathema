use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::stdout;
use std::time::Instant;

use anathema::compiler;
use anathema::render::*;
use anathema::vm::*;
use anathema::widgets::nodegen::gen2::Gen;
use anathema::widgets::*;
use anathema::widgets::{Viewport, Text};

fn context() -> HashMap<String, Value> {
    let longstring = read_to_string("./longlongtext.txt").unwrap();
    let mut root_ctx = HashMap::new();
    // let values = (0..3_000_000)
    let values = (0..2)
        .map(|i| {

            let inner_list = (0..3).map(|j| Value::String(format!("outer: {j } | counter: {i} - world and some extra text"))).collect::<Vec<Value>>();
            Value::List(inner_list)
        })
        .collect::<Vec<Value>>();
    let data = Value::List(values);
    let map = Value::Map(HashMap::from([
        ("list".to_string(), data)
    ]));

    root_ctx.insert("data".to_string(), map);
    // root_ctx.insert("counter".into(), 0.into());
    // root_ctx.insert("lark".into(), false.into());
    // root_ctx.insert("other".into(), false.into());
    // root_ctx.insert("longstring".into(), longstring.into());
    root_ctx
}

fn main() {
    let template = "
        // text 'layout time: {{ time }} | render time: {{ render-time }}'
    ";

    let template = "
        viewport [source: {{ data.list }}, binding: 'outer_list']
            text 'this is the top of a viewport'

            for lists in {{ outer_list }}
                viewport [source: {{ lists }}, binding: 'list']
                    for item in {{ list }}
                        text 'hello {{ item }}'
                        text 'entire list: {{ item }}'

            text 'this is the bottom of a viewport'
    ";

    let (inst, consts) = compiler::compile(template).unwrap();

    let vm = VirtualMachine::new(inst, consts);
    let templates = vm.exec().unwrap();

    let mut counter = 0;

    let lookup = WidgetLookup::default();

    let mut output = stdout();
    let (width, height) = size().unwrap();
    let mut screen = Screen::new(&mut output, (width, height)).unwrap();
    screen.clear_all(&mut output);

    let constraints = Constraints::new(width as usize, height as usize);

    let mut root_ctx = context();

    let mut widgets = vec![];

    loop {
        let mut now = Instant::now();
        // root_ctx.insert("counter".to_string(), counter.into());

        let mut values = Values::new(&root_ctx);
        let mut genny = Gen::new(&templates, &lookup);
        while let Some(mut widget) = genny.gen(&mut values) {
            let values = values.layout();
            widget.layout(constraints, &values, &lookup);
            widgets.push(widget);
        }

        // // diff(&mut prev, &mut widgets);

        for widget in &mut widgets {
            widget.position(Pos::ZERO);
        }

        for widget in &mut widgets {
            let mut ctx = PaintCtx::new(&mut screen, None);
            widget.paint(ctx);
        }

        // root_ctx.insert("time".to_string(), format!("{:?}", now.elapsed()).into());

        // prev = widgets.drain(..).collect();

        screen.render(&mut output);
        // root_ctx.insert("render-time".to_string(), format!("{:?}", now.elapsed()).into());
        widgets.clear();

        screen.erase();

        counter += 1;
        std::thread::sleep_ms(100);
    }
}
