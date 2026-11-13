//! Bilingual (`en` / `zh`) UI copy and language detection for the component site.
//!
//! Priority on load: `?lang=zh|en` query parameter > `localStorage["dxui-lang"]`
//! > browser `navigator.language` (`zh*` -> Chinese, otherwise English).

use dioxus::prelude::*;

pub const LANG_STORAGE_KEY: &str = "dxui-lang";

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SiteLang {
  #[default]
  En,
  Zh,
}

impl SiteLang {
  pub const fn code(self) -> &'static str {
    match self {
      Self::En => "en",
      Self::Zh => "zh",
    }
  }

  pub const fn html_lang(self) -> &'static str {
    match self {
      Self::En => "en",
      Self::Zh => "zh-CN",
    }
  }

  pub const fn toggle(self) -> Self {
    match self {
      Self::En => Self::Zh,
      Self::Zh => Self::En,
    }
  }

  pub const fn toggle_button_text(self) -> &'static str {
    match self {
      Self::En => "中文",
      Self::Zh => "EN",
    }
  }

  pub const fn toggle_aria_label(self) -> &'static str {
    match self {
      Self::En => "Switch language to Chinese",
      Self::Zh => "切换语言为英文",
    }
  }

  pub const fn main_site_href(self) -> &'static str {
    match self {
      Self::En => "https://dioxuspress.dev/?lang=en",
      Self::Zh => "https://dioxuspress.dev/?lang=zh",
    }
  }

  pub fn parse_tag(raw: &str) -> Option<Self> {
    let trimmed = raw.trim();
    if trimmed.eq_ignore_ascii_case("zh")
      || trimmed.get(..3).is_some_and(|prefix| {
        prefix.eq_ignore_ascii_case("zh-") || prefix.eq_ignore_ascii_case("zh_")
      })
    {
      Some(Self::Zh)
    } else if trimmed.eq_ignore_ascii_case("en")
      || trimmed.get(..3).is_some_and(|prefix| {
        prefix.eq_ignore_ascii_case("en-") || prefix.eq_ignore_ascii_case("en_")
      })
    {
      Some(Self::En)
    } else {
      None
    }
  }
}

/// Pure priority resolution (`?lang=` > `localStorage` > `navigator.language` > `En`).
pub fn resolve_lang(
  query: Option<&str>,
  stored: Option<&str>,
  navigator: Option<&str>,
) -> SiteLang {
  query
    .and_then(SiteLang::parse_tag)
    .or_else(|| stored.and_then(SiteLang::parse_tag))
    .or_else(|| navigator.and_then(SiteLang::parse_tag))
    .unwrap_or(SiteLang::En)
}

pub fn use_site_lang() -> Signal<SiteLang> {
  use_context::<Signal<SiteLang>>()
}

#[cfg(target_arch = "wasm32")]
mod web {
  use wasm_bindgen::prelude::*;

  #[wasm_bindgen(inline_js = r#"
export function readBrowserLangInputs(storageKey) {
  let query = "";
  let stored = "";
  let nav = "";
  try {
    query = new URLSearchParams(window.location.search).get("lang") || "";
  } catch {}
  try {
    stored = window.localStorage.getItem(storageKey) || "";
  } catch {}
  try {
    nav = (window.navigator && (window.navigator.language || (window.navigator.languages && window.navigator.languages[0]))) || "";
  } catch {}
  return JSON.stringify([query, stored, nav]);
}

export function syncBrowserLang(storageKey, code, htmlLang, persistStorage) {
  if (persistStorage) {
    try {
      window.localStorage.setItem(storageKey, code);
    } catch {}
  }
  try {
    document.documentElement.lang = htmlLang;
  } catch {}
  try {
    const url = new URL(window.location.href);
    if (url.searchParams.has("lang") && url.searchParams.get("lang") !== code) {
      url.searchParams.set("lang", code);
      window.history.replaceState(window.history.state, "", url.pathname + url.search + url.hash);
    }
  } catch {}
}
"#)]
  extern "C" {
    #[wasm_bindgen(js_name = readBrowserLangInputs)]
    pub(super) fn read_browser_lang_inputs(storage_key: &str) -> String;

    #[wasm_bindgen(js_name = syncBrowserLang)]
    pub(super) fn sync_browser_lang(
      storage_key: &str,
      code: &str,
      html_lang: &str,
      persist_storage: bool,
    );
  }
}

