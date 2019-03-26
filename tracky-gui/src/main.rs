extern crate azul;

use azul::{prelude::*, widgets::{label::Label}};

fn main() {
    let mut app = App::new(MyDataModel { }, AppConfig::default()).unwrap();
    let window = app.create_window(WindowCreateOptions::default(), css::native()).unwrap();
    app.run(window).unwrap();
}

struct MyDataModel { }

impl Layout for MyDataModel {
    fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
        let label = Label::new("Hello Azul!").dom();

        Dom::new(NodeType::Div)
            .with_child(label)
    }
}