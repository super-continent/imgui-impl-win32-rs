use std::mem;

use imgui::{BackendFlags, Context, ImString, Key};
use thiserror::Error;
use winapi::shared::{
    minwindef::*,
    ntdef::LARGE_INTEGER,
    windef::{
        RECT,
        HWND,
    }
};
use winapi::um::{profileapi::*, winuser::*};

pub struct Win32Impl {
    hwnd: HWND,
    frequency: LARGE_INTEGER,
    time: LARGE_INTEGER,
}

impl Win32Impl {
    pub unsafe fn init(imgui: &mut Context, hwnd: HWND) -> Result<Win32Impl, Win32ImplError> {
        let mut frequency: LARGE_INTEGER = mem::zeroed();
        let mut time: LARGE_INTEGER = mem::zeroed();

        if FALSE == QueryPerformanceFrequency(&mut frequency) {
            return Err(Win32ImplError::ExternalError("QueryPerformanceFrequency failed".into()));
        };
        if FALSE == QueryPerformanceCounter(&mut time) {
            return Err(Win32ImplError::ExternalError("QueryPerformanceCounter failed".into()));
        };

        let io = imgui.io_mut();

        io.backend_flags |= BackendFlags::HAS_MOUSE_CURSORS;
        io.backend_flags |= BackendFlags::HAS_SET_MOUSE_POS;

        io.key_map[Key::Tab as usize] = VK_TAB as u32;
        io.key_map[Key::LeftArrow as usize] = VK_LEFT as u32;
        io.key_map[Key::RightArrow as usize] = VK_RIGHT as u32;
        io.key_map[Key::UpArrow as usize] = VK_UP as u32;
        io.key_map[Key::DownArrow as usize] = VK_DOWN as u32;
        io.key_map[Key::PageUp as usize] = VK_PRIOR as u32;
        io.key_map[Key::PageDown as usize] = VK_NEXT as u32;
        io.key_map[Key::Home as usize] = VK_HOME as u32;
        io.key_map[Key::End as usize] = VK_END as u32;
        io.key_map[Key::Insert as usize] = VK_INSERT as u32;
        io.key_map[Key::Delete as usize] = VK_DELETE as u32;
        io.key_map[Key::Backspace as usize] = VK_BACK as u32;
        io.key_map[Key::Space as usize] = VK_SPACE as u32;
        io.key_map[Key::KeyPadEnter as usize] = VK_RETURN as u32;
        io.key_map[Key::Escape as usize] = VK_ESCAPE as u32;
        io.key_map[Key::KeyPadEnter as usize] = VK_RETURN as u32;
        io.key_map[Key::A as usize] = 'A' as u32;
        io.key_map[Key::C as usize] = 'C' as u32;
        io.key_map[Key::V as usize] = 'V' as u32;
        io.key_map[Key::X as usize] = 'X' as u32;
        io.key_map[Key::Y as usize] = 'Y' as u32;
        io.key_map[Key::Z as usize] = 'Z' as u32;

        imgui.set_platform_name(Some(ImString::from(format!(
            "imgui_rs_impl_win32 {}",
            env!("CARGO_PKG_VERSION")
        ))));

        return Ok(Win32Impl {
            hwnd,
            frequency,
            time,
        });
    }

    pub unsafe fn prepare_frame(&mut self, imgui: &mut Context) -> Result<(), Win32ImplError> {
        let io = imgui.io_mut();

        // Set up display size every frame to handle resizing
        let mut rect: RECT = mem::zeroed();
        if FALSE == GetClientRect(self.hwnd, &mut rect) {
            return Err(Win32ImplError::ExternalError("GetClientRect failed".into()));
        };
        io.display_size = [(rect.right - rect.left) as f32, (rect.bottom - rect.top) as f32];
        
        // Perform time step
        let mut current_time: LARGE_INTEGER = mem::zeroed();
        QueryPerformanceCounter(&mut current_time);

        let current_time_i = *current_time.QuadPart();
        let last_time_i = *self.time.QuadPart();
        let ticks_per_second_i  = *self.frequency.QuadPart();
        
        io.delta_time = ((current_time_i - last_time_i) / ticks_per_second_i) as f32;
        self.time = current_time;

        // Read key states
        io.key_ctrl = (GetKeyState(VK_CONTROL) as u16 & 0x8000) != 0;
        io.key_shift = (GetKeyState(VK_SHIFT) as u16 & 0x8000) != 0;
        io.key_alt = (GetKeyState(VK_MENU) as u16 & 0x8000) != 0;
        io.key_super = false;

        // TODO mouse cursor impl

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Win32ImplError {
    #[error("Failed to prepare frame - {0}")]
    ExternalError(String),
}
