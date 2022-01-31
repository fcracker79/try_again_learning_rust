pub trait Draw {
    fn draw(&self);
    // for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically
    // This fails when defining Screen::components
    // fn unsafe1(&self) -> Self;
    // fn unsafe2<T>(&self, x: T);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("<BUTTON>\n");
    }

    // fn unsafe1(&self) -> Self {
    //     Button {
    //         width: 50,
    //         height: 10,
    //         label: String::from("OK"),
    //     }
    // }
    // fn unsafe2<T>(&self, x: T) {
    // }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("<BOX>({}, {})", self.width, self.height);
    }

    // fn unsafe2<T>(&self, x: T) {
    // }

    // fn unsafe1(&self) -> Self {
    //     SelectBox {
    //         width: 75,
    //         height: 10,
    //         options: vec![
    //             String::from("Yes"),
    //             String::from("Maybe"),
    //             String::from("No"),
    //         ],
    //     }
    // }
}
