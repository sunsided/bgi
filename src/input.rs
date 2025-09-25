//! Input handling for the BGI library.

use crate::window::WindowId;

/// Key codes (compatible with BGI key definitions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Key {
    // Alphanumeric keys
    A = b'A' as u32,
    B = b'B' as u32,
    C = b'C' as u32,
    D = b'D' as u32,
    E = b'E' as u32,
    F = b'F' as u32,
    G = b'G' as u32,
    H = b'H' as u32,
    I = b'I' as u32,
    J = b'J' as u32,
    K = b'K' as u32,
    L = b'L' as u32,
    M = b'M' as u32,
    N = b'N' as u32,
    O = b'O' as u32,
    P = b'P' as u32,
    Q = b'Q' as u32,
    R = b'R' as u32,
    S = b'S' as u32,
    T = b'T' as u32,
    U = b'U' as u32,
    V = b'V' as u32,
    W = b'W' as u32,
    X = b'X' as u32,
    Y = b'Y' as u32,
    Z = b'Z' as u32,

    Num0 = b'0' as u32,
    Num1 = b'1' as u32,
    Num2 = b'2' as u32,
    Num3 = b'3' as u32,
    Num4 = b'4' as u32,
    Num5 = b'5' as u32,
    Num6 = b'6' as u32,
    Num7 = b'7' as u32,
    Num8 = b'8' as u32,
    Num9 = b'9' as u32,

    // Special keys (using high values to avoid conflicts)
    Home = 0x1001,
    Left = 0x1002,
    Up = 0x1003,
    Right = 0x1004,
    Down = 0x1005,
    PageUp = 0x1006,
    PageDown = 0x1007,
    End = 0x1008,
    Insert = 0x1009,
    Delete = 0x100A,

    F1 = 0x1010,
    F2 = 0x1011,
    F3 = 0x1012,
    F4 = 0x1013,
    F5 = 0x1014,
    F6 = 0x1015,
    F7 = 0x1016,
    F8 = 0x1017,
    F9 = 0x1018,
    F10 = 0x1019,
    F11 = 0x101A,
    F12 = 0x101B,

    CapsLock = 0x1020,
    LeftCtrl = 0x1021,
    RightCtrl = 0x1022,
    LeftShift = 0x1023,
    RightShift = 0x1024,
    LeftAlt = 0x1025,
    RightAlt = 0x1026,
    AltGr = 0x1027,
    LeftGui = 0x1028,
    RightGui = 0x1029,
    Menu = 0x102A,
    Tab = 0x102B,
    Backspace = 0x102C,
    Return = 0x102D,
    Pause = 0x102E,
    ScrollLock = 0x102F,
    Escape = 0x1030,

    Space = b' ' as u32,
}

impl Key {
    /// Convert key to integer value for compatibility.
    pub fn to_int(self) -> u32 {
        self as u32
    }

