#![allow(unused_imports)]
use crate::components::*;
use alloc::{format, vec, vec::Vec};
use momenta::prelude::*;

#[component]
pub fn CardsPage() -> Node {
    rsx! {
        <article class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <CategoryHeader title="Cards" description="Flexible content containers with headers, footers, images, and various layouts for displaying grouped information." count={10} />

            <Showcase id="basic-card" title="Basic Card" description="A simple card with title and description."
                code={r#"rsx! {
    <div class="rounded-xl border border-border bg-card p-5">
        <h3 class="text-base font-semibold">"Card Title"</h3>
        <p class="mt-2 text-sm text-muted-foreground leading-relaxed">
            "This is a basic card component with a title and description."
        </p>
    </div>
}"#}>
                <div class="w-full grid gap-4 sm:grid-cols-2">
                    <div class="rounded-xl border border-border bg-card p-5">
                        <h3 class="text-base font-semibold">"Getting Started"</h3>
                        <p class="mt-2 text-sm text-muted-foreground leading-relaxed">"Install Momenta and create your first reactive component in minutes."</p>
                    </div>
                    <div class="rounded-xl border border-border bg-card p-5">
                        <h3 class="text-base font-semibold">"Documentation"</h3>
                        <p class="mt-2 text-sm text-muted-foreground leading-relaxed">"Comprehensive guides covering signals, components, and routing."</p>
                    </div>
                </div>
            </Showcase>

            <Showcase id="image-card" title="Card with Image" description="Card with a top image, commonly used for blog posts or products."
                code={r#"rsx! {
    <div class="rounded-xl border border-border bg-card overflow-hidden">
        <div class="h-48 bg-gradient-to-br from-primary/20 to-cyan-500/20"></div>
        <div class="p-5">
            <h3 class="text-base font-semibold">"Building with Momenta"</h3>
            <p class="mt-2 text-sm text-muted-foreground">"Learn reactive patterns."</p>
        </div>
    </div>
}"#}>
                <div class="w-full grid gap-4 sm:grid-cols-2">
                    <div class="rounded-xl border border-border bg-card overflow-hidden">
                        <div class="h-40 bg-gradient-to-br from-primary/20 via-violet-500/20 to-cyan-500/20 flex items-center justify-center">
                            <i class="fas fa-image text-3xl text-muted-foreground/30"></i>
                        </div>
                        <div class="p-5">
                            <div class="flex items-center gap-2 mb-2">
                                <span class="inline-flex items-center rounded-full bg-primary/10 text-primary px-2 py-0.5 text-[10px] font-medium">"Tutorial"</span>
                            </div>
                            <h3 class="text-base font-semibold">"Building with Momenta"</h3>
                            <p class="mt-1.5 text-sm text-muted-foreground leading-relaxed">"A step-by-step guide to building reactive web applications."</p>
                        </div>
                    </div>
                    <div class="rounded-xl border border-border bg-card overflow-hidden">
                        <div class="h-40 bg-gradient-to-br from-orange-500/20 via-red-500/20 to-pink-500/20 flex items-center justify-center">
                            <i class="fas fa-code text-3xl text-muted-foreground/30"></i>
                        </div>
                        <div class="p-5">
                            <div class="flex items-center gap-2 mb-2">
                                <span class="inline-flex items-center rounded-full bg-orange-500/10 text-orange-600 dark:text-orange-400 px-2 py-0.5 text-[10px] font-medium">"Deep Dive"</span>
                            </div>
                            <h3 class="text-base font-semibold">"Understanding Signals"</h3>
                            <p class="mt-1.5 text-sm text-muted-foreground leading-relaxed">"Fine-grained reactivity explained from first principles."</p>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="profile-card" title="Profile Card" description="User profile cards with avatar and details."
                code={r#"rsx! {
    <div class="rounded-xl border border-border bg-card p-5 text-center">
        <div class="mx-auto h-16 w-16 rounded-full bg-primary/10 flex items-center justify-center">
            <span class="text-xl font-bold text-primary">"JD"</span>
        </div>
        <h3 class="mt-3 text-base font-semibold">"Jane Doe"</h3>
        <p class="text-sm text-muted-foreground">"Senior Engineer"</p>
    </div>
}"#}>
                <div class="w-full grid gap-4 sm:grid-cols-3">
                    <div class="rounded-xl border border-border bg-card p-5 text-center">
                        <div class="mx-auto h-16 w-16 rounded-full bg-primary/10 flex items-center justify-center">
                            <span class="text-xl font-bold text-primary">"JD"</span>
                        </div>
                        <h3 class="mt-3 text-base font-semibold">"Jane Doe"</h3>
                        <p class="text-sm text-muted-foreground">"Senior Engineer"</p>
                        <div class="mt-3 flex justify-center gap-2">
                            <a href="#" class="text-muted-foreground hover:text-foreground transition-colors"><i class="fab fa-github"></i></a>
                            <a href="#" class="text-muted-foreground hover:text-foreground transition-colors"><i class="fab fa-twitter"></i></a>
                        </div>
                    </div>
                    <div class="rounded-xl border border-border bg-card p-5 text-center">
                        <div class="mx-auto h-16 w-16 rounded-full bg-green-500/10 flex items-center justify-center">
                            <span class="text-xl font-bold text-green-600 dark:text-green-400">"AB"</span>
                        </div>
                        <h3 class="mt-3 text-base font-semibold">"Alex Brown"</h3>
                        <p class="text-sm text-muted-foreground">"Product Designer"</p>
                        <div class="mt-3 flex justify-center gap-2">
                            <a href="#" class="text-muted-foreground hover:text-foreground transition-colors"><i class="fab fa-dribbble"></i></a>
                            <a href="#" class="text-muted-foreground hover:text-foreground transition-colors"><i class="fab fa-figma"></i></a>
                        </div>
                    </div>
                    <div class="rounded-xl border border-border bg-card p-5 text-center">
                        <div class="mx-auto h-16 w-16 rounded-full bg-violet-500/10 flex items-center justify-center">
                            <span class="text-xl font-bold text-violet-600 dark:text-violet-400">"MK"</span>
                        </div>
                        <h3 class="mt-3 text-base font-semibold">"Max Kim"</h3>
                        <p class="text-sm text-muted-foreground">"DevOps Lead"</p>
                        <div class="mt-3 flex justify-center gap-2">
                            <a href="#" class="text-muted-foreground hover:text-foreground transition-colors"><i class="fab fa-linkedin"></i></a>
                            <a href="#" class="text-muted-foreground hover:text-foreground transition-colors"><i class="fas fa-globe"></i></a>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="pricing-card" title="Pricing Card" description="Pricing tier card with features list and CTA."
                code={r#"rsx! {
    <div class="rounded-xl border-2 border-primary bg-card p-6">
        <span class="text-xs font-semibold text-primary uppercase">"Popular"</span>
        <h3 class="mt-2 text-2xl font-bold">"Pro"</h3>
        <div class="mt-1"><span class="text-3xl font-bold">"$29"</span><span class="text-muted-foreground">/mo</span></div>
        <ul class="mt-4 space-y-2 text-sm">
            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-xs"></i>"Unlimited projects"</li>
        </ul>
        <button class="mt-6 w-full rounded-lg bg-primary py-2 text-sm font-medium text-primary-foreground">"Get Started"</button>
    </div>
}"#}>
                <div class="w-full grid gap-4 sm:grid-cols-3">
                    <div class="rounded-xl border border-border bg-card p-6">
                        <span class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">"Starter"</span>
                        <h3 class="mt-2 text-2xl font-bold">"Free"</h3>
                        <p class="mt-1 text-sm text-muted-foreground">"For side projects"</p>
                        <ul class="mt-4 space-y-2.5 text-sm">
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-xs"></i>"3 projects"</li>
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-xs"></i>"1 GB storage"</li>
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-xs"></i>"Community support"</li>
                        </ul>
                        <button class="mt-6 w-full rounded-lg border border-border py-2 text-sm font-medium hover:bg-muted transition-colors">"Get Started"</button>
                    </div>
                    <div class="rounded-xl border-2 border-primary bg-card p-6 relative">
                        <span class="absolute -top-3 left-1/2 -translate-x-1/2 inline-flex items-center rounded-full bg-primary px-3 py-0.5 text-[10px] font-semibold text-primary-foreground">"POPULAR"</span>
                        <span class="text-xs font-semibold text-primary uppercase tracking-wider">"Pro"</span>
                        <h3 class="mt-2 text-2xl font-bold">"$29"<span class="text-base font-normal text-muted-foreground">"/mo"</span></h3>
                        <p class="mt-1 text-sm text-muted-foreground">"For growing teams"</p>
                        <ul class="mt-4 space-y-2.5 text-sm">
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-xs"></i>"Unlimited projects"</li>
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-xs"></i>"100 GB storage"</li>
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-xs"></i>"Priority support"</li>
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-xs"></i>"Custom domains"</li>
                        </ul>
                        <button class="mt-6 w-full rounded-lg bg-primary py-2 text-sm font-medium text-primary-foreground hover:opacity-90 transition-opacity">"Get Started"</button>
                    </div>
                    <div class="rounded-xl border border-border bg-card p-6">
                        <span class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">"Enterprise"</span>
                        <h3 class="mt-2 text-2xl font-bold">"Custom"</h3>
                        <p class="mt-1 text-sm text-muted-foreground">"For large orgs"</p>
                        <ul class="mt-4 space-y-2.5 text-sm">
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-xs"></i>"Everything in Pro"</li>
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-xs"></i>"SSO & SAML"</li>
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-xs"></i>"Dedicated support"</li>
                            <li class="flex items-center gap-2"><i class="fas fa-check text-green-500 text-xs"></i>"SLA guarantee"</li>
                        </ul>
                        <button class="mt-6 w-full rounded-lg border border-border py-2 text-sm font-medium hover:bg-muted transition-colors">"Contact Sales"</button>
                    </div>
                </div>
            </Showcase>

            <Showcase id="stat-card" title="Stat Card" description="Compact cards for displaying metrics and KPIs."
                code={r#"rsx! {
    <div class="rounded-xl border border-border bg-card p-5">
        <div class="flex items-center justify-between">
            <span class="text-sm text-muted-foreground">"Total Revenue"</span>
            <i class="fas fa-dollar-sign text-muted-foreground/50"></i>
        </div>
        <div class="mt-2 text-2xl font-bold">"$45,231"</div>
        <p class="mt-1 text-xs text-green-600">"+20.1% from last month"</p>
    </div>
}"#}>
                <div class="w-full grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
                    <div class="rounded-xl border border-border bg-card p-5">
                        <div class="flex items-center justify-between">
                            <span class="text-sm text-muted-foreground">"Total Revenue"</span>
                            <i class="fas fa-dollar-sign text-muted-foreground/40"></i>
                        </div>
                        <div class="mt-2 text-2xl font-bold">"$45,231"</div>
                        <p class="mt-1 text-xs text-green-600 dark:text-green-400">"+20.1% from last month"</p>
                    </div>
                    <div class="rounded-xl border border-border bg-card p-5">
                        <div class="flex items-center justify-between">
                            <span class="text-sm text-muted-foreground">"Users"</span>
                            <i class="fas fa-users text-muted-foreground/40"></i>
                        </div>
                        <div class="mt-2 text-2xl font-bold">"2,350"</div>
                        <p class="mt-1 text-xs text-green-600 dark:text-green-400">"+180 this week"</p>
                    </div>
                    <div class="rounded-xl border border-border bg-card p-5">
                        <div class="flex items-center justify-between">
                            <span class="text-sm text-muted-foreground">"Active Now"</span>
                            <i class="fas fa-signal text-muted-foreground/40"></i>
                        </div>
                        <div class="mt-2 text-2xl font-bold">"573"</div>
                        <p class="mt-1 text-xs text-green-600 dark:text-green-400">"+201 since last hour"</p>
                    </div>
                    <div class="rounded-xl border border-border bg-card p-5">
                        <div class="flex items-center justify-between">
                            <span class="text-sm text-muted-foreground">"Bounce Rate"</span>
                            <i class="fas fa-chart-line text-muted-foreground/40"></i>
                        </div>
                        <div class="mt-2 text-2xl font-bold">"12.5%"</div>
                        <p class="mt-1 text-xs text-red-600 dark:text-red-400">"+2.1% from last month"</p>
                    </div>
                </div>
            </Showcase>

            <Showcase id="product-card" title="Product Card" description="E-commerce product cards with price and action."
                code={r#"rsx! {
    <div class="rounded-xl border border-border bg-card overflow-hidden group">
        <div class="h-48 bg-muted flex items-center justify-center">
            <i class="fas fa-box text-4xl text-muted-foreground/30"></i>
        </div>
        <div class="p-4">
            <h3 class="font-semibold">"Product Name"</h3>
            <p class="text-lg font-bold mt-1">"$99.00"</p>
            <button class="mt-3 w-full rounded-lg bg-primary py-2 text-sm font-medium text-primary-foreground">"Add to Cart"</button>
        </div>
    </div>
}"#}>
                <div class="w-full grid gap-4 sm:grid-cols-3">
                    <div class="rounded-xl border border-border bg-card overflow-hidden group">
                        <div class="h-40 bg-gradient-to-br from-blue-500/10 to-cyan-500/10 flex items-center justify-center relative">
                            <i class="fas fa-laptop text-4xl text-blue-500/30"></i>
                            <span class="absolute top-2 right-2 rounded-full bg-red-500 px-2 py-0.5 text-[10px] font-bold text-white">"SALE"</span>
                        </div>
                        <div class="p-4">
                            <p class="text-xs text-muted-foreground">"Electronics"</p>
                            <h3 class="font-semibold mt-0.5">"MacBook Pro"</h3>
                            <div class="flex items-center gap-2 mt-1">
                                <p class="text-base font-bold">"$1,299"</p>
                                <p class="text-sm text-muted-foreground line-through">"$1,499"</p>
                            </div>
                            <button class="mt-3 w-full rounded-lg bg-primary py-2 text-sm font-medium text-primary-foreground hover:opacity-90 transition-opacity">"Add to Cart"</button>
                        </div>
                    </div>
                    <div class="rounded-xl border border-border bg-card overflow-hidden group">
                        <div class="h-40 bg-gradient-to-br from-purple-500/10 to-pink-500/10 flex items-center justify-center">
                            <i class="fas fa-headphones text-4xl text-purple-500/30"></i>
                        </div>
                        <div class="p-4">
                            <p class="text-xs text-muted-foreground">"Audio"</p>
                            <h3 class="font-semibold mt-0.5">"Wireless Headphones"</h3>
                            <div class="flex items-center gap-1 mt-1">
                                <div class="flex text-amber-400 text-xs">
                                    <i class="fas fa-star"></i><i class="fas fa-star"></i><i class="fas fa-star"></i><i class="fas fa-star"></i><i class="fas fa-star-half-alt"></i>
                                </div>
                                <span class="text-xs text-muted-foreground">"(128)"</span>
                            </div>
                            <p class="text-base font-bold mt-1">"$79.99"</p>
                            <button class="mt-3 w-full rounded-lg bg-primary py-2 text-sm font-medium text-primary-foreground hover:opacity-90 transition-opacity">"Add to Cart"</button>
                        </div>
                    </div>
                    <div class="rounded-xl border border-border bg-card overflow-hidden group">
                        <div class="h-40 bg-gradient-to-br from-green-500/10 to-emerald-500/10 flex items-center justify-center">
                            <i class="fas fa-keyboard text-4xl text-green-500/30"></i>
                        </div>
                        <div class="p-4">
                            <p class="text-xs text-muted-foreground">"Accessories"</p>
                            <h3 class="font-semibold mt-0.5">"Mechanical Keyboard"</h3>
                            <p class="text-base font-bold mt-1">"$149.00"</p>
                            <button class="mt-3 w-full rounded-lg border border-border py-2 text-sm font-medium hover:bg-muted transition-colors">"View Details"</button>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="feature-card" title="Feature Card" description="Cards highlighting a feature or capability."
                code={r#"rsx! {
    <div class="rounded-xl border border-border bg-card p-5">
        <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10 text-primary">
            <i class="fas fa-bolt"></i>
        </div>
        <h3 class="mt-3 text-base font-semibold">"Lightning Fast"</h3>
        <p class="mt-1.5 text-sm text-muted-foreground">"Zero-cost abstractions compiled to efficient WebAssembly."</p>
    </div>
}"#}>
                <div class="w-full grid gap-4 sm:grid-cols-3">
                    <div class="rounded-xl border border-border bg-card p-5 hover:border-primary/30 transition-colors">
                        <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10 text-primary">
                            <i class="fas fa-bolt"></i>
                        </div>
                        <h3 class="mt-3 text-base font-semibold">"Lightning Fast"</h3>
                        <p class="mt-1.5 text-sm text-muted-foreground leading-relaxed">"Zero-cost abstractions compiled to efficient WebAssembly."</p>
                    </div>
                    <div class="rounded-xl border border-border bg-card p-5 hover:border-primary/30 transition-colors">
                        <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-green-500/10 text-green-600 dark:text-green-400">
                            <i class="fas fa-shield-alt"></i>
                        </div>
                        <h3 class="mt-3 text-base font-semibold">"Type Safe"</h3>
                        <p class="mt-1.5 text-sm text-muted-foreground leading-relaxed">"Catch errors at compile time with Rust's type system."</p>
                    </div>
                    <div class="rounded-xl border border-border bg-card p-5 hover:border-primary/30 transition-colors">
                        <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-violet-500/10 text-violet-600 dark:text-violet-400">
                            <i class="fas fa-cubes"></i>
                        </div>
                        <h3 class="mt-3 text-base font-semibold">"Composable"</h3>
                        <p class="mt-1.5 text-sm text-muted-foreground leading-relaxed">"Build complex UIs from simple, reusable components."</p>
                    </div>
                </div>
            </Showcase>

            <Showcase id="testimonial-card" title="Testimonial Card" description="Customer quote cards for social proof."
                code={r#"rsx! {
    <div class="rounded-xl border border-border bg-card p-5">
        <div class="flex gap-1 text-amber-400 text-xs mb-3">
            <i class="fas fa-star"></i><i class="fas fa-star"></i><i class="fas fa-star"></i><i class="fas fa-star"></i><i class="fas fa-star"></i>
        </div>
        <p class="text-sm leading-relaxed italic">"Momenta lets me build UIs faster than any other Rust framework."</p>
        <div class="mt-4 flex items-center gap-3">
            <div class="h-9 w-9 rounded-full bg-primary/10 flex items-center justify-center text-xs font-bold text-primary">"SR"</div>
            <div>
                <p class="text-sm font-medium">"Sarah R."</p>
                <p class="text-xs text-muted-foreground">"Full-stack Developer"</p>
            </div>
        </div>
    </div>
}"#}>
                <div class="w-full grid gap-4 sm:grid-cols-2">
                    <div class="rounded-xl border border-border bg-card p-5">
                        <div class="flex gap-1 text-amber-400 text-xs mb-3">
                            <i class="fas fa-star"></i><i class="fas fa-star"></i><i class="fas fa-star"></i><i class="fas fa-star"></i><i class="fas fa-star"></i>
                        </div>
                        <p class="text-sm leading-relaxed">"\"Momenta combined the best of React's DX with Rust's performance. I shipped my side project in a weekend.\""</p>
                        <div class="mt-4 flex items-center gap-3">
                            <div class="h-9 w-9 rounded-full bg-primary/10 flex items-center justify-center text-xs font-bold text-primary">"SR"</div>
                            <div>
                                <p class="text-sm font-medium">"Sarah R."</p>
                                <p class="text-xs text-muted-foreground">"Full-stack Developer"</p>
                            </div>
                        </div>
                    </div>
                    <div class="rounded-xl border border-border bg-card p-5">
                        <div class="flex gap-1 text-amber-400 text-xs mb-3">
                            <i class="fas fa-star"></i><i class="fas fa-star"></i><i class="fas fa-star"></i><i class="fas fa-star"></i><i class="fas fa-star"></i>
                        </div>
                        <p class="text-sm leading-relaxed">"\"The signal system is incredibly intuitive. Coming from SolidJS, I felt right at home with Momenta's reactivity model.\""</p>
                        <div class="mt-4 flex items-center gap-3">
                            <div class="h-9 w-9 rounded-full bg-green-500/10 flex items-center justify-center text-xs font-bold text-green-600 dark:text-green-400">"MT"</div>
                            <div>
                                <p class="text-sm font-medium">"Mike T."</p>
                                <p class="text-xs text-muted-foreground">"Frontend Architect"</p>
                            </div>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="horizontal-card" title="Horizontal Card" description="Side-by-side layout for compact information."
                code={r#"rsx! {
    <div class="flex rounded-xl border border-border bg-card overflow-hidden">
        <div class="w-32 shrink-0 bg-muted flex items-center justify-center">
            <i class="fas fa-image text-2xl text-muted-foreground/30"></i>
        </div>
        <div class="p-4">
            <h3 class="font-semibold">"Article Title"</h3>
            <p class="mt-1 text-sm text-muted-foreground">"Brief description here."</p>
        </div>
    </div>
}"#}>
                <div class="w-full space-y-3">
                    <div class="flex rounded-xl border border-border bg-card overflow-hidden">
                        <div class="w-28 shrink-0 bg-gradient-to-br from-primary/10 to-cyan-500/10 flex items-center justify-center">
                            <i class="fas fa-newspaper text-2xl text-primary/30"></i>
                        </div>
                        <div class="p-4 min-w-0">
                            <span class="text-[10px] font-medium text-primary uppercase tracking-wider">"News"</span>
                            <h3 class="font-semibold mt-0.5 truncate">"Momenta 2.0 Released with SSR Support"</h3>
                            <p class="mt-1 text-sm text-muted-foreground line-clamp-2">"The latest release brings server-side rendering, hydration, and streaming HTML."</p>
                        </div>
                    </div>
                    <div class="flex rounded-xl border border-border bg-card overflow-hidden">
                        <div class="w-28 shrink-0 bg-gradient-to-br from-amber-500/10 to-orange-500/10 flex items-center justify-center">
                            <i class="fas fa-podcast text-2xl text-amber-500/30"></i>
                        </div>
                        <div class="p-4 min-w-0">
                            <span class="text-[10px] font-medium text-amber-600 dark:text-amber-400 uppercase tracking-wider">"Podcast"</span>
                            <h3 class="font-semibold mt-0.5 truncate">"Ep. 42: The Future of Rust on the Web"</h3>
                            <p class="mt-1 text-sm text-muted-foreground">"A conversation about WebAssembly and reactive frameworks."</p>
                        </div>
                    </div>
                </div>
            </Showcase>

            <Showcase id="card-footer" title="Card with Footer" description="Card with distinct header and footer areas."
                code={r#"rsx! {
    <div class="rounded-xl border border-border bg-card overflow-hidden">
        <div class="p-5">
            <h3 class="text-base font-semibold">"Project Alpha"</h3>
            <p class="mt-1 text-sm text-muted-foreground">"A reactive web application."</p>
        </div>
        <div class="border-t border-border bg-muted/30 px-5 py-3 flex items-center justify-between">
            <span class="text-xs text-muted-foreground">"Updated 2 hours ago"</span>
            <button class="text-xs text-primary font-medium">"View →"</button>
        </div>
    </div>
}"#}>
                <div class="w-full grid gap-4 sm:grid-cols-2">
                    <div class="rounded-xl border border-border bg-card overflow-hidden">
                        <div class="p-5">
                            <div class="flex items-center gap-2">
                                <div class="h-8 w-8 rounded-lg bg-primary/10 flex items-center justify-center">
                                    <i class="fas fa-folder text-primary text-sm"></i>
                                </div>
                                <div>
                                    <h3 class="text-sm font-semibold">"Project Alpha"</h3>
                                    <p class="text-xs text-muted-foreground">"Reactive web app"</p>
                                </div>
                            </div>
                            <p class="mt-3 text-sm text-muted-foreground leading-relaxed">"A full-stack application built with Momenta and server-side rendering."</p>
                        </div>
                        <div class="border-t border-border bg-muted/30 px-5 py-3 flex items-center justify-between">
                            <span class="text-xs text-muted-foreground">"Updated 2h ago"</span>
                            <button class="text-xs text-primary font-medium hover:underline">"View →"</button>
                        </div>
                    </div>
                    <div class="rounded-xl border border-border bg-card overflow-hidden">
                        <div class="p-5">
                            <div class="flex items-center gap-2">
                                <div class="h-8 w-8 rounded-lg bg-green-500/10 flex items-center justify-center">
                                    <i class="fas fa-code-branch text-green-600 dark:text-green-400 text-sm"></i>
                                </div>
                                <div>
                                    <h3 class="text-sm font-semibold">"UI Library"</h3>
                                    <p class="text-xs text-muted-foreground">"Component system"</p>
                                </div>
                            </div>
                            <p class="mt-3 text-sm text-muted-foreground leading-relaxed">"A comprehensive collection of reusable UI components for Momenta apps."</p>
                        </div>
                        <div class="border-t border-border bg-muted/30 px-5 py-3 flex items-center justify-between">
                            <span class="text-xs text-muted-foreground">"Updated 5m ago"</span>
                            <button class="text-xs text-primary font-medium hover:underline">"View →"</button>
                        </div>
                    </div>
                </div>
            </Showcase>

            <div class="mt-8 flex items-center justify-between border-t border-border pt-6">
                <a href={docs_href("/ui/alerts")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "← Alerts & Banners"
                </a>
                <a href={docs_href("/ui/inputs")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "Forms & Inputs →"
                </a>
            </div>
        </article>
    }
}
