use std::thread;
use std::time::Duration;

use metrics::{
    decrement_gauge, histogram, increment_counter, increment_gauge, register_histogram, Unit,
};
use metrics_exporter_tcp::TcpBuilder;

use quanta::Clock;
use rand::{thread_rng, Rng};

fn main() {
    tracing_subscriber::fmt::init();

    let builder = TcpBuilder::new();
    builder.install().expect("failed to install TCP recorder");

    let mut clock = Clock::new();
    let mut last = None;

    register_histogram!("tcp_server_loop_delta_ns", Unit::Nanoseconds);

    loop {
        increment_counter!("tcp_server_loops", "system" => "foo");

        if let Some(t) = last {
            let delta: Duration = clock.now() - t;
            histogram!("tcp_server_loop_delta_ns", delta, "system" => "foo");
        }

        let increment_gauge = thread_rng().gen_bool(0.75);
        if increment_gauge {
            increment_gauge!("lucky_iterations", 1.0);
        } else {
            decrement_gauge!("lucky_iterations", 1.0);
        }

        last = Some(clock.now());

        let sleep_time = thread_rng().gen_range(250, 750);

        thread::sleep(Duration::from_millis(sleep_time));
    }
}
