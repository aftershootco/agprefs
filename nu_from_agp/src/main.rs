use nu_from_agp::Agp;
use nu_plugin::{serve_plugin, MsgPackSerializer};

fn main() {
    serve_plugin(&mut Agp, MsgPackSerializer {})
}
