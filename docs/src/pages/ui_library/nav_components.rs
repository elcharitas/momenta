#![allow(unused_imports)]
use crate::components::*;
use alloc::{format, vec, vec::Vec};
use momenta::prelude::*;

#[component]
pub fn NavComponentsPage() -> Node {
    rsx! {
        <article class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <CategoryHeader title="Navigation" description="Components for navigating between pages, sections, and views. Includes navbars, footers, breadcrumbs, tabs, and more." count={12} />

            <Showcase id="simple-navbar" title="Simple Navbar" description="Minimal horizontal navigation bar with logo and links."
                code={r##"rsx! {
    <nav class="flex items-center justify-between px-6 py-3 border-b border-border">
        <a href="#" class="text-base font-semibold">"Acme Inc"</a>
        <div class="flex items-center gap-4">
            <a href="#" class="text-sm text-muted-foreground hover:text-foreground">"Home"</a>
            <a href="#" class="text-sm text-muted-foreground hover:text-foreground">"About"</a>
            <a href="#" class="text-sm text-muted-foreground hover:text-foreground">"Contact"</a>
        </div>
    </nav>
}"##}>
                <div class="w-full rounded-lg border border-border overflow-hidden">
                    <nav class="flex items-center justify-between px-6 py-3 bg-card">
                        <a href="#" class="text-base font-semibold">"Acme Inc"</a>
                        <div class="flex items-center gap-4">
                            <a href="#" class="text-sm font-medium text-primary">"Home"</a>
                            <a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"About"</a>
                            <a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"Blog"</a>
                            <a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"Contact"</a>
                        </div>
                    </nav>
                </div>
            </Showcase>

            <Showcase id="navbar-cta" title="Navbar with CTA" description="Navigation bar with call-to-action button."
                code={r##"rsx! {
    <nav class="flex items-center justify-between px-6 py-3 border-b border-border">
        <div class="flex items-center gap-8">
            <a href="#" class="font-semibold">"Brand"</a>
            <div class="flex gap-4">
                <a href="#" class="text-sm text-muted-foreground hover:text-foreground">"Features"</a>
                <a href="#" class="text-sm text-muted-foreground hover:text-foreground">"Pricing"</a>
            </div>
        </div>
        <div class="flex items-center gap-2">
            <a href="#" class="text-sm text-muted-foreground hover:text-foreground">"Sign in"</a>
            <button class="rounded-lg bg-primary px-4 py-1.5 text-sm font-medium text-primary-foreground">"Get Started"</button>
        </div>
    </nav>
}"##}>
                <div class="w-full rounded-lg border border-border overflow-hidden">
                    <nav class="flex items-center justify-between px-6 py-3 bg-card">
                        <div class="flex items-center gap-8">
                            <a href="#" class="font-semibold">"🚀 Launchpad"</a>
                            <div class="hidden sm:flex gap-4">
                                <a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"Features"</a>
                                <a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"Pricing"</a>
                                <a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"Docs"</a>
                            </div>
                        </div>
                        <div class="flex items-center gap-3">
                            <a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"Sign in"</a>
                            <button class="rounded-lg bg-primary px-4 py-1.5 text-sm font-medium text-primary-foreground hover:opacity-90 transition-opacity">"Get Started"</button>
                        </div>
                    </nav>
                </div>
            </Showcase>

            <Showcase id="breadcrumbs" title="Breadcrumbs" description="Path-based navigation for hierarchical content."
                code={r##"rsx! {
    <nav class="flex items-center gap-1.5 text-sm">
        <a href="#" class="text-muted-foreground hover:text-foreground">"Home"</a>
        <span class="text-muted-foreground/50">"/"</span>
        <a href="#" class="text-muted-foreground hover:text-foreground">"Docs"</a>
        <span class="text-muted-foreground/50">"/"</span>
        <span class="font-medium">"Components"</span>
    </nav>
}"##}>
                <div class="space-y-4">
                    <nav class="flex items-center gap-1.5 text-sm">
                        <a href="#" class="text-muted-foreground hover:text-foreground transition-colors">"Home"</a>
                        <i class="fas fa-chevron-right text-[8px] text-muted-foreground/50"></i>
                        <a href="#" class="text-muted-foreground hover:text-foreground transition-colors">"Documentation"</a>
                        <i class="fas fa-chevron-right text-[8px] text-muted-foreground/50"></i>
                        <span class="font-medium">"Components"</span>
                    </nav>
                    <nav class="flex items-center gap-1.5 text-sm">
                        <a href="#" class="text-muted-foreground hover:text-foreground transition-colors"><i class="fas fa-home text-xs"></i></a>
                        <span class="text-muted-foreground/50">"/"</span>
                        <a href="#" class="text-muted-foreground hover:text-foreground transition-colors">"Products"</a>
                        <span class="text-muted-foreground/50">"/"</span>
                        <a href="#" class="text-muted-foreground hover:text-foreground transition-colors">"Electronics"</a>
                        <span class="text-muted-foreground/50">"/"</span>
                        <span class="font-medium">"Keyboards"</span>
                    </nav>
                </div>
            </Showcase>

            <Showcase id="tabs-nav" title="Tabs" description="Tabbed interface for switching between views."
                code={r##"rsx! {
    <div class="border-b border-border">
        <div class="flex gap-0">
            <button class="px-4 py-2.5 text-sm font-medium border-b-2 border-primary text-primary">"General"</button>
            <button class="px-4 py-2.5 text-sm text-muted-foreground hover:text-foreground">"Security"</button>
            <button class="px-4 py-2.5 text-sm text-muted-foreground hover:text-foreground">"Billing"</button>
        </div>
    </div>
}"##}>
                <div class="w-full space-y-6">
                    <div class="border-b border-border">
                        <div class="flex gap-0">
                            <button class="px-4 py-2.5 text-sm font-medium border-b-2 border-primary text-primary -mb-px">"General"</button>
                            <button class="px-4 py-2.5 text-sm text-muted-foreground hover:text-foreground border-b-2 border-transparent -mb-px transition-colors">"Security"</button>
                            <button class="px-4 py-2.5 text-sm text-muted-foreground hover:text-foreground border-b-2 border-transparent -mb-px transition-colors">"Billing"</button>
                            <button class="px-4 py-2.5 text-sm text-muted-foreground hover:text-foreground border-b-2 border-transparent -mb-px transition-colors">"Notifications"</button>
                        </div>
                    </div>
                    <div class="inline-flex rounded-lg bg-muted p-1">
                        <button class="rounded-md bg-background px-3 py-1.5 text-sm font-medium shadow-sm">"Overview"</button>
                        <button class="rounded-md px-3 py-1.5 text-sm text-muted-foreground hover:text-foreground transition-colors">"Analytics"</button>
                        <button class="rounded-md px-3 py-1.5 text-sm text-muted-foreground hover:text-foreground transition-colors">"Reports"</button>
                    </div>
                </div>
            </Showcase>

            <Showcase id="vertical-tabs" title="Vertical Tabs" description="Side-oriented tab navigation."
                code={r##"rsx! {
    <div class="flex gap-4">
        <div class="w-48 space-y-0.5 border-r border-border pr-4">
            <button class="w-full text-left px-3 py-1.5 text-sm rounded-md bg-muted font-medium">"Profile"</button>
            <button class="w-full text-left px-3 py-1.5 text-sm rounded-md text-muted-foreground hover:bg-muted/50">"Account"</button>
            <button class="w-full text-left px-3 py-1.5 text-sm rounded-md text-muted-foreground hover:bg-muted/50">"Security"</button>
        </div>
        <div class="flex-1">
            <h3 class="font-semibold">"Profile Settings"</h3>
            <p class="text-sm text-muted-foreground mt-1">"Manage your profile information."</p>
        </div>
    </div>
}"##}>
                <div class="w-full flex gap-4 min-h-[160px]">
                    <div class="w-44 shrink-0 space-y-0.5 border-r border-border pr-4">
                        <button class="w-full text-left px-3 py-1.5 text-sm rounded-md bg-muted font-medium">"Profile"</button>
                        <button class="w-full text-left px-3 py-1.5 text-sm rounded-md text-muted-foreground hover:bg-muted/50 transition-colors">"Account"</button>
                        <button class="w-full text-left px-3 py-1.5 text-sm rounded-md text-muted-foreground hover:bg-muted/50 transition-colors">"Appearance"</button>
                        <button class="w-full text-left px-3 py-1.5 text-sm rounded-md text-muted-foreground hover:bg-muted/50 transition-colors">"Notifications"</button>
                        <button class="w-full text-left px-3 py-1.5 text-sm rounded-md text-muted-foreground hover:bg-muted/50 transition-colors">"Security"</button>
                    </div>
                    <div class="flex-1">
                        <h3 class="font-semibold">"Profile Settings"</h3>
                        <p class="text-sm text-muted-foreground mt-1 leading-relaxed">"Manage your public profile and personal information."</p>
                    </div>
                </div>
            </Showcase>

            <Showcase id="pagination" title="Pagination" description="Page navigation for paginated content."
                code={r##"rsx! {
    <nav class="flex items-center gap-1">
        <button class="h-8 w-8 rounded-md border border-border text-sm hover:bg-muted">"«"</button>
        <button class="h-8 w-8 rounded-md bg-primary text-sm font-medium text-primary-foreground">"1"</button>
        <button class="h-8 w-8 rounded-md border border-border text-sm hover:bg-muted">"2"</button>
        <button class="h-8 w-8 rounded-md border border-border text-sm hover:bg-muted">"3"</button>
        <button class="h-8 w-8 rounded-md border border-border text-sm hover:bg-muted">"»"</button>
    </nav>
}"##}>
                <div class="space-y-4">
                    <nav class="flex items-center gap-1">
                        <button class="h-8 w-8 rounded-md border border-border text-sm hover:bg-muted transition-colors flex items-center justify-center">
                            <i class="fas fa-chevron-left text-xs"></i>
                        </button>
                        <button class="h-8 w-8 rounded-md bg-primary text-sm font-medium text-primary-foreground flex items-center justify-center">"1"</button>
                        <button class="h-8 w-8 rounded-md border border-border text-sm hover:bg-muted transition-colors flex items-center justify-center">"2"</button>
                        <button class="h-8 w-8 rounded-md border border-border text-sm hover:bg-muted transition-colors flex items-center justify-center">"3"</button>
                        <span class="h-8 w-8 flex items-center justify-center text-sm text-muted-foreground">"…"</span>
                        <button class="h-8 w-8 rounded-md border border-border text-sm hover:bg-muted transition-colors flex items-center justify-center">"8"</button>
                        <button class="h-8 w-8 rounded-md border border-border text-sm hover:bg-muted transition-colors flex items-center justify-center">
                            <i class="fas fa-chevron-right text-xs"></i>
                        </button>
                    </nav>
                    <div class="flex items-center justify-between text-sm text-muted-foreground">
                        <span>"Showing 1-10 of 80 results"</span>
                        <div class="flex items-center gap-2">
                            <button class="text-sm text-muted-foreground hover:text-foreground transition-colors">"Previous"</button>
                            <span class="text-muted-foreground/40">"|"</span>
                            <button class="text-sm text-primary font-medium">"Next"</button>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="steps" title="Steps" description="Multi-step progress indicator."
                code={r##"rsx! {
    <div class="flex items-center gap-2">
        <div class="flex items-center gap-2">
            <div class="h-8 w-8 rounded-full bg-primary flex items-center justify-center text-xs font-bold text-primary-foreground">"1"</div>
            <span class="text-sm font-medium">"Details"</span>
        </div>
        <div class="h-px w-8 bg-primary"></div>
        <div class="flex items-center gap-2">
            <div class="h-8 w-8 rounded-full bg-primary flex items-center justify-center text-xs font-bold text-primary-foreground">"2"</div>
            <span class="text-sm font-medium">"Payment"</span>
        </div>
        <div class="h-px w-8 bg-border"></div>
        <div class="flex items-center gap-2">
            <div class="h-8 w-8 rounded-full border-2 border-border flex items-center justify-center text-xs font-medium text-muted-foreground">"3"</div>
            <span class="text-sm text-muted-foreground">"Confirm"</span>
        </div>
    </div>
}"##}>
                <div class="w-full">
                    <div class="flex items-center">
                        <div class="flex items-center gap-2">
                            <div class="h-8 w-8 rounded-full bg-primary flex items-center justify-center text-xs font-bold text-primary-foreground">
                                <i class="fas fa-check text-[10px]"></i>
                            </div>
                            <span class="text-sm font-medium hidden sm:inline">"Account"</span>
                        </div>
                        <div class="h-px flex-1 mx-2 bg-primary"></div>
                        <div class="flex items-center gap-2">
                            <div class="h-8 w-8 rounded-full bg-primary flex items-center justify-center text-xs font-bold text-primary-foreground">"2"</div>
                            <span class="text-sm font-medium hidden sm:inline">"Billing"</span>
                        </div>
                        <div class="h-px flex-1 mx-2 bg-border"></div>
                        <div class="flex items-center gap-2">
                            <div class="h-8 w-8 rounded-full border-2 border-border flex items-center justify-center text-xs font-medium text-muted-foreground">"3"</div>
                            <span class="text-sm text-muted-foreground hidden sm:inline">"Review"</span>
                        </div>
                        <div class="h-px flex-1 mx-2 bg-border"></div>
                        <div class="flex items-center gap-2">
                            <div class="h-8 w-8 rounded-full border-2 border-border flex items-center justify-center text-xs font-medium text-muted-foreground">"4"</div>
                            <span class="text-sm text-muted-foreground hidden sm:inline">"Confirm"</span>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="simple-footer" title="Simple Footer" description="Minimal footer with copyright and links."
                code={r##"rsx! {
    <footer class="border-t border-border px-6 py-4 flex items-center justify-between">
        <p class="text-sm text-muted-foreground">"© 2026 Acme Inc"</p>
        <div class="flex gap-4">
            <a href="#" class="text-sm text-muted-foreground hover:text-foreground">"Privacy"</a>
            <a href="#" class="text-sm text-muted-foreground hover:text-foreground">"Terms"</a>
        </div>
    </footer>
}"##}>
                <div class="w-full rounded-lg border border-border overflow-hidden">
                    <footer class="bg-card px-6 py-4 flex items-center justify-between">
                        <p class="text-sm text-muted-foreground">"© 2026 Acme Inc. All rights reserved."</p>
                        <div class="flex gap-4">
                            <a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"Privacy"</a>
                            <a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"Terms"</a>
                            <a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"Contact"</a>
                        </div>
                    </footer>
                </div>
            </Showcase>

            <Showcase id="footer-columns" title="Footer with Columns" description="Multi-column footer with link groups."
                code={r##"rsx! {
    <footer class="border-t border-border px-6 py-8">
        <div class="grid grid-cols-2 md:grid-cols-4 gap-8">
            <div>
                <h4 class="text-sm font-semibold mb-3">"Product"</h4>
                <ul class="space-y-2">
                    <li><a href="#" class="text-sm text-muted-foreground hover:text-foreground">"Features"</a></li>
                    <li><a href="#" class="text-sm text-muted-foreground hover:text-foreground">"Pricing"</a></li>
                </ul>
            </div>
        </div>
    </footer>
}"##}>
                <div class="w-full rounded-lg border border-border overflow-hidden">
                    <footer class="bg-card px-6 py-8">
                        <div class="grid grid-cols-2 md:grid-cols-4 gap-8">
                            <div>
                                <h4 class="text-sm font-semibold mb-3">"Product"</h4>
                                <ul class="space-y-2">
                                    <li><a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"Features"</a></li>
                                    <li><a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"Pricing"</a></li>
                                    <li><a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"Changelog"</a></li>
                                </ul>
                            </div>
                            <div>
                                <h4 class="text-sm font-semibold mb-3">"Company"</h4>
                                <ul class="space-y-2">
                                    <li><a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"About"</a></li>
                                    <li><a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"Blog"</a></li>
                                    <li><a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"Careers"</a></li>
                                </ul>
                            </div>
                            <div>
                                <h4 class="text-sm font-semibold mb-3">"Resources"</h4>
                                <ul class="space-y-2">
                                    <li><a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"Docs"</a></li>
                                    <li><a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"Guides"</a></li>
                                    <li><a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"Support"</a></li>
                                </ul>
                            </div>
                            <div>
                                <h4 class="text-sm font-semibold mb-3">"Legal"</h4>
                                <ul class="space-y-2">
                                    <li><a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"Privacy"</a></li>
                                    <li><a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"Terms"</a></li>
                                    <li><a href="#" class="text-sm text-muted-foreground hover:text-foreground transition-colors">"License"</a></li>
                                </ul>
                            </div>
                        </div>
                        <div class="mt-8 pt-6 border-t border-border flex items-center justify-between">
                            <p class="text-sm text-muted-foreground">"© 2026 Acme Inc"</p>
                            <div class="flex gap-3">
                                <a href="#" class="text-muted-foreground hover:text-foreground transition-colors"><i class="fab fa-github"></i></a>
                                <a href="#" class="text-muted-foreground hover:text-foreground transition-colors"><i class="fab fa-twitter"></i></a>
                                <a href="#" class="text-muted-foreground hover:text-foreground transition-colors"><i class="fab fa-discord"></i></a>
                            </div>
                        </div>
                    </footer>
                </div>
            </Showcase>

            <Showcase id="bottom-nav" title="Bottom Navigation" description="Mobile-friendly bottom navigation bar."
                code={r##"rsx! {
    <nav class="flex items-center justify-around border-t border-border py-2 bg-card">
        <button class="flex flex-col items-center gap-0.5 text-primary">
            <i class="fas fa-home text-lg"></i>
            <span class="text-[10px] font-medium">"Home"</span>
        </button>
        <button class="flex flex-col items-center gap-0.5 text-muted-foreground">
            <i class="fas fa-search text-lg"></i>
            <span class="text-[10px]">"Search"</span>
        </button>
    </nav>
}"##}>
                <div class="w-full max-w-sm mx-auto rounded-lg border border-border overflow-hidden">
                    <nav class="flex items-center justify-around py-2 bg-card">
                        <button class="flex flex-col items-center gap-0.5 px-3 py-1 text-primary">
                            <i class="fas fa-home text-lg"></i>
                            <span class="text-[10px] font-medium">"Home"</span>
                        </button>
                        <button class="flex flex-col items-center gap-0.5 px-3 py-1 text-muted-foreground hover:text-foreground transition-colors">
                            <i class="fas fa-search text-lg"></i>
                            <span class="text-[10px]">"Search"</span>
                        </button>
                        <button class="flex flex-col items-center gap-0.5 px-3 py-1 text-muted-foreground hover:text-foreground transition-colors">
                            <i class="fas fa-plus-circle text-lg"></i>
                            <span class="text-[10px]">"Create"</span>
                        </button>
                        <button class="flex flex-col items-center gap-0.5 px-3 py-1 text-muted-foreground hover:text-foreground transition-colors relative">
                            <i class="fas fa-bell text-lg"></i>
                            <span class="absolute top-0 right-2 h-2 w-2 rounded-full bg-red-500"></span>
                            <span class="text-[10px]">"Alerts"</span>
                        </button>
                        <button class="flex flex-col items-center gap-0.5 px-3 py-1 text-muted-foreground hover:text-foreground transition-colors">
                            <i class="fas fa-user text-lg"></i>
                            <span class="text-[10px]">"Profile"</span>
                        </button>
                    </nav>
                </div>
            </Showcase>

            <Showcase id="link-group" title="Link Group" description="Grouped navigation links with icons."
                code={r##"rsx! {
    <div class="space-y-1">
        <a href="#" class="flex items-center gap-3 px-3 py-2 rounded-lg bg-muted text-sm font-medium">
            <i class="fas fa-home text-sm w-5 text-center"></i>"Dashboard"
        </a>
        <a href="#" class="flex items-center gap-3 px-3 py-2 rounded-lg text-sm text-muted-foreground hover:bg-muted/50">
            <i class="fas fa-chart-bar text-sm w-5 text-center"></i>"Analytics"
        </a>
    </div>
}"##}>
                <div class="w-full max-w-xs">
                    <div class="space-y-0.5">
                        <a href="#" class="flex items-center gap-3 px-3 py-2 rounded-lg bg-muted text-sm font-medium">
                            <i class="fas fa-home text-sm w-5 text-center text-primary"></i>"Dashboard"
                        </a>
                        <a href="#" class="flex items-center gap-3 px-3 py-2 rounded-lg text-sm text-muted-foreground hover:bg-muted/50 transition-colors">
                            <i class="fas fa-chart-bar text-sm w-5 text-center"></i>"Analytics"
                        </a>
                        <a href="#" class="flex items-center gap-3 px-3 py-2 rounded-lg text-sm text-muted-foreground hover:bg-muted/50 transition-colors">
                            <i class="fas fa-users text-sm w-5 text-center"></i>"Team"
                        </a>
                        <a href="#" class="flex items-center gap-3 px-3 py-2 rounded-lg text-sm text-muted-foreground hover:bg-muted/50 transition-colors">
                            <i class="fas fa-cog text-sm w-5 text-center"></i>"Settings"
                        </a>
                    </div>
                </div>
            </Showcase>

            <Showcase id="sidebar-nav" title="Sidebar Navigation" description="Full sidebar with sections and grouped links."
                code={r##"rsx! {
    <aside class="w-56 border-r border-border p-4">
        <div class="mb-4">
            <span class="text-xs font-semibold uppercase tracking-wider text-muted-foreground/60">"Overview"</span>
            <div class="mt-2 space-y-0.5">
                <a href="#" class="block px-3 py-1.5 text-sm rounded-md bg-muted font-medium">"Dashboard"</a>
                <a href="#" class="block px-3 py-1.5 text-sm rounded-md text-muted-foreground hover:bg-muted/50">"Analytics"</a>
            </div>
        </div>
    </aside>
}"##}>
                <div class="w-full max-w-xs rounded-lg border border-border overflow-hidden">
                    <aside class="bg-card p-4">
                        <div class="flex items-center gap-2 px-2 mb-4">
                            <div class="h-7 w-7 rounded-lg bg-primary flex items-center justify-center text-primary-foreground text-xs font-bold">"A"</div>
                            <span class="text-sm font-semibold">"Acme Inc"</span>
                        </div>
                        <div class="mb-4">
                            <span class="px-2 text-[10px] font-semibold uppercase tracking-wider text-muted-foreground/60">"Overview"</span>
                            <div class="mt-1.5 space-y-0.5">
                                <a href="#" class="flex items-center gap-2 px-2 py-1.5 text-sm rounded-md bg-primary/10 font-medium text-primary">
                                    <i class="fas fa-home text-xs w-4 text-center"></i>"Dashboard"
                                </a>
                                <a href="#" class="flex items-center gap-2 px-2 py-1.5 text-sm rounded-md text-muted-foreground hover:bg-muted/50 transition-colors">
                                    <i class="fas fa-chart-line text-xs w-4 text-center"></i>"Analytics"
                                </a>
                            </div>
                        </div>
                        <div class="mb-4">
                            <span class="px-2 text-[10px] font-semibold uppercase tracking-wider text-muted-foreground/60">"Manage"</span>
                            <div class="mt-1.5 space-y-0.5">
                                <a href="#" class="flex items-center gap-2 px-2 py-1.5 text-sm rounded-md text-muted-foreground hover:bg-muted/50 transition-colors">
                                    <i class="fas fa-users text-xs w-4 text-center"></i>"Team"
                                </a>
                                <a href="#" class="flex items-center gap-2 px-2 py-1.5 text-sm rounded-md text-muted-foreground hover:bg-muted/50 transition-colors">
                                    <i class="fas fa-folder text-xs w-4 text-center"></i>"Projects"
                                </a>
                                <a href="#" class="flex items-center gap-2 px-2 py-1.5 text-sm rounded-md text-muted-foreground hover:bg-muted/50 transition-colors">
                                    <i class="fas fa-cog text-xs w-4 text-center"></i>"Settings"
                                </a>
                            </div>
                        </div>
                    </aside>
                </div>
            </Showcase>

            <div class="mt-8 flex items-center justify-between border-t border-border pt-6">
                <a href={docs_href("/ui/inputs")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "← Forms & Inputs"
                </a>
                <a href={docs_href("/ui/data-display")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "Data Display →"
                </a>
            </div>
        </article>
    }
}
