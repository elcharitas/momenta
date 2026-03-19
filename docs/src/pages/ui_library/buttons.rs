#![allow(unused_imports)]
use crate::components::*;
use alloc::{format, vec, vec::Vec};
use momenta::prelude::*;

#[component]
pub fn ButtonsPage() -> Node {
    rsx! {
        <article class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <CategoryHeader title="Buttons" description="Interactive button components in various styles, sizes, and states. All buttons use semantic HTML and support keyboard navigation." count={12} />

            <Showcase id="primary-button" title="Primary Button" description="The default action button with solid background."
                code={r#"rsx! {
    <button class="inline-flex items-center justify-center rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:opacity-90 transition-opacity">
        "Get Started"
    </button>
}"#}>
                <div class="flex flex-wrap items-center gap-3">
                    <button class="inline-flex items-center justify-center rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:opacity-90 transition-opacity">
                        "Get Started"
                    </button>
                    <button class="inline-flex items-center justify-center rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground opacity-50 cursor-not-allowed">
                        "Disabled"
                    </button>
                </div>
            </Showcase>

            <Showcase id="secondary-button" title="Secondary Button" description="A muted button for secondary actions."
                code={r#"rsx! {
    <button class="inline-flex items-center justify-center rounded-lg bg-secondary px-4 py-2 text-sm font-medium text-secondary-foreground hover:bg-secondary/80 transition-colors">
        "Secondary"
    </button>
}"#}>
                <div class="flex flex-wrap items-center gap-3">
                    <button class="inline-flex items-center justify-center rounded-lg bg-secondary px-4 py-2 text-sm font-medium text-secondary-foreground hover:bg-secondary/80 transition-colors">
                        "Secondary"
                    </button>
                    <button class="inline-flex items-center justify-center rounded-lg bg-secondary px-4 py-2 text-sm font-medium text-secondary-foreground opacity-50 cursor-not-allowed">
                        "Disabled"
                    </button>
                </div>
            </Showcase>

            <Showcase id="outline-button" title="Outline Button" description="A bordered button with transparent background."
                code={r#"rsx! {
    <button class="inline-flex items-center justify-center rounded-lg border border-border px-4 py-2 text-sm font-medium hover:bg-muted transition-colors">
        "Outline"
    </button>
}"#}>
                <div class="flex flex-wrap items-center gap-3">
                    <button class="inline-flex items-center justify-center rounded-lg border border-border px-4 py-2 text-sm font-medium hover:bg-muted transition-colors">
                        "Outline"
                    </button>
                    <button class="inline-flex items-center justify-center rounded-lg border border-primary text-primary px-4 py-2 text-sm font-medium hover:bg-primary/10 transition-colors">
                        "Primary Outline"
                    </button>
                </div>
            </Showcase>

            <Showcase id="ghost-button" title="Ghost Button" description="A transparent button that shows background on hover."
                code={r#"rsx! {
    <button class="inline-flex items-center justify-center rounded-lg px-4 py-2 text-sm font-medium hover:bg-muted transition-colors">
        "Ghost"
    </button>
}"#}>
                <div class="flex flex-wrap items-center gap-3">
                    <button class="inline-flex items-center justify-center rounded-lg px-4 py-2 text-sm font-medium hover:bg-muted transition-colors">
                        "Ghost"
                    </button>
                    <button class="inline-flex items-center justify-center rounded-lg px-4 py-2 text-sm font-medium text-primary hover:bg-primary/10 transition-colors">
                        "Ghost Primary"
                    </button>
                </div>
            </Showcase>

            <Showcase id="destructive-button" title="Destructive Button" description="A red button for dangerous or irreversible actions."
                code={r#"rsx! {
    <button class="inline-flex items-center justify-center rounded-lg bg-destructive px-4 py-2 text-sm font-medium text-destructive-foreground hover:opacity-90 transition-opacity">
        "Delete Account"
    </button>
}"#}>
                <div class="flex flex-wrap items-center gap-3">
                    <button class="inline-flex items-center justify-center rounded-lg bg-destructive px-4 py-2 text-sm font-medium text-destructive-foreground hover:opacity-90 transition-opacity">
                        "Delete Account"
                    </button>
                    <button class="inline-flex items-center justify-center rounded-lg border border-destructive text-destructive px-4 py-2 text-sm font-medium hover:bg-destructive/10 transition-colors">
                        "Remove"
                    </button>
                </div>
            </Showcase>

            <Showcase id="icon-left-button" title="Button with Left Icon" description="An icon placed before the label for visual context."
                code={r#"rsx! {
    <button class="inline-flex items-center gap-2 rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:opacity-90 transition-opacity">
        <i class="fas fa-download text-xs"></i>
        "Download"
    </button>
}"#}>
                <div class="flex flex-wrap items-center gap-3">
                    <button class="inline-flex items-center gap-2 rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:opacity-90 transition-opacity">
                        <i class="fas fa-download text-xs"></i>
                        "Download"
                    </button>
                    <button class="inline-flex items-center gap-2 rounded-lg border border-border px-4 py-2 text-sm font-medium hover:bg-muted transition-colors">
                        <i class="fas fa-plus text-xs"></i>
                        "Add Item"
                    </button>
                    <button class="inline-flex items-center gap-2 rounded-lg bg-green-600 px-4 py-2 text-sm font-medium text-white hover:bg-green-700 transition-colors">
                        <i class="fas fa-check text-xs"></i>
                        "Approve"
                    </button>
                </div>
            </Showcase>

            <Showcase id="icon-right-button" title="Button with Right Icon" description="An icon placed after the label, often for navigation cues."
                code={r#"rsx! {
    <button class="inline-flex items-center gap-2 rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:opacity-90 transition-opacity">
        "Continue"
        <i class="fas fa-arrow-right text-xs"></i>
    </button>
}"#}>
                <div class="flex flex-wrap items-center gap-3">
                    <button class="inline-flex items-center gap-2 rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:opacity-90 transition-opacity">
                        "Continue"
                        <i class="fas fa-arrow-right text-xs"></i>
                    </button>
                    <button class="inline-flex items-center gap-2 rounded-lg border border-border px-4 py-2 text-sm font-medium hover:bg-muted transition-colors">
                        "Learn more"
                        <i class="fas fa-external-link-alt text-xs"></i>
                    </button>
                </div>
            </Showcase>

            <Showcase id="icon-only-button" title="Icon Only Button" description="Compact buttons containing only an icon, typically for toolbars."
                code={r#"rsx! {
    <button class="inline-flex items-center justify-center rounded-lg border border-border h-9 w-9 hover:bg-muted transition-colors">
        <i class="fas fa-heart text-sm"></i>
    </button>
}"#}>
                <div class="flex flex-wrap items-center gap-3">
                    <button class="inline-flex items-center justify-center rounded-lg border border-border h-9 w-9 hover:bg-muted transition-colors">
                        <i class="fas fa-heart text-sm"></i>
                    </button>
                    <button class="inline-flex items-center justify-center rounded-lg bg-primary h-9 w-9 text-primary-foreground hover:opacity-90 transition-opacity">
                        <i class="fas fa-plus text-sm"></i>
                    </button>
                    <button class="inline-flex items-center justify-center rounded-lg bg-muted h-9 w-9 hover:bg-muted/80 transition-colors">
                        <i class="fas fa-ellipsis-v text-sm"></i>
                    </button>
                    <button class="inline-flex items-center justify-center rounded-lg hover:bg-muted h-9 w-9 transition-colors text-muted-foreground hover:text-foreground">
                        <i class="fas fa-cog text-sm"></i>
                    </button>
                    <button class="inline-flex items-center justify-center rounded-full bg-destructive h-9 w-9 text-destructive-foreground hover:opacity-90 transition-opacity">
                        <i class="fas fa-trash text-xs"></i>
                    </button>
                </div>
            </Showcase>

            <Showcase id="loading-button" title="Loading Button" description="Buttons showing a loading spinner to indicate pending action."
                code={r#"rsx! {
    <button class="inline-flex items-center gap-2 rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground opacity-80 cursor-wait">
        <i class="fas fa-spinner fa-spin text-xs"></i>
        "Processing..."
    </button>
}"#}>
                <div class="flex flex-wrap items-center gap-3">
                    <button class="inline-flex items-center gap-2 rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground opacity-80 cursor-wait">
                        <i class="fas fa-spinner fa-spin text-xs"></i>
                        "Processing..."
                    </button>
                    <button class="inline-flex items-center gap-2 rounded-lg border border-border px-4 py-2 text-sm font-medium opacity-80 cursor-wait">
                        <i class="fas fa-circle-notch fa-spin text-xs"></i>
                        "Loading..."
                    </button>
                    <button class="inline-flex items-center justify-center rounded-lg bg-secondary h-9 w-9 opacity-80 cursor-wait">
                        <i class="fas fa-spinner fa-spin text-sm"></i>
                    </button>
                </div>
            </Showcase>

            <Showcase id="button-sizes" title="Button Sizes" description="Buttons available in small, default, and large sizes."
                code={r#"rsx! {
    <button class="inline-flex items-center rounded-md bg-primary px-2.5 py-1 text-xs font-medium text-primary-foreground">
        "Small"
    </button>
    <button class="inline-flex items-center rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground">
        "Default"
    </button>
    <button class="inline-flex items-center rounded-lg bg-primary px-6 py-2.5 text-base font-medium text-primary-foreground">
        "Large"
    </button>
}"#}>
                <div class="flex flex-wrap items-center gap-3">
                    <button class="inline-flex items-center justify-center rounded-md bg-primary px-2.5 py-1 text-xs font-medium text-primary-foreground hover:opacity-90 transition-opacity">
                        "Small"
                    </button>
                    <button class="inline-flex items-center justify-center rounded-lg bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:opacity-90 transition-opacity">
                        "Default"
                    </button>
                    <button class="inline-flex items-center justify-center rounded-lg bg-primary px-6 py-2.5 text-base font-medium text-primary-foreground hover:opacity-90 transition-opacity">
                        "Large"
                    </button>
                </div>
            </Showcase>

            <Showcase id="pill-button" title="Pill / Rounded Button" description="Fully rounded buttons for tags, filters, or soft UI."
                code={r#"rsx! {
    <button class="inline-flex items-center rounded-full bg-primary px-5 py-2 text-sm font-medium text-primary-foreground hover:opacity-90 transition-opacity">
        "Subscribe"
    </button>
}"#}>
                <div class="flex flex-wrap items-center gap-3">
                    <button class="inline-flex items-center rounded-full bg-primary px-5 py-2 text-sm font-medium text-primary-foreground hover:opacity-90 transition-opacity">
                        "Subscribe"
                    </button>
                    <button class="inline-flex items-center rounded-full border border-border px-5 py-2 text-sm font-medium hover:bg-muted transition-colors">
                        "Filter"
                    </button>
                    <button class="inline-flex items-center gap-2 rounded-full bg-green-600/10 text-green-600 px-4 py-1.5 text-sm font-medium hover:bg-green-600/20 transition-colors">
                        <span class="h-1.5 w-1.5 rounded-full bg-green-500"></span>
                        "Active"
                    </button>
                </div>
            </Showcase>

            <Showcase id="button-group" title="Button Group" description="Multiple buttons grouped together as a single control."
                code={r#"rsx! {
    <div class="inline-flex rounded-lg border border-border overflow-hidden">
        <button class="px-4 py-2 text-sm font-medium bg-muted">"Left"</button>
        <button class="px-4 py-2 text-sm font-medium border-l border-border hover:bg-muted">"Center"</button>
        <button class="px-4 py-2 text-sm font-medium border-l border-border hover:bg-muted">"Right"</button>
    </div>
}"#}>
                <div class="flex flex-wrap items-center gap-4">
                    <div class="inline-flex rounded-lg border border-border overflow-hidden">
                        <button class="px-4 py-2 text-sm font-medium bg-muted">"Left"</button>
                        <button class="px-4 py-2 text-sm font-medium border-l border-border hover:bg-muted transition-colors">"Center"</button>
                        <button class="px-4 py-2 text-sm font-medium border-l border-border hover:bg-muted transition-colors">"Right"</button>
                    </div>
                    <div class="inline-flex rounded-lg border border-border overflow-hidden">
                        <button class="px-3 py-2 text-sm bg-primary text-primary-foreground">
                            <i class="fas fa-bold"></i>
                        </button>
                        <button class="px-3 py-2 text-sm border-l border-border hover:bg-muted transition-colors">
                            <i class="fas fa-italic"></i>
                        </button>
                        <button class="px-3 py-2 text-sm border-l border-border hover:bg-muted transition-colors">
                            <i class="fas fa-underline"></i>
                        </button>
                        <button class="px-3 py-2 text-sm border-l border-border hover:bg-muted transition-colors">
                            <i class="fas fa-strikethrough"></i>
                        </button>
                    </div>
                </div>
            </Showcase>

            <div class="mt-8 flex items-center justify-between border-t border-border pt-6">
                <a href={docs_href("/ui")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "← All Components"
                </a>
                <a href={docs_href("/ui/badges")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "Badges & Tags →"
                </a>
            </div>
        </article>
    }
}
