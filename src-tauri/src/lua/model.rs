macro_rules! define_enum_with_into {
    (
        $(#[$meta:meta])*
        $vis:vis enum $Name:ident {
            $(
                $(#[$var_meta:meta])*
                $Variant:ident => $Expr:expr
            ),* $(,)?
        }
        => $Target:ty
    ) => {
        $(#[$meta])*
        #[derive(strum::EnumString)]
        $vis enum $Name { $( $(#[$var_meta])* $Variant ),* }

        impl From<$Name> for $Target {
            #[inline(always)]
            fn from(v: $Name) -> Self {
                match v { $( $(#[$var_meta])* $Name::$Variant => $Expr ),* }
            }
        }
    };
}

define_enum_with_into! {
    pub enum Direction {
        Press => enigo::Direction::Press,
        Release => enigo::Direction::Release,
        Click => enigo::Direction::Click,
    } => enigo::Direction
}

define_enum_with_into! {
    pub enum KeySend {
        F1 => enigo::Key::F1,
        F2 => enigo::Key::F2,
        F3 => enigo::Key::F3,
        F4 => enigo::Key::F4,
        F5 => enigo::Key::F5,
        F6 => enigo::Key::F6,
        F7 => enigo::Key::F7,
        F8 => enigo::Key::F8,
        F9 => enigo::Key::F9,
        F10 => enigo::Key::F10,
        F11 => enigo::Key::F11,
        F12 => enigo::Key::F12,
        F13 => enigo::Key::F13,
        F14 => enigo::Key::F14,
        F15 => enigo::Key::F15,
        F16 => enigo::Key::F16,
        F17 => enigo::Key::F17,
        F18 => enigo::Key::F18,
        F19 => enigo::Key::F19,
        F20 => enigo::Key::F20,
        LShift => enigo::Key::LShift,
        RShift => enigo::Key::RShift,
        LCtrl => enigo::Key::LControl,
        RCtrl => enigo::Key::RControl,
        Tab => enigo::Key::Tab,
        Space => enigo::Key::Space,

        #[cfg(target_os = "macos")] LCommand => enigo::Key::Meta,
        #[cfg(target_os = "windows")] LCommand => enigo::Key::LWin,
        #[cfg(target_os = "macos")] RCommand => enigo::Key::Other(0x36),
        #[cfg(target_os = "windows")] RCommand => enigo::Key::RWin,

        #[cfg(target_os = "macos")] Option => enigo::Key::Option,
        #[cfg(target_os = "windows")] Option => enigo::Key::Alt,

        CapsLock => enigo::Key::CapsLock,
        Enter => enigo::Key::Return,
        Esc => enigo::Key::Escape,
        Backspace => enigo::Key::Backspace,
        Delete => enigo::Key::Delete,

        Up => enigo::Key::UpArrow,
        Down => enigo::Key::DownArrow,
        Left => enigo::Key::LeftArrow,
        Right => enigo::Key::RightArrow,

        Home => enigo::Key::Home,
        End => enigo::Key::End,
        PageUp => enigo::Key::PageUp,
        PageDown => enigo::Key::PageDown,

        #[cfg(target_os = "macos")] Numpad0 => enigo::Key::Other(82),
        #[cfg(target_os = "windows")] Numpad0 => enigo::Key::Numpad0,
        #[cfg(target_os = "macos")] Numpad1 => enigo::Key::Other(83),
        #[cfg(target_os = "windows")] Numpad1 => enigo::Key::Numpad1,
        #[cfg(target_os = "macos")] Numpad2 => enigo::Key::Other(84),
        #[cfg(target_os = "windows")] Numpad2 => enigo::Key::Numpad2,
        #[cfg(target_os = "macos")] Numpad3 => enigo::Key::Other(85),
        #[cfg(target_os = "windows")] Numpad3 => enigo::Key::Numpad3,
        #[cfg(target_os = "macos")] Numpad4 => enigo::Key::Other(86),
        #[cfg(target_os = "windows")] Numpad4 => enigo::Key::Numpad4,
        #[cfg(target_os = "macos")] Numpad5 => enigo::Key::Other(87),
        #[cfg(target_os = "windows")] Numpad5 => enigo::Key::Numpad5,
        #[cfg(target_os = "macos")] Numpad6 => enigo::Key::Other(88),
        #[cfg(target_os = "windows")] Numpad6 => enigo::Key::Numpad6,
        #[cfg(target_os = "macos")] Numpad7 => enigo::Key::Other(89),
        #[cfg(target_os = "windows")] Numpad7 => enigo::Key::Numpad7,
        #[cfg(target_os = "macos")] Numpad8 => enigo::Key::Other(91),
        #[cfg(target_os = "windows")] Numpad8 => enigo::Key::Numpad8,
        #[cfg(target_os = "macos")] Numpad9 => enigo::Key::Other(92),
        #[cfg(target_os = "windows")] Numpad9 => enigo::Key::Numpad9,
        #[cfg(target_os = "macos")] NumpadAdd => enigo::Key::Other(69),
        #[cfg(target_os = "windows")] NumpadAdd => enigo::Key::Add,
        #[cfg(target_os = "macos")] NumpadSubtract => enigo::Key::Other(78),
        #[cfg(target_os = "windows")] NumpadSubtract => enigo::Key::Subtract,
        #[cfg(target_os = "macos")] NumpadMultiply => enigo::Key::Other(67),
        #[cfg(target_os = "windows")] NumpadMultiply => enigo::Key::Multiply,
        #[cfg(target_os = "macos")] NumpadDivide => enigo::Key::Other(75),
        #[cfg(target_os = "windows")] NumpadDivide => enigo::Key::Divide,
        #[cfg(target_os = "macos")] NumpadEnter => enigo::Key::Other(76),
        #[cfg(target_os = "windows")] NumpadEnter => enigo::Key::Return,
        #[cfg(target_os = "macos")] NumpadDecimal => enigo::Key::Other(65),
        #[cfg(target_os = "windows")] NumpadDecimal => enigo::Key::Decimal,
    } => enigo::Key
}

define_enum_with_into! {
    pub enum Coordinate {
        Abs => enigo::Coordinate::Abs,
        Rel => enigo::Coordinate::Rel,
    } => enigo::Coordinate
}

define_enum_with_into! {
    #[derive(Debug)]
    pub enum ButtonSend {
        Left => enigo::Button::Left,
        Right => enigo::Button::Right,
        Middle => enigo::Button::Middle,
        ScrollUp => enigo::Button::ScrollUp,
        ScrollDown => enigo::Button::ScrollDown,
        ScrollLeft => enigo::Button::ScrollLeft,
        ScrollRight => enigo::Button::ScrollRight,
    } => enigo::Button
}
