use vizia::prelude::*;

const STYLE: &str = r#"
    .container {
        size: 1s;
        background-image: url("sample.png");
    }

    .filter {
        size: 200px;
        left: 300px;
        top: 300px;
        backdrop-filter: blur(16px);
        position: absolute;
        corner-radius: 32px;
        background-color: rgba(255, 255, 255, 0.4);
        border-width: 2px;
        border-color: rgba(255, 255, 255, 0.8);
        padding: 30px;
    }

    label {
        text-wrap: true;
        font-size: 30.0;
        size: 1s;
    }
"#;

#[derive(Lens)]
pub struct AppData {
    skew: f32,
}

pub enum AppEvent {
    SetSkew(f32),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetSkew(val) => {
                self.skew = *val;
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        // Load an image into the binary
        cx.load_image(
            "sample.png",
            include_bytes!("../resources/images/sample-hut-400x300.png"),
            ImageRetentionPolicy::DropWhenUnusedForOneFrame,
        );

        // Element::new(cx).class("container");

        FilterElement::new(cx);
    })
    .title("Backdrop Filter")
    .inner_size((800, 400))
    .run()
}

#[derive(Lens)]
pub struct FilterElement {
    left: Units,
    top: Units,
}

impl FilterElement {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self { left: Units::Pixels(0.0), top: Units::Pixels(0.0) }
            .build(cx, |cx| {
                VStack::new(cx, |_cx| {
                    // Label::new(cx, "This is some text");
                })
                .class("filter")
                .left(FilterElement::left)
                .top(FilterElement::top);
            })
            .class("container")
    }
}

impl View for FilterElement {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, _| {
            if let WindowEvent::MouseMove(x, y) = window_event {
                self.left = Pixels(*x / cx.scale_factor());
                self.top = Pixels(*y / cx.scale_factor());
            }
        })
    }
}
