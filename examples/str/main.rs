use chaorder::{
    draw::Drawer,
    illustration::{Illustration, ParseIllustration},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source = "It was a bright cold day in April, and the clocks were striking thirteen.
Winston Smith, his chin nuzzled into his breast in an effort to escape the vile wind,
slipped quickly through the glass doors of Victory Mansions,
though not quickly enough to prevent a swirl of gritty dust from entering along with him.
";

    let illustration: Illustration = ParseIllustration::from_str(source);

    let mut draw: Drawer = Drawer::new(illustration)
        .with_char_range_reduction_factor(3)
        .into();
    draw.run()
}