    /// Create key from integer value.
    pub fn from_int(value: u32) -> Option<Self> {
        match value {
            0x1001 => Some(Self::Home),
            0x1002 => Some(Self::Left),
            0x1003 => Some(Self::Up),
            0x1004 => Some(Self::Right),
            0x1005 => Some(Self::Down),
            0x1006 => Some(Self::PageUp),
            0x1007 => Some(Self::PageDown),
            0x1008 => Some(Self::End),
            0x1009 => Some(Self::Insert),
            0x100A => Some(Self::Delete),
            0x1010 => Some(Self::F1),
            0x1011 => Some(Self::F2),
            0x1012 => Some(Self::F3),
            0x1013 => Some(Self::F4),
            0x1014 => Some(Self::F5),
            0x1015 => Some(Self::F6),
            0x1016 => Some(Self::F7),
            0x1017 => Some(Self::F8),
            0x1018 => Some(Self::F9),
            0x1019 => Some(Self::F10),
            0x101A => Some(Self::F11),
            0x101B => Some(Self::F12),
            0x1020 => Some(Self::CapsLock),
            0x1021 => Some(Self::LeftCtrl),
            0x1022 => Some(Self::RightCtrl),
            0x1023 => Some(Self::LeftShift),
            0x1024 => Some(Self::RightShift),
            0x1025 => Some(Self::LeftAlt),
            0x1026 => Some(Self::RightAlt),
            0x1027 => Some(Self::AltGr),
            0x1028 => Some(Self::LeftGui),
            0x1029 => Some(Self::RightGui),
            0x102A => Some(Self::Menu),
            0x102B => Some(Self::Tab),
            0x102C => Some(Self::Backspace),
            0x102D => Some(Self::Return),
            0x102E => Some(Self::Pause),
            0x102F => Some(Self::ScrollLock),
            0x1030 => Some(Self::Escape),
            b' ' => Some(Self::Space),
            b'A' => Some(Self::A),
            b'B' => Some(Self::B),
            b'C' => Some(Self::C),
            b'D' => Some(Self::D),
            b'E' => Some(Self::E),
            b'F' => Some(Self::F),
            b'G' => Some(Self::G),
            b'H' => Some(Self::H),
            b'I' => Some(Self::I),
            b'J' => Some(Self::J),
            b'K' => Some(Self::K),
            b'L' => Some(Self::L),
            b'M' => Some(Self::M),
            b'N' => Some(Self::N),
            b'O' => Some(Self::O),
            b'P' => Some(Self::P),
            b'Q' => Some(Self::Q),
            b'R' => Some(Self::R),
            b'S' => Some(Self::S),
            b'T' => Some(Self::T),
            b'U' => Some(Self::U),
            b'V' => Some(Self::V),
            b'W' => Some(Self::W),
            b'X' => Some(Self::X),
            b'Y' => Some(Self::Y),
            b'Z' => Some(Self::Z),
            b'0' => Some(Self::Num0),
            b'1' => Some(Self::Num1),
            b'2' => Some(Self::Num2),
            b'3' => Some(Self::Num3),
            b'4' => Some(Self::Num4),
            b'5' => Some(Self::Num5),
            b'6' => Some(Self::Num6),
            b'7' => Some(Self::Num7),
            b'8' => Some(Self::Num8),
            b'9' => Some(Self::Num9),
            _ => None,
        }
    }

    /// Convert from winit key.
    #[cfg(feature = "winit-backend")]
    pub fn from_winit_key(winit_key: &winit::keyboard::Key) -> Result<Self, &'static str> {
        use winit::keyboard::{Key as WinitKey, NamedKey};

        match winit_key {
            WinitKey::Named(NamedKey::ArrowLeft) => Ok(Self::Left),
            WinitKey::Named(NamedKey::ArrowRight) => Ok(Self::Right),
            WinitKey::Named(NamedKey::ArrowUp) => Ok(Self::Up),
            WinitKey::Named(NamedKey::ArrowDown) => Ok(Self::Down),
            WinitKey::Named(NamedKey::Home) => Ok(Self::Home),
            WinitKey::Named(NamedKey::End) => Ok(Self::End),
            WinitKey::Named(NamedKey::PageUp) => Ok(Self::PageUp),
            WinitKey::Named(NamedKey::PageDown) => Ok(Self::PageDown),
            WinitKey::Named(NamedKey::Insert) => Ok(Self::Insert),
            WinitKey::Named(NamedKey::Delete) => Ok(Self::Delete),
            WinitKey::Named(NamedKey::F1) => Ok(Self::F1),
            WinitKey::Named(NamedKey::F2) => Ok(Self::F2),
            WinitKey::Named(NamedKey::F3) => Ok(Self::F3),
            WinitKey::Named(NamedKey::F4) => Ok(Self::F4),
            WinitKey::Named(NamedKey::F5) => Ok(Self::F5),
            WinitKey::Named(NamedKey::F6) => Ok(Self::F6),
            WinitKey::Named(NamedKey::F7) => Ok(Self::F7),
            WinitKey::Named(NamedKey::F8) => Ok(Self::F8),
            WinitKey::Named(NamedKey::F9) => Ok(Self::F9),
            WinitKey::Named(NamedKey::F10) => Ok(Self::F10),
            WinitKey::Named(NamedKey::F11) => Ok(Self::F11),
            WinitKey::Named(NamedKey::F12) => Ok(Self::F12),
            WinitKey::Named(NamedKey::CapsLock) => Ok(Self::CapsLock),
            WinitKey::Named(NamedKey::Control) => Ok(Self::LeftCtrl),
            WinitKey::Named(NamedKey::Shift) => Ok(Self::LeftShift),
            WinitKey::Named(NamedKey::Alt) => Ok(Self::LeftAlt),
            WinitKey::Named(NamedKey::Super) => Ok(Self::LeftGui),
            WinitKey::Named(NamedKey::ContextMenu) => Ok(Self::Menu),
            WinitKey::Named(NamedKey::Tab) => Ok(Self::Tab),
            WinitKey::Named(NamedKey::Backspace) => Ok(Self::Backspace),
            WinitKey::Named(NamedKey::Enter) => Ok(Self::Return),
            WinitKey::Named(NamedKey::Pause) => Ok(Self::Pause),
            WinitKey::Named(NamedKey::ScrollLock) => Ok(Self::ScrollLock),
            WinitKey::Named(NamedKey::Escape) => Ok(Self::Escape),
            WinitKey::Named(NamedKey::Space) => Ok(Self::Space),
            WinitKey::Character(c) => match c.as_str() {
                "a" | "A" => Ok(Self::A),
                "b" | "B" => Ok(Self::B),
                "c" | "C" => Ok(Self::C),
                "d" | "D" => Ok(Self::D),
                "e" | "E" => Ok(Self::E),
                "f" | "F" => Ok(Self::F),
                "g" | "G" => Ok(Self::G),
                "h" | "H" => Ok(Self::H),
                "i" | "I" => Ok(Self::I),
                "j" | "J" => Ok(Self::J),
                "k" | "K" => Ok(Self::K),
                "l" | "L" => Ok(Self::L),
                "m" | "M" => Ok(Self::M),
                "n" | "N" => Ok(Self::N),
                "o" | "O" => Ok(Self::O),
                "p" | "P" => Ok(Self::P),
                "q" | "Q" => Ok(Self::Q),
                "r" | "R" => Ok(Self::R),
                "s" | "S" => Ok(Self::S),
                "t" | "T" => Ok(Self::T),
                "u" | "U" => Ok(Self::U),
                "v" | "V" => Ok(Self::V),
                "w" | "W" => Ok(Self::W),
                "x" | "X" => Ok(Self::X),
                "y" | "Y" => Ok(Self::Y),
                "z" | "Z" => Ok(Self::Z),
                "0" => Ok(Self::Num0),
                "1" => Ok(Self::Num1),
                "2" => Ok(Self::Num2),
                "3" => Ok(Self::Num3),
                "4" => Ok(Self::Num4),
                "5" => Ok(Self::Num5),
                "6" => Ok(Self::Num6),
                "7" => Ok(Self::Num7),
                "8" => Ok(Self::Num8),
                "9" => Ok(Self::Num9),
                " " => Ok(Self::Space),
                _ => Err("Unsupported character key"),
            },
            _ => Err("Unsupported winit key"),
        }
    }
}

