mod create_window;
mod state;

use create_window::run;

fn main() {
    pollster::block_on(run());
}
