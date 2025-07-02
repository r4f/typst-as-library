use typst_as_library::TypstWrapperWorld;
use typst_pdf::{self, PdfOptions};
use std::fs;

fn main() {
    //DejaVu Sans Mono is one of the built-in fonts of typst.
    let content = "
    = #text(font: \"DejaVu Sans Mono\", fallback: false)[
        Hello, World!
    ]";

    // All the abstraction needed is here (!)
    let world = TypstWrapperWorld::new(
        content.to_owned()
    );

    // Render document
    let document = typst::compile(&world)
        .output
        .expect("Error compiling typst");

    // Output to pdf
    let pdf = typst_pdf::pdf(&document,&PdfOptions::default())
        .expect("Error exporting PDF");
    fs::write("./output.pdf", pdf)
        .expect("Error writing PDF.");
}
