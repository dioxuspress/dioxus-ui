## API 概览

- `InputOtp`
- `InputOtpGroup`
- `InputOtpSlot`
- `InputOtpSeparator`
- `InputOtpHiddenInput`
- `InputOtpInputMode`
- `OtpSlotState`
- `otp_slots`
- `otp_slots_with_disabled`
- `otp_is_complete`
- `otp_insert_char`
- `otp_insert_char_filtered`
- `otp_delete_char`
- `otp_apply_paste`
- `otp_apply_paste_filtered`
- `otp_next_index`
- `otp_previous_index`
- `otp_clamp_value`
- `input_otp_sanitize`
- `input_otp_class`
- `input_otp_group_class`
- `input_otp_slot_class`
- `input_otp_separator_class`
- `input_otp_hidden_input_class`

## 值变更处理

`InputOtp` 内部维护验证码字符串状态（参见 [RFC 0077](../../../../docs/rfcs/0077-component-owned-state.md)）。传入验证码长度 `length` 后，组件会从 `default_value` 初始化，或在应用受控管理时跟随 `value`，并在每次变更时携带清洗后的验证码调用 `on_value_change`。每个 `InputOtpSlot` 展示对应 `index` 位置上的字符，而 `InputOtpHiddenInput` 负责接收键盘输入：

```rust
rsx! {
  Label { id: "code-label", "Verification code" }
  InputOtp { length: 6, on_value_change: move |code: String| verify(&code),
    InputOtpGroup {
      for index in 0..6 {
        InputOtpSlot { key: "{index}", index }
      }
    }
    InputOtpHiddenInput { "aria-labelledby": "code-label" }
  }
}
```

- `input_otp_sanitize` 仅保留当前输入模式允许的字符，并截断至 `length` 长度。`Numeric` 模式仅保留 ASCII 数字，`Text` 模式保留字母和数字，因此粘贴 `123-456` 会自动清洗为 `123456`。
- 键入字符会在末尾追加，`Backspace` 会删除最后一个字符，粘贴或系统自动填充则会直接填入完整验证码。不支持直接编辑中间某个槽位的字符。
- 下一个待输入字符所在的槽位处于激活态（active）；当验证码填满时，最后一个槽位处于激活态。根组件上的 `disabled` 和 `invalid` 会作用于所有槽位及隐藏输入框；单个槽位上的 `disabled` 则仅禁用该槽位。
- 所有子组件必须位于 `InputOtp` 内部；否则它们不会渲染任何内容，并会在日志中提示缺失的根组件。
- 页面脚本会在 Dioxus 读取原生输入框值之前先行过滤非法字符，因此被拒绝的字符绝不会残留在输入框内导致需要多按一次 `Backspace` 才能删除。
- 禁用状态下的输入框不会触发任何事件。

## 无障碍说明

建议将 `InputOtp` 与可见的 `Label` 以及应用维护的描述或错误提示文本搭配使用。可通过 `aria-labelledby` 或 `id` 配合 `Label` 为输入框命名；根组件与隐藏输入框均支持透传全局属性。`InputOtpHiddenInput` 渲染带有 `inputmode` 和可选 `autocomplete="one-time-code"` 的原生文本输入框，使表单、屏幕阅读器和移动端验证码键盘都能访问真实的输入控件。

隐藏输入框以零不透明度覆盖在所有槽位上方，因此点击任意槽位都会使其获得焦点。请将其渲染在根节点为 `relative` 定位的 `InputOtp` 内部。视觉槽位仅作为验证码的展示镜像。校验、提交以及重新发送倒计时逻辑请保留在应用层处理。

## 纯函数辅助工具

`otp_insert_char_filtered`、`otp_delete_char`、`otp_apply_paste_filtered`、`otp_next_index` 和 `otp_previous_index` 仍保留导出，供自行构建独立单槽输入框的应用使用。

## 移动端说明

处理纯数字验证码时，请将 `InputOtpHiddenInput` 与 `InputOtpInputMode::Numeric` 搭配使用。`autocomplete` 默认值为 `one-time-code`；若应用不希望启用平台原生的短信验证码自动填充，请传入 `None`。

## 质量门禁

M32 里程碑已通过 crate 单元测试、注册表文档检查、feature 编译检查、生成的源码拷贝夹具冒烟测试以及 Web/Desktop 示例冒烟输出验证了 `InputOtp`。
