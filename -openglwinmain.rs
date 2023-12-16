extern crate glutin;
extern crate gl;

use std::collections::{self, HashMap};

use glutin::event::{
        Event,
        WindowEvent,
        VirtualKeyCode
};
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

pub trait InputReader {
    fn key_down(&self, key : &glutin::event::VirtualKeyCode) -> bool;
}
pub trait InputWriter {
    fn write_key(&mut self, key : &glutin::event::VirtualKeyCode, value : bool);
}

pub struct InputState {
    keys : HashMap<glutin::event::VirtualKeyCode, bool>,
}

impl InputWriter for InputState {
    fn write_key(&mut self, key : &glutin::event::VirtualKeyCode, value : bool) {
        self.keys.insert(*key, value);
    }
}

impl InputReader for InputState {
    fn key_down(&self, key : &glutin::event::VirtualKeyCode) -> bool {
        if self.keys.contains_key(key) {
            return self.keys[key];
        }
        return false;
    }
}


fn main() {
    
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("[opengl::rust]");

    let windowed_context = ContextBuilder::new()
                                .build_windowed(wb, &el)
                                    .unwrap();
    
    let windowed_context = unsafe {
         windowed_context.make_current().unwrap() 
    };
    
    gl::load_with(
        |s| windowed_context.get_proc_address(s) as *const _
    );
    
    let mut inputManager = InputState {
        keys : HashMap::new(),
    };
    
    el.run(move |event, _, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Wait;
        
        if inputManager.key_down(&VirtualKeyCode::Escape) {
            *control_flow = glutin::event_loop::ControlFlow::Exit;
        }
        
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = glutin::event_loop::ControlFlow::Exit,
                WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {
                   let key = input.virtual_keycode.unwrap();
                   let isDown = input.state == glutin::event::ElementState::Pressed;
                   inputManager.write_key(&key, isDown);
                }
                _ => (),
            },
            _ => (),
        }
    });
}