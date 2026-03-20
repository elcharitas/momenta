#![allow(unused_imports)]
use crate::components::*;
use alloc::{format, vec, vec::Vec};
use momenta::prelude::*;

#[component]
pub fn OverlaysPage() -> Node {
    rsx! {
        <article class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <CategoryHeader title="Overlays" description="Modal dialogs, drawers, sheets and overlay panels for focused interactions." count={6} />

            <Showcase id="modal" title="Modal Dialog" description="Centered overlay dialog with backdrop."
                code={r#"rsx! {
    // Backdrop
    <div class="fixed inset-0 bg-black/50 backdrop-blur-sm z-50 flex items-center justify-center">
        <div class="bg-background rounded-xl shadow-xl border border-border w-full max-w-md p-6">
            <h3 class="text-lg font-semibold mb-2">"Modal Title"</h3>
            <p class="text-sm text-muted-foreground mb-6">"This is the modal content."</p>
            <div class="flex justify-end gap-3">
                <button class="px-4 py-2 text-sm border border-border rounded-lg">"Cancel"</button>
                <button class="px-4 py-2 text-sm bg-primary text-primary-foreground rounded-lg">"Confirm"</button>
            </div>
        </div>
    </div>
}"#}>
                <div class="w-full">
                    <div class="relative rounded-xl bg-muted/30 border border-border p-8 flex items-center justify-center min-h-[250px]">
                        <div class="absolute inset-0 bg-black/20 dark:bg-black/40 rounded-xl"></div>
                        <div class="relative bg-background rounded-xl shadow-xl border border-border w-full max-w-sm p-6 z-10">
                            <div class="flex items-center justify-between mb-4">
                                <h3 class="text-base font-semibold">"Delete Project"</h3>
                                <button class="h-7 w-7 rounded-md hover:bg-muted flex items-center justify-center text-muted-foreground">
                                    <i class="fas fa-times text-xs"></i>
                                </button>
                            </div>
                            <p class="text-sm text-muted-foreground mb-6">"Are you sure you want to delete this project? This action cannot be undone and all data will be permanently lost."</p>
                            <div class="flex justify-end gap-3">
                                <button class="px-4 py-2 text-sm border border-border rounded-lg hover:bg-muted transition-colors">"Cancel"</button>
                                <button class="px-4 py-2 text-sm bg-red-500 text-white rounded-lg hover:bg-red-600 transition-colors">"Delete"</button>
                            </div>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="confirmation" title="Confirmation Dialog" description="Simple yes/no confirmation overlay."
                code={r#"rsx! {
    <div class="bg-background rounded-xl shadow-xl border border-border max-w-sm p-6 text-center">
        <div class="h-12 w-12 rounded-full bg-amber-500/10 mx-auto mb-4 flex items-center justify-center">
            <i class="fas fa-exclamation-triangle text-xl text-amber-500"></i>
        </div>
        <h3 class="text-lg font-semibold mb-2">"Are you sure?"</h3>
        <p class="text-sm text-muted-foreground mb-6">"This will discard your unsaved changes."</p>
        <div class="flex gap-3">
            <button class="flex-1 px-4 py-2 text-sm border border-border rounded-lg">"Cancel"</button>
            <button class="flex-1 px-4 py-2 text-sm bg-amber-500 text-white rounded-lg">"Discard"</button>
        </div>
    </div>
}"#}>
                <div class="w-full flex items-center justify-center">
                    <div class="bg-background rounded-xl shadow-lg border border-border max-w-xs p-6 text-center">
                        <div class="h-12 w-12 rounded-full bg-amber-500/10 mx-auto mb-4 flex items-center justify-center">
                            <i class="fas fa-exclamation-triangle text-xl text-amber-500"></i>
                        </div>
                        <h3 class="text-base font-semibold mb-2">"Discard changes?"</h3>
                        <p class="text-sm text-muted-foreground mb-5">"You have unsaved changes. Are you sure you want to leave this page?"</p>
                        <div class="flex gap-3">
                            <button class="flex-1 px-3 py-2 text-sm border border-border rounded-lg hover:bg-muted transition-colors">"Stay"</button>
                            <button class="flex-1 px-3 py-2 text-sm bg-amber-500 text-white rounded-lg hover:bg-amber-600 transition-colors">"Discard"</button>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="drawer" title="Drawer / Slide-Over" description="Side panel that slides in from the right."
                code={r#"rsx! {
    <div class="fixed inset-0 z-50 flex">
        <div class="flex-1 bg-black/50"></div>
        <div class="w-80 bg-background border-l border-border p-6 shadow-xl">
            <div class="flex items-center justify-between mb-6">
                <h3 class="text-lg font-semibold">"Notifications"</h3>
                <button class="h-7 w-7 rounded-md hover:bg-muted flex items-center justify-center">
                    <i class="fas fa-times"></i>
                </button>
            </div>
            <div class="space-y-4">
                // Notification items
            </div>
        </div>
    </div>
}"#}>
                <div class="w-full">
                    <div class="relative rounded-xl border border-border overflow-hidden flex min-h-[280px]">
                        <div class="flex-1 bg-muted/20 p-4 flex items-center justify-center">
                            <p class="text-sm text-muted-foreground">"Page Content"</p>
                        </div>
                        <div class="w-64 bg-background border-l border-border p-4 shadow-lg">
                            <div class="flex items-center justify-between mb-4">
                                <h3 class="text-sm font-semibold">"Notifications"</h3>
                                <button class="h-6 w-6 rounded hover:bg-muted flex items-center justify-center text-muted-foreground">
                                    <i class="fas fa-times text-[10px]"></i>
                                </button>
                            </div>
                            <div class="space-y-3">
                                <div class="p-2.5 rounded-lg bg-primary/5 border border-primary/10">
                                    <p class="text-xs font-medium">"New deployment"</p>
                                    <p class="text-[10px] text-muted-foreground mt-0.5">"2 minutes ago"</p>
                                </div>
                                <div class="p-2.5 rounded-lg bg-muted/50">
                                    <p class="text-xs font-medium">"Build completed"</p>
                                    <p class="text-[10px] text-muted-foreground mt-0.5">"15 minutes ago"</p>
                                </div>
                                <div class="p-2.5 rounded-lg bg-muted/50">
                                    <p class="text-xs font-medium">"New team member"</p>
                                    <p class="text-[10px] text-muted-foreground mt-0.5">"1 hour ago"</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="sheet" title="Bottom Sheet" description="Panel that slides up from the bottom."
                code={r#"rsx! {
    <div class="fixed inset-0 z-50 flex flex-col">
        <div class="flex-1 bg-black/50"></div>
        <div class="bg-background border-t border-border rounded-t-xl p-6 shadow-xl">
            <div class="w-10 h-1 rounded-full bg-muted mx-auto mb-4"></div>
            <h3 class="text-lg font-semibold mb-4">"Share"</h3>
            <div class="grid grid-cols-4 gap-4">
                // Share options
            </div>
        </div>
    </div>
}"#}>
                <div class="w-full max-w-sm mx-auto">
                    <div class="relative rounded-xl border border-border overflow-hidden min-h-[300px] flex flex-col">
                        <div class="flex-1 bg-muted/20 p-4 flex items-center justify-center">
                            <p class="text-sm text-muted-foreground">"Page Content"</p>
                        </div>
                        <div class="bg-background border-t border-border rounded-t-xl p-4 shadow-lg">
                            <div class="w-8 h-1 rounded-full bg-muted mx-auto mb-3"></div>
                            <h3 class="text-sm font-semibold mb-3">"Share this project"</h3>
                            <div class="grid grid-cols-4 gap-3">
                                <div class="flex flex-col items-center gap-1">
                                    <div class="h-10 w-10 rounded-full bg-blue-500/10 flex items-center justify-center"><i class="fab fa-twitter text-sm text-blue-500"></i></div>
                                    <span class="text-[10px] text-muted-foreground">"Twitter"</span>
                                </div>
                                <div class="flex flex-col items-center gap-1">
                                    <div class="h-10 w-10 rounded-full bg-violet-500/10 flex items-center justify-center"><i class="fab fa-github text-sm text-violet-500"></i></div>
                                    <span class="text-[10px] text-muted-foreground">"GitHub"</span>
                                </div>
                                <div class="flex flex-col items-center gap-1">
                                    <div class="h-10 w-10 rounded-full bg-green-500/10 flex items-center justify-center"><i class="fas fa-link text-sm text-green-500"></i></div>
                                    <span class="text-[10px] text-muted-foreground">"Copy Link"</span>
                                </div>
                                <div class="flex flex-col items-center gap-1">
                                    <div class="h-10 w-10 rounded-full bg-amber-500/10 flex items-center justify-center"><i class="fas fa-envelope text-sm text-amber-500"></i></div>
                                    <span class="text-[10px] text-muted-foreground">"Email"</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="command-palette" title="Command Palette" description="Keyboard-driven command search overlay."
                code={r#"rsx! {
    <div class="fixed inset-0 bg-black/50 z-50 flex items-start justify-center pt-[20vh]">
        <div class="bg-background rounded-xl shadow-2xl border border-border w-full max-w-lg overflow-hidden">
            <div class="flex items-center gap-3 px-4 py-3 border-b border-border">
                <i class="fas fa-search text-muted-foreground"></i>
                <input type="text" placeholder="Type a command..." class="bg-transparent outline-none flex-1 text-sm" />
                <kbd class="px-2 py-0.5 text-xs bg-muted rounded border border-border">"Esc"</kbd>
            </div>
            <div class="py-2 max-h-64 overflow-y-auto">
                // Command items
            </div>
        </div>
    </div>
}"#}>
                <div class="w-full flex items-center justify-center">
                    <div class="bg-background rounded-xl shadow-lg border border-border w-full max-w-md overflow-hidden">
                        <div class="flex items-center gap-3 px-4 py-3 border-b border-border">
                            <i class="fas fa-search text-sm text-muted-foreground"></i>
                            <span class="text-sm text-muted-foreground">"Search commands..."</span>
                            <div class="ml-auto">
                                <kbd class="px-1.5 py-0.5 text-[10px] bg-muted rounded border border-border font-mono">"Esc"</kbd>
                            </div>
                        </div>
                        <div class="py-1">
                            <div class="px-2 py-1">
                                <p class="text-[10px] text-muted-foreground font-medium px-2 py-1">"Suggestions"</p>
                            </div>
                            <div class="px-2">
                                <div class="flex items-center gap-3 px-2 py-2 rounded-md bg-primary/5">
                                    <i class="fas fa-file text-xs text-primary w-4"></i>
                                    <span class="text-sm">"New File"</span>
                                    <kbd class="ml-auto px-1.5 py-0.5 text-[10px] bg-muted rounded border border-border font-mono">"Ctrl+N"</kbd>
                                </div>
                                <div class="flex items-center gap-3 px-2 py-2 rounded-md hover:bg-muted">
                                    <i class="fas fa-search text-xs text-muted-foreground w-4"></i>
                                    <span class="text-sm">"Find in Files"</span>
                                    <kbd class="ml-auto px-1.5 py-0.5 text-[10px] bg-muted rounded border border-border font-mono">"Ctrl+Shift+F"</kbd>
                                </div>
                                <div class="flex items-center gap-3 px-2 py-2 rounded-md hover:bg-muted">
                                    <i class="fas fa-cog text-xs text-muted-foreground w-4"></i>
                                    <span class="text-sm">"Settings"</span>
                                    <kbd class="ml-auto px-1.5 py-0.5 text-[10px] bg-muted rounded border border-border font-mono">"Ctrl+,"</kbd>
                                </div>
                                <div class="flex items-center gap-3 px-2 py-2 rounded-md hover:bg-muted">
                                    <i class="fas fa-terminal text-xs text-muted-foreground w-4"></i>
                                    <span class="text-sm">"Toggle Terminal"</span>
                                    <kbd class="ml-auto px-1.5 py-0.5 text-[10px] bg-muted rounded border border-border font-mono">"Ctrl+`"</kbd>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="notification-panel" title="Notification Panel" description="Floating notification/toast panel."
                code={r#"rsx! {
    <div class="fixed top-4 right-4 w-80 space-y-2 z-50">
        <div class="bg-background rounded-lg shadow-lg border border-border p-4 flex items-start gap-3">
            <i class="fas fa-check-circle text-green-500 mt-0.5"></i>
            <div class="flex-1">
                <p class="text-sm font-medium">"Success"</p>
                <p class="text-xs text-muted-foreground">"Your changes have been saved."</p>
            </div>
            <button><i class="fas fa-times text-xs text-muted-foreground"></i></button>
        </div>
    </div>
}"#}>
                <div class="w-full max-w-sm ml-auto space-y-2">
                    <div class="bg-background rounded-lg shadow-md border border-border p-3 flex items-start gap-3">
                        <i class="fas fa-check-circle text-green-500 mt-0.5"></i>
                        <div class="flex-1 min-w-0">
                            <p class="text-sm font-medium">"Changes saved"</p>
                            <p class="text-xs text-muted-foreground mt-0.5">"Your project settings have been updated."</p>
                        </div>
                        <button class="h-5 w-5 rounded hover:bg-muted flex items-center justify-center shrink-0">
                            <i class="fas fa-times text-[10px] text-muted-foreground"></i>
                        </button>
                    </div>
                    <div class="bg-background rounded-lg shadow-md border border-border p-3 flex items-start gap-3">
                        <i class="fas fa-info-circle text-blue-500 mt-0.5"></i>
                        <div class="flex-1 min-w-0">
                            <p class="text-sm font-medium">"New version available"</p>
                            <p class="text-xs text-muted-foreground mt-0.5">"Momenta v2.1.0 is now available."</p>
                        </div>
                        <button class="h-5 w-5 rounded hover:bg-muted flex items-center justify-center shrink-0">
                            <i class="fas fa-times text-[10px] text-muted-foreground"></i>
                        </button>
                    </div>
                    <div class="bg-background rounded-lg shadow-md border border-red-500/20 p-3 flex items-start gap-3">
                        <i class="fas fa-exclamation-circle text-red-500 mt-0.5"></i>
                        <div class="flex-1 min-w-0">
                            <p class="text-sm font-medium">"Build failed"</p>
                            <p class="text-xs text-muted-foreground mt-0.5">"Error in line 42: unexpected token."</p>
                        </div>
                        <button class="h-5 w-5 rounded hover:bg-muted flex items-center justify-center shrink-0">
                            <i class="fas fa-times text-[10px] text-muted-foreground"></i>
                        </button>
                    </div>
                </div>
            </Showcase>

            <div class="mt-8 flex items-center justify-between border-t border-border pt-6">
                <a href={docs_href("/ui/feedback")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "← Feedback"
                </a>
                <a href={docs_href("/ui/marketing")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "Marketing →"
                </a>
            </div>
        </article>
    }
}
