extern crate chess;
extern crate getopts;
extern crate shakmaty;
extern crate chess_move_gen;

use getopts::Options;
use std::env;

mod perft;
use perft::perform_perft;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_owned();
    let mut depth = 6;
    let mut cache = 0;
    let mut chess = true;
    let mut shakmaty = false;
    let mut chess_mg = false;

    let mut opts = Options::new();
    opts.optopt("f", "fen", "set the FEN to perfromt the perft on.", "FEN");
    opts.optopt("d", "depth", "set the depth to process the perft.", "DEPTH");
    opts.optopt("a", "cache", "set the cache size to a particular value.  Use 0 for no cache.", "CACHE_SIZE");
    opts.optflag("c", "chess", "use the 'chess' library only (default).");
    opts.optflag("g", "chess-move-gen", "use the 'chess-move-gen library only.");
    opts.optflag("s", "shakmaty", "use the 'shakmaty' library only.");
    opts.optflag("a", "all", "use all supported libraries library.");
    opts.optflag("m", "movegen", "use the movegen structure ('chess' only).");
    opts.optflag("h", "help", "print this help menu.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    match matches.opt_str("f") {
        None => {},
        Some(x) => fen = x.to_owned(),
    }

    match matches.opt_str("d") {
        None => {},
        Some(x) => depth = x.parse::<u64>().unwrap()
    }

    match matches.opt_str("a") {
        None => {},
        Some(x) => cache = x.parse::<usize>().unwrap()
    }

    if matches.opt_present("c") {
        chess = true;
        shakmaty = false;
        chess_mg = false;
    }
    if matches.opt_present("s") {
        chess = false;
        shakmaty = true;
        chess_mg = false;
    }
    if matches.opt_present("g") {
        chess = false;
        shakmaty = false;
        chess_mg = true;
    }
    if matches.opt_present("a") {
        chess = true;
        shakmaty = true;
        chess_mg = true;
    }

    perform_perft(fen, depth, cache, matches.opt_present("m"), chess, shakmaty, chess_mg);
}
