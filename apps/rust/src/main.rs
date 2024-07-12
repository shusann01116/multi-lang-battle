use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    time::{Duration, Instant},
};

fn main() {
    println!("START!!!!");
    let mut sum = Duration::new(0, 0);
    let count = 5;

    for _ in 0..count {
        let now = Instant::now();

        let reader = BufReader::new(File::open("/fixtures/sample.csv").unwrap());
        let mut writer = BufWriter::new(File::create("./sample.csv").unwrap());

        let _: Vec<_> = reader
            .lines()
            .flatten()
            .map(|line| {
                writer.write(line.as_ref()).unwrap();
                writer.write(b"\n").unwrap();
            })
            .collect();

        writer.flush().unwrap();
        sum += now.elapsed();
        println!("Time Result: {:.4}", now.elapsed().as_secs_f32());
    }

    println!("Rust Average: {:.4}", (sum / count).as_secs_f32());
}
