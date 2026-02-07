use macroquad::prelude::*;

#[macroquad::main("Test")]
async fn main() {
    loop {
        next_frame().await
    }
}
