#![allow(unused_imports)]

pub mod alerts;
pub mod badges;
pub mod buttons;
pub mod cards;
pub mod data_display;
pub mod feedback;
pub mod inputs;
pub mod layout;
pub mod marketing;
pub mod nav_components;
pub mod overlays;
pub mod typography;

use crate::components::*;
use alloc::{format, string::ToString, vec, vec::Vec};
use momenta::prelude::*;

pub use alerts::*;
pub use badges::*;
pub use buttons::*;
pub use cards::*;
pub use data_display::*;
pub use feedback::*;
pub use inputs::*;
pub use layout::*;
pub use marketing::*;
pub use nav_components::*;
pub use overlays::*;
pub use typography::*;

static UI_CATEGORIES: &[(&str, &str, &str, &str, usize)] = &[
    (
        "buttons",
        "Buttons",
        "Interactive button components in various styles, sizes, and states.",
        "fas fa-hand-pointer",
        12,
    ),
    (
        "badges",
        "Badges & Tags",
        "Small status descriptors for UI elements.",
        "fas fa-tag",
        8,
    ),
    (
        "alerts",
        "Alerts & Banners",
        "Contextual feedback messages and notifications.",
        "fas fa-bell",
        7,
    ),
    (
        "cards",
        "Cards",
        "Flexible content containers with multiple variants.",
        "fas fa-square",
        10,
    ),
    (
        "inputs",
        "Forms & Inputs",
        "Form controls for collecting user input.",
        "fas fa-keyboard",
        14,
    ),
    (
        "navigation",
        "Navigation",
        "Components for navigating between pages and sections.",
        "fas fa-compass",
        12,
    ),
    (
        "data-display",
        "Data Display",
        "Components for presenting data and information.",
        "fas fa-table",
        10,
    ),
    (
        "layout",
        "Layout",
        "Structural components for page composition.",
        "fas fa-th-large",
        8,
    ),
    (
        "feedback",
        "Feedback",
        "Loading, progress, and state indicators.",
        "fas fa-spinner",
        8,
    ),
    (
        "overlays",
        "Overlays",
        "Modals, drawers, and overlay panels.",
        "fas fa-window-restore",
        6,
    ),
    (
        "marketing",
        "Marketing Sections",
        "Full-width landing page sections and blocks.",
        "fas fa-bullhorn",
        12,
    ),
    (
        "typography",
        "Typography",
        "Text display and formatting components.",
        "fas fa-font",
        6,
    ),
];

#[component]
pub fn UIOverviewPage() -> Node {
    rsx! {
        <div class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <header class="mb-10">
                <div class="inline-flex items-center gap-2 rounded-full border border-border/50 bg-card px-3 py-1 mb-4">
                    <span class="h-1.5 w-1.5 rounded-full bg-primary"></span>
                    <span class="text-xs font-medium text-muted-foreground">113 components</span>
                </div>
                <h1 class="text-3xl font-bold tracking-tight sm:text-4xl">UI Component Library</h1>
                <p class="mt-3 text-lg text-muted-foreground leading-relaxed max-w-3xl">
                    "Production-ready UI components built with Momenta's rsx! macro and Tailwind CSS.
                    Each component includes a live preview and copy-ready code."
                </p>
            </header>

            <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                {UI_CATEGORIES.iter().map(|(slug, title, desc, icon, count)| rsx! {
                        <a href={docs_href(&format!("/ui/{}", slug))} class="card-link group">
                            <div class="flex items-start gap-3">
                                <div class="mt-0.5 flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-primary/10 text-primary">
                                    <i class={format!("{} text-base", icon)}></i>
                                </div>
                                <div class="min-w-0">
                                    <div class="flex items-center gap-2">
                                        <h3 class="text-sm font-semibold group-hover:text-primary transition-colors">{*title}</h3>
                                        <span class="text-[11px] text-muted-foreground/60 font-medium">{format!("{}", count)}</span>
                                    </div>
                                    <p class="text-xs text-muted-foreground mt-0.5 leading-relaxed">{*desc}</p>
                                </div>
                            </div>
                        </a>
                }).collect::<Vec<_>>()}
            </div>

            <div class="mt-12 rounded-xl border border-border/50 bg-card/50 p-6">
                <h2 class="text-lg font-semibold mb-3">Using these components</h2>
                <p class="text-sm text-muted-foreground leading-relaxed mb-4">
                    "Each component is self-contained rsx! markup styled with Tailwind CSS utility classes.
                    Copy the code into your Momenta project and customize colors, spacing, and content to match your design."
                </p>
                <div class="flex flex-wrap gap-2">
                    <span class="doc-chip">Copy & paste ready</span>
                    <span class="doc-chip">Tailwind CSS</span>
                    <span class="doc-chip">Dark mode</span>
                    <span class="doc-chip">Responsive</span>
                    <span class="doc-chip">Accessible markup</span>
                </div>
            </div>
        </div>
    }
}

