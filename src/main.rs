mod create_window;
mod application;

use create_window::run;

fn main() {
    pollster::block_on(run());
}
