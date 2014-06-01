use super::super::{Component, ClippingBox, UiContext};

pub struct Button {
    id: &'static str,
    text: ~str
}

impl Component<bool, int> for Button {
    fn id(&self)-> &'static str { self.id }

    fn draw(&self, cbox: ClippingBox, c: &int) {

    }

    fn act(&self, uic: &mut UiContext<int>) -> bool {
        let (id, hot, active)  = (self.id, uic.get_hot(), uic.get_active());
        if hot.is_none() || active.is_none() {
            return false;
        }

        let hot = hot.unwrap();
        let active = active.unwrap();
        if hot == id && active == id {
            uic.set_active(None);
            return true;
        }
        return false;
    }
}
