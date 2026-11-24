use dioxus::prelude::*;
use dioxus_shadcn::{
  Bubble, BubbleAlign, BubbleContent, BubbleGroup, BubbleReactionAlign, BubbleReactionSide,
  BubbleReactions, BubbleVariant,
};

#[component]
pub fn BubbleReactionsDemo() -> Element {
  rsx! {
    BubbleGroup { class: "max-w-md gap-4",
      Bubble { align: BubbleAlign::Start, variant: BubbleVariant::Outline,
        BubbleContent { variant: BubbleVariant::Outline, "Outline bubble with reactions aligned above." }
        BubbleReactions { side: BubbleReactionSide::Top, align: BubbleReactionAlign::Start, "Pinned by design" }
      }
      Bubble { align: BubbleAlign::End, variant: BubbleVariant::Default,
        BubbleContent { variant: BubbleVariant::Default, "Default primary bubble with centered reactions." }
        BubbleReactions { side: BubbleReactionSide::Bottom, align: BubbleReactionAlign::Center, "Shipped · 4" }
      }
      Bubble { align: BubbleAlign::Start, variant: BubbleVariant::Muted,
        BubbleContent { variant: BubbleVariant::Muted, "Muted system note with end-aligned metadata." }
        BubbleReactions { side: BubbleReactionSide::Bottom, align: BubbleReactionAlign::End, "Seen 2m ago" }
      }
      Bubble { align: BubbleAlign::Start, variant: BubbleVariant::Ghost,
        BubbleContent { variant: BubbleVariant::Ghost, "Ghost bubble for inline editorial commentary." }
      }
    }
  }
}
