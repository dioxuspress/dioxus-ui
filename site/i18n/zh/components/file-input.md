## API 概览

- `FileInput`
- `FileDropzone`
- `file_input_class`
- `file_dropzone_class`

```rust
rsx! {
  Label { r#for: "avatar", "Avatar" }
  FileInput {
    id: "avatar",
    accept: "image/*",
    onchange: move |event: FormEvent| {
      for file in event.files() {
        println!("chose {}", file.name());
      }
    },
  }
}
```

文件输入框不绑定 `value`（原生文件输入控件不支持传入值）。`onchange` 会传递表单事件，可通过 `event.files()` 读取用户选择的文件。`accept`、`multiple` 及其他属性会透传至底层 `input` 元素。`invalid` 会设置警示色边框与 `aria-invalid`。与 `Attachment` 一样，文件上传传输逻辑由应用自行负责。

## 拖拽上传区域

`FileDropzone` 接收拖放到区域内的文件，并以 `Vec<FileData>`（即 `dioxus::html::FileData`，提供 `name()`、`size()`、`content_type()` 及字节内容读取）调用 `on_files`：

```rust
rsx! {
  FileDropzone {
    "aria-label": "Upload area",
    on_files: move |files: Vec<FileData>| {
      for file in files {
        println!("dropped {}", file.name());
      }
    },
    "Drop files here"
    FileInput { class: "mt-3", multiple: true, onchange: move |event: FormEvent| upload(event.files()) }
  }
}
```

- 当文件悬停在区域上方时，组件会设置 `data-dragging="true"`，基础样式类会据此呈现主色边框与背景；组件内部通过进入与离开计数跟踪悬停状态，因此指针在子元素之间移动时不会误清除高亮。
- 未包含文件的拖放操作，或拖放到处于 `disabled` 状态的区域上时，不会触发任何回调。
- `class` 会覆盖合并在默认的虚线边框、内边距与居中文本样式之上。

## 无障碍说明

原生文件输入框自带选择按钮、已选文件名展示以及完整的键盘交互。请使用 `Label` 为其命名（参见 [RFC 0060](../../../../docs/rfcs/0060-input-components.md)）。

由于拖拽操作依赖指针设备，请在 `FileDropzone` 内部或旁边放置 `FileInput` 或其他文件选择控件。请使用 `aria-label` 为拖拽区域命名，并在其独立存在时设置 `role: "region"`。