/// Reads `?lang=`, `localStorage`, and `navigator.language` synchronously on WASM
/// so the first rendered frame uses the resolved language without flashing.
pub fn detect_initial_lang() -> SiteLang {
  #[cfg(target_arch = "wasm32")]
  {
    let raw = web::read_browser_lang_inputs(LANG_STORAGE_KEY);
    let parts = parse_json_triple(&raw);
    let query = parts.as_ref().map(|(q, _, _)| q.as_str()).filter(|s| !s.is_empty());
    let stored = parts.as_ref().map(|(_, s, _)| s.as_str()).filter(|s| !s.is_empty());
    let nav = parts.as_ref().map(|(_, _, n)| n.as_str()).filter(|s| !s.is_empty());
    let lang = resolve_lang(query, stored, nav);
    let persist = query.and_then(SiteLang::parse_tag).is_some();
    web::sync_browser_lang(LANG_STORAGE_KEY, lang.code(), lang.html_lang(), persist);
    lang
  }
  #[cfg(not(target_arch = "wasm32"))]
  {
    resolve_lang(None, None, None)
  }
}

/// Persists an explicit language switch to `localStorage`, `<html lang>`, and `?lang=` if present.
pub fn persist_lang(lang: SiteLang) {
  #[cfg(target_arch = "wasm32")]
  {
    web::sync_browser_lang(LANG_STORAGE_KEY, lang.code(), lang.html_lang(), true);
  }
  #[cfg(not(target_arch = "wasm32"))]
  {
    let _ = (LANG_STORAGE_KEY, lang.code(), lang.html_lang());
  }
}

#[cfg(any(target_arch = "wasm32", test))]
fn parse_json_triple(raw: &str) -> Option<(String, String, String)> {
  let inner = raw.trim().strip_prefix('[')?.strip_suffix(']')?;
  let mut items = Vec::with_capacity(3);
  let mut chars = inner.chars().peekable();
  while let Some(&ch) = chars.peek() {
    if ch.is_whitespace() || ch == ',' {
      chars.next();
      continue;
    }
    if ch != '"' {
      return None;
    }
    chars.next();
    let mut current = String::new();
    let mut closed = false;
    while let Some(c) = chars.next() {
      match c {
        '"' => {
          closed = true;
          break;
        }
        '\\' => {
          if let Some(escaped) = chars.next() {
            current.push(escaped);
          }
        }
        other => current.push(other),
      }
    }
    if !closed {
      return None;
    }
    items.push(current);
  }
  if items.len() == 3 {
    let mut iter = items.into_iter();
    Some((iter.next()?, iter.next()?, iter.next()?))
  } else {
    None
  }
}

pub fn category_label(lang: SiteLang, id: &str, fallback: &'static str) -> &'static str {
  match lang {
    SiteLang::En => fallback,
    SiteLang::Zh => zh_category_label(id).unwrap_or(fallback),
  }
}

fn zh_category_label(id: &str) -> Option<&'static str> {
  Some(match id {
    "actions" => "操作与命令",
    "forms" => "表单与输入",
    "overlays" => "浮层与弹窗",
    "navigation" => "导航与菜单",
    "layout" => "布局与容器",
    "data-display" => "数据展示",
    "feedback" => "状态与反馈",
    "messaging" => "消息与会话",
    _ => return None,
  })
}