pub fn ui_library_on_this_page(category: &str) -> Vec<(&'static str, &'static str)> {
    match category {
        "buttons" => vec![
            ("primary-button", "Primary"),
            ("secondary-button", "Secondary"),
            ("outline-button", "Outline"),
            ("ghost-button", "Ghost"),
            ("destructive-button", "Destructive"),
            ("icon-left-button", "Icon Left"),
            ("icon-right-button", "Icon Right"),
            ("icon-only-button", "Icon Only"),
            ("loading-button", "Loading"),
            ("button-sizes", "Sizes"),
            ("pill-button", "Pill / Rounded"),
            ("button-group", "Button Group"),
        ],
        "badges" => vec![
            ("default-badge", "Default"),
            ("colored-badges", "Colored"),
            ("outline-badge", "Outline"),
            ("badge-dot", "With Dot"),
            ("pill-badge", "Pill"),
            ("removable-tag", "Removable Tag"),
            ("status-badge", "Status"),
            ("notification-badge", "Notification"),
        ],
        "alerts" => vec![
            ("info-alert", "Info"),
            ("success-alert", "Success"),
            ("warning-alert", "Warning"),
            ("error-alert", "Error"),
            ("alert-description", "With Description"),
            ("alert-actions", "With Actions"),
            ("banner", "Banner"),
        ],
        "cards" => vec![
            ("basic-card", "Basic"),
            ("image-card", "With Image"),
            ("profile-card", "Profile"),
            ("pricing-card", "Pricing"),
            ("stat-card", "Stat"),
            ("product-card", "Product"),
            ("feature-card", "Feature"),
            ("testimonial-card", "Testimonial"),
            ("horizontal-card", "Horizontal"),
            ("card-footer", "With Footer"),
        ],
        "inputs" => vec![
            ("text-input", "Text Input"),
            ("input-label", "With Label"),
            ("input-error", "With Error"),
            ("textarea", "Textarea"),
            ("select-input", "Select"),
            ("checkbox", "Checkbox"),
            ("radio-group", "Radio Group"),
            ("toggle-switch", "Toggle Switch"),
            ("range-slider", "Range Slider"),
            ("file-upload", "File Upload"),
            ("search-input", "Search Input"),
            ("password-input", "Password"),
            ("input-group", "Input Group"),
            ("floating-label", "Floating Label"),
        ],
        "navigation" => vec![
            ("simple-navbar", "Navbar Simple"),
            ("navbar-cta", "Navbar with CTA"),
            ("breadcrumbs", "Breadcrumbs"),
            ("tabs-nav", "Tabs"),
            ("vertical-tabs", "Vertical Tabs"),
            ("pagination", "Pagination"),
            ("steps", "Steps"),
            ("simple-footer", "Footer Simple"),
            ("footer-columns", "Footer Columns"),
            ("bottom-nav", "Bottom Nav"),
            ("link-group", "Link Group"),
            ("sidebar-nav", "Sidebar"),
        ],
        "data-display" => vec![
            ("simple-table", "Simple Table"),
            ("striped-table", "Striped Table"),
            ("description-list", "Description List"),
            ("avatar", "Avatar"),
            ("avatar-group", "Avatar Group"),
            ("key-value", "Key-Value"),
            ("timeline", "Timeline"),
            ("stat-widget", "Stat Widget"),
            ("progress-list", "Progress List"),
            ("badge-list", "Badge List"),
        ],
        "layout" => vec![
            ("container", "Container"),
            ("two-column", "Two Column"),
            ("three-column", "Three Column"),
            ("flex-row", "Flex Row"),
            ("stack", "Stack"),
            ("divider", "Divider"),
            ("grid-auto", "Auto Grid"),
            ("sidebar-layout", "Sidebar Layout"),
        ],
        "feedback" => vec![
            ("progress-bar", "Progress Bar"),
            ("multi-progress", "Multiple Progress"),
            ("spinner", "Spinner"),
            ("skeleton", "Skeleton"),
            ("empty-state", "Empty State"),
            ("error-state", "Error State"),
            ("success-state", "Success State"),
            ("loading-overlay", "Loading Overlay"),
        ],
        "overlays" => vec![
            ("modal", "Modal"),
            ("confirmation-dialog", "Confirmation"),
            ("drawer", "Drawer"),
            ("sheet", "Sheet"),
            ("command-palette", "Command Palette"),
            ("notification-panel", "Notification Panel"),
        ],
        "marketing" => vec![
            ("hero-centered", "Hero Centered"),
            ("hero-split", "Hero Split"),
            ("cta-simple", "CTA Simple"),
            ("cta-image", "CTA with Image"),
            ("feature-grid", "Feature Grid"),
            ("feature-list", "Feature List"),
            ("testimonials-section", "Testimonials"),
            ("pricing-table", "Pricing Table"),
            ("newsletter", "Newsletter"),
            ("logo-cloud", "Logo Cloud"),
            ("stats-section", "Stats Section"),
            ("faq", "FAQ"),
        ],
        "typography" => vec![
            ("headings", "Headings"),
            ("paragraph-styles", "Paragraphs"),
            ("blockquote", "Blockquote"),
            ("inline-code", "Inline Code"),
            ("lists-display", "Lists"),
            ("kbd-display", "Keyboard Shortcuts"),
        ],
        _ => vec![],
    }
}
