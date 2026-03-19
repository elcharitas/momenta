#![allow(unused_imports)]
use crate::components::*;
use alloc::{format, vec, vec::Vec};
use momenta::prelude::*;

#[component]
pub fn DataDisplayPage() -> Node {
    rsx! {
        <article class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <CategoryHeader title="Data Display" description="Components for presenting structured data, lists, and information in clear, readable formats." count={10} />

            <Showcase id="simple-table" title="Simple Table" description="Basic table with header and rows."
                code={r#"rsx! {
    <table class="w-full text-sm">
        <thead>
            <tr class="border-b border-border">
                <th class="text-left py-3 px-4 font-medium text-muted-foreground">"Name"</th>
                <th class="text-left py-3 px-4 font-medium text-muted-foreground">"Status"</th>
                <th class="text-right py-3 px-4 font-medium text-muted-foreground">"Amount"</th>
            </tr>
        </thead>
        <tbody>
            <tr class="border-b border-border">
                <td class="py-3 px-4">"Alice Johnson"</td>
                <td class="py-3 px-4"><span class="text-green-600">"Active"</span></td>
                <td class="py-3 px-4 text-right">"$250.00"</td>
            </tr>
        </tbody>
    </table>
}"#}>
                <div class="w-full overflow-x-auto rounded-lg border border-border">
                    <table class="w-full text-sm">
                        <thead>
                            <tr class="border-b border-border bg-muted/50">
                                <th class="text-left py-3 px-4 font-medium text-muted-foreground">"Name"</th>
                                <th class="text-left py-3 px-4 font-medium text-muted-foreground">"Status"</th>
                                <th class="text-left py-3 px-4 font-medium text-muted-foreground">"Role"</th>
                                <th class="text-right py-3 px-4 font-medium text-muted-foreground">"Amount"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="border-b border-border">
                                <td class="py-3 px-4 font-medium">"Alice Johnson"</td>
                                <td class="py-3 px-4"><span class="inline-flex items-center rounded-full bg-green-500/10 text-green-600 dark:text-green-400 px-2 py-0.5 text-xs font-medium">"Active"</span></td>
                                <td class="py-3 px-4 text-muted-foreground">"Admin"</td>
                                <td class="py-3 px-4 text-right">"$250.00"</td>
                            </tr>
                            <tr class="border-b border-border">
                                <td class="py-3 px-4 font-medium">"Bob Smith"</td>
                                <td class="py-3 px-4"><span class="inline-flex items-center rounded-full bg-amber-500/10 text-amber-600 dark:text-amber-400 px-2 py-0.5 text-xs font-medium">"Pending"</span></td>
                                <td class="py-3 px-4 text-muted-foreground">"Editor"</td>
                                <td class="py-3 px-4 text-right">"$150.00"</td>
                            </tr>
                            <tr>
                                <td class="py-3 px-4 font-medium">"Carol White"</td>
                                <td class="py-3 px-4"><span class="inline-flex items-center rounded-full bg-red-500/10 text-red-600 dark:text-red-400 px-2 py-0.5 text-xs font-medium">"Inactive"</span></td>
                                <td class="py-3 px-4 text-muted-foreground">"Viewer"</td>
                                <td class="py-3 px-4 text-right">"$0.00"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </Showcase>

            <Showcase id="striped-table" title="Striped Table" description="Table with alternating row colors."
                code={r#"rsx! {
    <table class="w-full text-sm">
        <thead><tr class="border-b border-border"><th class="text-left py-3 px-4 font-medium">"Item"</th></tr></thead>
        <tbody>
            <tr class="bg-muted/30"><td class="py-2.5 px-4">"Row 1"</td></tr>
            <tr><td class="py-2.5 px-4">"Row 2"</td></tr>
        </tbody>
    </table>
}"#}>
                <div class="w-full overflow-x-auto rounded-lg border border-border">
                    <table class="w-full text-sm">
                        <thead>
                            <tr class="border-b border-border bg-muted/50">
                                <th class="text-left py-3 px-4 font-medium text-muted-foreground">"#"</th>
                                <th class="text-left py-3 px-4 font-medium text-muted-foreground">"Package"</th>
                                <th class="text-left py-3 px-4 font-medium text-muted-foreground">"Version"</th>
                                <th class="text-right py-3 px-4 font-medium text-muted-foreground">"Downloads"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="bg-muted/20"><td class="py-2.5 px-4 text-muted-foreground">"1"</td><td class="py-2.5 px-4 font-medium">"momenta"</td><td class="py-2.5 px-4 text-muted-foreground">"0.2.0"</td><td class="py-2.5 px-4 text-right">"12,450"</td></tr>
                            <tr><td class="py-2.5 px-4 text-muted-foreground">"2"</td><td class="py-2.5 px-4 font-medium">"momenta-core"</td><td class="py-2.5 px-4 text-muted-foreground">"0.2.0"</td><td class="py-2.5 px-4 text-right">"10,200"</td></tr>
                            <tr class="bg-muted/20"><td class="py-2.5 px-4 text-muted-foreground">"3"</td><td class="py-2.5 px-4 font-medium">"momenta-router"</td><td class="py-2.5 px-4 text-muted-foreground">"0.2.0"</td><td class="py-2.5 px-4 text-right">"8,300"</td></tr>
                            <tr><td class="py-2.5 px-4 text-muted-foreground">"4"</td><td class="py-2.5 px-4 font-medium">"momenta-ssr"</td><td class="py-2.5 px-4 text-muted-foreground">"0.2.0"</td><td class="py-2.5 px-4 text-right">"5,100"</td></tr>
                        </tbody>
                    </table>
                </div>
            </Showcase>

            <Showcase id="description-list" title="Description List" description="Key-value pairs for displaying metadata."
                code={r#"rsx! {
    <dl class="space-y-4">
        <div class="flex justify-between">
            <dt class="text-sm text-muted-foreground">"Full name"</dt>
            <dd class="text-sm font-medium">"Jane Cooper"</dd>
        </div>
    </dl>
}"#}>
                <div class="w-full max-w-md">
                    <dl class="divide-y divide-border">
                        <div class="flex justify-between py-3">
                            <dt class="text-sm text-muted-foreground">"Full name"</dt>
                            <dd class="text-sm font-medium">"Jane Cooper"</dd>
                        </div>
                        <div class="flex justify-between py-3">
                            <dt class="text-sm text-muted-foreground">"Email"</dt>
                            <dd class="text-sm font-medium">"jane@example.com"</dd>
                        </div>
                        <div class="flex justify-between py-3">
                            <dt class="text-sm text-muted-foreground">"Role"</dt>
                            <dd class="text-sm font-medium">"Admin"</dd>
                        </div>
                        <div class="flex justify-between py-3">
                            <dt class="text-sm text-muted-foreground">"Status"</dt>
                            <dd><span class="inline-flex items-center rounded-full bg-green-500/10 text-green-600 dark:text-green-400 px-2 py-0.5 text-xs font-medium">"Active"</span></dd>
                        </div>
                    </dl>
                </div>
            </Showcase>

            <Showcase id="avatar" title="Avatar" description="User avatar with initials or placeholder."
                code={r#"rsx! {
    <div class="h-10 w-10 rounded-full bg-primary/10 flex items-center justify-center">
        <span class="text-sm font-bold text-primary">"JD"</span>
    </div>
}"#}>
                <div class="flex items-center gap-3">
                    <div class="h-8 w-8 rounded-full bg-primary/10 flex items-center justify-center">
                        <span class="text-xs font-bold text-primary">"AB"</span>
                    </div>
                    <div class="h-10 w-10 rounded-full bg-green-500/10 flex items-center justify-center">
                        <span class="text-sm font-bold text-green-600 dark:text-green-400">"CD"</span>
                    </div>
                    <div class="h-12 w-12 rounded-full bg-violet-500/10 flex items-center justify-center">
                        <span class="text-base font-bold text-violet-600 dark:text-violet-400">"EF"</span>
                    </div>
                    <div class="h-14 w-14 rounded-full bg-amber-500/10 flex items-center justify-center">
                        <span class="text-lg font-bold text-amber-600 dark:text-amber-400">"GH"</span>
                    </div>
                    <div class="h-10 w-10 rounded-full bg-muted flex items-center justify-center">
                        <i class="fas fa-user text-sm text-muted-foreground"></i>
                    </div>
                </div>
            </Showcase>

            <Showcase id="avatar-group" title="Avatar Group" description="Overlapping avatars for team display."
                code={r#"rsx! {
    <div class="flex -space-x-2">
        <div class="h-9 w-9 rounded-full bg-blue-500/10 ring-2 ring-background flex items-center justify-center">
            <span class="text-xs font-bold text-blue-600">"A"</span>
        </div>
        <div class="h-9 w-9 rounded-full bg-green-500/10 ring-2 ring-background flex items-center justify-center">
            <span class="text-xs font-bold text-green-600">"B"</span>
        </div>
        <div class="h-9 w-9 rounded-full bg-muted ring-2 ring-background flex items-center justify-center">
            <span class="text-xs font-bold text-muted-foreground">"+3"</span>
        </div>
    </div>
}"#}>
                <div class="flex items-center gap-6">
                    <div class="flex -space-x-2">
                        <div class="h-9 w-9 rounded-full bg-blue-500/10 ring-2 ring-background flex items-center justify-center"><span class="text-xs font-bold text-blue-600 dark:text-blue-400">"A"</span></div>
                        <div class="h-9 w-9 rounded-full bg-green-500/10 ring-2 ring-background flex items-center justify-center"><span class="text-xs font-bold text-green-600 dark:text-green-400">"B"</span></div>
                        <div class="h-9 w-9 rounded-full bg-violet-500/10 ring-2 ring-background flex items-center justify-center"><span class="text-xs font-bold text-violet-600 dark:text-violet-400">"C"</span></div>
                        <div class="h-9 w-9 rounded-full bg-amber-500/10 ring-2 ring-background flex items-center justify-center"><span class="text-xs font-bold text-amber-600 dark:text-amber-400">"D"</span></div>
                        <div class="h-9 w-9 rounded-full bg-muted ring-2 ring-background flex items-center justify-center"><span class="text-xs font-bold text-muted-foreground">"+5"</span></div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="key-value" title="Key-Value List" description="Compact key-value data display."
                code={r#"rsx! {
    <div class="rounded-lg border border-border divide-y divide-border">
        <div class="flex items-center justify-between px-4 py-2.5">
            <span class="text-sm text-muted-foreground">"Version"</span>
            <span class="text-sm font-mono">"2.0.0"</span>
        </div>
    </div>
}"#}>
                <div class="w-full max-w-sm rounded-lg border border-border divide-y divide-border overflow-hidden">
                    <div class="flex items-center justify-between px-4 py-2.5 bg-muted/30">
                        <span class="text-sm text-muted-foreground">"Framework"</span>
                        <span class="text-sm font-medium">"Momenta"</span>
                    </div>
                    <div class="flex items-center justify-between px-4 py-2.5">
                        <span class="text-sm text-muted-foreground">"Version"</span>
                        <code class="text-sm font-mono bg-muted px-1.5 py-0.5 rounded">"0.2.0"</code>
                    </div>
                    <div class="flex items-center justify-between px-4 py-2.5 bg-muted/30">
                        <span class="text-sm text-muted-foreground">"Language"</span>
                        <span class="text-sm font-medium">"Rust"</span>
                    </div>
                    <div class="flex items-center justify-between px-4 py-2.5">
                        <span class="text-sm text-muted-foreground">"Target"</span>
                        <span class="text-sm font-medium">"wasm32"</span>
                    </div>
                    <div class="flex items-center justify-between px-4 py-2.5 bg-muted/30">
                        <span class="text-sm text-muted-foreground">"License"</span>
                        <span class="text-sm font-medium">"MIT"</span>
                    </div>
                </div>
            </Showcase>

            <Showcase id="timeline" title="Timeline" description="Chronological event timeline."
                code={r#"rsx! {
    <div class="space-y-6 relative before:absolute before:left-[15px] before:top-2 before:bottom-2 before:w-px before:bg-border">
        <div class="flex gap-4 relative">
            <div class="h-8 w-8 shrink-0 rounded-full bg-primary flex items-center justify-center z-10">
                <i class="fas fa-check text-xs text-primary-foreground"></i>
            </div>
            <div>
                <p class="text-sm font-medium">"Project created"</p>
                <p class="text-xs text-muted-foreground">"March 1, 2026"</p>
            </div>
        </div>
    </div>
}"#}>
                <div class="w-full max-w-md">
                    <div class="space-y-6 relative before:absolute before:left-[15px] before:top-2 before:bottom-2 before:w-px before:bg-border">
                        <div class="flex gap-4 relative">
                            <div class="h-8 w-8 shrink-0 rounded-full bg-primary flex items-center justify-center z-10">
                                <i class="fas fa-check text-xs text-primary-foreground"></i>
                            </div>
                            <div class="pt-0.5">
                                <p class="text-sm font-medium">"v2.0 released"</p>
                                <p class="text-xs text-muted-foreground mt-0.5">"March 15, 2026"</p>
                                <p class="text-sm text-muted-foreground mt-1">"SSR, hydration, and streaming support."</p>
                            </div>
                        </div>
                        <div class="flex gap-4 relative">
                            <div class="h-8 w-8 shrink-0 rounded-full bg-green-500 flex items-center justify-center z-10">
                                <i class="fas fa-code text-xs text-white"></i>
                            </div>
                            <div class="pt-0.5">
                                <p class="text-sm font-medium">"Router v2 shipped"</p>
                                <p class="text-xs text-muted-foreground mt-0.5">"March 10, 2026"</p>
                                <p class="text-sm text-muted-foreground mt-1">"Dynamic route params and nested routing."</p>
                            </div>
                        </div>
                        <div class="flex gap-4 relative">
                            <div class="h-8 w-8 shrink-0 rounded-full bg-muted flex items-center justify-center z-10">
                                <i class="fas fa-flag text-xs text-muted-foreground"></i>
                            </div>
                            <div class="pt-0.5">
                                <p class="text-sm font-medium">"Project started"</p>
                                <p class="text-xs text-muted-foreground mt-0.5">"January 1, 2026"</p>
                            </div>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="stat-widget" title="Stat Widget" description="Inline stat with trend indicator."
                code={r#"rsx! {
    <div class="flex items-center gap-4">
        <div>
            <p class="text-2xl font-bold">"8,249"</p>
            <p class="text-sm text-muted-foreground">"Total Users"</p>
        </div>
        <span class="inline-flex items-center gap-1 text-sm text-green-600 font-medium">
            <i class="fas fa-arrow-up text-xs"></i>"+12%"
        </span>
    </div>
}"#}>
                <div class="w-full grid gap-4 sm:grid-cols-3">
                    <div class="flex items-center gap-4 rounded-lg border border-border p-4">
                        <div class="h-10 w-10 rounded-lg bg-blue-500/10 flex items-center justify-center shrink-0">
                            <i class="fas fa-users text-blue-500"></i>
                        </div>
                        <div>
                            <p class="text-xl font-bold">"8,249"</p>
                            <div class="flex items-center gap-2">
                                <p class="text-xs text-muted-foreground">"Total Users"</p>
                                <span class="inline-flex items-center gap-0.5 text-[10px] text-green-600 dark:text-green-400 font-medium"><i class="fas fa-arrow-up text-[8px]"></i>"+12%"</span>
                            </div>
                        </div>
                    </div>
                    <div class="flex items-center gap-4 rounded-lg border border-border p-4">
                        <div class="h-10 w-10 rounded-lg bg-green-500/10 flex items-center justify-center shrink-0">
                            <i class="fas fa-dollar-sign text-green-500"></i>
                        </div>
                        <div>
                            <p class="text-xl font-bold">"$12.4k"</p>
                            <div class="flex items-center gap-2">
                                <p class="text-xs text-muted-foreground">"Revenue"</p>
                                <span class="inline-flex items-center gap-0.5 text-[10px] text-green-600 dark:text-green-400 font-medium"><i class="fas fa-arrow-up text-[8px]"></i>"+8%"</span>
                            </div>
                        </div>
                    </div>
                    <div class="flex items-center gap-4 rounded-lg border border-border p-4">
                        <div class="h-10 w-10 rounded-lg bg-red-500/10 flex items-center justify-center shrink-0">
                            <i class="fas fa-bug text-red-500"></i>
                        </div>
                        <div>
                            <p class="text-xl font-bold">"23"</p>
                            <div class="flex items-center gap-2">
                                <p class="text-xs text-muted-foreground">"Open Issues"</p>
                                <span class="inline-flex items-center gap-0.5 text-[10px] text-red-600 dark:text-red-400 font-medium"><i class="fas fa-arrow-down text-[8px]"></i>"-5%"</span>
                            </div>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="progress-list" title="Progress List" description="List items with progress indicators."
                code={r#"rsx! {
    <div class="space-y-3">
        <div class="flex items-center gap-3">
            <span class="text-sm w-24 shrink-0">"Rust"</span>
            <div class="flex-1 h-2 rounded-full bg-muted overflow-hidden">
                <div class="h-full w-[85%] rounded-full bg-primary"></div>
            </div>
            <span class="text-sm text-muted-foreground w-10 text-right">"85%"</span>
        </div>
    </div>
}"#}>
                <div class="w-full max-w-md space-y-3">
                    <div class="flex items-center gap-3">
                        <span class="text-sm w-24 shrink-0">"Rust"</span>
                        <div class="flex-1 h-2 rounded-full bg-muted overflow-hidden"><div class="h-full rounded-full bg-primary" style="width: 85%"></div></div>
                        <span class="text-sm text-muted-foreground w-10 text-right">"85%"</span>
                    </div>
                    <div class="flex items-center gap-3">
                        <span class="text-sm w-24 shrink-0">"TypeScript"</span>
                        <div class="flex-1 h-2 rounded-full bg-muted overflow-hidden"><div class="h-full rounded-full bg-blue-500" style="width: 72%"></div></div>
                        <span class="text-sm text-muted-foreground w-10 text-right">"72%"</span>
                    </div>
                    <div class="flex items-center gap-3">
                        <span class="text-sm w-24 shrink-0">"Python"</span>
                        <div class="flex-1 h-2 rounded-full bg-muted overflow-hidden"><div class="h-full rounded-full bg-green-500" style="width: 60%"></div></div>
                        <span class="text-sm text-muted-foreground w-10 text-right">"60%"</span>
                    </div>
                    <div class="flex items-center gap-3">
                        <span class="text-sm w-24 shrink-0">"Go"</span>
                        <div class="flex-1 h-2 rounded-full bg-muted overflow-hidden"><div class="h-full rounded-full bg-cyan-500" style="width: 45%"></div></div>
                        <span class="text-sm text-muted-foreground w-10 text-right">"45%"</span>
                    </div>
                </div>
            </Showcase>

            <Showcase id="badge-list" title="Badge List" description="List items annotated with badges."
                code={r#"rsx! {
    <div class="divide-y divide-border rounded-lg border border-border">
        <div class="flex items-center justify-between px-4 py-3">
            <span class="text-sm font-medium">"momenta"</span>
            <span class="rounded-full bg-green-500/10 text-green-600 px-2 py-0.5 text-xs font-medium">"stable"</span>
        </div>
    </div>
}"#}>
                <div class="w-full max-w-sm divide-y divide-border rounded-lg border border-border overflow-hidden">
                    <div class="flex items-center justify-between px-4 py-3">
                        <div class="flex items-center gap-2">
                            <i class="fas fa-cube text-xs text-primary"></i>
                            <span class="text-sm font-medium">"momenta"</span>
                        </div>
                        <span class="rounded-full bg-green-500/10 text-green-600 dark:text-green-400 px-2 py-0.5 text-xs font-medium">"stable"</span>
                    </div>
                    <div class="flex items-center justify-between px-4 py-3">
                        <div class="flex items-center gap-2">
                            <i class="fas fa-cube text-xs text-primary"></i>
                            <span class="text-sm font-medium">"momenta-router"</span>
                        </div>
                        <span class="rounded-full bg-green-500/10 text-green-600 dark:text-green-400 px-2 py-0.5 text-xs font-medium">"stable"</span>
                    </div>
                    <div class="flex items-center justify-between px-4 py-3">
                        <div class="flex items-center gap-2">
                            <i class="fas fa-cube text-xs text-amber-500"></i>
                            <span class="text-sm font-medium">"momenta-ssr"</span>
                        </div>
                        <span class="rounded-full bg-amber-500/10 text-amber-600 dark:text-amber-400 px-2 py-0.5 text-xs font-medium">"beta"</span>
                    </div>
                    <div class="flex items-center justify-between px-4 py-3">
                        <div class="flex items-center gap-2">
                            <i class="fas fa-cube text-xs text-violet-500"></i>
                            <span class="text-sm font-medium">"momenta-cli"</span>
                        </div>
                        <span class="rounded-full bg-violet-500/10 text-violet-600 dark:text-violet-400 px-2 py-0.5 text-xs font-medium">"alpha"</span>
                    </div>
                </div>
            </Showcase>

            <div class="mt-8 flex items-center justify-between border-t border-border pt-6">
                <a href={docs_href("/ui/navigation")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "← Navigation"
                </a>
                <a href={docs_href("/ui/layout")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "Layout →"
                </a>
            </div>
        </article>
    }
}
