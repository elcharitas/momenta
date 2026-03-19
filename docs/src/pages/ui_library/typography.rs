#![allow(unused_imports)]
use crate::components::*;
use alloc::{format, vec, vec::Vec};
use momenta::prelude::*;

#[component]
pub fn TypographyPage() -> Node {
    rsx! {
        <article class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <CategoryHeader title="Typography" description="Text styles, headings, prose elements, and inline formatting components." count={6} />

            <Showcase id="headings" title="Headings" description="Heading hierarchy from H1 to H6."
                code={r#"rsx! {
    <h1 class="text-4xl font-bold tracking-tight">"Heading 1"</h1>
    <h2 class="text-3xl font-bold tracking-tight">"Heading 2"</h2>
    <h3 class="text-2xl font-semibold">"Heading 3"</h3>
    <h4 class="text-xl font-semibold">"Heading 4"</h4>
    <h5 class="text-lg font-medium">"Heading 5"</h5>
    <h6 class="text-base font-medium">"Heading 6"</h6>
}"#}>
                <div class="w-full space-y-3">
                    <h1 class="text-3xl font-bold tracking-tight">"Heading 1"</h1>
                    <h2 class="text-2xl font-bold tracking-tight">"Heading 2"</h2>
                    <h3 class="text-xl font-semibold">"Heading 3"</h3>
                    <h4 class="text-lg font-semibold">"Heading 4"</h4>
                    <h5 class="text-base font-medium">"Heading 5"</h5>
                    <h6 class="text-sm font-medium text-muted-foreground">"Heading 6"</h6>
                </div>
            </Showcase>

            <Showcase id="paragraphs" title="Paragraphs" description="Text paragraph styles with various sizes and weights."
                code={r#"rsx! {
    <p class="text-lg leading-relaxed">"Lead paragraph with larger text."</p>
    <p class="text-base leading-relaxed text-muted-foreground">"Standard body text."</p>
    <p class="text-sm text-muted-foreground">"Small helper text."</p>
    <p class="text-xs text-muted-foreground">"Caption text."</p>
}"#}>
                <div class="w-full space-y-4">
                    <div>
                        <span class="text-[10px] uppercase tracking-wider text-muted-foreground mb-1 block">"Lead"</span>
                        <p class="text-base leading-relaxed">"Momenta is a reactive Rust framework that compiles to WebAssembly for building modern, high-performance web applications."</p>
                    </div>
                    <div>
                        <span class="text-[10px] uppercase tracking-wider text-muted-foreground mb-1 block">"Body"</span>
                        <p class="text-sm leading-relaxed text-muted-foreground">"Components are defined using the rsx! macro, which provides a familiar JSX-like syntax while leveraging Rust's type system for compile-time guarantees."</p>
                    </div>
                    <div>
                        <span class="text-[10px] uppercase tracking-wider text-muted-foreground mb-1 block">"Small"</span>
                        <p class="text-xs text-muted-foreground">"This is small helper text, useful for captions or metadata."</p>
                    </div>
                    <div>
                        <span class="text-[10px] uppercase tracking-wider text-muted-foreground mb-1 block">"Muted"</span>
                        <p class="text-sm text-muted-foreground/60">"Muted text for secondary information that doesn't need visual emphasis."</p>
                    </div>
                </div>
            </Showcase>

            <Showcase id="blockquote" title="Blockquote" description="Styled blockquotes for highlighting quotes or callouts."
                code={r#"rsx! {
    <blockquote class="border-l-4 border-primary pl-4 py-2 italic text-muted-foreground">
        <p>"The best way to predict the future is to invent it."</p>
        <footer class="mt-2 text-sm not-italic">
            "— Alan Kay"
        </footer>
    </blockquote>
}"#}>
                <div class="w-full space-y-4">
                    <blockquote class="border-l-4 border-primary pl-4 py-1">
                        <p class="text-sm italic text-muted-foreground">"The best way to predict the future is to invent it."</p>
                        <footer class="mt-1.5 text-xs not-italic text-muted-foreground/70">"— Alan Kay"</footer>
                    </blockquote>
                    <blockquote class="border-l-4 border-amber-500 pl-4 py-1 bg-amber-500/5 rounded-r-lg pr-4">
                        <p class="text-sm italic text-muted-foreground">"Simplicity is the ultimate sophistication."</p>
                        <footer class="mt-1.5 text-xs not-italic text-muted-foreground/70">"— Leonardo da Vinci"</footer>
                    </blockquote>
                </div>
            </Showcase>

            <Showcase id="inline-code" title="Inline Code" description="Code spans and formatted code text."
                code={r#"rsx! {
    <p>
        "Use " <code class="px-1.5 py-0.5 bg-muted rounded text-sm font-mono">"create_signal()"</code>
        " to create a new reactive signal."
    </p>
}"#}>
                <div class="w-full space-y-3">
                    <p class="text-sm">
                        "Use "
                        <code class="px-1.5 py-0.5 bg-muted rounded text-xs font-mono">"create_signal()"</code>
                        " to create a new reactive signal."
                    </p>
                    <p class="text-sm">
                        "Install with "
                        <code class="px-1.5 py-0.5 bg-muted rounded text-xs font-mono">"cargo add momenta"</code>
                        " and add to your "
                        <code class="px-1.5 py-0.5 bg-muted rounded text-xs font-mono">"Cargo.toml"</code>
                        "."
                    </p>
                    <p class="text-sm">
                        "The "
                        <code class="px-1.5 py-0.5 bg-primary/10 text-primary rounded text-xs font-mono">"#[component]"</code>
                        " attribute marks a function as a component."
                    </p>
                </div>
            </Showcase>

            <Showcase id="lists" title="Lists" description="Ordered and unordered lists with various styles."
                code={r#"rsx! {
    <ul class="list-disc list-inside space-y-1 text-sm">
        <li>"First item"</li>
        <li>"Second item"</li>
    </ul>
    <ol class="list-decimal list-inside space-y-1 text-sm">
        <li>"Step one"</li>
        <li>"Step two"</li>
    </ol>
}"#}>
                <div class="w-full grid sm:grid-cols-2 gap-6">
                    <div>
                        <p class="text-xs uppercase tracking-wider text-muted-foreground mb-2">"Unordered"</p>
                        <ul class="space-y-1.5 text-sm">
                            <li class="flex items-center gap-2"><span class="h-1.5 w-1.5 rounded-full bg-foreground shrink-0"></span>"Reactive signals system"</li>
                            <li class="flex items-center gap-2"><span class="h-1.5 w-1.5 rounded-full bg-foreground shrink-0"></span>"Component-based architecture"</li>
                            <li class="flex items-center gap-2"><span class="h-1.5 w-1.5 rounded-full bg-foreground shrink-0"></span>"Server-side rendering"</li>
                            <li class="flex items-center gap-2"><span class="h-1.5 w-1.5 rounded-full bg-foreground shrink-0"></span>"Built-in router"</li>
                        </ul>
                    </div>
                    <div>
                        <p class="text-xs uppercase tracking-wider text-muted-foreground mb-2">"Ordered"</p>
                        <ol class="space-y-1.5 text-sm">
                            <li class="flex items-center gap-2"><span class="text-xs font-bold text-primary w-5 shrink-0">"1."</span>"Install Rust and wasm-pack"</li>
                            <li class="flex items-center gap-2"><span class="text-xs font-bold text-primary w-5 shrink-0">"2."</span>"Create a new project"</li>
                            <li class="flex items-center gap-2"><span class="text-xs font-bold text-primary w-5 shrink-0">"3."</span>"Add Momenta as dependency"</li>
                            <li class="flex items-center gap-2"><span class="text-xs font-bold text-primary w-5 shrink-0">"4."</span>"Build and serve"</li>
                        </ol>
                    </div>
                    <div class="sm:col-span-2">
                        <p class="text-xs uppercase tracking-wider text-muted-foreground mb-2">"Checklist"</p>
                        <ul class="space-y-1.5 text-sm">
                            <li class="flex items-center gap-2"><i class="fas fa-check-square text-xs text-green-500"></i><span class="line-through text-muted-foreground">"Set up project structure"</span></li>
                            <li class="flex items-center gap-2"><i class="fas fa-check-square text-xs text-green-500"></i><span class="line-through text-muted-foreground">"Define components"</span></li>
                            <li class="flex items-center gap-2"><i class="far fa-square text-xs text-muted-foreground"></i>"Add routing"</li>
                            <li class="flex items-center gap-2"><i class="far fa-square text-xs text-muted-foreground"></i>"Deploy to production"</li>
                        </ul>
                    </div>
                </div>
            </Showcase>

            <Showcase id="kbd" title="Keyboard Shortcuts" description="Keyboard key indicators for shortcuts and commands."
                code={r#"rsx! {
    <div class="flex items-center gap-1">
        <kbd class="px-2 py-1 text-xs bg-muted border border-border rounded font-mono">"Ctrl"</kbd>
        <span class="text-xs text-muted-foreground">"+"</span>
        <kbd class="px-2 py-1 text-xs bg-muted border border-border rounded font-mono">"S"</kbd>
        <span class="text-sm ml-2">"Save"</span>
    </div>
}"#}>
                <div class="w-full space-y-3">
                    <div class="flex items-center gap-4">
                        <div class="flex items-center gap-1">
                            <kbd class="px-2 py-1 text-xs bg-muted border border-border rounded font-mono shadow-sm">"⌘"</kbd>
                            <span class="text-xs text-muted-foreground">"+"</span>
                            <kbd class="px-2 py-1 text-xs bg-muted border border-border rounded font-mono shadow-sm">"S"</kbd>
                        </div>
                        <span class="text-sm text-muted-foreground">"Save"</span>
                    </div>
                    <div class="flex items-center gap-4">
                        <div class="flex items-center gap-1">
                            <kbd class="px-2 py-1 text-xs bg-muted border border-border rounded font-mono shadow-sm">"⌘"</kbd>
                            <span class="text-xs text-muted-foreground">"+"</span>
                            <kbd class="px-2 py-1 text-xs bg-muted border border-border rounded font-mono shadow-sm">"⇧"</kbd>
                            <span class="text-xs text-muted-foreground">"+"</span>
                            <kbd class="px-2 py-1 text-xs bg-muted border border-border rounded font-mono shadow-sm">"P"</kbd>
                        </div>
                        <span class="text-sm text-muted-foreground">"Command Palette"</span>
                    </div>
                    <div class="flex items-center gap-4">
                        <div class="flex items-center gap-1">
                            <kbd class="px-2 py-1 text-xs bg-muted border border-border rounded font-mono shadow-sm">"⌘"</kbd>
                            <span class="text-xs text-muted-foreground">"+"</span>
                            <kbd class="px-2 py-1 text-xs bg-muted border border-border rounded font-mono shadow-sm">"K"</kbd>
                        </div>
                        <span class="text-sm text-muted-foreground">"Search"</span>
                    </div>
                    <div class="flex items-center gap-4">
                        <kbd class="px-2.5 py-1 text-xs bg-muted border border-border rounded font-mono shadow-sm">"Esc"</kbd>
                        <span class="text-sm text-muted-foreground">"Close / Cancel"</span>
                    </div>
                </div>
            </Showcase>

            <div class="mt-8 flex items-center justify-between border-t border-border pt-6">
                <a href={docs_href("/ui/marketing")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "← Marketing"
                </a>
                <a href={docs_href("/ui")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "Back to Overview →"
                </a>
            </div>
        </article>
    }
}
