use chaorder::{
    draw::TerminalDrawBoard,
    illustration::{Illustration, ParseIllustration},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let illustration: Illustration =
        ParseIllustration::from_txt(&std::path::Path::new("./assets/foo.txt")).unwrap();

    let mut board = TerminalDrawBoard::from(illustration);
    board.draw(None, None, None, None)
}
