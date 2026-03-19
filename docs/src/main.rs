#![no_std]

extern crate alloc;

mod components;
mod pages;

use alloc::vec;
use components::*;
use momenta::prelude::*;
use momenta_router::{RouterContext, RouterMode, routes};
use pages::*;

#[component]
fn App() -> Node {
    let router = RouterContext::with_base(RouterMode::Pathname, docs_base_path());
    let current_path = router.current_path();
    let theme = create_signal("dark");
    let mobile_menu_open = create_signal(false);

    create_effect(|| {
        highlightAll();
    });

    create_effect(move || {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Ok(Some(storage)) = window.local_storage() else {
            return;
        };
        if let Ok(Some(saved)) = storage.get_item("theme") {
            if saved != "light" {
                return;
            }
            theme.set("light");

            let Some(doc) = window.document() else { return };
            if let Some(el) = doc.document_element() {
                let _ = el.class_list().remove_1("dark");
            };

            sync_docs_theme("light");
        };
    });

    rsx! {
        <div class="min-h-screen bg-background text-foreground transition-colors duration-200">
            <Header {theme} {mobile_menu_open} current_path={current_path} />

            <div class="flex pt-14">
                {when!(current_path.get() != "/" =>
                    <aside class="hidden lg:block w-64 shrink-0 border-r border-border/50">
                        <div class="sticky top-14 h-[calc(100vh-3.5rem)] overflow-y-auto py-6 px-1">
                            <Navigation {router} />
                        </div>
                    </aside>
                )}

                {when!(mobile_menu_open =>
                    <div class="lg:hidden fixed inset-0 z-50 flex">
                        <div class="fixed inset-0 bg-black/40 backdrop-blur-sm" on:click={move |_| mobile_menu_open.set(false)}></div>
                        <div class="relative flex w-full max-w-xs flex-col bg-background border-r border-border shadow-xl fade-in">
                            <div class="flex items-center justify-between px-5 py-4 border-b border-border/50">
                                <span class="text-xs font-semibold uppercase tracking-wider text-muted-foreground">Navigation</span>
                                <button type="button" on:click={move |_| mobile_menu_open.set(false)} class="p-2 rounded-md hover:bg-muted transition-colors">
                                    <i class="fas fa-times text-sm"></i>
                                </button>
                            </div>
                            <div class="overflow-y-auto p-4">
                                <Navigation {router} />
                            </div>
                        </div>
                    </div>
                )}

                <main class="flex-1 min-w-0">
                    {when!(current_path.get() != "/" && !docs_on_this_page_sections(&current_path.get()).is_empty() =>
                        <div class="xl:hidden px-6 pt-5 sm:px-8 lg:px-10">
                            <OnThisPage current_path={current_path.get()} compact={true} />
                        </div>
                    )}

                    {routes!(router, current_path, {
                        "/" => |_| rsx! { <HomePage {router} /> },
                        "/getting-started" => |_| rsx! { <GettingStartedPage /> },
                        "/philosophy" => |_| rsx! { <PhilosophyPage /> },
                        "/rsx" => |_| rsx! { <RsxPage /> },
                        "/signals" => |_| rsx! { <SignalsPage /> },
                        "/computed-signals" => |_| rsx! { <ComputedSignalsPage /> },
                        "/effects" => |_| rsx! { <EffectsPage /> },
                        "/resources" => |_| rsx! { <ResourcesPage /> },
                        "/components" => |_| rsx! { <ComponentsPage /> },
                        "/classes" => |_| rsx! { <ClassesPage /> },
                        "/when" => |_| rsx! { <ShowPage /> },
                        "/lists" => |_| rsx! { <ForPage /> },
                        "/performance" => |_| rsx! { <PerformancePage /> },
                        "/ssr" => |_| rsx! { <SsrPage /> },
                        "/deployment" => |_| rsx! { <DeploymentPage /> },
                        "/examples" => |_| rsx! { <ExamplesPage /> },
                        "/routing" => |_| rsx! { <RoutingPage /> },
                        "/routing/:section" => |_| rsx! { <RoutingPage /> },
                        "/examples/counter" => |_| rsx! { <CounterExample /> },
                        "/examples/todomvc" => |_| rsx! { <TodoMVCPage /> },
                        "/examples/hackernews" => |_| rsx! { <HackerNewsPage /> },
                        "/examples/realworld" => |_| rsx! { <RealWorldPage /> },
                        "/ui" => |_| rsx! { <UIOverviewPage /> },
                        "/ui/buttons" => |_| rsx! { <ButtonsPage /> },
                        "/ui/badges" => |_| rsx! { <BadgesPage /> },
                        "/ui/alerts" => |_| rsx! { <AlertsPage /> },
                        "/ui/cards" => |_| rsx! { <CardsPage /> },
                        "/ui/inputs" => |_| rsx! { <InputsPage /> },
                        "/ui/navigation" => |_| rsx! { <NavComponentsPage /> },
                        "/ui/data-display" => |_| rsx! { <DataDisplayPage /> },
                        "/ui/layout" => |_| rsx! { <LayoutPage /> },
                        "/ui/feedback" => |_| rsx! { <FeedbackPage /> },
                        "/ui/overlays" => |_| rsx! { <OverlaysPage /> },
                        "/ui/marketing" => |_| rsx! { <MarketingPage /> },
                        "/ui/typography" => |_| rsx! { <TypographyPage /> },
                    })}
                </main>

                {when!(current_path.get() != "/" && !docs_on_this_page_sections(&current_path.get()).is_empty() =>
                    <aside class="hidden xl:block w-72 shrink-0 border-l border-border/50 bg-background/55">
                        <div class="sticky top-14 max-h-[calc(100vh-3.5rem)] overflow-y-auto px-5 py-6">
                            <OnThisPage current_path={current_path.get()} compact={false} />
                        </div>
                    </aside>
                )}
            </div>
        </div>
    }
}

fn main() {
    render_root::<App>("#app");
}