pub fn component_description(lang: SiteLang, slug: &str, fallback: &'static str) -> &'static str {
  match lang {
    SiteLang::En => fallback,
    SiteLang::Zh => zh_component_description(slug).unwrap_or(fallback),
  }
}

fn zh_component_description(slug: &str) -> Option<&'static str> {
  Some(match slug {
    // actions
    "button" => "支持多种视觉变体、尺寸规格与触控密度自适应的按钮组件。",
    "button-group" => "用于将一组关联命令按钮组合排列的按钮组容器。",
    "command" => "将搜索输入框与可高亮选项列表自动关联的命令面板组件。",
    "fab" => "带可选快捷展开菜单（Speed Dial）的悬浮操作按钮。",
    "kbd" => "用于展示键盘快捷键组合提示的语义化按键组件。",
    "swap" => "在两个元素之间平滑切换显示状态的开关按钮组件。",
    "theme-controller" => "在文档根节点应用、跟随系统并持久化记忆配色主题的控制器。",
    "toggle" => "支持受控按下（Pressed）状态的双态切换按钮组件。",
    "toggle-group" => "由根节点统一管理单选或多选按下状态的切换按钮组。",
    // forms
    "calendar" => "基于纯日期网格无障碍原语构建的日历选择组件。",
    "checkbox" => "支持选中、未选中与禁用状态的复选框组件。",
    "date-picker" => "组合触发器、选中值展示与日历浮层面板的日期选择器部件。",
    "field" => "用于统一组织标签、控件、辅助说明与校验错误的表单字段布局部件。",
    "file-input" => "样式化的原生文件上传输入框，完整透传原生变更事件。",
    "input" => "支持禁用态与校验错误态的单行文本输入框组件。",
    "input-group" => "支持在输入框前后组合图标、文本前缀或操作按钮的输入组组件。",
    "input-otp" => "由根节点集中管理验证码字符状态的一次性口令（OTP）输入框。",
    "label" => "用于将说明文字与表单控件进行无障碍关联的标签组件。",
    "native-select" => "样式化的原生下拉选择框（select / optgroup / option）组件。",
    "number-input" => "带增减步进按钮与键盘微调（spinbutton）语义的数值输入框。",
    "radio-group" => "由根节点受控管理当前选中项的单选按钮组组件。",
    "rating" => "基于原生单选组语义构建的可无障碍操作星级评分组件。",
    "select" => "基于无障碍选择原语构建的可定制下拉选择器组件。",
    "slider" => "支持受控数值调节与键盘步进的水平滑动条组件。",
    "switch" => "支持开启、关闭与禁用状态的拨动开关组件。",
    "tags-input" => "支持回车添加与逐项移除胶囊标签的多标签输入框组件。",
    "textarea" => "支持禁用态与校验错误态的多行文本域组件。",
    // overlays
    "alert-dialog" => "基于对话框原语构建的模态确认与高危操作二次拦截弹窗。",
    "combobox" => "结合气泡浮层与关键词过滤能力的可搜索下拉组合框部件。",
    "context-menu" => "支持鼠标右键或键盘快捷键唤起的上下文菜单部件。",
    "dialog" => "具备焦点捕获、遮罩层与键盘关闭能力的模态对话框组件。",
    "drawer" => "面向移动端交互体验优化的底部滑出抽屉浮层组件。",
    "dropdown" => "支持键盘方向键漫游与菜单项分组的下拉菜单组件。",
    "hover-card" => "悬停或聚焦触发器时展示富媒体预览信息的悬浮卡片组件。",
    "menubar" => "适合桌面端与编辑器顶部的多级水平菜单栏组件。",
    "popover" => "锚定在触发元素旁展示交互表单或富内容的弹出气泡组件。",
    "sheet" => "从屏幕四侧边缘滑出的侧边面板（Sheet）浮层组件。",
    "tooltip" => "悬停或键盘聚焦时展示简短补充说明的文字提示组件。",
    // navigation
    "breadcrumb" => "表达页面层级位置并支持折叠省略的语义化面包屑导航部件。",
    "code-tabs" => "带下划线激活指示、键盘左右漫游与代码面板的多语言对比标签页。",
    "dock" => "适用于移动端或应用壳层的底部快捷导航栏组件。",
    "menu" => "支持分组标题、激活态高亮与可折叠子项的纵向导航菜单。",
    "navigation-menu" => "具备完整导航语义的受控顶部多栏导航菜单部件。",
    "pagination" => "包含页码链接、上下页按钮与省略号占位的分页导航组件。",
    "sidebar" => "由 Provider 统一管理桌面折叠与移动抽屉状态的应用侧边栏框架。",
    "steps" => "展示已完成、进行中与待处理进度的编号步骤条组件。",
    "tabs" => "支持受控或默认激活态切换的选项卡面板组件。",
    "tree" => "支持节点展开折叠与 WAI-ARIA 树形键盘导航的层级树组件。",
    // layout
    "accordion" => "由根节点管理单个或多个面板展开状态的垂直手风琴折叠组件。",
    "aspect-ratio" => "按固定宽高比约束图片、视频或嵌入内容的比例容器。",
    "card" => "包含头部、主体内容与底部操作区的通用卡片容器组件。",
    "carousel" => "由根节点管理当前激活幻灯片索引的轮播展示组件。",
    "collapsible" => "控制单块内容区域展开或收起的可折叠面板组件。",
    "direction" => "为子树局部指定原生从左到右（LTR）或从右到左（RTL）书写方向的容器。",
    "indicator" => "在目标元素边角叠加角标计数或在线状态点的定位容器。",
    "item" => "用于构建列表行、设置项与联系人条目的通用列表项组合部件。",
    "mockup" => "提供浏览器、窗口、代码终端与手机外框的内容展示样机容器。",
    "resizable" => "支持鼠标与键盘拖拽调整分栏比例的可调尺寸面板组部件。",
    "scroll-area" => "统一封装滚动视口与定制滚动条样式的原生滚动区域组件。",
    "separator" => "用于水平或垂直分隔内容区块的视觉与语义分割线组件。",
    // data-display
    "avatar" => "支持头像图片加载与首字母回退占位的用户头像组件。",
    "badge" => "提供多种静态语义状态变体的轻量徽标与标签胶囊组件。",
    "chart" => "对源码拷贝友好、零重型运行时依赖的 SVG 图表组合部件。",
    "countdown" => "展示应用层剩余秒数并支持无障碍播报的倒计时组件。",
    "data-table" => "提供受控筛选、列排序与行多选状态辅助函数的数据表格部件。",
    "diff" => "支持键盘左右调节分割线的前后视图对比（Diff）组件。",
    "empty" => "用于展示无数据、空搜索结果及引导操作的空状态布局部件。",
    "progress" => "具备完整无障碍进度数值语义的线性进度条组件。",
    "radial-progress" => "中心可展示百分比或自定义标签的环形进度指示组件。",
    "stat" => "组合指标标题、核心数值、趋势说明与图标的统计数值卡片组件。",
    "table" => "提供统一边框、表头与行悬停样式的语义化数据表格组件。",
    "timeline" => "包含时间戳、节点标记与事件内容的有序时间线展示组件。",
    "typography" => "面向长文与技术文档排版的语义化标题、段落与引用样式部件。",
    // feedback
    "alert" => "包含标题与详细说明的页面内静态提示与告警横幅组件。",
    "skeleton" => "在异步内容加载完成前保持版面稳定的脉冲骨架屏占位组件。",
    "sonner" => "开箱即用的堆叠式轻提示（Sonner Toast）部件与通知队列辅助函数。",
    "spinner" => "用于按钮内联或区块等待反馈的轻量加载旋转指示器。",
    "status" => "带可选无障碍状态标签的语义色呼吸/常亮状态圆点组件。",
    "toast" => "支持受控生命周期、操作按钮与队列管理的通知提示部件。",
    // messaging
    "attachment" => "与具体服务商解耦的文件附件预览行、上传进度与操作组件。",
    "bubble" => "面向即时通讯与 AI 对话场景的消息气泡与表情回应容器。",
    "marker" => "用于会话流内嵌状态提示、高亮边框行与日期分割线的标记组件。",
    "message" => "与数据源无关的聊天消息行布局（头像、署名、正文与操作栏）组件。",
    "message-scroller" => "内置纯滚动意图判定辅助函数的受控消息流自动跟随滚动容器。",
    _ => return None,
  })
}

