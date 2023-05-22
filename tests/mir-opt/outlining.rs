// unit-test: InstSimplify

// EMIT_MIR outlining.opt1.InstSimplify.diff
fn opt1(x: bool) -> u32 {
    if x != true {
        0
    } else {
        1
    }
}

fn main() {
    opt1(false);
}
