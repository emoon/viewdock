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

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect {
            x: x,
            y: y,
            width: width,
            height: height
        }
    }
}

pub struct View {
    pub handle: ViewHandle,
    pub rect: Rect
}

pub enum Direction {
    Vertical,
    Horizontal,
    Full,
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
    pub split: Option<Box<Split>>,
    pub rect: Rect,
}

impl Workspace {
    /// Construct a new workspace. The rect has to be y >= 0, x >= 0, width > 0 and height > 0
    pub fn new(rect: Rect) -> Result<Workspace> {
        if rect.x < 0.0 {
            return Err(Error::IllegalSize("x has to be non-negative".to_owned()));
        }

        if rect.y < 0.0 {
            return Err(Error::IllegalSize("y has to be non-negative".to_owned()));
        }

        if rect.width <= 0.0 {
            return Err(Error::IllegalSize("width has to be larger than 0.0".to_owned()));
        }

        if rect.height <= 0.0 {
            return Err(Error::IllegalSize("height has to be larger than 0.0".to_owned()));
        }

        Ok(Workspace {
            split: None,
            rect: rect })
    }
}

#[cfg(test)]
mod test {
    use {Workspace, Rect};

    #[test]
    fn test_validate_x_less_than_zero() {
        assert_eq!(Workspace::new(Rect::new(-0.1, 0.0, 1.0, 1.0)).is_err(), true);
    }

    #[test]
    fn test_validate_y_less_than_zero() {
        assert_eq!(Workspace::new(Rect::new(0.0, -0.1, 1.0, 1.0)).is_err(), true);
    }

    #[test]
    fn test_validate_width_zero() {
        assert_eq!(Workspace::new(Rect::new(0.0, 0.0, 0.0, 1.0)).is_err(), true);
    }

    #[test]
    fn test_validate_height_zero() {
        assert_eq!(Workspace::new(Rect::new(0.0, 0.0, 1.0, 0.0)).is_err(), true);
    }

    #[test]
    fn test_validate_width_less_than_zero() {
        assert_eq!(Workspace::new(Rect::new(0.0, 0.0, -1.0, 0.0)).is_err(), true);
    }

    #[test]
    fn test_validate_height_less_than_zero() {
        assert_eq!(Workspace::new(Rect::new(0.0, 0.0, 0.0, -1.0)).is_err(), true);
    }

    #[test]
    fn test_validate_workspace_ok() {
        assert_eq!(Workspace::new(Rect::new(0.0, 0.0, 1024.0, 1024.0)).is_ok(), true);
    }
}
