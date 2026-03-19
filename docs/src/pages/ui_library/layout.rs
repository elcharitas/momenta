#![allow(unused_imports)]
use crate::components::*;
use alloc::{format, vec, vec::Vec};
use momenta::prelude::*;

#[component]
pub fn LayoutPage() -> Node {
    rsx! {
        <article class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <CategoryHeader title="Layout" description="Structural components for building page layouts, grids, and organizing content." count={8} />

            <Showcase id="container" title="Container" description="Centered container with max-width constraint."
                code={r#"rsx! {
    <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="py-8">
            <h2 class="text-2xl font-bold">"Page Content"</h2>
            <p class="text-muted-foreground mt-2">"Centered container with responsive padding."</p>
        </div>
    </div>
}"#}>
                <div class="w-full">
                    <div class="border border-dashed border-primary/30 rounded-lg">
                        <div class="max-w-lg mx-auto px-4 sm:px-6">
                            <div class="py-6 text-center">
                                <h2 class="text-lg font-bold">"Centered Container"</h2>
                                <p class="text-sm text-muted-foreground mt-1">"Content is constrained to a max-width and centered."</p>
                            </div>
                        </div>
                    </div>
                    <p class="text-xs text-muted-foreground text-center mt-2">"Dashed border shows the full-width area. Content is centered within."</p>
                </div>
            </Showcase>

            <Showcase id="two-column" title="Two Column" description="Side-by-side columns with responsive stacking."
                code={r#"rsx! {
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div class="p-4 bg-muted/30 rounded-lg">"Left Column"</div>
        <div class="p-4 bg-muted/30 rounded-lg">"Right Column"</div>
    </div>
}"#}>
                <div class="w-full grid grid-cols-1 sm:grid-cols-2 gap-4">
                    <div class="p-6 bg-primary/5 border border-primary/20 rounded-lg flex flex-col items-center justify-center min-h-[80px]">
                        <i class="fas fa-columns text-primary mb-2"></i>
                        <span class="text-sm font-medium">"Left Column"</span>
                    </div>
                    <div class="p-6 bg-primary/5 border border-primary/20 rounded-lg flex flex-col items-center justify-center min-h-[80px]">
                        <i class="fas fa-columns text-primary mb-2"></i>
                        <span class="text-sm font-medium">"Right Column"</span>
                    </div>
                </div>
            </Showcase>

            <Showcase id="three-column" title="Three Column Grid" description="Three-column grid layout for content organization."
                code={r#"rsx! {
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div class="p-4 bg-muted/30 rounded-lg">"Column 1"</div>
        <div class="p-4 bg-muted/30 rounded-lg">"Column 2"</div>
        <div class="p-4 bg-muted/30 rounded-lg">"Column 3"</div>
    </div>
}"#}>
                <div class="w-full grid grid-cols-1 sm:grid-cols-3 gap-3">
                    <div class="p-4 bg-blue-500/5 border border-blue-500/20 rounded-lg text-center min-h-[70px] flex items-center justify-center">
                        <span class="text-sm font-medium">"Column 1"</span>
                    </div>
                    <div class="p-4 bg-green-500/5 border border-green-500/20 rounded-lg text-center min-h-[70px] flex items-center justify-center">
                        <span class="text-sm font-medium">"Column 2"</span>
                    </div>
                    <div class="p-4 bg-violet-500/5 border border-violet-500/20 rounded-lg text-center min-h-[70px] flex items-center justify-center">
                        <span class="text-sm font-medium">"Column 3"</span>
                    </div>
                </div>
            </Showcase>

            <Showcase id="flex-row" title="Flex Row" description="Horizontal flex layout with various alignments."
                code={r#"rsx! {
    <div class="flex items-center justify-between gap-4">
        <div class="p-3 bg-muted rounded">"Start"</div>
        <div class="p-3 bg-muted rounded">"Center"</div>
        <div class="p-3 bg-muted rounded">"End"</div>
    </div>
}"#}>
                <div class="w-full space-y-4">
                    <div>
                        <p class="text-xs text-muted-foreground mb-2">"justify-between"</p>
                        <div class="flex items-center justify-between gap-2 p-3 bg-muted/30 rounded-lg border border-dashed border-border">
                            <div class="px-3 py-1.5 bg-primary/10 rounded text-xs font-medium">"Start"</div>
                            <div class="px-3 py-1.5 bg-primary/10 rounded text-xs font-medium">"Center"</div>
                            <div class="px-3 py-1.5 bg-primary/10 rounded text-xs font-medium">"End"</div>
                        </div>
                    </div>
                    <div>
                        <p class="text-xs text-muted-foreground mb-2">"justify-center"</p>
                        <div class="flex items-center justify-center gap-2 p-3 bg-muted/30 rounded-lg border border-dashed border-border">
                            <div class="px-3 py-1.5 bg-primary/10 rounded text-xs font-medium">"A"</div>
                            <div class="px-3 py-1.5 bg-primary/10 rounded text-xs font-medium">"B"</div>
                            <div class="px-3 py-1.5 bg-primary/10 rounded text-xs font-medium">"C"</div>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="stack" title="Vertical Stack" description="Vertically stacked elements with consistent spacing."
                code={r#"rsx! {
    <div class="space-y-4">
        <div class="p-4 bg-muted/30 rounded-lg">"Item 1"</div>
        <div class="p-4 bg-muted/30 rounded-lg">"Item 2"</div>
        <div class="p-4 bg-muted/30 rounded-lg">"Item 3"</div>
    </div>
}"#}>
                <div class="w-full max-w-sm space-y-3">
                    <div class="p-3 bg-primary/5 border border-primary/20 rounded-lg flex items-center gap-3">
                        <div class="h-8 w-8 rounded bg-primary/10 flex items-center justify-center shrink-0"><span class="text-xs font-bold text-primary">"1"</span></div>
                        <span class="text-sm font-medium">"First stack item"</span>
                    </div>
                    <div class="p-3 bg-primary/5 border border-primary/20 rounded-lg flex items-center gap-3">
                        <div class="h-8 w-8 rounded bg-primary/10 flex items-center justify-center shrink-0"><span class="text-xs font-bold text-primary">"2"</span></div>
                        <span class="text-sm font-medium">"Second stack item"</span>
                    </div>
                    <div class="p-3 bg-primary/5 border border-primary/20 rounded-lg flex items-center gap-3">
                        <div class="h-8 w-8 rounded bg-primary/10 flex items-center justify-center shrink-0"><span class="text-xs font-bold text-primary">"3"</span></div>
                        <span class="text-sm font-medium">"Third stack item"</span>
                    </div>
                </div>
            </Showcase>

            <Showcase id="divider" title="Divider" description="Horizontal and vertical dividers for separating content."
                code={r#"rsx! {
    <div class="space-y-4">
        <p>"Content above"</p>
        <hr class="border-border" />
        <p>"Content below"</p>
    </div>
    // With text
    <div class="relative">
        <div class="absolute inset-0 flex items-center">
            <span class="w-full border-t border-border"></span>
        </div>
        <div class="relative flex justify-center text-xs uppercase">
            <span class="bg-background px-2 text-muted-foreground">"or"</span>
        </div>
    </div>
}"#}>
                <div class="w-full space-y-6">
                    <div>
                        <p class="text-sm mb-3">"Standard divider"</p>
                        <hr class="border-border" />
                        <p class="text-sm mt-3 text-muted-foreground">"Content below the divider"</p>
                    </div>
                    <div class="relative py-2">
                        <div class="absolute inset-0 flex items-center">
                            <span class="w-full border-t border-border"></span>
                        </div>
                        <div class="relative flex justify-center text-xs uppercase">
                            <span class="bg-background px-3 text-muted-foreground">"or continue with"</span>
                        </div>
                    </div>
                    <div class="flex items-center gap-4 h-10">
                        <span class="text-sm">"Left"</span>
                        <div class="h-full w-px bg-border"></div>
                        <span class="text-sm">"Center"</span>
                        <div class="h-full w-px bg-border"></div>
                        <span class="text-sm">"Right"</span>
                    </div>
                </div>
            </Showcase>

            <Showcase id="grid-auto" title="Auto Grid" description="Responsive grid that automatically adjusts columns."
                code={r#"rsx! {
    <div class="grid grid-cols-[repeat(auto-fill,minmax(150px,1fr))] gap-4">
        <div class="p-4 bg-muted/30 rounded-lg text-center">"Item"</div>
    </div>
}"#}>
                <div class="w-full grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
                    <div class="p-4 bg-muted/30 rounded-lg text-center border border-border/50 aspect-square flex items-center justify-center">
                        <span class="text-xs font-medium text-muted-foreground">"1"</span>
                    </div>
                    <div class="p-4 bg-muted/30 rounded-lg text-center border border-border/50 aspect-square flex items-center justify-center">
                        <span class="text-xs font-medium text-muted-foreground">"2"</span>
                    </div>
                    <div class="p-4 bg-muted/30 rounded-lg text-center border border-border/50 aspect-square flex items-center justify-center">
                        <span class="text-xs font-medium text-muted-foreground">"3"</span>
                    </div>
                    <div class="p-4 bg-muted/30 rounded-lg text-center border border-border/50 aspect-square flex items-center justify-center">
                        <span class="text-xs font-medium text-muted-foreground">"4"</span>
                    </div>
                    <div class="p-4 bg-muted/30 rounded-lg text-center border border-border/50 aspect-square flex items-center justify-center">
                        <span class="text-xs font-medium text-muted-foreground">"5"</span>
                    </div>
                    <div class="p-4 bg-muted/30 rounded-lg text-center border border-border/50 aspect-square flex items-center justify-center">
                        <span class="text-xs font-medium text-muted-foreground">"6"</span>
                    </div>
                </div>
            </Showcase>

            <Showcase id="sidebar-layout" title="Sidebar Layout" description="Page layout with fixed sidebar and main content area."
                code={r##"rsx! {
    <div class="flex gap-6">
        <aside class="w-48 shrink-0">
            <nav class="space-y-1">
                <a href="#" class="block px-3 py-2 text-sm rounded-md bg-primary/10 text-primary font-medium">"Dashboard"</a>
                <a href="#" class="block px-3 py-2 text-sm rounded-md text-muted-foreground hover:bg-muted">"Settings"</a>
            </nav>
        </aside>
        <main class="flex-1">
            <p>"Main content area."</p>
        </main>
    </div>
}"##}>
                <div class="w-full flex gap-4 overflow-x-auto">
                    <aside class="w-36 sm:w-44 shrink-0">
                        <nav class="space-y-1">
                            <div class="px-3 py-2 text-xs rounded-md bg-primary/10 text-primary font-medium">"Dashboard"</div>
                            <div class="px-3 py-2 text-xs rounded-md text-muted-foreground hover:bg-muted">"Projects"</div>
                            <div class="px-3 py-2 text-xs rounded-md text-muted-foreground hover:bg-muted">"Team"</div>
                            <div class="px-3 py-2 text-xs rounded-md text-muted-foreground hover:bg-muted">"Settings"</div>
                        </nav>
                    </aside>
                    <main class="flex-1 min-w-0">
                        <div class="rounded-lg border border-dashed border-border p-6 min-h-[120px] flex items-center justify-center">
                            <div class="text-center">
                                <i class="fas fa-th-large text-2xl text-muted-foreground/50 mb-2"></i>
                                <p class="text-sm text-muted-foreground">"Main Content Area"</p>
                            </div>
                        </div>
                    </main>
                </div>
            </Showcase>

            <div class="mt-8 flex items-center justify-between border-t border-border pt-6">
                <a href={docs_href("/ui/data-display")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "← Data Display"
                </a>
                <a href={docs_href("/ui/feedback")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "Feedback →"
                </a>
            </div>
        </article>
    }
}
