use std::num::abs;

pub mod components {
    pub mod button;
}

#[deriving(PartialEq, Clone)]
pub struct DBox {
    x: f32,
    y: f32,
    w: f32,
    h: f32
}

impl DBox {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        if x < self.x || y < self.y {
            return false;
        }
        if x > self.x + self.w || y > self.y + self.h {
            return false;
        }
        return true;
    }

    pub fn intersects(&self, other: &DBox) -> bool {
        (abs(self.x - other.x) * 2.0 < (self.w + other.w)) &&
        (abs(self.y - other.y) * 2.0 < (self.h + other.h))
    }
}

pub struct ClippingBox {
    global_dimension: DBox,
    clipping_box: DBox
}

pub trait Component<R, Canvas> {
    fn id(&self)-> &'static str;
    fn draw(&self, cbox: ClippingBox, c: &Canvas);
    fn act(&self, uic: &mut UiContext<Canvas>)-> R;
}

pub struct UiContext<'a, Canvas> {
    hot: Option<&'static str>,
    active: Option<&'static str>,
    canvas: &'a mut Canvas
}

impl <'a, Canvas> UiContext<'a, Canvas> {
    pub fn new(canvas: &'a mut Canvas) -> UiContext<'a, Canvas> {
        UiContext {
            hot: None,
            active: None,
            canvas: canvas
        }
    }

    pub fn with<R, C: Component<R, Canvas>> (&mut self, component: C, clipping: ClippingBox)-> R {
        component.draw(clipping, self.canvas);
        component.act(self)
    }

    pub fn get_hot(&self)-> Option<&'static str> {
        self.hot
    }

    pub fn get_active(&self)-> Option<&'static str> {
        self.active
    }

    pub fn set_hot(&mut self, hot: Option<&'static str>) {
        self.hot = hot;
    }
    pub fn set_active(&mut self, active: Option<&'static str>) {
        self.active = active;
    }
}

fn main(){}
