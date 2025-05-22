---@meta

--===== mouse =====--
---マウス関連の関数を提供するモジュール
---@class mouse
mouse = {}

---マウスの位置を取得する
---@return number x x座標
---@return number y y座標
function mouse.get_pos() end

---マウスを指定した位置に移動する
---@param x number x座標
---@param y number y座標
---@param coord Coord 座標系
function mouse.move(x, y, coord) end

---マウスのボタンを押す
---@param button Button.Send | number ボタン
function mouse.press(button) end

---マウスのボタンを離す
---@param button Button.Send | number ボタン
function mouse.release(button) end

---マウスのボタンを押して、離す
---@param button Button.Send | number ボタン
function mouse.click(button) end

---マウスのボタンが押されているかどうかを取得する
---@param button Button.Query | number ボタンの番号
---@return boolean pressed 押されているかどうか
function mouse.is_pressing(button) end

mouse = mouse
