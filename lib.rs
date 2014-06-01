use std::hash::Hash;

#[deriving(PartialEq, Clone)]
struct DBox {
    x: f32,
    y: f32,
    w: f32,
    h: f32
}

trait ClippingBox {
    fn global_dimension(&self)-> DBox;
    fn clipping_box(&self)-> DBox;
}

trait Component<R, Canvas> {
    fn id(&self)-> &'static str;
    fn draw<Canvas, Clip:ClippingBox>(&self, cbox: Clip, c: &Canvas);
    fn act(&self, uic: &mut UiContext<Canvas>)-> R;
}

struct UiContext<'a, Canvas> {
    hot: Option<&'static str>,
    active: Option<&'static str>,
    canvas: &'a mut Canvas
}

impl <'a, Canvas> UiContext<'a, Canvas> {
    fn new(canvas: &'a mut Canvas) -> UiContext<'a, Canvas> {
        UiContext {
            hot: None,
            active: None,
            canvas: canvas
        }
    }

    fn with<R, Clip: ClippingBox, C: Component<R, Canvas>>
        (&mut self, component: C, clipping: Clip)-> R {
        component.draw(clipping, self.canvas);
        component.act(self)
    }
}

fn main(){}
