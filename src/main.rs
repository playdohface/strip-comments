fn main() {
    let mut args = std::env::args();
    let source = if let (Some(_), Some(sourcepath)) = (args.next(), args.next()) {
        std::fs::read_to_string(sourcepath)
    } else {
        eprintln!("Please specify a source file as the first argument.");
        std::process::exit(1);
    };

    let result = match source {
        Ok(source) => strip_comments::strip_comments(&source),
        Err(e) => {
            eprintln!("Could not open file: {}", e);
            std::process::exit(1);
        }
    };
    print!("{result}");
}
