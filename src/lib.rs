mod error;
pub use self::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub struct ViewHandle(u64);

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub struct View {
    pub handle: ViewHandle,
    pub rect: Rect
}

pub enum Direction {
    Vertical,
    Horizontal,
}

pub struct Container {
    pub views: Vec<View>,
}

pub struct Split {
    pub split: Option<Box<Split>>,
    pub left_top: Container,
    pub right_bottom: Container,
    pub percent: f32,
    pub direction: Direction,
}

pub struct Workspace {
    pub split: Option<Split>,
    pub view: Option<View>,
    pub rect: Rect,
}


impl Workspace {
    pub fn new(rect: Rect) -> Result<Workspace> {
        if rect.y < 0.0 {
            return Err(Error::IllegalSize("y has to be non-negative".to_owned()));
        }

        Ok(Workspace {
            split: None,
            view: None,
            rect: rect })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}
