#![allow(unused_imports)]
use crate::components::*;
use alloc::{format, vec, vec::Vec};
use momenta::prelude::*;

#[component]
pub fn BadgesPage() -> Node {
    rsx! {
        <article class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <CategoryHeader title="Badges & Tags" description="Small status descriptors and labels for UI elements. Use them to highlight status, categories, or counts." count={8} />

            <Showcase id="default-badge" title="Default Badge" description="Simple inline badge with muted styling."
                code={r#"rsx! {
    <span class="inline-flex items-center rounded-md bg-muted px-2 py-0.5 text-xs font-medium text-muted-foreground">
        "Badge"
    </span>
}"#}>
                <div class="flex flex-wrap items-center gap-2">
                    <span class="inline-flex items-center rounded-md bg-muted px-2 py-0.5 text-xs font-medium text-muted-foreground">"Badge"</span>
                    <span class="inline-flex items-center rounded-md bg-muted px-2 py-0.5 text-xs font-medium text-muted-foreground">"Default"</span>
                    <span class="inline-flex items-center rounded-md bg-muted px-2 py-0.5 text-xs font-medium text-muted-foreground">"v0.2.0"</span>
                </div>
            </Showcase>

            <Showcase id="colored-badges" title="Colored Badges" description="Semantic color badges for status indication."
                code={r#"rsx! {
    <span class="inline-flex items-center rounded-md bg-green-500/10 text-green-600 dark:text-green-400 px-2 py-0.5 text-xs font-medium">
        "Success"
    </span>
}"#}>
                <div class="flex flex-wrap items-center gap-2">
                    <span class="inline-flex items-center rounded-md bg-blue-500/10 text-blue-600 dark:text-blue-400 px-2 py-0.5 text-xs font-medium">"Info"</span>
                    <span class="inline-flex items-center rounded-md bg-green-500/10 text-green-600 dark:text-green-400 px-2 py-0.5 text-xs font-medium">"Success"</span>
                    <span class="inline-flex items-center rounded-md bg-amber-500/10 text-amber-600 dark:text-amber-400 px-2 py-0.5 text-xs font-medium">"Warning"</span>
                    <span class="inline-flex items-center rounded-md bg-red-500/10 text-red-600 dark:text-red-400 px-2 py-0.5 text-xs font-medium">"Error"</span>
                    <span class="inline-flex items-center rounded-md bg-purple-500/10 text-purple-600 dark:text-purple-400 px-2 py-0.5 text-xs font-medium">"New"</span>
                    <span class="inline-flex items-center rounded-md bg-cyan-500/10 text-cyan-600 dark:text-cyan-400 px-2 py-0.5 text-xs font-medium">"Beta"</span>
                </div>
            </Showcase>

            <Showcase id="outline-badge" title="Outline Badge" description="Bordered badges with transparent background."
                code={r#"rsx! {
    <span class="inline-flex items-center rounded-md border border-border px-2 py-0.5 text-xs font-medium">
        "Outline"
    </span>
}"#}>
                <div class="flex flex-wrap items-center gap-2">
                    <span class="inline-flex items-center rounded-md border border-border px-2 py-0.5 text-xs font-medium">"Outline"</span>
                    <span class="inline-flex items-center rounded-md border border-blue-500/40 text-blue-600 dark:text-blue-400 px-2 py-0.5 text-xs font-medium">"Blue"</span>
                    <span class="inline-flex items-center rounded-md border border-green-500/40 text-green-600 dark:text-green-400 px-2 py-0.5 text-xs font-medium">"Green"</span>
                    <span class="inline-flex items-center rounded-md border border-red-500/40 text-red-600 dark:text-red-400 px-2 py-0.5 text-xs font-medium">"Red"</span>
                </div>
            </Showcase>

            <Showcase id="badge-dot" title="Badge with Dot" description="Badges with a colored dot indicator for live status."
                code={r#"rsx! {
    <span class="inline-flex items-center gap-1.5 rounded-md bg-green-500/10 px-2 py-0.5 text-xs font-medium text-green-600 dark:text-green-400">
        <span class="h-1.5 w-1.5 rounded-full bg-green-500"></span>
        "Online"
    </span>
}"#}>
                <div class="flex flex-wrap items-center gap-2">
                    <span class="inline-flex items-center gap-1.5 rounded-md bg-green-500/10 px-2 py-0.5 text-xs font-medium text-green-600 dark:text-green-400">
                        <span class="h-1.5 w-1.5 rounded-full bg-green-500"></span>
                        "Online"
                    </span>
                    <span class="inline-flex items-center gap-1.5 rounded-md bg-red-500/10 px-2 py-0.5 text-xs font-medium text-red-600 dark:text-red-400">
                        <span class="h-1.5 w-1.5 rounded-full bg-red-500"></span>
                        "Offline"
                    </span>
                    <span class="inline-flex items-center gap-1.5 rounded-md bg-amber-500/10 px-2 py-0.5 text-xs font-medium text-amber-600 dark:text-amber-400">
                        <span class="h-1.5 w-1.5 rounded-full bg-amber-500"></span>
                        "Away"
                    </span>
                    <span class="inline-flex items-center gap-1.5 rounded-md bg-muted px-2 py-0.5 text-xs font-medium text-muted-foreground">
                        <span class="h-1.5 w-1.5 rounded-full bg-muted-foreground/50"></span>
                        "Idle"
                    </span>
                </div>
            </Showcase>

            <Showcase id="pill-badge" title="Pill Badge" description="Fully rounded badges for soft, tag-like appearance."
                code={r#"rsx! {
    <span class="inline-flex items-center rounded-full bg-primary/10 text-primary px-2.5 py-0.5 text-xs font-medium">
        "Rust"
    </span>
}"#}>
                <div class="flex flex-wrap items-center gap-2">
                    <span class="inline-flex items-center rounded-full bg-primary/10 text-primary px-2.5 py-0.5 text-xs font-medium">"Rust"</span>
                    <span class="inline-flex items-center rounded-full bg-orange-500/10 text-orange-600 dark:text-orange-400 px-2.5 py-0.5 text-xs font-medium">"WebAssembly"</span>
                    <span class="inline-flex items-center rounded-full bg-cyan-500/10 text-cyan-600 dark:text-cyan-400 px-2.5 py-0.5 text-xs font-medium">"Tailwind"</span>
                    <span class="inline-flex items-center rounded-full bg-violet-500/10 text-violet-600 dark:text-violet-400 px-2.5 py-0.5 text-xs font-medium">"Reactive"</span>
                </div>
            </Showcase>

            <Showcase id="removable-tag" title="Removable Tag" description="Tags with a close button for dismissible labels."
                code={r#"rsx! {
    <span class="inline-flex items-center gap-1 rounded-md bg-muted pl-2 pr-1 py-0.5 text-xs font-medium">
        "React"
        <button class="ml-0.5 inline-flex items-center justify-center rounded hover:bg-foreground/10 h-4 w-4">
            <i class="fas fa-times text-[8px]"></i>
        </button>
    </span>
}"#}>
                <div class="flex flex-wrap items-center gap-2">
                    <span class="inline-flex items-center gap-1 rounded-md bg-blue-500/10 text-blue-600 dark:text-blue-400 pl-2 pr-1 py-0.5 text-xs font-medium">
                        "TypeScript"
                        <button class="ml-0.5 inline-flex items-center justify-center rounded hover:bg-blue-500/20 h-4 w-4 transition-colors">
                            <i class="fas fa-times text-[8px]"></i>
                        </button>
                    </span>
                    <span class="inline-flex items-center gap-1 rounded-md bg-green-500/10 text-green-600 dark:text-green-400 pl-2 pr-1 py-0.5 text-xs font-medium">
                        "Rust"
                        <button class="ml-0.5 inline-flex items-center justify-center rounded hover:bg-green-500/20 h-4 w-4 transition-colors">
                            <i class="fas fa-times text-[8px]"></i>
                        </button>
                    </span>
                    <span class="inline-flex items-center gap-1 rounded-md bg-muted pl-2 pr-1 py-0.5 text-xs font-medium text-muted-foreground">
                        "Python"
                        <button class="ml-0.5 inline-flex items-center justify-center rounded hover:bg-foreground/10 h-4 w-4 transition-colors">
                            <i class="fas fa-times text-[8px]"></i>
                        </button>
                    </span>
                </div>
            </Showcase>

            <Showcase id="status-badge" title="Status Badge" description="Badges indicating workflow or process status."
                code={r#"rsx! {
    <span class="inline-flex items-center gap-1.5 rounded-full border border-green-500/30 bg-green-500/10 px-2.5 py-0.5 text-xs font-medium text-green-600 dark:text-green-400">
        <i class="fas fa-check-circle text-[10px]"></i>
        "Completed"
    </span>
}"#}>
                <div class="flex flex-wrap items-center gap-2">
                    <span class="inline-flex items-center gap-1.5 rounded-full border border-green-500/30 bg-green-500/10 px-2.5 py-0.5 text-xs font-medium text-green-600 dark:text-green-400">
                        <i class="fas fa-check-circle text-[10px]"></i>
                        "Completed"
                    </span>
                    <span class="inline-flex items-center gap-1.5 rounded-full border border-amber-500/30 bg-amber-500/10 px-2.5 py-0.5 text-xs font-medium text-amber-600 dark:text-amber-400">
                        <i class="fas fa-clock text-[10px]"></i>
                        "Pending"
                    </span>
                    <span class="inline-flex items-center gap-1.5 rounded-full border border-blue-500/30 bg-blue-500/10 px-2.5 py-0.5 text-xs font-medium text-blue-600 dark:text-blue-400">
                        <i class="fas fa-sync fa-spin text-[10px]"></i>
                        "In Progress"
                    </span>
                    <span class="inline-flex items-center gap-1.5 rounded-full border border-red-500/30 bg-red-500/10 px-2.5 py-0.5 text-xs font-medium text-red-600 dark:text-red-400">
                        <i class="fas fa-times-circle text-[10px]"></i>
                        "Failed"
                    </span>
                </div>
            </Showcase>

            <Showcase id="notification-badge" title="Notification Badge" description="Small count badges overlaid on icons or avatars."
                code={r#"rsx! {
    <div class="relative inline-flex">
        <button class="inline-flex items-center justify-center rounded-lg border border-border h-10 w-10 hover:bg-muted">
            <i class="fas fa-bell text-sm"></i>
        </button>
        <span class="absolute -top-1 -right-1 flex h-4 w-4 items-center justify-center rounded-full bg-red-500 text-[10px] font-bold text-white">
            "3"
        </span>
    </div>
}"#}>
                <div class="flex flex-wrap items-center gap-6">
                    <div class="relative inline-flex">
                        <button class="inline-flex items-center justify-center rounded-lg border border-border h-10 w-10 hover:bg-muted transition-colors">
                            <i class="fas fa-bell text-sm"></i>
                        </button>
                        <span class="absolute -top-1 -right-1 flex h-4 w-4 items-center justify-center rounded-full bg-red-500 text-[10px] font-bold text-white">"3"</span>
                    </div>
                    <div class="relative inline-flex">
                        <button class="inline-flex items-center justify-center rounded-lg border border-border h-10 w-10 hover:bg-muted transition-colors">
                            <i class="fas fa-envelope text-sm"></i>
                        </button>
                        <span class="absolute -top-1 -right-1 flex h-5 min-w-[1.25rem] items-center justify-center rounded-full bg-primary px-1 text-[10px] font-bold text-primary-foreground">"12"</span>
                    </div>
                    <div class="relative inline-flex">
                        <button class="inline-flex items-center justify-center rounded-lg border border-border h-10 w-10 hover:bg-muted transition-colors">
                            <i class="fas fa-shopping-cart text-sm"></i>
                        </button>
                        <span class="absolute -top-0.5 -right-0.5 h-2.5 w-2.5 rounded-full bg-red-500 ring-2 ring-background"></span>
                    </div>
                </div>
            </Showcase>

            <div class="mt-8 flex items-center justify-between border-t border-border pt-6">
                <a href={docs_href("/ui/buttons")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "← Buttons"
                </a>
                <a href={docs_href("/ui/alerts")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "Alerts & Banners →"
                </a>
            </div>
        </article>
    }
}
