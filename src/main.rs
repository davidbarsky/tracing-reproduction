use tracing::{span, Level, Span};

fn main() {
    let span = span!(Level::INFO, "hello!");
    let curr = Span::current();
    span.follows_from(curr.id());
}
