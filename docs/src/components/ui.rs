use crate::CounterExample;
use alloc::{format, vec, vec::Vec};
use momenta::prelude::*;

pub struct CodeBlockProps {
    pub code: &'static str,
    pub language: &'static str,
    pub filename: Option<&'static str>,
    #[allow(dead_code)]
    pub highlight: Option<&'static str>,
}

#[component]
pub fn CodeBlock(props: &CodeBlockProps) -> Node {
    rsx! {
        <div class="my-6 overflow-hidden rounded-lg border border-border/50">
            {when!(let Some(filename) = props.filename =>
                <div class="flex items-center border-b border-border/40 bg-card px-4 py-2">
                    <span class="text-xs font-mono text-muted-foreground">{filename}</span>
                </div>
            )}
            <pre class="bg-card overflow-x-auto m-0 p-0">
                <code class={format!("language-{} text-[13px] leading-relaxed", props.language)}>{props.code}</code>
            </pre>
        </div>
    }
}

pub struct NoteProps {
    pub variant: &'static str,
    pub children: Vec<Node>,
}

#[component]
pub fn Note(props: &NoteProps) -> Node {
    let (bg, border_color, icon) = match props.variant {
        "info" => (
            "bg-blue-50 dark:bg-blue-950/30",
            "border-l-blue-500",
            "fa-info-circle text-blue-500",
        ),
        "warning" => (
            "bg-amber-50 dark:bg-amber-950/30",
            "border-l-amber-500",
            "fa-exclamation-triangle text-amber-500",
        ),
        "tip" => (
            "bg-emerald-50 dark:bg-emerald-950/30",
            "border-l-emerald-500",
            "fa-lightbulb text-emerald-500",
        ),
        _ => (
            "bg-muted/30",
            "border-l-muted-foreground",
            "fa-info-circle text-muted-foreground",
        ),
    };

    rsx! {
        <div class={format!("my-6 rounded-md border-l-4 {} {} px-4 py-3", border_color, bg)}>
            <div class="flex gap-3">
                <i class={format!("fas {} mt-0.5 text-sm", icon)}></i>
                <div class="text-sm leading-relaxed">
                    {&props.children}
                </div>
            </div>
        </div>
    }
}

pub struct DocPageHeaderProps {
    pub title: &'static str,
    pub summary: &'static str,
    pub chips: Vec<&'static str>,
    pub stats: Vec<(&'static str, &'static str)>,
}

#[component]
pub fn DocPageHeader(props: &DocPageHeaderProps) -> Node {
    rsx! {
        <header class="doc-hero not-prose mb-8 overflow-hidden rounded-[24px] border border-border/60 px-5 py-6 sm:px-6 sm:py-6">
            <div class="relative z-10 grid gap-6 lg:grid-cols-[minmax(0,1.35fr)_minmax(17rem,0.85fr)] lg:items-end">
                <div class="max-w-3xl">
                    <h1 class="text-[1.85rem] font-bold tracking-tight sm:text-[2.2rem] lg:text-[2.4rem] lg:leading-[1.08]">
                        {props.title}
                    </h1>
                    <p class="mt-3 max-w-2xl text-sm leading-6 text-muted-foreground sm:text-[15px] sm:leading-7">
                        {props.summary}
                    </p>
                    <div class="mt-5 flex flex-wrap gap-2">
                        {props.chips.iter().map(|chip| rsx! {
                            <span class="doc-chip">{*chip}</span>
                        }).collect::<Vec<_>>()}
                    </div>
                </div>

                <div class="grid gap-3 sm:grid-cols-3 lg:grid-cols-1">
                    {props.stats.iter().map(|(label, value)| rsx! {
                        <div class="doc-stat-card">
                            <div class="text-[11px] font-semibold uppercase tracking-[0.16em] text-muted-foreground/80">{*label}</div>
                            <div class="mt-1.5 text-[13px] font-medium leading-5 text-foreground">{*value}</div>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </header>
    }
}

pub struct TheoryCardProps {
    pub icon: &'static str,
    pub title: &'static str,
    pub children: Vec<Node>,
}

#[component]
pub fn TheoryCard(props: &TheoryCardProps) -> Node {
    rsx! {
        <div class="theory-card not-prose">
            <div class="flex items-start gap-3">
                <div class="theory-card-icon">
                    <i class={format!("{} text-[13px]", props.icon)}></i>
                </div>
                <div class="min-w-0 flex-1">
                    <h3 class="text-sm font-semibold tracking-tight text-foreground">{props.title}</h3>
                    <div class="mt-1.5 space-y-1.5 text-[13px] leading-5 text-muted-foreground">
                        {&props.children}
                    </div>
                </div>
            </div>
        </div>
    }
}

pub struct PlaygroundProps {
    pub code: &'static str,
}

#[component]
pub fn Playground(props: &PlaygroundProps) -> Node {
    rsx! {
        <div class="my-8 overflow-hidden rounded-lg border border-border/50">
            <div class="flex flex-col md:flex-row items-stretch h-full">
                <div class="md:w-1/2 border-r border-border/40 flex flex-col">
                    <div class="border-b border-border/40 bg-card px-4 py-2">
                        <span class="text-xs font-mono text-muted-foreground">Code</span>
                    </div>
                    <div class="bg-card flex-1">
                        <pre class="overflow-x-auto h-[275px] p-0 m-0">
                            <code class="language-rust text-xs leading-relaxed overflow-x">{props.code}</code>
                        </pre>
                    </div>
                </div>
                <div class="md:w-1/2 flex flex-col">
                    <div class="border-b border-border/40 bg-card px-4 py-2">
                        <span class="text-xs font-mono text-muted-foreground">Output</span>
                    </div>
                    <div class="flex-1 text-sm">
                        <CounterExample />
                    </div>
                </div>
            </div>
        </div>
    }
}
