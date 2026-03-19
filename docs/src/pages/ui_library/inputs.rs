#![allow(unused_imports)]
use crate::components::*;
use alloc::{format, vec, vec::Vec};
use momenta::prelude::*;

#[component]
pub fn InputsPage() -> Node {
    rsx! {
        <article class="px-6 py-10 sm:px-8 lg:px-10 fade-in">
            <CategoryHeader title="Forms & Inputs" description="Form controls for collecting user input. Built with accessibility in mind, using proper labels and ARIA attributes." count={14} />

            <Showcase id="text-input" title="Text Input" description="Basic single-line text input field."
                code={r#"rsx! {
    <input type="text" placeholder="Enter your name"
        class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring" />
}"#}>
                <div class="w-full max-w-sm">
                    <input type="text" placeholder="Enter your name" class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring" />
                </div>
            </Showcase>

            <Showcase id="input-label" title="Input with Label" description="Labeled input field with helper text."
                code={r#"rsx! {
    <div class="space-y-1.5">
        <label class="text-sm font-medium">"Email address"</label>
        <input type="email" placeholder="you@example.com"
            class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring" />
        <p class="text-xs text-muted-foreground">"We'll never share your email."</p>
    </div>
}"#}>
                <div class="w-full max-w-sm space-y-1.5">
                    <label class="text-sm font-medium">"Email address"</label>
                    <input type="email" placeholder="you@example.com" class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring" />
                    <p class="text-xs text-muted-foreground">"We'll never share your email."</p>
                </div>
            </Showcase>

            <Showcase id="input-error" title="Input with Error" description="Input field showing validation error state."
                code={r#"rsx! {
    <div class="space-y-1.5">
        <label class="text-sm font-medium">"Password"</label>
        <input type="password" value="short"
            class="w-full rounded-lg border border-red-500 bg-background px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-red-500" />
        <p class="text-xs text-red-500">"Password must be at least 8 characters."</p>
    </div>
}"#}>
                <div class="w-full max-w-sm space-y-1.5">
                    <label class="text-sm font-medium">"Password"</label>
                    <input type="password" value="short" class="w-full rounded-lg border border-red-500 bg-background px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-red-500" />
                    <p class="text-xs text-red-500">"Password must be at least 8 characters."</p>
                </div>
            </Showcase>

            <Showcase id="textarea" title="Textarea" description="Multi-line text input for longer content."
                code={r#"rsx! {
    <div class="space-y-1.5">
        <label class="text-sm font-medium">"Message"</label>
        <textarea rows="3" placeholder="Type your message..."
            class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring resize-none"></textarea>
    </div>
}"#}>
                <div class="w-full max-w-sm space-y-1.5">
                    <label class="text-sm font-medium">"Message"</label>
                    <textarea rows={3} placeholder="Type your message here..." class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring resize-none"></textarea>
                </div>
            </Showcase>

            <Showcase id="select-input" title="Select" description="Dropdown selection control."
                code={r#"rsx! {
    <div class="space-y-1.5">
        <label class="text-sm font-medium">"Country"</label>
        <select class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-ring">
            <option>"Select a country"</option>
            <option>"United States"</option>
            <option>"United Kingdom"</option>
        </select>
    </div>
}"#}>
                <div class="w-full max-w-sm space-y-1.5">
                    <label class="text-sm font-medium">"Country"</label>
                    <select class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-ring appearance-none">
                        <option>"Select a country"</option>
                        <option>"United States"</option>
                        <option>"United Kingdom"</option>
                        <option>"Germany"</option>
                        <option>"Japan"</option>
                    </select>
                </div>
            </Showcase>

            <Showcase id="checkbox" title="Checkbox" description="Toggle option with label."
                code={r#"rsx! {
    <label class="flex items-center gap-2 cursor-pointer">
        <input type="checkbox" class="h-4 w-4 rounded border-border text-primary focus:ring-primary" />
        <span class="text-sm">"I agree to the terms"</span>
    </label>
}"#}>
                <div class="space-y-3">
                    <label class="flex items-center gap-2.5 cursor-pointer">
                        <input type="checkbox" checked={true} class="h-4 w-4 rounded border-border text-primary focus:ring-primary accent-primary" />
                        <span class="text-sm">"Remember me"</span>
                    </label>
                    <label class="flex items-center gap-2.5 cursor-pointer">
                        <input type="checkbox" class="h-4 w-4 rounded border-border text-primary focus:ring-primary accent-primary" />
                        <span class="text-sm">"I agree to the terms and conditions"</span>
                    </label>
                    <label class="flex items-center gap-2.5 cursor-pointer opacity-50">
                        <input type="checkbox" disabled={true} class="h-4 w-4 rounded border-border" />
                        <span class="text-sm">"Disabled option"</span>
                    </label>
                </div>
            </Showcase>

            <Showcase id="radio-group" title="Radio Group" description="Single selection from multiple options."
                code={r#"rsx! {
    <fieldset class="space-y-2">
        <legend class="text-sm font-medium mb-2">"Plan"</legend>
        <label class="flex items-center gap-2.5 cursor-pointer">
            <input type="radio" name="plan" checked="" class="h-4 w-4 accent-primary" />
            <span class="text-sm">"Free"</span>
        </label>
        <label class="flex items-center gap-2.5 cursor-pointer">
            <input type="radio" name="plan" class="h-4 w-4 accent-primary" />
            <span class="text-sm">"Pro"</span>
        </label>
    </fieldset>
}"#}>
                <fieldset class="space-y-2.5">
                    <legend class="text-sm font-medium mb-2">"Notification preference"</legend>
                    <label class="flex items-center gap-2.5 cursor-pointer">
                        <input type="radio" name="notif" checked={true} class="h-4 w-4 accent-primary" />
                        <span class="text-sm">"All notifications"</span>
                    </label>
                    <label class="flex items-center gap-2.5 cursor-pointer">
                        <input type="radio" name="notif" class="h-4 w-4 accent-primary" />
                        <span class="text-sm">"Mentions only"</span>
                    </label>
                    <label class="flex items-center gap-2.5 cursor-pointer">
                        <input type="radio" name="notif" class="h-4 w-4 accent-primary" />
                        <span class="text-sm">"None"</span>
                    </label>
                </fieldset>
            </Showcase>

            <Showcase id="toggle-switch" title="Toggle Switch" description="CSS-only toggle switch control."
                code={r#"rsx! {
    <label class="inline-flex items-center gap-3 cursor-pointer">
        <span class="text-sm">"Dark mode"</span>
        <div class="relative">
            <input type="checkbox" class="sr-only peer" checked="" />
            <div class="w-9 h-5 bg-muted rounded-full peer-checked:bg-primary transition-colors"></div>
            <div class="absolute top-0.5 left-0.5 w-4 h-4 bg-white rounded-full shadow peer-checked:translate-x-4 transition-transform"></div>
        </div>
    </label>
}"#}>
                <div class="space-y-4">
                    <label class="inline-flex items-center gap-3 cursor-pointer">
                        <span class="text-sm">"Dark mode"</span>
                        <div class="relative">
                            <input type="checkbox" class="sr-only peer" checked={true} />
                            <div class="w-9 h-5 bg-muted rounded-full peer-checked:bg-primary transition-colors"></div>
                            <div class="absolute top-0.5 left-0.5 w-4 h-4 bg-white rounded-full shadow peer-checked:translate-x-4 transition-transform"></div>
                        </div>
                    </label>
                    <label class="inline-flex items-center gap-3 cursor-pointer">
                        <span class="text-sm">"Email notifications"</span>
                        <div class="relative">
                            <input type="checkbox" class="sr-only peer" />
                            <div class="w-9 h-5 bg-muted rounded-full peer-checked:bg-primary transition-colors"></div>
                            <div class="absolute top-0.5 left-0.5 w-4 h-4 bg-white rounded-full shadow peer-checked:translate-x-4 transition-transform"></div>
                        </div>
                    </label>
                </div>
            </Showcase>

            <Showcase id="range-slider" title="Range Slider" description="Slider input for numeric values."
                code={r#"rsx! {
    <div class="space-y-1.5">
        <div class="flex justify-between">
            <label class="text-sm font-medium">"Volume"</label>
            <span class="text-sm text-muted-foreground">"75%"</span>
        </div>
        <input type="range" min="0" max="100" value="75"
            class="w-full h-2 bg-muted rounded-lg appearance-none accent-primary" />
    </div>
}"#}>
                <div class="w-full max-w-sm space-y-1.5">
                    <div class="flex justify-between">
                        <label class="text-sm font-medium">"Volume"</label>
                        <span class="text-sm text-muted-foreground">"75%"</span>
                    </div>
                    <input type="range" min="0" max="100" value="75" class="w-full h-2 bg-muted rounded-lg appearance-none accent-primary cursor-pointer" />
                </div>
            </Showcase>

            <Showcase id="file-upload" title="File Upload" description="Drag-and-drop file upload area."
                code={r#"rsx! {
    <div class="rounded-lg border-2 border-dashed border-border p-8 text-center hover:border-primary/50 transition-colors cursor-pointer">
        <i class="fas fa-cloud-upload-alt text-3xl text-muted-foreground/50 mb-3"></i>
        <p class="text-sm font-medium">"Drop files here or click to upload"</p>
        <p class="text-xs text-muted-foreground mt-1">"PNG, JPG up to 10MB"</p>
    </div>
}"#}>
                <div class="w-full max-w-sm">
                    <div class="rounded-lg border-2 border-dashed border-border p-8 text-center hover:border-primary/50 transition-colors cursor-pointer">
                        <i class="fas fa-cloud-upload-alt text-3xl text-muted-foreground/50 mb-3"></i>
                        <p class="text-sm font-medium">"Drop files here or click to upload"</p>
                        <p class="text-xs text-muted-foreground mt-1">"PNG, JPG, GIF up to 10MB"</p>
                    </div>
                </div>
            </Showcase>

            <Showcase id="search-input" title="Search Input" description="Input with search icon for filtering content."
                code={r#"rsx! {
    <div class="relative">
        <i class="fas fa-search absolute left-3 top-1/2 -translate-y-1/2 text-xs text-muted-foreground"></i>
        <input type="search" placeholder="Search..."
            class="w-full rounded-lg border border-border bg-background pl-9 pr-3 py-2 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring" />
    </div>
}"#}>
                <div class="w-full max-w-sm">
                    <div class="relative">
                        <i class="fas fa-search absolute left-3 top-1/2 -translate-y-1/2 text-xs text-muted-foreground"></i>
                        <input type="search" placeholder="Search documentation..." class="w-full rounded-lg border border-border bg-background pl-9 pr-3 py-2 text-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring" />
                    </div>
                </div>
            </Showcase>

            <Showcase id="password-input" title="Password Input" description="Password field with visibility toggle."
                code={r#"rsx! {
    <div class="relative">
        <input type="password" placeholder="Enter password"
            class="w-full rounded-lg border border-border bg-background px-3 py-2 pr-10 text-sm focus:outline-none focus:ring-2 focus:ring-ring" />
        <button class="absolute right-2 top-1/2 -translate-y-1/2 p-1 text-muted-foreground hover:text-foreground">
            <i class="fas fa-eye text-xs"></i>
        </button>
    </div>
}"#}>
                <div class="w-full max-w-sm">
                    <div class="relative">
                        <input type="password" placeholder="Enter password" value="mypassword" class="w-full rounded-lg border border-border bg-background px-3 py-2 pr-10 text-sm focus:outline-none focus:ring-2 focus:ring-ring" />
                        <button class="absolute right-2 top-1/2 -translate-y-1/2 p-1 text-muted-foreground hover:text-foreground transition-colors">
                            <i class="fas fa-eye text-xs"></i>
                        </button>
                    </div>
                </div>
            </Showcase>

            <Showcase id="input-group" title="Input Group" description="Input with prefix or suffix add-ons."
                code={r#"rsx! {
    <div class="flex rounded-lg border border-border overflow-hidden focus-within:ring-2 focus-within:ring-ring">
        <span class="inline-flex items-center bg-muted px-3 text-sm text-muted-foreground border-r border-border">"https://"</span>
        <input type="text" placeholder="example.com"
            class="flex-1 bg-background px-3 py-2 text-sm focus:outline-none" />
    </div>
}"#}>
                <div class="w-full max-w-md space-y-3">
                    <div class="flex rounded-lg border border-border overflow-hidden focus-within:ring-2 focus-within:ring-ring">
                        <span class="inline-flex items-center bg-muted px-3 text-sm text-muted-foreground border-r border-border">"https://"</span>
                        <input type="text" placeholder="example.com" class="flex-1 bg-background px-3 py-2 text-sm focus:outline-none min-w-0" />
                    </div>
                    <div class="flex rounded-lg border border-border overflow-hidden focus-within:ring-2 focus-within:ring-ring">
                        <span class="inline-flex items-center bg-muted px-3 border-r border-border">
                            <i class="fas fa-dollar-sign text-sm text-muted-foreground"></i>
                        </span>
                        <input type="number" placeholder="0.00" class="flex-1 bg-background px-3 py-2 text-sm focus:outline-none min-w-0" />
                        <span class="inline-flex items-center bg-muted px-3 text-sm text-muted-foreground border-l border-border">"USD"</span>
                    </div>
                </div>
            </Showcase>

            <Showcase id="floating-label" title="Floating Label" description="Input label that floats above on focus."
                code={r#"rsx! {
    <div class="relative">
        <input type="text" id="floating" placeholder=" "
            class="peer w-full rounded-lg border border-border bg-background px-3 pt-5 pb-1 text-sm focus:outline-none focus:ring-2 focus:ring-ring" />
        <label for="floating"
            class="absolute left-3 top-1 text-[10px] font-medium text-primary transition-all peer-placeholder-shown:top-2.5 peer-placeholder-shown:text-sm peer-placeholder-shown:text-muted-foreground peer-placeholder-shown:font-normal peer-focus:top-1 peer-focus:text-[10px] peer-focus:font-medium peer-focus:text-primary">
            "Email address"
        </label>
    </div>
}"#}>
                <div class="w-full max-w-sm">
                    <div class="relative">
                        <input type="text" id="floating" placeholder=" " class="peer w-full rounded-lg border border-border bg-background px-3 pt-5 pb-1 text-sm focus:outline-none focus:ring-2 focus:ring-ring" />
                        <label class="absolute left-3 top-1 text-[10px] font-medium text-primary transition-all peer-placeholder-shown:top-2.5 peer-placeholder-shown:text-sm peer-placeholder-shown:text-muted-foreground peer-placeholder-shown:font-normal peer-focus:top-1 peer-focus:text-[10px] peer-focus:font-medium peer-focus:text-primary pointer-events-none">
                            "Email address"
                        </label>
                    </div>
                </div>
            </Showcase>

            <div class="mt-8 flex items-center justify-between border-t border-border pt-6">
                <a href={docs_href("/ui/cards")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "← Cards"
                </a>
                <a href={docs_href("/ui/navigation")} class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                    "Navigation →"
                </a>
            </div>
        </article>
    }
}
