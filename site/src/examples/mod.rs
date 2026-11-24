//! Live examples (RFC 0052). Each example is one file whose component the
//! component page renders and whose text, read with `include_str!`, the page
//! shows, so the shown source is the code that runs. The component is named
//! after the file, such as `CalendarMonthDemo` in `calendar_month.rs`, so
//! examples copied into one app keep distinct names.

use dioxus::prelude::*;

pub struct Example {
  pub slug: &'static str,
  pub title: &'static str,
  pub source: &'static str,
  pub render: fn() -> Element,
}

macro_rules! examples {
  ($($module:ident => $demo:ident, $slug:literal, $title:literal;)*) => {
    $(mod $module;)*

    pub const EXAMPLES: &[Example] = &[
      $(Example {
        slug: $slug,
        title: $title,
        source: include_str!(concat!(stringify!($module), ".rs")),
        render: $module::$demo,
      },)*
    ];

    #[cfg(test)]
    #[test]
    fn demos_are_named_after_their_files() {
      $(assert_eq!(demo_name(stringify!($module)), stringify!($demo));)*
    }
  };
}

examples! {
  button_variants => ButtonVariantsDemo, "button", "Variants";
  button_sizes => ButtonSizesDemo, "button", "Sizes and states";
  button_icons => ButtonIconsDemo, "button", "Icons, loading, and full width";
  button_group_basic => ButtonGroupBasicDemo, "button-group", "Orientation";
  command_palette => CommandPaletteDemo, "command", "Command palette";
  command_dialog => CommandDialogDemo, "command", "Dialog command palette";
  kbd_shortcuts => KbdShortcutsDemo, "kbd", "Shortcuts";
  menu_docs => MenuDocsDemo, "menu", "Docs navigation";
  mockup_frames => MockupFramesDemo, "mockup", "Browser, code, and phone";
  mockup_window => MockupWindowDemo, "mockup", "Desktop window frame";
  theme_controller_picker => ThemeControllerPickerDemo, "theme-controller", "System, light, and dark";
  toggle_basic => ToggleBasicDemo, "toggle", "Variants";
  toggle_sizes => ToggleSizesDemo, "toggle", "Sizes and toolbar states";
  toggle_group_single => ToggleGroupSingleDemo, "toggle-group", "Single selection";
  toggle_group_formatting => ToggleGroupFormattingDemo, "toggle-group", "Multiple formatting";
  calendar_month => CalendarMonthDemo, "calendar", "Month";
  calendar_wheel => CalendarWheelDemo, "calendar", "Year/Month dropdown & wheel";
  checkbox_basic => CheckboxBasicDemo, "checkbox", "States";
  checkbox_cards => CheckboxCardsDemo, "checkbox", "Selectable cards";
  date_picker_basic => DatePickerBasicDemo, "date-picker", "Date picker";
  date_picker_input => DatePickerInputDemo, "date-picker", "Typed date";
  date_time_picker_calendar => DateTimePickerCalendarDemo, "date-time-picker", "Calendar & time picker";
  date_time_picker_wheel => DateTimePickerWheelDemo, "date-time-picker", "Full wheel & time input";
  field_basic => FieldBasicDemo, "field", "Description and error";
  input_states => InputStatesDemo, "input", "States";
  input_icons => InputIconsDemo, "input", "Leading icon and password toggle";
  input_validation => InputValidationDemo, "input", "Live email validation";
  input_group_addons => InputGroupAddonsDemo, "input-group", "Addons";
  input_otp_basic => InputOtpBasicDemo, "input-otp", "Six digits";
  label_basic => LabelBasicDemo, "label", "Labels";
  native_select_basic => NativeSelectBasicDemo, "native-select", "Groups";
  radio_group_basic => RadioGroupBasicDemo, "radio-group", "Plan picker";
  radio_group_cards => RadioGroupCardsDemo, "radio-group", "Card option group";
  radio_group_horizontal => RadioGroupHorizontalDemo, "radio-group", "Horizontal alignment";
  select_basic => SelectBasicDemo, "select", "Select";
  select_multiple => SelectMultipleDemo, "select", "Multiple";
  slider_basic => SliderBasicDemo, "slider", "Orientation and states";
  slider_range => SliderRangeDemo, "slider", "Price range";
  switch_basic => SwitchBasicDemo, "switch", "States";
  switch_settings => SwitchSettingsDemo, "switch", "Notification settings";
  textarea_basic => TextareaBasicDemo, "textarea", "Character count";
  textarea_form => TextareaFormDemo, "textarea", "Feedback form with validation";
  alert_dialog_confirm => AlertDialogConfirmDemo, "alert-dialog", "Confirm deletion";
  alert_dialog_input => AlertDialogInputDemo, "alert-dialog", "Typed confirmation";
  combobox_search => ComboboxSearchDemo, "combobox", "Search";
  combobox_groups => ComboboxGroupsDemo, "combobox", "Grouped options and status";
  context_menu_basic => ContextMenuBasicDemo, "context-menu", "Right-click menu";
  context_menu_submenu => ContextMenuSubmenuDemo, "context-menu", "Submenu and radio";
  dialog_form => DialogFormDemo, "dialog", "Form in a dialog";
  dialog_scrollable => DialogScrollableDemo, "dialog", "Scrollable terms";
  dialog_share => DialogShareDemo, "dialog", "Share link with copy";
  drawer_basic => DrawerBasicDemo, "drawer", "Bottom drawer";
  drawer_actions => DrawerActionsDemo, "drawer", "Action sheet";
  dropdown_actions => DropdownActionsDemo, "dropdown", "Actions menu";
  dropdown_options => DropdownOptionsDemo, "dropdown", "View options";
  hover_card_profile => HoverCardProfileDemo, "hover-card", "Profile card";
  menubar_editor => MenubarEditorDemo, "menubar", "Editor menus";
  menubar_submenu => MenubarSubmenuDemo, "menubar", "Submenus and radio choices";
  popover_basic => PopoverBasicDemo, "popover", "Form in a popover";
  popover_filter => PopoverFilterDemo, "popover", "Filter panel";
  sheet_side => SheetSideDemo, "sheet", "Sides";
  sheet_filter => SheetFilterDemo, "sheet", "Filter drawer";
  tooltip_basic => TooltipBasicDemo, "tooltip", "Tooltip";
  tooltip_sides => TooltipSidesDemo, "tooltip", "Four sides";
  tooltip_shortcut => TooltipShortcutDemo, "tooltip", "With keyboard shortcut";
  breadcrumb_basic => BreadcrumbBasicDemo, "breadcrumb", "Collapsed path";
  breadcrumb_custom => BreadcrumbCustomDemo, "breadcrumb", "Icons and custom separators";
  navigation_menu_basic => NavigationMenuBasicDemo, "navigation-menu", "Product menu";
  navigation_menu_mega => NavigationMenuMegaDemo, "navigation-menu", "Mega menu";
  pagination_basic => PaginationBasicDemo, "pagination", "Pages";
  pagination_summary => PaginationSummaryDemo, "pagination", "Item range and page size";
  sidebar_collapsible => SidebarCollapsibleDemo, "sidebar", "Collapsible";
  sidebar_right_rail => SidebarRightRailDemo, "sidebar", "Right sidebar with rail";
  code_tabs_compare => CodeTabsCompareDemo, "code-tabs", "Three-way syntax comparison";
  tabs_account => TabsAccountDemo, "tabs", "Account settings";
  tabs_vertical => TabsVerticalDemo, "tabs", "Vertical sidebar tabs";
  tabs_underline => TabsUnderlineDemo, "tabs", "Underline and badges";
  tree_files => TreeFilesDemo, "tree", "Project files";
  accordion_faq => AccordionFaqDemo, "accordion", "FAQ";
  accordion_multiple => AccordionMultipleDemo, "accordion", "Multiple open items";
  accordion_bordered => AccordionBorderedDemo, "accordion", "Bordered cards";
  aspect_ratio_basic => AspectRatioBasicDemo, "aspect-ratio", "Ratios";
  card_basic => CardBasicDemo, "card", "Form card";
  card_variants => CardVariantsDemo, "card", "Border, dash, and sizes";
  card_media => CardMediaDemo, "card", "Media, side, and overlay";
  carousel_basic => CarouselBasicDemo, "carousel", "Slides";
  carousel_vertical => CarouselVerticalDemo, "carousel", "Vertical looping slides";
  collapsible_basic => CollapsibleBasicDemo, "collapsible", "Show more";
  direction_rtl => DirectionRtlDemo, "direction", "Right to left";
  item_list => ItemListDemo, "item", "People";
  resizable_panels => ResizablePanelsDemo, "resizable", "Two panels";
  scroll_area_tags => ScrollAreaTagsDemo, "scroll-area", "Tags";
  separator_basic => SeparatorBasicDemo, "separator", "Orientation";
  avatar_basic => AvatarBasicDemo, "avatar", "Image and fallback";
  avatar_sizes => AvatarSizesDemo, "avatar", "Sizes and shapes";
  avatar_group => AvatarGroupDemo, "avatar", "Avatar group and status";
  badge_variants => BadgeVariantsDemo, "badge", "Variants";
  badge_status => BadgeStatusDemo, "badge", "Status";
  badge_composition => BadgeCompositionDemo, "badge", "Dots, counts, and removable";
  stat_revenue => StatRevenueDemo, "stat", "Revenue";
  stat_vertical => StatVerticalDemo, "stat", "Vertical KPI stack";
  stat_actions => StatActionsDemo, "stat", "Stats with figures and actions";
  timeline_release => TimelineReleaseDemo, "timeline", "Release history";
  timeline_cards => TimelineCardsDemo, "timeline", "Card entries";
  timeline_compact => TimelineCompactDemo, "timeline", "Activity feed";
  steps_checkout => StepsCheckoutDemo, "steps", "Checkout";
  steps_descriptions => StepsDescriptionsDemo, "steps", "Detailed milestones";
  steps_clickable => StepsClickableDemo, "steps", "Clickable onboarding";
  indicator_counts => IndicatorCountsDemo, "indicator", "Counts and presence";
  mask_shapes => MaskShapesDemo, "mask", "Geometric shapes";
  mask_shapes_gallery => MaskShapesGalleryDemo, "mask", "Shape gallery and halves";
  stack_cards => StackCardsDemo, "stack", "Stacked cards";
  hover_gallery_product => HoverGalleryProductDemo, "hover-gallery", "Product angles";
  hover_3d_card => Hover3dCardDemo, "hover-3d", "3D tilt card";
  hover_3d_intensity => Hover3dIntensityDemo, "hover-3d", "Tilt intensity comparison";
  text_rotate_headline => TextRotateHeadlineDemo, "text-rotate", "Rotating headline";
  aura_spotlight => AuraSpotlightDemo, "aura", "Ambient spotlight";
  aura_tones => AuraTonesDemo, "aura", "All semantic tones";
  status_presence => StatusPresenceDemo, "status", "Service health";
  status_sizes => StatusSizesDemo, "status", "Sizes and accessible labels";
  radial_progress_usage => RadialProgressUsageDemo, "radial-progress", "Sizes and labels";
  countdown_sale => CountdownSaleDemo, "countdown", "Sale timer";
  diff_compare => DiffCompareDemo, "diff", "Design comparison";
  rating_review => RatingReviewDemo, "rating", "Review";
  number_input_cart => NumberInputCartDemo, "number-input", "Quantity and weight";
  tags_input_topics => TagsInputTopicsDemo, "tags-input", "Topics";
  file_input_upload => FileInputUploadDemo, "file-input", "Documents";
  file_input_dropzone => FileInputDropzoneDemo, "file-input", "Drag-and-drop dropzone";
  swap_icons => SwapIconsDemo, "swap", "Icons and text";
  dock_phone => DockPhoneDemo, "dock", "Phone tabs";
  fab_speed_dial => FabSpeedDialDemo, "fab", "Speed dial";
  chart_revenue => ChartRevenueDemo, "chart", "Area and line";
  chart_traffic => ChartTrafficDemo, "chart", "Donut";
  chart_bar => ChartBarDemo, "chart", "Bar chart with tooltip";
  data_table_users => DataTableUsersDemo, "data-table", "Filter, sort, and select";
  data_table_states => DataTableStatesDemo, "data-table", "Pagination and states";
  empty_projects => EmptyProjectsDemo, "empty", "No projects";
  empty_search => EmptySearchDemo, "empty", "No search results";
  empty_permission => EmptyPermissionDemo, "empty", "Access restricted";
  progress_upload => ProgressUploadDemo, "progress", "Upload";
  progress_tones => ProgressTonesDemo, "progress", "Semantic tones and sizes";
  progress_steps => ProgressStepsDemo, "progress", "Multi-metric storage quota";
  table_invoices => TableInvoicesDemo, "table", "Invoices";
  table_striped => TableStripedDemo, "table", "Striped and compact";
  table_status => TableStatusDemo, "table", "Users with avatars and actions";
  typography_article => TypographyArticleDemo, "typography", "Article";
  alert_variants => AlertVariantsDemo, "alert", "Variants";
  alert_status => AlertStatusDemo, "alert", "Status";
  alert_actions => AlertActionsDemo, "alert", "With icons and actions";
  skeleton_card => SkeletonCardDemo, "skeleton", "Loading profile";
  skeleton_list => SkeletonListDemo, "skeleton", "Feed and list placeholders";
  skeleton_table => SkeletonTableDemo, "skeleton", "Table placeholder";
  sonner_variants => SonnerVariantsDemo, "sonner", "Variants";
  sonner_async => SonnerAsyncDemo, "sonner", "Loading and status updates";
  spinner_sizes => SpinnerSizesDemo, "spinner", "Sizes";
  toast_undo => ToastUndoDemo, "toast", "Undo action";
  toast_variants => ToastVariantsDemo, "toast", "Severity and placement";
  attachment_states => AttachmentStatesDemo, "attachment", "Upload states";
  attachment_gallery => AttachmentGalleryDemo, "attachment", "Sizes and vertical cards";
  bubble_chat => BubbleChatDemo, "bubble", "Variants";
  bubble_reactions => BubbleReactionsDemo, "bubble", "Variants and reactions";
  marker_variants => MarkerVariantsDemo, "marker", "Variants";
  message_thread => MessageThreadDemo, "message", "Thread";
  message_scroller_chat => MessageScrollerChatDemo, "message-scroller", "Follow new messages";
  filter_pills => FilterPillsDemo, "filter", "Filter pills";
  link_variants => LinkVariantsDemo, "link", "Link variants";
  loading_variants => LoadingVariantsDemo, "loading", "Loading variants";
  join_items => JoinItemsDemo, "join", "Joined controls";
  join_vertical => JoinVerticalDemo, "join", "Vertical stack and filter bar";
  navbar_slots => NavbarSlotsDemo, "navbar", "Navbar slots";
  navbar_search => NavbarSearchDemo, "navbar", "Search and avatar";
  navbar_surface => NavbarSurfaceDemo, "navbar", "Primary and subtle surfaces";
  hero_showcase => HeroShowcaseDemo, "hero", "Hero showcase";
  hero_split => HeroSplitDemo, "hero", "Split with visual";
  hero_auth => HeroAuthDemo, "hero", "Hero with sign-in card";
  footer_columns => FooterColumnsDemo, "footer", "Footer columns";
  footer_centered => FooterCenteredDemo, "footer", "Centered social footer";
  footer_newsletter => FooterNewsletterDemo, "footer", "Newsletter subscription";
  megamenu_columns => MegamenuColumnsDemo, "megamenu", "Megamenu columns";
  validator_states => ValidatorStatesDemo, "validator", "Validation states";
}

#[cfg(test)]
fn demo_name(module: &str) -> String {
  let mut name: String = module
    .split('_')
    .map(|word| {
      let mut chars = word.chars();
      chars
        .next()
        .map(|first| first.to_ascii_uppercase().to_string() + chars.as_str())
        .unwrap_or_default()
    })
    .collect();
  name.push_str("Demo");
  name
}
