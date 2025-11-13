#[allow(async_fn_in_trait)]
pub trait Scene {
    fn update(&mut self);
}
