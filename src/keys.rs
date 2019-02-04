use failure::{Error, Fail};

pub struct KeyDefinition {
    pub key: &'static str,
    pub key_code: u8,
    pub code: &'static str,
    pub text: &'static str,
}


//  Generated the following in node using Puppeteer:
//  keys = require("./lib/USKeyboardLayout.js")
//  toStruct = (kD) => `KeyDefinition { key: "${kD.key}", key_code: "${kD.keyCode}", code: "${kD.code}", text: "${kD.text || kD.key}" }`
//  output = Object.values(keys).map(toStruct).join(",\n")
//  const fs = require('fs');
//  fs.writeFile("/tmp/blah1", output)

pub const USKEYBOARD_LAYOUT: [KeyDefinition; 244] = [
    KeyDefinition { key: "0", key_code: 48, code: "Digit0", text: "0" },
    KeyDefinition { key: "1", key_code: 49, code: "Digit1", text: "1" },
    KeyDefinition { key: "2", key_code: 50, code: "Digit2", text: "2" },
    KeyDefinition { key: "3", key_code: 51, code: "Digit3", text: "3" },
    KeyDefinition { key: "4", key_code: 52, code: "Digit4", text: "4" },
    KeyDefinition { key: "5", key_code: 53, code: "Digit5", text: "5" },
    KeyDefinition { key: "6", key_code: 54, code: "Digit6", text: "6" },
    KeyDefinition { key: "7", key_code: 55, code: "Digit7", text: "7" },
    KeyDefinition { key: "8", key_code: 56, code: "Digit8", text: "8" },
    KeyDefinition { key: "9", key_code: 57, code: "Digit9", text: "9" },
    KeyDefinition { key: "Power", key_code: 0, code: "Power", text: "Power" },
    KeyDefinition { key: "Eject", key_code: 0, code: "Eject", text: "Eject" },
    KeyDefinition { key: "Cancel", key_code: 3, code: "Abort", text: "Cancel" },
    KeyDefinition { key: "Help", key_code: 6, code: "Help", text: "Help" },
    KeyDefinition { key: "Backspace", key_code: 8, code: "Backspace", text: "Backspace" },
    KeyDefinition { key: "Tab", key_code: 9, code: "Tab", text: "Tab" },
    KeyDefinition { key: "Clear", key_code: 12, code: "Numpad5", text: "Clear" },
    KeyDefinition { key: "Enter", key_code: 13, code: "Enter", text: "\r" },
    KeyDefinition { key: "Shift", key_code: 16, code: "ShiftLeft", text: "Shift" },
    KeyDefinition { key: "Shift", key_code: 16, code: "ShiftRight", text: "Shift" },
    KeyDefinition { key: "Control", key_code: 17, code: "ControlLeft", text: "Control" },
    KeyDefinition { key: "Control", key_code: 17, code: "ControlRight", text: "Control" },
    KeyDefinition { key: "Alt", key_code: 18, code: "AltLeft", text: "Alt" },
    KeyDefinition { key: "Alt", key_code: 18, code: "AltRight", text: "Alt" },
    KeyDefinition { key: "Pause", key_code: 19, code: "Pause", text: "Pause" },
    KeyDefinition { key: "CapsLock", key_code: 20, code: "CapsLock", text: "CapsLock" },
    KeyDefinition { key: "Escape", key_code: 27, code: "Escape", text: "Escape" },
    KeyDefinition { key: "Convert", key_code: 28, code: "Convert", text: "Convert" },
    KeyDefinition { key: "NonConvert", key_code: 29, code: "NonConvert", text: "NonConvert" },
    KeyDefinition { key: " ", key_code: 32, code: "Space", text: " " },
    KeyDefinition { key: "PageUp", key_code: 33, code: "Numpad9", text: "PageUp" },
    KeyDefinition { key: "PageUp", key_code: 33, code: "PageUp", text: "PageUp" },
    KeyDefinition { key: "PageDown", key_code: 34, code: "Numpad3", text: "PageDown" },
    KeyDefinition { key: "PageDown", key_code: 34, code: "PageDown", text: "PageDown" },
    KeyDefinition { key: "End", key_code: 35, code: "End", text: "End" },
    KeyDefinition { key: "End", key_code: 35, code: "Numpad1", text: "End" },
    KeyDefinition { key: "Home", key_code: 36, code: "Home", text: "Home" },
    KeyDefinition { key: "Home", key_code: 36, code: "Numpad7", text: "Home" },
    KeyDefinition { key: "ArrowLeft", key_code: 37, code: "ArrowLeft", text: "ArrowLeft" },
    KeyDefinition { key: "ArrowLeft", key_code: 37, code: "Numpad4", text: "ArrowLeft" },
    KeyDefinition { key: "ArrowUp", key_code: 38, code: "Numpad8", text: "ArrowUp" },
    KeyDefinition { key: "ArrowUp", key_code: 38, code: "ArrowUp", text: "ArrowUp" },
    KeyDefinition { key: "ArrowRight", key_code: 39, code: "ArrowRight", text: "ArrowRight" },
    KeyDefinition { key: "ArrowRight", key_code: 39, code: "Numpad6", text: "ArrowRight" },
    KeyDefinition { key: "ArrowDown", key_code: 40, code: "Numpad2", text: "ArrowDown" },
    KeyDefinition { key: "ArrowDown", key_code: 40, code: "ArrowDown", text: "ArrowDown" },
    KeyDefinition { key: "Select", key_code: 41, code: "Select", text: "Select" },
    KeyDefinition { key: "Execute", key_code: 43, code: "Open", text: "Execute" },
    KeyDefinition { key: "PrintScreen", key_code: 44, code: "PrintScreen", text: "PrintScreen" },
    KeyDefinition { key: "Insert", key_code: 45, code: "Insert", text: "Insert" },
    KeyDefinition { key: "Insert", key_code: 45, code: "Numpad0", text: "Insert" },
    KeyDefinition { key: "Delete", key_code: 46, code: "Delete", text: "Delete" },
    KeyDefinition { key: " ", key_code: 46, code: "NumpadDecimal", text: " " },
    KeyDefinition { key: "0", key_code: 48, code: "Digit0", text: "0" },
    KeyDefinition { key: "1", key_code: 49, code: "Digit1", text: "1" },
    KeyDefinition { key: "2", key_code: 50, code: "Digit2", text: "2" },
    KeyDefinition { key: "3", key_code: 51, code: "Digit3", text: "3" },
    KeyDefinition { key: "4", key_code: 52, code: "Digit4", text: "4" },
    KeyDefinition { key: "5", key_code: 53, code: "Digit5", text: "5" },
    KeyDefinition { key: "6", key_code: 54, code: "Digit6", text: "6" },
    KeyDefinition { key: "7", key_code: 55, code: "Digit7", text: "7" },
    KeyDefinition { key: "8", key_code: 56, code: "Digit8", text: "8" },
    KeyDefinition { key: "9", key_code: 57, code: "Digit9", text: "9" },
    KeyDefinition { key: "a", key_code: 65, code: "KeyA", text: "a" },
    KeyDefinition { key: "b", key_code: 66, code: "KeyB", text: "b" },
    KeyDefinition { key: "c", key_code: 67, code: "KeyC", text: "c" },
    KeyDefinition { key: "d", key_code: 68, code: "KeyD", text: "d" },
    KeyDefinition { key: "e", key_code: 69, code: "KeyE", text: "e" },
    KeyDefinition { key: "f", key_code: 70, code: "KeyF", text: "f" },
    KeyDefinition { key: "g", key_code: 71, code: "KeyG", text: "g" },
    KeyDefinition { key: "h", key_code: 72, code: "KeyH", text: "h" },
    KeyDefinition { key: "i", key_code: 73, code: "KeyI", text: "i" },
    KeyDefinition { key: "j", key_code: 74, code: "KeyJ", text: "j" },
    KeyDefinition { key: "k", key_code: 75, code: "KeyK", text: "k" },
    KeyDefinition { key: "l", key_code: 76, code: "KeyL", text: "l" },
    KeyDefinition { key: "m", key_code: 77, code: "KeyM", text: "m" },
    KeyDefinition { key: "n", key_code: 78, code: "KeyN", text: "n" },
    KeyDefinition { key: "o", key_code: 79, code: "KeyO", text: "o" },
    KeyDefinition { key: "p", key_code: 80, code: "KeyP", text: "p" },
    KeyDefinition { key: "q", key_code: 81, code: "KeyQ", text: "q" },
    KeyDefinition { key: "r", key_code: 82, code: "KeyR", text: "r" },
    KeyDefinition { key: "s", key_code: 83, code: "KeyS", text: "s" },
    KeyDefinition { key: "t", key_code: 84, code: "KeyT", text: "t" },
    KeyDefinition { key: "u", key_code: 85, code: "KeyU", text: "u" },
    KeyDefinition { key: "v", key_code: 86, code: "KeyV", text: "v" },
    KeyDefinition { key: "w", key_code: 87, code: "KeyW", text: "w" },
    KeyDefinition { key: "x", key_code: 88, code: "KeyX", text: "x" },
    KeyDefinition { key: "y", key_code: 89, code: "KeyY", text: "y" },
    KeyDefinition { key: "z", key_code: 90, code: "KeyZ", text: "z" },
    KeyDefinition { key: "Meta", key_code: 91, code: "MetaLeft", text: "Meta" },
    KeyDefinition { key: "Meta", key_code: 92, code: "MetaRight", text: "Meta" },
    KeyDefinition { key: "ContextMenu", key_code: 93, code: "ContextMenu", text: "ContextMenu" },
    KeyDefinition { key: "*", key_code: 106, code: "NumpadMultiply", text: "*" },
    KeyDefinition { key: "+", key_code: 107, code: "NumpadAdd", text: "+" },
    KeyDefinition { key: "-", key_code: 109, code: "NumpadSubtract", text: "-" },
    KeyDefinition { key: "/", key_code: 111, code: "NumpadDivide", text: "/" },
    KeyDefinition { key: "F1", key_code: 112, code: "F1", text: "F1" },
    KeyDefinition { key: "F2", key_code: 113, code: "F2", text: "F2" },
    KeyDefinition { key: "F3", key_code: 114, code: "F3", text: "F3" },
    KeyDefinition { key: "F4", key_code: 115, code: "F4", text: "F4" },
    KeyDefinition { key: "F5", key_code: 116, code: "F5", text: "F5" },
    KeyDefinition { key: "F6", key_code: 117, code: "F6", text: "F6" },
    KeyDefinition { key: "F7", key_code: 118, code: "F7", text: "F7" },
    KeyDefinition { key: "F8", key_code: 119, code: "F8", text: "F8" },
    KeyDefinition { key: "F9", key_code: 120, code: "F9", text: "F9" },
    KeyDefinition { key: "F10", key_code: 121, code: "F10", text: "F10" },
    KeyDefinition { key: "F11", key_code: 122, code: "F11", text: "F11" },
    KeyDefinition { key: "F12", key_code: 123, code: "F12", text: "F12" },
    KeyDefinition { key: "F13", key_code: 124, code: "F13", text: "F13" },
    KeyDefinition { key: "F14", key_code: 125, code: "F14", text: "F14" },
    KeyDefinition { key: "F15", key_code: 126, code: "F15", text: "F15" },
    KeyDefinition { key: "F16", key_code: 127, code: "F16", text: "F16" },
    KeyDefinition { key: "F17", key_code: 128, code: "F17", text: "F17" },
    KeyDefinition { key: "F18", key_code: 129, code: "F18", text: "F18" },
    KeyDefinition { key: "F19", key_code: 130, code: "F19", text: "F19" },
    KeyDefinition { key: "F20", key_code: 131, code: "F20", text: "F20" },
    KeyDefinition { key: "F21", key_code: 132, code: "F21", text: "F21" },
    KeyDefinition { key: "F22", key_code: 133, code: "F22", text: "F22" },
    KeyDefinition { key: "F23", key_code: 134, code: "F23", text: "F23" },
    KeyDefinition { key: "F24", key_code: 135, code: "F24", text: "F24" },
    KeyDefinition { key: "NumLock", key_code: 144, code: "NumLock", text: "NumLock" },
    KeyDefinition { key: "ScrollLock", key_code: 145, code: "ScrollLock", text: "ScrollLock" },
    KeyDefinition { key: "AudioVolumeMute", key_code: 173, code: "AudioVolumeMute", text: "AudioVolumeMute" },
    KeyDefinition { key: "AudioVolumeDown", key_code: 174, code: "AudioVolumeDown", text: "AudioVolumeDown" },
    KeyDefinition { key: "AudioVolumeUp", key_code: 175, code: "AudioVolumeUp", text: "AudioVolumeUp" },
    KeyDefinition { key: "MediaTrackNext", key_code: 176, code: "MediaTrackNext", text: "MediaTrackNext" },
    KeyDefinition { key: "MediaTrackPrevious", key_code: 177, code: "MediaTrackPrevious", text: "MediaTrackPrevious" },
    KeyDefinition { key: "MediaStop", key_code: 178, code: "MediaStop", text: "MediaStop" },
    KeyDefinition { key: "MediaPlayPause", key_code: 179, code: "MediaPlayPause", text: "MediaPlayPause" },
    KeyDefinition { key: ";", key_code: 186, code: "Semicolon", text: ";" },
    KeyDefinition { key: "=", key_code: 187, code: "Equal", text: "=" },
    KeyDefinition { key: "=", key_code: 187, code: "NumpadEqual", text: "=" },
    KeyDefinition { key: ",", key_code: 188, code: "Comma", text: "," },
    KeyDefinition { key: "-", key_code: 189, code: "Minus", text: "-" },
    KeyDefinition { key: ".", key_code: 190, code: "Period", text: "." },
    KeyDefinition { key: "/", key_code: 191, code: "Slash", text: "/" },
    KeyDefinition { key: "`", key_code: 192, code: "Backquote", text: "`" },
    KeyDefinition { key: "[", key_code: 219, code: "BracketLeft", text: "[" },
    KeyDefinition { key: "\\", key_code: 220, code: "Backslash", text: "\\" },
    KeyDefinition { key: "]", key_code: 221, code: "BracketRight", text: "]" },
    KeyDefinition { key: "'", key_code: 222, code: "Quote", text: "'" },
    KeyDefinition { key: "AltGraph", key_code: 225, code: "AltGraph", text: "AltGraph" },
    KeyDefinition { key: "CrSel", key_code: 247, code: "Props", text: "CrSel" },
    KeyDefinition { key: "Cancel", key_code: 3, code: "Abort", text: "Cancel" },
    KeyDefinition { key: "Clear", key_code: 12, code: "Numpad5", text: "Clear" },
    KeyDefinition { key: "Shift", key_code: 16, code: "ShiftLeft", text: "Shift" },
    KeyDefinition { key: "Control", key_code: 17, code: "ControlLeft", text: "Control" },
    KeyDefinition { key: "Alt", key_code: 18, code: "AltLeft", text: "Alt" },
    KeyDefinition { key: "Accept", key_code: 30, code: "undefined", text: "Accept" },
    KeyDefinition { key: "ModeChange", key_code: 31, code: "undefined", text: "ModeChange" },
    KeyDefinition { key: " ", key_code: 32, code: "Space", text: " " },
    KeyDefinition { key: "Print", key_code: 42, code: "undefined", text: "Print" },
    KeyDefinition { key: "Execute", key_code: 43, code: "Open", text: "Execute" },
    KeyDefinition { key: " ", key_code: 46, code: "NumpadDecimal", text: " " },
    KeyDefinition { key: "a", key_code: 65, code: "KeyA", text: "a" },
    KeyDefinition { key: "b", key_code: 66, code: "KeyB", text: "b" },
    KeyDefinition { key: "c", key_code: 67, code: "KeyC", text: "c" },
    KeyDefinition { key: "d", key_code: 68, code: "KeyD", text: "d" },
    KeyDefinition { key: "e", key_code: 69, code: "KeyE", text: "e" },
    KeyDefinition { key: "f", key_code: 70, code: "KeyF", text: "f" },
    KeyDefinition { key: "g", key_code: 71, code: "KeyG", text: "g" },
    KeyDefinition { key: "h", key_code: 72, code: "KeyH", text: "h" },
    KeyDefinition { key: "i", key_code: 73, code: "KeyI", text: "i" },
    KeyDefinition { key: "j", key_code: 74, code: "KeyJ", text: "j" },
    KeyDefinition { key: "k", key_code: 75, code: "KeyK", text: "k" },
    KeyDefinition { key: "l", key_code: 76, code: "KeyL", text: "l" },
    KeyDefinition { key: "m", key_code: 77, code: "KeyM", text: "m" },
    KeyDefinition { key: "n", key_code: 78, code: "KeyN", text: "n" },
    KeyDefinition { key: "o", key_code: 79, code: "KeyO", text: "o" },
    KeyDefinition { key: "p", key_code: 80, code: "KeyP", text: "p" },
    KeyDefinition { key: "q", key_code: 81, code: "KeyQ", text: "q" },
    KeyDefinition { key: "r", key_code: 82, code: "KeyR", text: "r" },
    KeyDefinition { key: "s", key_code: 83, code: "KeyS", text: "s" },
    KeyDefinition { key: "t", key_code: 84, code: "KeyT", text: "t" },
    KeyDefinition { key: "u", key_code: 85, code: "KeyU", text: "u" },
    KeyDefinition { key: "v", key_code: 86, code: "KeyV", text: "v" },
    KeyDefinition { key: "w", key_code: 87, code: "KeyW", text: "w" },
    KeyDefinition { key: "x", key_code: 88, code: "KeyX", text: "x" },
    KeyDefinition { key: "y", key_code: 89, code: "KeyY", text: "y" },
    KeyDefinition { key: "z", key_code: 90, code: "KeyZ", text: "z" },
    KeyDefinition { key: "Meta", key_code: 91, code: "MetaLeft", text: "Meta" },
    KeyDefinition { key: "*", key_code: 106, code: "NumpadMultiply", text: "*" },
    KeyDefinition { key: "+", key_code: 107, code: "NumpadAdd", text: "+" },
    KeyDefinition { key: "-", key_code: 109, code: "NumpadSubtract", text: "-" },
    KeyDefinition { key: "/", key_code: 111, code: "NumpadDivide", text: "/" },
    KeyDefinition { key: ";", key_code: 186, code: "Semicolon", text: ";" },
    KeyDefinition { key: "=", key_code: 187, code: "Equal", text: "=" },
    KeyDefinition { key: ",", key_code: 188, code: "Comma", text: "," },
    KeyDefinition { key: ".", key_code: 190, code: "Period", text: "." },
    KeyDefinition { key: "`", key_code: 192, code: "Backquote", text: "`" },
    KeyDefinition { key: "[", key_code: 219, code: "BracketLeft", text: "[" },
    KeyDefinition { key: "]", key_code: 221, code: "BracketRight", text: "]" },
    KeyDefinition { key: "'", key_code: 222, code: "Quote", text: "'" },
    KeyDefinition { key: "Attn", key_code: 246, code: "undefined", text: "Attn" },
    KeyDefinition { key: "CrSel", key_code: 247, code: "Props", text: "CrSel" },
    KeyDefinition { key: "ExSel", key_code: 248, code: "undefined", text: "ExSel" },
    KeyDefinition { key: "EraseEof", key_code: 249, code: "undefined", text: "EraseEof" },
    KeyDefinition { key: "Play", key_code: 250, code: "undefined", text: "Play" },
    KeyDefinition { key: "ZoomOut", key_code: 251, code: "undefined", text: "ZoomOut" },
    KeyDefinition { key: ")", key_code: 48, code: "Digit0", text: ")" },
    KeyDefinition { key: "!", key_code: 49, code: "Digit1", text: "!" },
    KeyDefinition { key: "@", key_code: 50, code: "Digit2", text: "@" },
    KeyDefinition { key: "#", key_code: 51, code: "Digit3", text: "#" },
    KeyDefinition { key: "$", key_code: 52, code: "Digit4", text: "$" },
    KeyDefinition { key: "%", key_code: 53, code: "Digit5", text: "%" },
    KeyDefinition { key: "^", key_code: 54, code: "Digit6", text: "^" },
    KeyDefinition { key: "&", key_code: 55, code: "Digit7", text: "&" },
    KeyDefinition { key: "(", key_code: 57, code: "Digit9", text: "(" },
    KeyDefinition { key: "A", key_code: 65, code: "KeyA", text: "A" },
    KeyDefinition { key: "B", key_code: 66, code: "KeyB", text: "B" },
    KeyDefinition { key: "C", key_code: 67, code: "KeyC", text: "C" },
    KeyDefinition { key: "D", key_code: 68, code: "KeyD", text: "D" },
    KeyDefinition { key: "E", key_code: 69, code: "KeyE", text: "E" },
    KeyDefinition { key: "F", key_code: 70, code: "KeyF", text: "F" },
    KeyDefinition { key: "G", key_code: 71, code: "KeyG", text: "G" },
    KeyDefinition { key: "H", key_code: 72, code: "KeyH", text: "H" },
    KeyDefinition { key: "I", key_code: 73, code: "KeyI", text: "I" },
    KeyDefinition { key: "J", key_code: 74, code: "KeyJ", text: "J" },
    KeyDefinition { key: "K", key_code: 75, code: "KeyK", text: "K" },
    KeyDefinition { key: "L", key_code: 76, code: "KeyL", text: "L" },
    KeyDefinition { key: "M", key_code: 77, code: "KeyM", text: "M" },
    KeyDefinition { key: "N", key_code: 78, code: "KeyN", text: "N" },
    KeyDefinition { key: "O", key_code: 79, code: "KeyO", text: "O" },
    KeyDefinition { key: "P", key_code: 80, code: "KeyP", text: "P" },
    KeyDefinition { key: "Q", key_code: 81, code: "KeyQ", text: "Q" },
    KeyDefinition { key: "R", key_code: 82, code: "KeyR", text: "R" },
    KeyDefinition { key: "S", key_code: 83, code: "KeyS", text: "S" },
    KeyDefinition { key: "T", key_code: 84, code: "KeyT", text: "T" },
    KeyDefinition { key: "U", key_code: 85, code: "KeyU", text: "U" },
    KeyDefinition { key: "V", key_code: 86, code: "KeyV", text: "V" },
    KeyDefinition { key: "W", key_code: 87, code: "KeyW", text: "W" },
    KeyDefinition { key: "X", key_code: 88, code: "KeyX", text: "X" },
    KeyDefinition { key: "Y", key_code: 89, code: "KeyY", text: "Y" },
    KeyDefinition { key: "Z", key_code: 90, code: "KeyZ", text: "Z" },
    KeyDefinition { key: ":", key_code: 186, code: "Semicolon", text: ":" },
    KeyDefinition { key: "<", key_code: 188, code: "Comma", text: "<" },
    KeyDefinition { key: "_", key_code: 189, code: "Minus", text: "_" },
    KeyDefinition { key: ">", key_code: 190, code: "Period", text: ">" },
    KeyDefinition { key: "?", key_code: 191, code: "Slash", text: "?" },
    KeyDefinition { key: "~", key_code: 192, code: "Backquote", text: "~" },
    KeyDefinition { key: "{", key_code: 219, code: "BracketLeft", text: "{" },
    KeyDefinition { key: "|", key_code: 220, code: "Backslash", text: "|" },
    KeyDefinition { key: "}", key_code: 221, code: "BracketRight", text: "}" },
    KeyDefinition { key: "\"", key_code: 222, code: "Quote", text: "\"" }
];

#[derive(Debug, Fail)]
#[fail(display = "Key not found: {}", key)]
pub struct KeyNotFoundError {
    key: String
}

pub fn get_key_definition(key: &str) -> Result<&KeyDefinition, Error> {
    if let Some(definition) = USKEYBOARD_LAYOUT.iter().find(|key_definition| key_definition.key == key) {
        Ok(definition)
    } else {
        Err(KeyNotFoundError { key: key.to_string() }.into() )
    }
}
