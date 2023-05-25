// unit-test: Outline

// EMIT_MIR outlining.foo.Outline.diff
fn foo(x: impl Into<Bar>) {
    let mut x: Bar = x.into();

    // non-generic code
    x.incr();
}

struct Bar(u32);

impl Bar {
    fn incr(&mut self) {
        self.0 += 1;
    }
}

impl From<u32> for Bar {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<u16> for Bar {
    fn from(value: u16) -> Self {
        Self(value as _)
    }
}

fn main() {
    foo(1_u32);
    foo(2_u16);
}
