use chaorder::{
    draw::Drawer,
    illustration::{Illustration, ParseIllustration},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let illustration: Illustration =
        ParseIllustration::from_txt(&std::path::Path::new("./assets/foo.txt")).unwrap();

    let mut draw = Drawer::new(illustration).build();
    draw.run()
}
