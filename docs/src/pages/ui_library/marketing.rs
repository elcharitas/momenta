#![allow(unused_imports)]
use crate::components::*;
use alloc::{format, vec, vec::Vec};
use momenta::prelude::*;

#[component]
pub fn MarketingPage() -> Node {
    rsx! {
        <article class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <CategoryHeader title="Marketing" description="Landing page sections: heroes, CTAs, pricing, testimonials and more for marketing pages." count={12} />

            <Showcase id="hero-centered" title="Hero - Centered" description="Centered hero section with headline, description, and CTA."
                code={r#"rsx! {
    <section class="py-20 text-center">
        <h1 class="text-4xl sm:text-5xl font-bold tracking-tight mb-6">"Build faster with Momenta"</h1>
        <p class="text-lg text-muted-foreground max-w-2xl mx-auto mb-8">"A reactive Rust framework for building modern web applications."</p>
        <div class="flex items-center justify-center gap-4">
            <a class="px-6 py-3 bg-primary text-primary-foreground rounded-lg font-medium">"Get Started"</a>
            <a class="px-6 py-3 border border-border rounded-lg font-medium">"Learn More"</a>
        </div>
    </section>
}"#}>
                <div class="w-full">
                    <section class="py-10 text-center">
                        <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-primary/10 text-primary text-xs font-medium mb-4">
                            <i class="fas fa-rocket text-[10px]"></i>"Now in v2.0"
                        </div>
                        <h1 class="text-2xl sm:text-3xl font-bold tracking-tight mb-3">"Build faster with Momenta"</h1>
                        <p class="text-sm text-muted-foreground max-w-md mx-auto mb-6">"The reactive Rust framework for building modern, high-performance web applications compiled to WASM."</p>
                        <div class="flex items-center justify-center gap-3">
                            <button class="px-5 py-2.5 bg-primary text-primary-foreground rounded-lg text-sm font-medium hover:bg-primary/90 transition-colors">"Get Started"</button>
                            <button class="px-5 py-2.5 border border-border rounded-lg text-sm font-medium hover:bg-muted transition-colors">"Documentation"</button>
                        </div>
                    </section>
                </div>
            </Showcase>

            <Showcase id="hero-split" title="Hero - Split" description="Hero section with content on one side and visual on the other."
                code={r#"rsx! {
    <section class="grid md:grid-cols-2 gap-12 items-center py-16">
        <div>
            <h1 class="text-4xl font-bold mb-4">"Ship with confidence"</h1>
            <p class="text-lg text-muted-foreground mb-6">"Description text here."</p>
            <button class="px-6 py-3 bg-primary text-primary-foreground rounded-lg">"Start Building"</button>
        </div>
        <div class="bg-muted rounded-xl p-8">"Visual/Image"</div>
    </section>
}"#}>
                <div class="w-full">
                    <section class="grid sm:grid-cols-2 gap-6 items-center py-4">
                        <div>
                            <h1 class="text-xl sm:text-2xl font-bold mb-2">"Ship with confidence"</h1>
                            <p class="text-sm text-muted-foreground mb-4">"Type-safe, reactive, and compiled to native WASM. Build UIs that are fast by default."</p>
                            <button class="px-4 py-2 bg-primary text-primary-foreground rounded-lg text-sm font-medium hover:bg-primary/90 transition-colors">"Start Building"</button>
                        </div>
                        <div class="bg-gradient-to-br from-primary/5 to-violet-500/5 border border-border/50 rounded-xl p-6 flex items-center justify-center min-h-[140px]">
                            <div class="text-center">
                                <i class="fas fa-code text-3xl text-primary/50 mb-2"></i>
                                <p class="text-xs text-muted-foreground">"Code Preview"</p>
                            </div>
                        </div>
                    </section>
                </div>
            </Showcase>

            <Showcase id="cta-simple" title="CTA - Simple" description="Call-to-action banner with background."
                code={r#"rsx! {
    <section class="rounded-2xl bg-primary p-8 sm:p-12 text-center">
        <h2 class="text-2xl font-bold text-primary-foreground mb-3">"Ready to get started?"</h2>
        <p class="text-primary-foreground/80 mb-6">"Join thousands of developers building with Momenta."</p>
        <button class="px-6 py-3 bg-background text-foreground rounded-lg font-medium">"Get Started Free"</button>
    </section>
}"#}>
                <div class="w-full">
                    <section class="rounded-xl bg-primary p-6 sm:p-8 text-center">
                        <h2 class="text-lg sm:text-xl font-bold text-primary-foreground mb-2">"Ready to get started?"</h2>
                        <p class="text-sm text-primary-foreground/80 mb-4">"Join thousands of developers building with Momenta."</p>
                        <button class="px-5 py-2.5 bg-background text-foreground rounded-lg text-sm font-medium hover:bg-background/90 transition-colors">"Get Started Free"</button>
                    </section>
                </div>
            </Showcase>

            <Showcase id="cta-image" title="CTA - With Image" description="CTA section with side image or illustration."
                code={r#"rsx! {
    <section class="rounded-2xl border border-border overflow-hidden grid md:grid-cols-2">
        <div class="p-8 flex flex-col justify-center">
            <h2 class="text-2xl font-bold mb-3">"Start your journey"</h2>
            <p class="text-muted-foreground mb-6">"Description."</p>
            <div><button class="px-6 py-3 bg-primary text-primary-foreground rounded-lg">"Sign Up"</button></div>
        </div>
        <div class="bg-gradient-to-br from-primary/10 to-violet-500/10 p-8">"Image"</div>
    </section>
}"#}>
                <div class="w-full">
                    <section class="rounded-xl border border-border overflow-hidden grid sm:grid-cols-2">
                        <div class="p-6 flex flex-col justify-center">
                            <h2 class="text-lg font-bold mb-2">"Start your journey"</h2>
                            <p class="text-sm text-muted-foreground mb-4">"From zero to production in minutes. Our CLI sets up everything you need."</p>
                            <div>
                                <button class="px-4 py-2 bg-primary text-primary-foreground rounded-lg text-sm font-medium">"Sign Up"</button>
                            </div>
                        </div>
                        <div class="bg-gradient-to-br from-primary/5 to-violet-500/10 p-6 flex items-center justify-center min-h-[120px]">
                            <div class="flex items-center gap-3">
                                <div class="h-12 w-12 rounded-lg bg-background/50 border border-border flex items-center justify-center"><i class="fas fa-terminal text-primary"></i></div>
                                <div class="text-left">
                                    <p class="text-sm font-mono font-medium">"cargo install momenta"</p>
                                    <p class="text-xs text-muted-foreground">"One command to start"</p>
                                </div>
                            </div>
                        </div>
                    </section>
                </div>
            </Showcase>

            <Showcase id="feature-grid" title="Feature Grid" description="Grid of feature cards with icons."
                code={r#"rsx! {
    <section class="py-16">
        <h2 class="text-2xl font-bold text-center mb-4">"Features"</h2>
        <p class="text-center text-muted-foreground mb-12">"Everything you need to build modern apps."</p>
        <div class="grid md:grid-cols-3 gap-8">
            <div class="text-center">
                <div class="h-12 w-12 bg-primary/10 rounded-lg mx-auto mb-4 flex items-center justify-center">
                    <i class="fas fa-bolt text-primary"></i>
                </div>
                <h3 class="font-semibold mb-2">"Blazing Fast"</h3>
                <p class="text-sm text-muted-foreground">"Compiled to native WASM for maximum performance."</p>
            </div>
        </div>
    </section>
}"#}>
                <div class="w-full">
                    <div class="text-center mb-6">
                        <h2 class="text-lg font-bold mb-1">"Why Momenta?"</h2>
                        <p class="text-sm text-muted-foreground">"Everything you need to build modern web apps."</p>
                    </div>
                    <div class="grid sm:grid-cols-3 gap-4">
                        <div class="text-center p-4 rounded-lg border border-border">
                            <div class="h-10 w-10 bg-blue-500/10 rounded-lg mx-auto mb-3 flex items-center justify-center"><i class="fas fa-bolt text-blue-500"></i></div>
                            <h3 class="text-sm font-semibold mb-1">"Blazing Fast"</h3>
                            <p class="text-xs text-muted-foreground">"Compiled to native WASM."</p>
                        </div>
                        <div class="text-center p-4 rounded-lg border border-border">
                            <div class="h-10 w-10 bg-green-500/10 rounded-lg mx-auto mb-3 flex items-center justify-center"><i class="fas fa-shield-alt text-green-500"></i></div>
                            <h3 class="text-sm font-semibold mb-1">"Type Safe"</h3>
                            <p class="text-xs text-muted-foreground">"Rust's type system at work."</p>
                        </div>
                        <div class="text-center p-4 rounded-lg border border-border">
                            <div class="h-10 w-10 bg-violet-500/10 rounded-lg mx-auto mb-3 flex items-center justify-center"><i class="fas fa-sync text-violet-500"></i></div>
                            <h3 class="text-sm font-semibold mb-1">"Reactive"</h3>
                            <p class="text-xs text-muted-foreground">"Fine-grained reactivity built in."</p>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="feature-list" title="Feature List" description="List-style feature section with descriptions."
                code={r#"rsx! {
    <section class="space-y-8">
        <div class="flex items-start gap-4">
            <div class="h-10 w-10 shrink-0 rounded-lg bg-primary/10 flex items-center justify-center">
                <i class="fas fa-zap text-primary"></i>
            </div>
            <div>
                <h3 class="font-semibold mb-1">"Feature Title"</h3>
                <p class="text-sm text-muted-foreground">"Feature description."</p>
            </div>
        </div>
    </section>
}"#}>
                <div class="w-full space-y-4">
                    <div class="flex items-start gap-3">
                        <div class="h-9 w-9 shrink-0 rounded-lg bg-primary/10 flex items-center justify-center mt-0.5"><i class="fas fa-bolt text-sm text-primary"></i></div>
                        <div>
                            <h3 class="text-sm font-semibold">"Zero-cost abstractions"</h3>
                            <p class="text-xs text-muted-foreground mt-0.5">"Components compile away, leaving only the minimal DOM operations needed."</p>
                        </div>
                    </div>
                    <div class="flex items-start gap-3">
                        <div class="h-9 w-9 shrink-0 rounded-lg bg-green-500/10 flex items-center justify-center mt-0.5"><i class="fas fa-server text-sm text-green-500"></i></div>
                        <div>
                            <h3 class="text-sm font-semibold">"Server-side rendering"</h3>
                            <p class="text-xs text-muted-foreground mt-0.5">"Render on the server for fast initial loads and SEO."</p>
                        </div>
                    </div>
                    <div class="flex items-start gap-3">
                        <div class="h-9 w-9 shrink-0 rounded-lg bg-amber-500/10 flex items-center justify-center mt-0.5"><i class="fas fa-route text-sm text-amber-500"></i></div>
                        <div>
                            <h3 class="text-sm font-semibold">"Built-in routing"</h3>
                            <p class="text-xs text-muted-foreground mt-0.5">"Client-side routing with dynamic parameters and nested routes."</p>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="testimonials" title="Testimonials Section" description="Customer or user testimonials in a grid."
                code={r#"rsx! {
    <section class="py-16">
        <h2 class="text-2xl font-bold text-center mb-8">"What developers say"</h2>
        <div class="grid md:grid-cols-2 gap-6">
            <div class="p-6 rounded-xl border border-border">
                <p class="text-sm italic mb-4">"Quote text..."</p>
                <div class="flex items-center gap-3">
                    <div class="h-10 w-10 rounded-full bg-primary/10"></div>
                    <div>
                        <p class="text-sm font-medium">"Name"</p>
                        <p class="text-xs text-muted-foreground">"Title"</p>
                    </div>
                </div>
            </div>
        </div>
    </section>
}"#}>
                <div class="w-full">
                    <h2 class="text-base font-bold text-center mb-4">"What developers say"</h2>
                    <div class="grid sm:grid-cols-2 gap-3">
                        <div class="p-4 rounded-lg border border-border">
                            <div class="flex gap-1 mb-2">
                                <i class="fas fa-star text-amber-400 text-[10px]"></i><i class="fas fa-star text-amber-400 text-[10px]"></i><i class="fas fa-star text-amber-400 text-[10px]"></i><i class="fas fa-star text-amber-400 text-[10px]"></i><i class="fas fa-star text-amber-400 text-[10px]"></i>
                            </div>
                            <p class="text-xs italic text-muted-foreground mb-3">"Momenta's reactive system is incredibly intuitive. Coming from React, the transition was seamless."</p>
                            <div class="flex items-center gap-2">
                                <div class="h-7 w-7 rounded-full bg-blue-500/10 flex items-center justify-center"><span class="text-[10px] font-bold text-blue-500">"SK"</span></div>
                                <div><p class="text-xs font-medium">"Sarah K."</p><p class="text-[10px] text-muted-foreground">"Frontend Lead"</p></div>
                            </div>
                        </div>
                        <div class="p-4 rounded-lg border border-border">
                            <div class="flex gap-1 mb-2">
                                <i class="fas fa-star text-amber-400 text-[10px]"></i><i class="fas fa-star text-amber-400 text-[10px]"></i><i class="fas fa-star text-amber-400 text-[10px]"></i><i class="fas fa-star text-amber-400 text-[10px]"></i><i class="fas fa-star text-amber-400 text-[10px]"></i>
                            </div>
                            <p class="text-xs italic text-muted-foreground mb-3">"The compile-time guarantees catch so many bugs. Our production error rate dropped 70%."</p>
                            <div class="flex items-center gap-2">
                                <div class="h-7 w-7 rounded-full bg-green-500/10 flex items-center justify-center"><span class="text-[10px] font-bold text-green-500">"MR"</span></div>
                                <div><p class="text-xs font-medium">"Mike R."</p><p class="text-[10px] text-muted-foreground">"CTO, StartupCo"</p></div>
                            </div>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="pricing-table" title="Pricing Table" description="Pricing cards with feature lists."
                code={r#"rsx! {
    <div class="grid md:grid-cols-3 gap-6">
        <div class="rounded-xl border border-border p-6">
            <h3 class="font-semibold">"Free"</h3>
            <p class="text-3xl font-bold mt-2">"$0"<span class="text-sm text-muted-foreground font-normal">"/mo"</span></p>
            <ul class="mt-6 space-y-2 text-sm">
                <li class="flex items-center gap-2"><i class="fas fa-check text-green-500"></i>"Feature 1"</li>
            </ul>
            <button class="w-full mt-6 px-4 py-2 border border-border rounded-lg text-sm">"Get Started"</button>
        </div>
    </div>
}"#}>
                <div class="w-full grid sm:grid-cols-3 gap-3">
                    <div class="rounded-lg border border-border p-4">
                        <h3 class="text-sm font-semibold">"Hobby"</h3>
                        <p class="text-xl font-bold mt-1">"$0"<span class="text-xs text-muted-foreground font-normal">"/mo"</span></p>
                        <ul class="mt-4 space-y-1.5 text-xs">
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-[10px]"></i>"Up to 3 projects"</li>
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-[10px]"></i>"Community support"</li>
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-[10px]"></i>"Basic analytics"</li>
                        </ul>
                        <button class="w-full mt-4 px-3 py-1.5 border border-border rounded-lg text-xs font-medium hover:bg-muted transition-colors">"Get Started"</button>
                    </div>
                    <div class="rounded-lg border-2 border-primary p-4 relative">
                        <div class="absolute -top-2.5 left-1/2 -translate-x-1/2 px-2 py-0.5 bg-primary text-primary-foreground text-[10px] font-medium rounded-full">"Popular"</div>
                        <h3 class="text-sm font-semibold">"Pro"</h3>
                        <p class="text-xl font-bold mt-1">"$19"<span class="text-xs text-muted-foreground font-normal">"/mo"</span></p>
                        <ul class="mt-4 space-y-1.5 text-xs">
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-[10px]"></i>"Unlimited projects"</li>
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-[10px]"></i>"Priority support"</li>
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-[10px]"></i>"Advanced analytics"</li>
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-[10px]"></i>"Custom domains"</li>
                        </ul>
                        <button class="w-full mt-4 px-3 py-1.5 bg-primary text-primary-foreground rounded-lg text-xs font-medium hover:bg-primary/90 transition-colors">"Start Free Trial"</button>
                    </div>
                    <div class="rounded-lg border border-border p-4">
                        <h3 class="text-sm font-semibold">"Enterprise"</h3>
                        <p class="text-xl font-bold mt-1">"Custom"</p>
                        <ul class="mt-4 space-y-1.5 text-xs">
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-[10px]"></i>"Everything in Pro"</li>
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-[10px]"></i>"SLA guarantee"</li>
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-[10px]"></i>"Dedicated support"</li>
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-[10px]"></i>"SSO / SAML"</li>
                        </ul>
                        <button class="w-full mt-4 px-3 py-1.5 border border-border rounded-lg text-xs font-medium hover:bg-muted transition-colors">"Contact Sales"</button>
                    </div>
                </div>
            </Showcase>

            <Showcase id="newsletter" title="Newsletter Signup" description="Email signup form for newsletters."
                code={r#"rsx! {
    <section class="rounded-xl bg-muted/30 border border-border p-8 text-center">
        <h2 class="text-xl font-bold mb-2">"Stay up to date"</h2>
        <p class="text-sm text-muted-foreground mb-6">"Get the latest news and updates."</p>
        <div class="flex gap-2 max-w-sm mx-auto">
            <input type="email" placeholder="Enter your email" class="flex-1 px-4 py-2 border border-border rounded-lg text-sm bg-background" />
            <button class="px-4 py-2 bg-primary text-primary-foreground rounded-lg text-sm">"Subscribe"</button>
        </div>
    </section>
}"#}>
                <div class="w-full">
                    <section class="rounded-lg bg-muted/30 border border-border p-6 text-center">
                        <i class="fas fa-envelope text-2xl text-primary/50 mb-3"></i>
                        <h2 class="text-base font-bold mb-1">"Stay up to date"</h2>
                        <p class="text-xs text-muted-foreground mb-4">"Get the latest Momenta news and release updates."</p>
                        <div class="flex gap-2 max-w-xs mx-auto">
                            <input type="email" placeholder="you@example.com" class="flex-1 px-3 py-2 border border-border rounded-lg text-xs bg-background min-w-0" />
                            <button class="px-3 py-2 bg-primary text-primary-foreground rounded-lg text-xs font-medium shrink-0">"Subscribe"</button>
                        </div>
                        <p class="text-[10px] text-muted-foreground mt-2">"No spam. Unsubscribe anytime."</p>
                    </section>
                </div>
            </Showcase>

            <Showcase id="logo-cloud" title="Logo Cloud" description="Grid of partner/client logos."
                code={r#"rsx! {
    <section class="py-12 text-center">
        <p class="text-sm text-muted-foreground mb-8">"Trusted by teams at"</p>
        <div class="flex flex-wrap items-center justify-center gap-8">
            <span class="text-2xl font-bold text-muted-foreground/50">"Acme"</span>
            <span class="text-2xl font-bold text-muted-foreground/50">"Globex"</span>
        </div>
    </section>
}"#}>
                <div class="w-full text-center">
                    <p class="text-xs text-muted-foreground mb-5">"Trusted by developers at"</p>
                    <div class="flex flex-wrap items-center justify-center gap-6 sm:gap-8">
                        <span class="text-lg font-bold text-muted-foreground/40">"Acme"</span>
                        <span class="text-lg font-bold text-muted-foreground/40">"Globex"</span>
                        <span class="text-lg font-bold text-muted-foreground/40">"Initech"</span>
                        <span class="text-lg font-bold text-muted-foreground/40">"Umbrella"</span>
                        <span class="text-lg font-bold text-muted-foreground/40">"Hooli"</span>
                    </div>
                </div>
            </Showcase>

            <Showcase id="stats-section" title="Stats Section" description="Display key metrics and numbers."
                code={r#"rsx! {
    <section class="grid grid-cols-2 md:grid-cols-4 gap-8 py-12">
        <div class="text-center">
            <p class="text-3xl font-bold">"10k+"</p>
            <p class="text-sm text-muted-foreground">"Developers"</p>
        </div>
    </section>
}"#}>
                <div class="w-full grid grid-cols-2 sm:grid-cols-4 gap-4 py-2">
                    <div class="text-center p-3">
                        <p class="text-2xl font-bold text-primary">"10k+"</p>
                        <p class="text-xs text-muted-foreground mt-0.5">"Developers"</p>
                    </div>
                    <div class="text-center p-3">
                        <p class="text-2xl font-bold text-primary">"500+"</p>
                        <p class="text-xs text-muted-foreground mt-0.5">"Projects"</p>
                    </div>
                    <div class="text-center p-3">
                        <p class="text-2xl font-bold text-primary">"99.9%"</p>
                        <p class="text-xs text-muted-foreground mt-0.5">"Uptime"</p>
                    </div>
                    <div class="text-center p-3">
                        <p class="text-2xl font-bold text-primary">"<1ms"</p>
                        <p class="text-xs text-muted-foreground mt-0.5">"Render time"</p>
                    </div>
                </div>
            </Showcase>

            <Showcase id="faq" title="FAQ Section" description="Frequently asked questions with expandable answers."
                code={r#"rsx! {
    <section class="max-w-2xl mx-auto py-16">
        <h2 class="text-2xl font-bold text-center mb-8">"FAQ"</h2>
        <div class="divide-y divide-border">
            <details class="py-4 group">
                <summary class="flex items-center justify-between cursor-pointer font-medium">
                    "Question here?"
                    <i class="fas fa-chevron-down group-open:rotate-180 transition-transform"></i>
                </summary>
                <p class="mt-3 text-sm text-muted-foreground">"Answer here."</p>
            </details>
        </div>
    </section>
}"#}>
                <div class="w-full max-w-md mx-auto">
                    <h2 class="text-base font-bold text-center mb-4">"Frequently Asked Questions"</h2>
                    <div class="divide-y divide-border rounded-lg border border-border overflow-hidden">
                        <details class="group" open={true}>
                            <summary class="flex items-center justify-between cursor-pointer px-4 py-3 text-sm font-medium hover:bg-muted/50">
                                "What is Momenta?"
                                <i class="fas fa-chevron-down text-xs text-muted-foreground group-open:rotate-180 transition-transform"></i>
                            </summary>
                            <p class="px-4 pb-3 text-xs text-muted-foreground">"Momenta is a reactive Rust framework for building web applications that compile to WebAssembly."</p>
                        </details>
                        <details class="group">
                            <summary class="flex items-center justify-between cursor-pointer px-4 py-3 text-sm font-medium hover:bg-muted/50">
                                "Do I need to know Rust?"
                                <i class="fas fa-chevron-down text-xs text-muted-foreground group-open:rotate-180 transition-transform"></i>
                            </summary>
                            <p class="px-4 pb-3 text-xs text-muted-foreground">"Basic Rust knowledge is helpful but our rsx! macro makes component authoring feel familiar to web developers."</p>
                        </details>
                        <details class="group">
                            <summary class="flex items-center justify-between cursor-pointer px-4 py-3 text-sm font-medium hover:bg-muted/50">
                                "Is it production-ready?"
                                <i class="fas fa-chevron-down text-xs text-muted-foreground group-open:rotate-180 transition-transform"></i>
                            </summary>
                            <p class="px-4 pb-3 text-xs text-muted-foreground">"Yes! Momenta is used in production by several companies and has a growing community."</p>
                        </details>
                    </div>
                </div>
            </Showcase>

            <div class="mt-8 flex items-center justify-between border-t border-border pt-6">
                <a href={docs_href("/ui/overlays")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "← Overlays"
                </a>
                <a href={docs_href("/ui/typography")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "Typography →"
                </a>
            </div>
        </article>
    }
}