pub fn block_title(lang: SiteLang, slug: &str, fallback: &'static str) -> &'static str {
  match lang {
    SiteLang::En => fallback,
    SiteLang::Zh => zh_block_meta(slug).map_or(fallback, |(title, _)| title),
  }
}

pub fn block_description(lang: SiteLang, slug: &str, fallback: &'static str) -> &'static str {
  match lang {
    SiteLang::En => fallback,
    SiteLang::Zh => zh_block_meta(slug).map_or(fallback, |(_, desc)| desc),
  }
}

fn zh_block_meta(slug: &str) -> Option<(&'static str, &'static str)> {
  Some(match slug {
    "chat" => (
      "Chat 即时会话",
      "包含会话列表、消息流、附件预览以及支持拖拽上传文件的输入编辑器的完整聊天界面。",
    ),
    "checkout" => (
      "Checkout 结账收银台",
      "左侧收货地址、配送方式与支付表单校验，右侧实时汇总订单明细的结账页面。",
    ),
    "dashboard" => (
      "Dashboard 数据看板",
      "包含响应式侧边栏、核心业务指标卡、趋势图表与可排序搜索订单表的管理后台壳层。",
    ),
    "files" => (
      "Files 文件管理器",
      "集成目录树、面包屑路径、拖拽上传区域及支持右键/键盘菜单文件表的网盘管理界面。",
    ),
    "inbox" => {
      ("Inbox 邮件收件箱", "包含邮件文件夹切换、可搜索会话列表与右侧阅读面板的三栏邮件客户端界面。")
    }
    "landing" => (
      "Landing 产品落地页",
      "涵盖首屏主视觉、核心特性矩阵、用户证言、早鸟订阅表单与页脚的产品宣传页。",
    ),
    "login" => ("Login 登录页", "内置邮箱格式与必填密码无障碍表单校验的账号登录界面。"),
    "pricing" => ("Pricing 定价方案页", "支持按月/按年计费周期一键切换的三档产品订阅定价页面。"),
    "schedule" => (
      "Schedule 日程排期表",
      "左侧迷你月历选周、右侧多日时间列导航以及新建日程弹窗校验的周视图日程页。",
    ),
    "settings" => (
      "Settings 账户设置页",
      "包含个人资料与通知偏好选项卡、未保存状态追踪及保存/重置操作的设置页面。",
    ),
    "signup" => ("Signup 注册页", "内置实时密码强度进度条与完整字段校验的新用户创建页面。"),
    _ => return None,
  })
}

pub fn example_title(lang: SiteLang, english_title: &'static str) -> &'static str {
  match lang {
    SiteLang::En => english_title,
    SiteLang::Zh => zh_example_title(english_title).unwrap_or(english_title),
  }
}

fn zh_example_title(english_title: &str) -> Option<&'static str> {
  Some(match english_title {
    "Variants" => "变体样式",
    "Sizes and states" => "尺寸与状态",
    "Orientation" => "排列方向",
    "Command palette" => "命令面板",
    "Shortcuts" => "快捷键提示",
    "Docs navigation" => "文档导航",
    "Browser, code, and phone" => "浏览器、代码与手机样机",
    "System, light, and dark" => "跟随系统、浅色与深色",
    "Single selection" => "单选切换",
    "Month" => "月历视图",
    "States" => "交互状态",
    "Date picker" => "日期选择浮层",
    "Typed date" => "键盘输入日期",
    "Description and error" => "辅助说明与错误提示",
    "Addons" => "前后缀组合",
    "Six digits" => "六位验证码",
    "Labels" => "关联标签",
    "Groups" => "分组选项",
    "Plan picker" => "套餐选择",
    "Select" => "下拉选择",
    "Multiple" => "多选模式",
    "Orientation and states" => "方向与状态",
    "Price range" => "价格区间",
    "Character count" => "字数统计",
    "Confirm deletion" => "确认删除弹窗",
    "Search" => "关键词搜索选择",
    "Right-click menu" => "右键上下文菜单",
    "Form in a dialog" => "对话框内表单",
    "Bottom drawer" => "底部滑出抽屉",
    "Actions menu" => "操作菜单",
    "View options" => "视图选项",
    "Profile card" => "个人资料悬浮卡",
    "Editor menus" => "编辑器菜单栏",
    "Form in a popover" => "弹出气泡表单",
    "Sides" => "四侧滑出方向",
    "Tooltip" => "悬停文字提示",
    "Collapsed path" => "折叠省略路径",
    "Product menu" => "产品导航菜单",
    "Mega menu" => "多栏大菜单",
    "Pages" => "分页器",
    "Collapsible" => "可折叠侧边栏",
    "Three-way syntax comparison" => "三语言语法对比",
    "Account settings" => "账户设置选项卡",
    "Project files" => "项目文件树",
    "FAQ" => "常见问题手风琴",
    "Ratios" => "固定宽高比",
    "Form card" => "表单卡片",
    "Slides" => "轮播切换",
    "Show more" => "展开更多",
    "Right to left" => "从右到左（RTL）",
    "People" => "成员列表项",
    "Two panels" => "双栏拖拽分屏",
    "Tags" => "滚动标签列表",
    "Image and fallback" => "图片与首字母回退",
    "Status" => "语义状态",
    "Revenue" => "营收统计指标",
    "Release history" => "版本发布时间线",
    "Checkout" => "结账步骤进度",
    "Counts and presence" => "角标计数与在线状态",
    "Service health" => "服务健康状态",
    "Sizes and labels" => "尺寸与中心标签",
    "Sale timer" => "活动倒计时",
    "Design comparison" => "设计稿前后对比",
    "Review" => "星级评价",
    "Quantity and weight" => "数量与重量步进",
    "Topics" => "话题标签输入",
    "Documents" => "文档文件上传",
    "Icons and text" => "图标与文字切换",
    "Phone tabs" => "手机底部导航栏",
    "Speed dial" => "悬浮快捷操作组",
    "Area and line" => "面积图与折线图",
    "Donut" => "环形占比图",
    "Filter, sort, and select" => "筛选、排序与行多选",
    "No projects" => "空项目占位",
    "Upload" => "上传进度条",
    "Invoices" => "账单明细表",
    "Article" => "文章排版示例",
    "Loading profile" => "资料卡加载骨架",
    "Sizes" => "尺寸规格",
    "Undo action" => "可撤销操作通知",
    "Upload states" => "附件上传状态",
    "Thread" => "消息对话串",
    "Follow new messages" => "自动跟随新消息",
    _ => return None,
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::blocks::BLOCKS;
  use crate::catalog::CATEGORIES;
  use crate::examples::EXAMPLES;

  #[test]
  fn priority_order_prefers_query_then_storage_then_navigator() {
    assert_eq!(resolve_lang(Some("zh"), Some("en"), Some("en-US")), SiteLang::Zh);
    assert_eq!(resolve_lang(Some("en"), Some("zh"), Some("zh-CN")), SiteLang::En);
    assert_eq!(resolve_lang(None, Some("zh"), Some("en-US")), SiteLang::Zh);
    assert_eq!(resolve_lang(Some("fr"), Some("zh"), Some("en-US")), SiteLang::Zh);
    assert_eq!(resolve_lang(None, None, Some("zh-CN")), SiteLang::Zh);
    assert_eq!(resolve_lang(None, None, Some("zh-Hant-TW")), SiteLang::Zh);
    assert_eq!(resolve_lang(None, None, Some("en-US")), SiteLang::En);
    assert_eq!(resolve_lang(None, None, Some("ja-JP")), SiteLang::En);
    assert_eq!(resolve_lang(None, None, None), SiteLang::En);
  }

  #[test]
  fn json_triple_parser_handles_browser_inputs() {
    assert_eq!(
      parse_json_triple(r#"["zh","en","zh-CN"]"#),
      Some(("zh".to_string(), "en".to_string(), "zh-CN".to_string()))
    );
    assert_eq!(
      parse_json_triple(r#"["","","en-US"]"#),
      Some((String::new(), String::new(), "en-US".to_string()))
    );
  }

  #[test]
  fn every_category_component_block_and_example_has_chinese_copy() {
    for category in CATEGORIES {
      assert!(
        zh_category_label(category.id).is_some(),
        "missing Chinese category label for `{}`",
        category.id
      );
      for component in category.components {
        assert!(
          zh_component_description(component.slug).is_some(),
          "missing Chinese component description for `{}`",
          component.slug
        );
      }
    }
    for block in BLOCKS {
      assert!(
        zh_block_meta(block.slug).is_some(),
        "missing Chinese block metadata for `{}`",
        block.slug
      );
    }
    for example in EXAMPLES {
      assert!(
        zh_example_title(example.title).is_some(),
        "missing Chinese example title for `{}` ({})",
        example.title,
        example.slug
      );
    }
  }

  #[test]
  fn every_component_and_block_has_bilingual_reference_html() {
    use crate::pages::{block_reference_html, reference_html};

    for category in CATEGORIES {
      for component in category.components {
        let en = reference_html(SiteLang::En, component.slug)
          .unwrap_or_else(|| panic!("missing English reference HTML for `{}`", component.slug));
        assert!(
          en.contains("<h2>API Surface</h2>") && en.contains("<h2>Accessibility Notes</h2>"),
          "English reference HTML for `{}` missing required headings",
          component.slug
        );

        let zh = reference_html(SiteLang::Zh, component.slug)
          .unwrap_or_else(|| panic!("missing Chinese reference HTML for `{}`", component.slug));
        assert!(
          zh.contains("<h2>API 概览</h2>") && zh.contains("<h2>无障碍说明</h2>"),
          "Chinese reference HTML for `{}` must include `API 概览` and `无障碍说明` headings",
          component.slug
        );
        for english_heading in [
          "<h2>API Surface</h2>",
          "<h2>Behavior</h2>",
          "<h2>Keyboard Behavior</h2>",
          "<h2>Events</h2>",
          "<h2>Density</h2>",
          "<h2>Ownership Boundaries</h2>",
          "<h2>Accessibility Notes</h2>",
        ] {
          assert!(
            !zh.contains(english_heading),
            "Chinese reference HTML for `{}` still contains untranslated `{english_heading}`",
            component.slug
          );
        }
      }
    }

    for block in BLOCKS {
      let en = block_reference_html(SiteLang::En, block.slug)
        .unwrap_or_else(|| panic!("missing English block reference HTML for `{}`", block.slug));
      assert!(
        en.contains("<h2>Behavior</h2>") && en.contains("<h2>Accessibility Notes</h2>"),
        "English block reference HTML for `{}` missing required headings",
        block.slug
      );

      let zh = block_reference_html(SiteLang::Zh, block.slug)
        .unwrap_or_else(|| panic!("missing Chinese block reference HTML for `{}`", block.slug));
      assert!(
        zh.contains("<h2>行为与交互</h2>") && zh.contains("<h2>无障碍说明</h2>"),
        "Chinese block reference HTML for `{}` must include `行为与交互` and `无障碍说明` headings",
        block.slug
      );
    }
  }
}
