use collections::boxed::Box;
use alloc::arc::Arc;
use core::cell::RefCell;

use scene_graph::{Scene, Component, ComponentManager, Id};
use camera::Camera;


struct CameraManagerData {
    scene: Option<Scene>,
    active_camera: Option<Camera>,
    components: usize,
}


#[derive(Clone)]
pub struct CameraManager {
    data: Arc<RefCell<CameraManagerData>>,
}

impl CameraManager {

    pub fn new() -> CameraManager {
        CameraManager {
            data: Arc::new(RefCell::new(CameraManagerData {
                scene: None,
                active_camera: None,
                components: 0usize,
            }))
        }
    }

    pub fn set_active_camera(&self, camera: Camera) -> &Self {
        let mut data = self.data.borrow_mut();

        if let Some(ref active_camera) = data.active_camera {
            active_camera.__set_active(false);
        }

        camera.__set_active(true);
        data.active_camera = Some(camera);

        self
    }
    pub fn active_camera(&self) -> Option<Camera> {
        match self.data.borrow().active_camera {
            Some(ref active_camera) => Some(active_camera.clone()),
            None => None,
        }
    }
}

impl ComponentManager for CameraManager {

    fn id(&self) -> Id { Id::of::<CameraManager>() }

    fn scene(&self) -> Option<Scene> {
        match self.data.borrow().scene {
            Some(ref scene) => Some(scene.clone()),
            None => None,
        }
    }
    fn set_scene(&self, scene: Option<Scene>) {
        self.data.borrow_mut().scene = scene;
    }

    fn order(&self) -> usize { 0 }
    fn is_empty(&self) -> bool {
        self.data.borrow().components == 0usize
    }

    fn destroy(&self) {}
    fn init(&self) {}
    fn update(&self) {}

    fn add_component(&self, component: &Box<Component>) {
        let component = component.downcast_ref::<Camera>().unwrap();

        component.set_camera_manager(Some(self.clone()));

        if component.active() {
            self.set_active_camera(component.clone());
        }

        self.data.borrow_mut().components += 1;
    }
    fn remove_component(&self, component: &Box<Component>) {
        let component = component.downcast_ref::<Camera>().unwrap();
        self.data.borrow_mut().components -= 1;
        component.set_camera_manager(None);
    }
}
