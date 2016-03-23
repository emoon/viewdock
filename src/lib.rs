
enum Direction {
    Vertical,
    Horizontal,
}


struct Container {
    temp: i32,
}

struct Split {
    split: Option<Box<Split>>,
    leftTop: Container,
    rightBottom: Container,
    percent: f32,
    direction: Direction,
}

struct Workspace {
    split: Split,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
