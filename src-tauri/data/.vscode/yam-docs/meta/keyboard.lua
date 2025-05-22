---@meta

--===== keyboard =====--
---キーボード関連の関数を提供するモジュール
---@class keyboard
keyboard = {}

---指定されたキーを押す
---@param key Key.Send キー
function keyboard.press(key) end

---指定されたキーを離す
---@param key Key.Send キー
function keyboard.release(key) end

---指定されたキーを押して、離す
---@param key Key.Send キー
function keyboard.click(key) end

---指定されたキーが押されているかどうかを取得する
---@param key Key.Query キー
---@return boolean pressed 押されているかどうか
function keyboard.is_pressing(key) end

--===== keyboard.char =====--

---Enumにないキー用の関数
keyboard.char = {}

---指定された文字を押す
---@param char string 文字
function keyboard.char.press(char) end

---指定された文字を離す
---@param char string 文字
function keyboard.char.release(char) end

---指定された文字を押して、離す
---@param char string 文字
function keyboard.char.click(char) end
