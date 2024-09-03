#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]

pub mod forth;
pub mod tutorial;

fn main() {
    fn arg(argc: usize, argv: &str) {
        eprintln!("argv[{argc}] = {argv:?}");
    }
    // \ arg
    let argv: Vec<String> = std::env::args().collect();
    let argc = argv.len();
    arg(0, &argv[0]);
    for (argc, argv) in argv.iter().enumerate().skip(1) {
        arg(argc, argv);
    }
    // /
    tutorial::tutorial();
    forth::check_memory();
    unsafe {
        // forth::vm();
    }
}
