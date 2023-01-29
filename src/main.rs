use std::io::stdout;
use std::time::Instant;

use anathema::compiler;
use anathema::render::*;
use anathema::vm::*;
use anathema::widgets::nodegen::WidgetGenerator;
use anathema::widgets::*;

fn context() -> Context {
    let mut root_ctx = Context::new();
    let values = (0..30_000)
        .map(|i| format!("{i} - world and some extra text").into())
        .collect::<Vec<Value>>();
    let data = Value::List(values);
    root_ctx.insert("data", data);
    root_ctx.insert("counter", 0.into());
    root_ctx
}

fn main() {
    let template = r#"
        // hstack
            // vstack
            //     text [color: blue] "START I have {{ counter }} bunnies in a hat"
            //     text "TIME {{ time }}"
            //     for item in {{ data }}
            //         text [color: red] "{{ item }} hello {{ counter }}"
            //             span [color: magenta] "- bleep bloop -"
            vstack
                text [color: blue] "START I have {{ counter }} bunnies in a hat"
                text "TIME {{ time }}"
                for item in {{ data }}
                    text [color: red] "{{ item }} hello {{ counter }}"
                        span [color: magenta] "- bleep bloop -"
    "#;

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

    loop {
        root_ctx.insert("counter", counter.into());

        let mut gen: WidgetGenerator = WidgetGenerator::new(&templates, &root_ctx, &lookup);

        let now = Instant::now();

        let mut widgets = vec![];
        while let Some(mut widget) = gen.next_widget_container() {
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

        root_ctx.insert("time", format!("{:?}", now.elapsed()).into());
        screen.render(&mut output);

        std::thread::sleep_ms(1000);
        screen.erase();

        counter += 1;
    }
}
