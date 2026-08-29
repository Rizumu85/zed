use gpui::{
    AnyWindowHandle, App, Bounds, Context, Window, WindowBounds, WindowKind, WindowOptions, div,
    point, popup::PopupAnchor, popup::PopupConstraintAdjustment, popup::PopupGravity,
    popup::PopupOptions, prelude::*, px, rgb, size,
};
use gpui_platform::application;

const ANCHOR_BOUNDS: Bounds<gpui::Pixels> = Bounds {
    origin: point(px(36.0), px(176.0)),
    size: size(px(220.0), px(36.0)),
};

struct ParentView;

impl Render for ParentView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().bg(rgb(0xf1f1f4)).child(
            div()
                .absolute()
                .left(ANCHOR_BOUNDS.origin.x)
                .top(ANCHOR_BOUNDS.origin.y)
                .w(ANCHOR_BOUNDS.size.width)
                .h(ANCHOR_BOUNDS.size.height)
                .flex()
                .items_center()
                .px_3()
                .rounded_md()
                .bg(rgb(0xffffff))
                .border_1()
                .border_color(rgb(0xd8d8dc))
                .child("Anchored popup"),
        )
    }
}

struct PopupView;

impl Render for PopupView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p_2()
            .flex()
            .flex_col()
            .gap_1()
            .rounded_xl()
            .bg(rgb(0xffffff))
            .border_1()
            .border_color(rgb(0xe4e4e8))
            .child(
                div()
                    .px_2()
                    .py_1()
                    .rounded_md()
                    .bg(rgb(0xf2f2f4))
                    .child("P1 · First item"),
            )
            .child(div().px_2().py_1().child("P2 · Second item"))
            .child(div().px_2().py_1().child("P3 · Third item"))
            .child(div().px_2().py_1().child("P4 · Fourth item"))
    }
}

fn open_popup(parent: AnyWindowHandle, cx: &mut App) -> anyhow::Result<()> {
    cx.open_window(
        WindowOptions {
            titlebar: None,
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(0.0), px(0.0)),
                size: size(px(260.0), px(204.0)),
            })),
            kind: WindowKind::AnchoredPopup(PopupOptions {
                parent,
                anchor_rect: ANCHOR_BOUNDS,
                anchor: PopupAnchor::BottomLeft,
                gravity: PopupGravity::BottomRight,
                constraint_adjustment: PopupConstraintAdjustment::SLIDE_X
                    | PopupConstraintAdjustment::FLIP_Y,
                offset: point(px(0.0), px(6.0)),
                grab: false,
            }),
            focus: false,
            window_background: gpui::WindowBackgroundAppearance::Transparent,
            ..Default::default()
        },
        |_, cx| cx.new(|_| PopupView),
    )?;
    Ok(())
}

fn main() {
    application().run(|cx: &mut App| {
        let parent = cx
            .open_window(
                WindowOptions {
                    titlebar: Some(gpui::TitlebarOptions {
                        title: Some("GPUI Windows Popup Probe".into()),
                        ..Default::default()
                    }),
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: point(px(240.0), px(160.0)),
                        size: size(px(420.0), px(260.0)),
                    })),
                    focus: false,
                    ..Default::default()
                },
                |_, cx| cx.new(|_| ParentView),
            )
            .expect("opening parent probe window");

        if let Err(error) = open_popup(parent.into(), cx) {
            eprintln!("[popup-probe] {error:#}");
        }
    });
}
