pub enum ComposeMode {
    AbsoluteDiff,
    Average,
    AverageLeftWeight,
}

impl ComposeMode {
    pub fn to_fn(&self) -> fn(u8, u8) -> u8 {
        match *self {
            ComposeMode::AbsoluteDiff => compose_absolute_diff,
            ComposeMode::Average => compose_average,
            ComposeMode::AverageLeftWeight => compose_average_left_weight,
        }
    }
}

fn compose_absolute_diff(left: u8, right: u8) -> u8 {
    let (left, right) = (left as i16, right as i16);
    (left - right).abs() as u8
}

fn compose_average(left: u8, right: u8) -> u8 {
    let (left, right) = (left as i16, right as i16);
    ((left + right) / 2) as u8
}

fn compose_average_left_weight(left: u8, right: u8) -> u8 {
    let (left, right) = (left as i16, right as i16);
    ((2 * left + right) / 3) as u8
}



pub fn compose(
    left: &PlanarSurface<Luma, u8>,
    right: &PlanarSurface<Luma, u8>,
    mode: ComposeMode,
) -> PlanarSurface<Luma, u8> {
    let comp = mode.to_fn();
    let mut out = PlanarSurface::new_black(left.width, left.height);

    for (l, r) in left.data.iter().zip(right.data.iter()) {
        out.push(comp(*l, *r));
    }
    LumaSurface {
        width: left.width,
        height: left.height,
        data: Cow::Owned(out),
    }
}
