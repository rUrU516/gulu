use std::{cell::RefCell, ptr::NonNull, time::Instant};

use block2::RcBlock;
use objc2::{rc::Retained, AnyThread, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
    NSAppearanceCustomization, NSBackingStoreType, NSColor, NSFont, NSImage, NSImageScaling,
    NSImageView, NSShadow, NSStatusWindowLevel, NSTextAlignment, NSTextField, NSView, NSWindow,
    NSWindowCollectionBehavior, NSWindowStyleMask,
};
use objc2_foundation::{NSData, NSPoint, NSRect, NSSize, NSString, NSTimer};
use tauri::{image::Image, tray::TrayIcon, AppHandle};

const WIDTH: f64 = 220.0;
const HEIGHT: f64 = 244.0;
const DURATION: f64 = 4.8;

// AppKit 对象只留在主线程；这个状态只在一次动画播放期间存在。
thread_local! {
    static ANIMATION: RefCell<Option<Animation>> = const { RefCell::new(None) };
}

struct Animation {
    window: Retained<NSWindow>,
    water: Retained<NSImageView>,
    label: Retained<NSTextField>,
    tray: TrayIcon,
    anchor: NSPoint,
    original_size: f64,
    started: Instant,
}

pub fn play(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let tray = app.tray_by_id("gulu").ok_or("找不到咕噜菜单栏图标")?;
    let animated_tray = tray.clone();

    // Tauri 在主线程提供底层图标，从真实按钮取坐标，避免猜测屏幕或缩放比例。
    tray.with_inner_tray_icon(move |native| -> Result<(), String> {
        let mtm = MainThreadMarker::new().ok_or("动画必须在 macOS 主线程创建")?;
        if ANIMATION.with(|state| state.borrow().is_some()) {
            return Ok(());
        }

        let status = native.ns_status_item().ok_or("菜单栏图标当前不可见")?;
        let button = status.button(mtm).ok_or("找不到菜单栏按钮")?;
        let status_window = button.window().ok_or("找不到菜单栏窗口")?;
        let screen = status_window.screen().ok_or("找不到图标所在屏幕")?;
        let cell = button.cell().ok_or("找不到图标绘制区域")?;
        let icon_rect = cell.imageRectForBounds(button.bounds());
        let icon_rect = button.convertRect_toView(icon_rect, None);
        let icon_rect = status_window.convertRectToScreen(icon_rect);
        if icon_rect.size.width <= 0.0 || icon_rect.size.height <= 0.0 {
            return Err("菜单栏图标尚未完成布局，请稍后再试".into());
        }

        let center = NSPoint::new(
            icon_rect.origin.x + icon_rect.size.width / 2.0,
            icon_rect.origin.y + icon_rect.size.height / 2.0,
        );
        let screen_frame = screen.frame();
        let x = (center.x - WIDTH / 2.0).clamp(
            screen_frame.origin.x,
            screen_frame.origin.x + screen_frame.size.width - WIDTH,
        );
        let y = center.y - HEIGHT + icon_rect.size.height / 2.0 + 4.0;
        let frame = NSRect::new(NSPoint::new(x, y), NSSize::new(WIDTH, HEIGHT));

        // SAFETY: 在主线程创建标准 AppKit 无边框窗口，并由 Retained 持有。
        let window = unsafe {
            NSWindow::initWithContentRect_styleMask_backing_defer(
                NSWindow::alloc(mtm),
                frame,
                NSWindowStyleMask::Borderless,
                NSBackingStoreType::Buffered,
                false,
            )
        };
        unsafe { window.setReleasedWhenClosed(false) };
        window.setOpaque(false);
        window.setBackgroundColor(Some(&NSColor::clearColor()));
        window.setHasShadow(false);
        window.setIgnoresMouseEvents(true);
        window.setHidesOnDeactivate(false);
        window.setLevel(NSStatusWindowLevel + 1);
        window.setCollectionBehavior(
            NSWindowCollectionBehavior::CanJoinAllSpaces
                | NSWindowCollectionBehavior::FullScreenAuxiliary
                | NSWindowCollectionBehavior::IgnoresCycle,
        );
        window.setAppearance(Some(&button.effectiveAppearance()));

        let content = NSView::initWithFrame(
            NSView::alloc(mtm),
            NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(WIDTH, HEIGHT)),
        );
        window.setContentView(Some(&content));
        let image = NSImage::initWithData(
            NSImage::alloc(),
            &NSData::with_bytes(include_bytes!("../icons/water-large.png")),
        )
        .ok_or("无法载入水滴图片")?;
        image.setTemplate(true);
        let water = NSImageView::initWithFrame(NSImageView::alloc(mtm), NSRect::ZERO);
        water.setImage(Some(&image));
        water.setImageScaling(NSImageScaling::ScaleAxesIndependently);
        water.setContentTintColor(Some(&NSColor::labelColor()));
        water.setWantsLayer(true);
        let water_shadow = NSShadow::new();
        water_shadow.setShadowColor(Some(&NSColor::whiteColor()));
        water_shadow.setShadowBlurRadius(4.0);
        water_shadow.setShadowOffset(NSSize::new(0.0, 0.0));
        water.setShadow(Some(&water_shadow));
        content.addSubview(&water);

        let label = NSTextField::labelWithString(&NSString::from_str("喝口水吧"), mtm);
        label.setAlignment(NSTextAlignment::Center);
        label.setFont(Some(&NSFont::systemFontOfSize(16.0)));
        label.setTextColor(Some(&NSColor::whiteColor()));
        label.setFrame(NSRect::new(
            NSPoint::new(10.0, 46.0),
            NSSize::new(200.0, 28.0),
        ));
        label.setWantsLayer(true);
        let text_shadow = NSShadow::new();
        text_shadow.setShadowColor(Some(&NSColor::blackColor()));
        text_shadow.setShadowBlurRadius(4.0);
        text_shadow.setShadowOffset(NSSize::new(0.0, -1.0));
        label.setShadow(Some(&text_shadow));
        content.addSubview(&label);

        let animation = Animation {
            window,
            water,
            label,
            tray: animated_tray,
            anchor: NSPoint::new(center.x - x, center.y - y),
            original_size: icon_rect.size.width,
            started: Instant::now(),
        };
        animation.draw(0.0);
        // 透明图片保留原菜单栏占位，水滴在窗口里接过同一位置。
        animation
            .tray
            .set_icon_with_as_template(Some(Image::new_owned(vec![0; 36 * 36 * 4], 36, 36)), true)
            .map_err(|error| error.to_string())?;
        animation.window.orderFrontRegardless();
        eprintln!(
            "喝水提醒开始：起点 ({:.1}, {:.1})，图标 {:.1}×{:.1}",
            center.x, center.y, icon_rect.size.width, icon_rect.size.height
        );
        ANIMATION.with(|state| *state.borrow_mut() = Some(animation));

        let tick = RcBlock::new(|timer: NonNull<NSTimer>| {
            // SAFETY: NSTimer 回调提供有效的 timer，且此定时器在主线程调度。
            tick_animation(unsafe { timer.as_ref() });
        });
        // SAFETY: block 不捕获对象，可跨线程传递；定时器在主线程 run loop 上运行。
        unsafe { NSTimer::scheduledTimerWithTimeInterval_repeats_block(1.0 / 60.0, true, &tick) };
        Ok(())
    })??;
    Ok(())
}

