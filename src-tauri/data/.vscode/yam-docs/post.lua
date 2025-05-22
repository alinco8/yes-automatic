local keys = {}

---キーが押された瞬間かどうかを取得する
---@param key Key.Query キー
---@return boolean pressed 押された瞬間かどうか
function keyboard.just_pressed(key)
    local pressed = keyboard.is_pressing(key)
    local just = pressed and not keys[key]
    keys[key] = pressed
    return just
end

local buttons = {}

---ボタンが押された瞬間かどうかを取得する
---@param button Button.Query ボタン
---@return boolean pressed 押された瞬間かどうか
function mouse.just_pressed(button)
    local pressed = mouse.is_pressing(button)
    local just = pressed and not buttons[button]
    buttons[button] = pressed
    return just
end
