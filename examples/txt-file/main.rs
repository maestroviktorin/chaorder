use chaorder::{
    draw::Drawer,
    illustration::{Illustration, ParseIllustration},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let illustration: Illustration =
        ParseIllustration::from_txt(&std::path::Path::new("./examples/txt-file/assets/crab.txt"))
            .unwrap();

    let mut draw = Drawer::new(illustration)
        .with_char_range(400)
        .with_start((0, 9))
        .build();
    draw.run()
}
