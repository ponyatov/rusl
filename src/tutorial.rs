use std::io;

pub fn tutorial() {
    // https://rutube.ru/video/b24398e527ed2154aac4cd9081b316f6/?r=wd
    let (mut x, y) = (0, 0);
    [x, _] = [1, 2];
    eprintln!("x:{x}");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