/// Mouse button identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MouseButton {
    /// Left mouse button.
    Left = 1,
    /// Middle mouse button.
    Middle = 2,
    /// Right mouse button.
    Right = 3,
}

impl MouseButton {
    /// Convert from winit mouse button.
    #[cfg(feature = "winit-backend")]
    pub fn from_winit_button(button: winit::event::MouseButton) -> Result<Self, &'static str> {
        match button {
            winit::event::MouseButton::Left => Ok(Self::Left),
            winit::event::MouseButton::Middle => Ok(Self::Middle),
            winit::event::MouseButton::Right => Ok(Self::Right),
            _ => Err("Unsupported mouse button"),
        }
    }
}

/// Mouse event data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseEvent {
    /// Mouse button.
    pub button: MouseButton,
    /// X coordinate.
    pub x: i32,
    /// Y coordinate.
    pub y: i32,
    /// Button state (pressed/released).
    pub pressed: bool,
    /// Double-click flag.
    pub double_click: bool,
}

/// Input event types.
#[derive(Debug, Clone)]
pub enum InputEvent {
    /// Key press/release event.
    Key {
        /// Window that received the event.
        window_id: WindowId,
        /// Key code.
        key: Key,
        /// Key state (pressed/released).
        pressed: bool,
        /// Character representation (if printable).
        character: Option<char>,
    },
    /// Mouse button event.
    MouseButton {
        /// Window that received the event.
        window_id: WindowId,
        /// Mouse event data.
        event: MouseEvent,
    },
    /// Mouse movement event.
    MouseMove {
        /// Window that received the event.
        window_id: WindowId,
        /// X coordinate.
        x: i32,
        /// Y coordinate.
        y: i32,
    },
    /// Mouse wheel event.
    MouseWheel {
        /// Window that received the event.
        window_id: WindowId,
        /// Wheel delta.
        delta: i32,
    },
    /// Window close event.
    WindowClose {
        /// Window being closed.
        window_id: WindowId,
    },
    /// Window resize event.
    WindowResize {
        /// Window being resized.
        window_id: WindowId,
        /// New width.
        width: u32,
        /// New height.
        height: u32,
    },
    /// Quit application event.
    Quit,
}
