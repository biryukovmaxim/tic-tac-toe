use engine::stream_in::StreamIn;
use engine::Engine;

fn main() {
    let engine = Engine::new(StreamIn::new(std::io::stdin()));
    engine.run();
}
