#![allow(unused_imports)]
use crate::components::*;
use alloc::{format, vec, vec::Vec};
use momenta::prelude::*;

#[component]
pub fn AlertsPage() -> Node {
    rsx! {
        <article class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <CategoryHeader title="Alerts & Banners" description="Contextual feedback messages for user actions, system status, and important information." count={7} />

            <Showcase id="info-alert" title="Info Alert" description="Informational messages for neutral guidance."
                code={r#"rsx! {
    <div class="rounded-lg border border-blue-500/30 bg-blue-50 dark:bg-blue-950/30 p-4">
        <div class="flex gap-3">
            <i class="fas fa-info-circle text-blue-500 mt-0.5"></i>
            <p class="text-sm text-blue-800 dark:text-blue-200">"A new version is available. Please update to continue."</p>
        </div>
    </div>
}"#}>
                <div class="w-full space-y-3">
                    <div class="rounded-lg border border-blue-500/30 bg-blue-50 dark:bg-blue-950/30 p-4">
                        <div class="flex gap-3">
                            <i class="fas fa-info-circle text-blue-500 mt-0.5"></i>
                            <p class="text-sm text-blue-800 dark:text-blue-200">"A new version is available. Please update to get the latest features."</p>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="success-alert" title="Success Alert" description="Positive confirmation that an action completed."
                code={r#"rsx! {
    <div class="rounded-lg border border-green-500/30 bg-green-50 dark:bg-green-950/30 p-4">
        <div class="flex gap-3">
            <i class="fas fa-check-circle text-green-500 mt-0.5"></i>
            <p class="text-sm text-green-800 dark:text-green-200">"Your changes have been saved successfully."</p>
        </div>
    </div>
}"#}>
                <div class="w-full space-y-3">
                    <div class="rounded-lg border border-green-500/30 bg-green-50 dark:bg-green-950/30 p-4">
                        <div class="flex gap-3">
                            <i class="fas fa-check-circle text-green-500 mt-0.5"></i>
                            <p class="text-sm text-green-800 dark:text-green-200">"Your changes have been saved successfully."</p>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="warning-alert" title="Warning Alert" description="Cautionary messages about potential issues."
                code={r#"rsx! {
    <div class="rounded-lg border border-amber-500/30 bg-amber-50 dark:bg-amber-950/30 p-4">
        <div class="flex gap-3">
            <i class="fas fa-exclamation-triangle text-amber-500 mt-0.5"></i>
            <p class="text-sm text-amber-800 dark:text-amber-200">"Your trial expires in 3 days. Upgrade now to keep access."</p>
        </div>
    </div>
}"#}>
                <div class="w-full space-y-3">
                    <div class="rounded-lg border border-amber-500/30 bg-amber-50 dark:bg-amber-950/30 p-4">
                        <div class="flex gap-3">
                            <i class="fas fa-exclamation-triangle text-amber-500 mt-0.5"></i>
                            <p class="text-sm text-amber-800 dark:text-amber-200">"Your trial expires in 3 days. Upgrade now to keep access."</p>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="error-alert" title="Error Alert" description="Critical messages indicating failures or blockers."
                code={r#"rsx! {
    <div class="rounded-lg border border-red-500/30 bg-red-50 dark:bg-red-950/30 p-4">
        <div class="flex gap-3">
            <i class="fas fa-times-circle text-red-500 mt-0.5"></i>
            <p class="text-sm text-red-800 dark:text-red-200">"Payment failed. Please check your card details and try again."</p>
        </div>
    </div>
}"#}>
                <div class="w-full space-y-3">
                    <div class="rounded-lg border border-red-500/30 bg-red-50 dark:bg-red-950/30 p-4">
                        <div class="flex gap-3">
                            <i class="fas fa-times-circle text-red-500 mt-0.5"></i>
                            <p class="text-sm text-red-800 dark:text-red-200">"Payment failed. Please check your card details and try again."</p>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="alert-description" title="Alert with Description" description="Alert with a title and detailed description."
                code={r#"rsx! {
    <div class="rounded-lg border border-blue-500/30 bg-blue-50 dark:bg-blue-950/30 p-4">
        <div class="flex gap-3">
            <i class="fas fa-info-circle text-blue-500 mt-0.5"></i>
            <div>
                <h4 class="text-sm font-semibold text-blue-800 dark:text-blue-200">"Update Available"</h4>
                <p class="mt-1 text-sm text-blue-700 dark:text-blue-300">"Version 2.0 includes performance improvements and new components."</p>
            </div>
        </div>
    </div>
}"#}>
                <div class="w-full space-y-3">
                    <div class="rounded-lg border border-blue-500/30 bg-blue-50 dark:bg-blue-950/30 p-4">
                        <div class="flex gap-3">
                            <i class="fas fa-info-circle text-blue-500 mt-0.5"></i>
                            <div>
                                <h4 class="text-sm font-semibold text-blue-800 dark:text-blue-200">"Update Available"</h4>
                                <p class="mt-1 text-sm text-blue-700 dark:text-blue-300">"Version 2.0 includes performance improvements, new UI components, and better accessibility support."</p>
                            </div>
                        </div>
                    </div>
                    <div class="rounded-lg border border-red-500/30 bg-red-50 dark:bg-red-950/30 p-4">
                        <div class="flex gap-3">
                            <i class="fas fa-exclamation-circle text-red-500 mt-0.5"></i>
                            <div>
                                <h4 class="text-sm font-semibold text-red-800 dark:text-red-200">"Deployment Failed"</h4>
                                <p class="mt-1 text-sm text-red-700 dark:text-red-300">"Build step failed at stage 3/5. Check the logs for details."</p>
                            </div>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="alert-actions" title="Alert with Actions" description="Alerts with action buttons for immediate response."
                code={r#"rsx! {
    <div class="rounded-lg border border-amber-500/30 bg-amber-50 dark:bg-amber-950/30 p-4">
        <div class="flex items-start gap-3">
            <i class="fas fa-exclamation-triangle text-amber-500 mt-0.5"></i>
            <div class="flex-1">
                <p class="text-sm text-amber-800 dark:text-amber-200">"Your session is about to expire."</p>
                <div class="mt-3 flex gap-2">
                    <button class="rounded-md bg-amber-600 px-3 py-1 text-xs font-medium text-white">"Extend"</button>
                    <button class="rounded-md px-3 py-1 text-xs font-medium text-amber-700 dark:text-amber-300">"Dismiss"</button>
                </div>
            </div>
        </div>
    </div>
}"#}>
                <div class="w-full space-y-3">
                    <div class="rounded-lg border border-amber-500/30 bg-amber-50 dark:bg-amber-950/30 p-4">
                        <div class="flex items-start gap-3">
                            <i class="fas fa-exclamation-triangle text-amber-500 mt-0.5"></i>
                            <div class="flex-1">
                                <p class="text-sm text-amber-800 dark:text-amber-200">"Your session is about to expire in 5 minutes."</p>
                                <div class="mt-3 flex gap-2">
                                    <button class="rounded-md bg-amber-600 px-3 py-1.5 text-xs font-medium text-white hover:bg-amber-700 transition-colors">"Extend Session"</button>
                                    <button class="rounded-md px-3 py-1.5 text-xs font-medium text-amber-700 dark:text-amber-300 hover:bg-amber-500/10 transition-colors">"Dismiss"</button>
                                </div>
                            </div>
                            <button class="text-amber-500 hover:text-amber-700 transition-colors">
                                <i class="fas fa-times text-sm"></i>
                            </button>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="banner" title="Banner" description="Full-width announcement banners for site-wide messages."
                code={r##"rsx! {
    <div class="w-full bg-primary px-4 py-2.5 text-center">
        <p class="text-sm font-medium text-primary-foreground">
            "🚀 Momenta v2.0 is here! "
            <a href="#" class="underline font-semibold">"See what's new →"</a>
        </p>
    </div>
}"##}>
                <div class="w-full space-y-3 -mx-6 -my-6">
                    <div class="w-full bg-primary px-4 py-2.5 text-center">
                        <p class="text-sm font-medium text-primary-foreground">
                            "🚀 Momenta v2.0 is here! "
                            <a href="#" class="underline font-semibold hover:opacity-80 transition-opacity">"See what's new →"</a>
                        </p>
                    </div>
                    <div class="w-full bg-gradient-to-r from-violet-600 to-blue-600 px-4 py-2.5 text-center">
                        <p class="text-sm font-medium text-white">
                            "✨ Join us at RustConf 2026 — "
                            <a href="#" class="underline font-semibold hover:opacity-80 transition-opacity">"Register now"</a>
                        </p>
                    </div>
                    <div class="w-full bg-amber-500 px-4 py-2.5">
                        <div class="flex items-center justify-center gap-2">
                            <i class="fas fa-exclamation-triangle text-xs text-amber-900"></i>
                            <p class="text-sm font-medium text-amber-900">"Scheduled maintenance on March 20, 2026 at 2:00 AM UTC"</p>
                        </div>
                    </div>
                </div>
            </Showcase>

            <div class="mt-8 flex items-center justify-between border-t border-border pt-6">
                <a href={docs_href("/ui/badges")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "← Badges & Tags"
                </a>
                <a href={docs_href("/ui/cards")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "Cards →"
                </a>
            </div>
        </article>
    }
}
