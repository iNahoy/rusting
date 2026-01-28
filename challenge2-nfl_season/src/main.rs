#![allow(unused_variables)]
const TOUCHDOWN_POINTS: i32 = 6;
fn main() {
    let season: &str = "winter";
    let points_scored: i32 = 28;

    let mut points_scored: i32 = 35;
    println!("{points_scored}");
    points_scored = 42;

    let event_time: &str = "06:00";
    let event_time: i32 = 6;

    println!(
        "print de tudo, viu? {2} {3} {1} {0}",
        TOUCHDOWN_POINTS, season, points_scored, event_time
    )
}
