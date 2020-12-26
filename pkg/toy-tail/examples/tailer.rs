use toy_tail::{watch, PrintHandler, RegexParser, TailContext};

fn main() {
    let path = "/tmp/toy";
    let prefix = "hello.example.log";

    println!("watching dir:{}, prefix:{}", path, prefix);
    let parser = RegexParser::new();
    if let Err(e) = parser {
        println!("regex build error. {}", e);
        return;
    }
    let mut ctx = TailContext::new(PrintHandler, parser.unwrap());
    match watch(path, prefix, &mut ctx) {
        Ok(_) => {
            println!("watch end.");
        }
        Err(e) => {
            println!("error: {:?}", e);
        }
    }
}
