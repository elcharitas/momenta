#![allow(unused_imports)]
use crate::components::*;
use alloc::{format, vec, vec::Vec};
use momenta::prelude::*;

#[component]
pub fn FeedbackPage() -> Node {
    rsx! {
        <article class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <CategoryHeader title="Feedback" description="Components that communicate status, progress, and loading states to users." count={8} />

            <Showcase id="progress-bar" title="Progress Bar" description="Linear progress indicator with customizable colors."
                code={r#"rsx! {
    <div class="space-y-3">
        <div class="flex justify-between text-sm">
            <span>"Uploading..."</span>
            <span class="text-muted-foreground">"65%"</span>
        </div>
        <div class="h-2 rounded-full bg-muted overflow-hidden">
            <div class="h-full rounded-full bg-primary transition-all duration-300" style="width: 65%"></div>
        </div>
    </div>
}"#}>
                <div class="w-full space-y-5">
                    <div>
                        <div class="flex justify-between text-sm mb-1.5">
                            <span class="font-medium">"Uploading..."</span>
                            <span class="text-muted-foreground">"65%"</span>
                        </div>
                        <div class="h-2 rounded-full bg-muted overflow-hidden">
                            <div class="h-full rounded-full bg-primary" style="width: 65%"></div>
                        </div>
                    </div>
                    <div>
                        <div class="flex justify-between text-sm mb-1.5">
                            <span class="font-medium">"Processing..."</span>
                            <span class="text-muted-foreground">"40%"</span>
                        </div>
                        <div class="h-2 rounded-full bg-muted overflow-hidden">
                            <div class="h-full rounded-full bg-amber-500" style="width: 40%"></div>
                        </div>
                    </div>
                    <div>
                        <div class="flex justify-between text-sm mb-1.5">
                            <span class="font-medium">"Complete!"</span>
                            <span class="text-muted-foreground">"100%"</span>
                        </div>
                        <div class="h-2 rounded-full bg-muted overflow-hidden">
                            <div class="h-full rounded-full bg-green-500" style="width: 100%"></div>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="multi-progress" title="Multiple Progress Bars" description="Stacked or segmented progress bars."
                code={r#"rsx! {
    <div class="h-3 rounded-full bg-muted overflow-hidden flex">
        <div class="h-full bg-primary" style="width: 45%"></div>
        <div class="h-full bg-amber-500" style="width: 20%"></div>
        <div class="h-full bg-green-500" style="width: 15%"></div>
    </div>
}"#}>
                <div class="w-full space-y-4">
                    <div class="h-3 rounded-full bg-muted overflow-hidden flex">
                        <div class="h-full bg-blue-500" style="width: 45%"></div>
                        <div class="h-full bg-amber-500" style="width: 20%"></div>
                        <div class="h-full bg-green-500" style="width: 15%"></div>
                    </div>
                    <div class="flex gap-4 text-xs text-muted-foreground">
                        <div class="flex items-center gap-1.5">
                            <div class="h-2.5 w-2.5 rounded-sm bg-blue-500"></div>
                            <span>"Rust 45%"</span>
                        </div>
                        <div class="flex items-center gap-1.5">
                            <div class="h-2.5 w-2.5 rounded-sm bg-amber-500"></div>
                            <span>"WASM 20%"</span>
                        </div>
                        <div class="flex items-center gap-1.5">
                            <div class="h-2.5 w-2.5 rounded-sm bg-green-500"></div>
                            <span>"JS 15%"</span>
                        </div>
                        <div class="flex items-center gap-1.5">
                            <div class="h-2.5 w-2.5 rounded-sm bg-muted"></div>
                            <span>"Other 20%"</span>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="spinner" title="Spinner" description="Animated loading spinner."
                code={r#"rsx! {
    <div class="flex items-center gap-2">
        <svg class="animate-spin h-5 w-5 text-primary" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke_width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
        </svg>
        <span class="text-sm">"Loading..."</span>
    </div>
}"#}>
                <div class="flex items-center gap-8">
                    <div class="flex flex-col items-center gap-2">
                        <svg class="animate-spin h-5 w-5 text-primary" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke_width="4"></circle>
                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
                        </svg>
                        <span class="text-xs text-muted-foreground">"Small"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <svg class="animate-spin h-8 w-8 text-primary" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke_width="4"></circle>
                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
                        </svg>
                        <span class="text-xs text-muted-foreground">"Medium"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <svg class="animate-spin h-12 w-12 text-primary" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke_width="4"></circle>
                            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
                        </svg>
                        <span class="text-xs text-muted-foreground">"Large"</span>
                    </div>
                    <div class="flex flex-col items-center gap-2">
                        <div class="flex items-center gap-2">
                            <svg class="animate-spin h-4 w-4 text-primary" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke_width="4"></circle>
                                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
                            </svg>
                            <span class="text-sm">"Loading..."</span>
                        </div>
                        <span class="text-xs text-muted-foreground">"With text"</span>
                    </div>
                </div>
            </Showcase>

            <Showcase id="skeleton" title="Skeleton Loader" description="Placeholder animations for loading content."
                code={r#"rsx! {
    <div class="space-y-3 animate-pulse">
        <div class="h-4 bg-muted rounded w-3/4"></div>
        <div class="h-4 bg-muted rounded w-full"></div>
        <div class="h-4 bg-muted rounded w-5/6"></div>
    </div>
}"#}>
                <div class="w-full space-y-6">
                    <div class="flex items-center gap-4 animate-pulse">
                        <div class="h-12 w-12 bg-muted rounded-full shrink-0"></div>
                        <div class="flex-1 space-y-2">
                            <div class="h-4 bg-muted rounded w-1/3"></div>
                            <div class="h-3 bg-muted rounded w-1/2"></div>
                        </div>
                    </div>
                    <div class="space-y-3 animate-pulse">
                        <div class="h-4 bg-muted rounded w-3/4"></div>
                        <div class="h-4 bg-muted rounded w-full"></div>
                        <div class="h-4 bg-muted rounded w-5/6"></div>
                    </div>
                    <div class="grid grid-cols-3 gap-3 animate-pulse">
                        <div class="h-20 bg-muted rounded-lg"></div>
                        <div class="h-20 bg-muted rounded-lg"></div>
                        <div class="h-20 bg-muted rounded-lg"></div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="empty-state" title="Empty State" description="Placeholder for when there's no content to display."
                code={r#"rsx! {
    <div class="flex flex-col items-center justify-center py-12 text-center">
        <i class="fas fa-inbox text-4xl text-muted-foreground/50 mb-4"></i>
        <h3 class="text-lg font-semibold mb-1">"No items found"</h3>
        <p class="text-sm text-muted-foreground mb-4">"Get started by creating a new item."</p>
        <button class="px-4 py-2 bg-primary text-primary-foreground text-sm rounded-lg">"Create Item"</button>
    </div>
}"#}>
                <div class="w-full flex flex-col items-center justify-center py-8 text-center">
                    <div class="h-16 w-16 rounded-full bg-muted/50 flex items-center justify-center mb-4">
                        <i class="fas fa-inbox text-2xl text-muted-foreground/50"></i>
                    </div>
                    <h3 class="text-base font-semibold mb-1">"No projects yet"</h3>
                    <p class="text-sm text-muted-foreground mb-4 max-w-xs">"Get started by creating your first project. You can add components and templates."</p>
                    <button class="px-4 py-2 bg-primary text-primary-foreground text-sm rounded-lg font-medium hover:bg-primary/90 transition-colors">
                        <i class="fas fa-plus mr-2 text-xs"></i>"Create Project"
                    </button>
                </div>
            </Showcase>

            <Showcase id="error-state" title="Error State" description="Error message with retry option."
                code={r#"rsx! {
    <div class="flex flex-col items-center justify-center py-12 text-center">
        <i class="fas fa-exclamation-triangle text-4xl text-red-500/50 mb-4"></i>
        <h3 class="text-lg font-semibold mb-1">"Something went wrong"</h3>
        <p class="text-sm text-muted-foreground mb-4">"We couldn't load the data."</p>
        <button class="px-4 py-2 bg-red-500 text-white text-sm rounded-lg">"Retry"</button>
    </div>
}"#}>
                <div class="w-full flex flex-col items-center justify-center py-8 text-center">
                    <div class="h-16 w-16 rounded-full bg-red-500/10 flex items-center justify-center mb-4">
                        <i class="fas fa-exclamation-triangle text-2xl text-red-500"></i>
                    </div>
                    <h3 class="text-base font-semibold mb-1">"Failed to load data"</h3>
                    <p class="text-sm text-muted-foreground mb-1">"Error: Connection timed out"</p>
                    <p class="text-xs text-muted-foreground mb-4">"Please check your connection and try again."</p>
                    <div class="flex gap-3">
                        <button class="px-4 py-2 border border-border text-sm rounded-lg font-medium hover:bg-muted transition-colors">"Go Back"</button>
                        <button class="px-4 py-2 bg-red-500 text-white text-sm rounded-lg font-medium hover:bg-red-600 transition-colors">
                            <i class="fas fa-redo mr-2 text-xs"></i>"Retry"
                        </button>
                    </div>
                </div>
            </Showcase>

            <Showcase id="success-state" title="Success State" description="Confirmation message after a completed action."
                code={r#"rsx! {
    <div class="flex flex-col items-center justify-center py-12 text-center">
        <div class="h-16 w-16 rounded-full bg-green-500/10 flex items-center justify-center mb-4">
            <i class="fas fa-check text-2xl text-green-500"></i>
        </div>
        <h3 class="text-lg font-semibold mb-1">"Successfully saved!"</h3>
        <p class="text-sm text-muted-foreground">"Your changes have been saved."</p>
    </div>
}"#}>
                <div class="w-full flex flex-col items-center justify-center py-8 text-center">
                    <div class="h-16 w-16 rounded-full bg-green-500/10 flex items-center justify-center mb-4">
                        <i class="fas fa-check-circle text-3xl text-green-500"></i>
                    </div>
                    <h3 class="text-base font-semibold mb-1">"Payment successful!"</h3>
                    <p class="text-sm text-muted-foreground mb-1">"Your order #12345 has been confirmed."</p>
                    <p class="text-xs text-muted-foreground mb-4">"A confirmation email has been sent to your inbox."</p>
                    <button class="px-4 py-2 bg-green-500 text-white text-sm rounded-lg font-medium hover:bg-green-600 transition-colors">
                        "View Order"
                    </button>
                </div>
            </Showcase>

            <Showcase id="loading-overlay" title="Loading Overlay" description="Full-area overlay for blocking interactions during loading."
                code={r#"rsx! {
    <div class="relative">
        <div class="p-6">
            <p>"Content underneath"</p>
        </div>
        <div class="absolute inset-0 bg-background/80 backdrop-blur-sm flex items-center justify-center rounded-lg">
            <div class="flex flex-col items-center gap-2">
                <svg class="animate-spin h-6 w-6 text-primary" viewBox="0 0 24 24">...</svg>
                <span class="text-sm text-muted-foreground">"Loading..."</span>
            </div>
        </div>
    </div>
}"#}>
                <div class="w-full max-w-sm">
                    <div class="relative rounded-lg border border-border overflow-hidden">
                        <div class="p-6 space-y-3 opacity-50">
                            <div class="h-4 bg-muted rounded w-3/4"></div>
                            <div class="h-4 bg-muted rounded w-full"></div>
                            <div class="h-4 bg-muted rounded w-5/6"></div>
                            <div class="h-10 bg-muted rounded w-1/3 mt-4"></div>
                        </div>
                        <div class="absolute inset-0 bg-background/60 backdrop-blur-[2px] flex items-center justify-center">
                            <div class="flex flex-col items-center gap-2">
                                <svg class="animate-spin h-8 w-8 text-primary" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke_width="4"></circle>
                                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
                                </svg>
                                <span class="text-sm font-medium">"Saving changes..."</span>
                            </div>
                        </div>
                    </div>
                </div>
            </Showcase>

            <div class="mt-8 flex items-center justify-between border-t border-border pt-6">
                <a href={docs_href("/ui/layout")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "← Layout"
                </a>
                <a href={docs_href("/ui/overlays")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "Overlays →"
                </a>
            </div>
        </article>
    }
}
