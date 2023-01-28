use std::io::stdout;
use std::time::Instant;

use anathema::compiler;
use anathema::render::*;
use anathema::vm::*;
use anathema::widgets::nodegen::WidgetGenerator;
use anathema::widgets::*;

fn main() {
    let template = r#"
        vstack
            for item in {{ data }}
                text [color: red] "{{ item }} hello "
                    span [color: magenta] "- bleep bloop -"
    "#;

    let (inst, consts) = compiler::compile(template).unwrap();

    let vm = VirtualMachine::new(inst, consts);
    let templates = vm.exec().unwrap();

    // eprintln!("{templates:#?}");
    // return;

    let mut root_ctx = Context::new();

    let mut counter = 0;

    let values = (0..30_000)
        .map(|i| format!("{i} - world and some extra text").into())
        .collect::<Vec<Value>>();
    let data = Value::List(values);
    root_ctx.insert("data", data);

    let lookup = WidgetLookup::default();

    let mut output = stdout();
    let (width, height) = size().unwrap();
    let mut screen = Screen::new(&mut output, (width, height)).unwrap();
    screen.clear_all(&mut output);

    let constraints = Constraints::new(width as usize, height as usize);

    loop {
        root_ctx.insert("number", counter.into());

        let mut gen: WidgetGenerator = WidgetGenerator::new(&templates, &lookup);
        let mut widgets = vec![];

        while let Some(mut widget) = gen.next_widget_container(&root_ctx) {
            widget.layout(constraints, &root_ctx, &lookup);
            widgets.push(widget);
        }

        for widget in &mut widgets {
            for child in &widget.children.widgets {
                let size = child.size;
                let x = size;
            }

            widget.position(Pos::ZERO);
        }

        for widget in &mut widgets {
            let mut ctx = PaintCtx::new(&mut screen, None);
            widget.paint(ctx);
        }

        screen.render(&mut output);

        std::thread::sleep_ms(1000);
        screen.erase();
    }
}