fn tick_animation(timer: &NSTimer) {
    ANIMATION.with(|state| {
        let mut state = state.borrow_mut();
        let Some(animation) = state.as_ref() else {
            timer.invalidate();
            return;
        };
        let elapsed = animation.started.elapsed().as_secs_f64();
        if elapsed >= DURATION {
            animation.window.orderOut(None);
            if let Err(error) = animation
                .tray
                .set_icon_with_as_template(Some(tauri::include_image!("icons/tray.png")), true)
            {
                eprintln!("恢复菜单栏水滴失败：{error}");
            }
            timer.invalidate();
            *state = None;
            eprintln!("喝水提醒结束：窗口已收起，菜单栏水滴已恢复");
        } else {
            animation.draw(elapsed);
        }
    });
}

impl Animation {
    fn draw(&self, elapsed: f64) {
        let (scale, travel, bob) = if elapsed < 0.65 {
            let p = elapsed / 0.65;
            let q = p - 1.0;
            let pop = 1.0 + 2.70158 * q.powi(3) + 1.70158 * q.powi(2);
            (pop, smooth(p), 0.0)
        } else if elapsed < 4.1 {
            let t = elapsed - 0.65;
            (1.0, 1.0, (t * 7.0).sin() * 5.0 * (-t * 0.6).exp())
        } else {
            let p = smooth((DURATION - elapsed) / (DURATION - 4.1));
            (p, p, 0.0)
        };
        let size = self.original_size + (96.0 - self.original_size) * scale;
        self.water.setFrame(NSRect::new(
            NSPoint::new(
                self.anchor.x - size / 2.0,
                self.anchor.y - 82.0 * travel + bob - size / 2.0,
            ),
            NSSize::new(size, size),
        ));
        let fade_in = smooth((elapsed - 0.45) / 0.35);
        let fade_out = smooth((4.25 - elapsed) / 0.35);
        self.label.setAlphaValue(fade_in * fade_out);
    }
}

fn smooth(p: f64) -> f64 {
    let p = p.clamp(0.0, 1.0);
    p * p * (3.0 - 2.0 * p)
}
