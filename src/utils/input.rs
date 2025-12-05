use std::io;

pub fn input_f64(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        match buffer.trim().parse::<f64>() {
            Ok(v) => return v,
            Err(_) => println!("Input tidak valid, coba lagi."),
        }
    }
}
