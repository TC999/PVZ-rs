// PvZ Portable Rust 翻译 — LawnDialog（草坪对话框）
// 对应 C++ src/Lawn/Widget/LawnDialog.h / LawnDialog.cpp

#![allow(dead_code)]

use crate::framework::graphics::graphics::Graphics;
use crate::framework::widget::dialog::Dialog;
use crate::framework::widget::widget_manager::WidgetManager;
use crate::framework::key_codes::KeyCode;
use crate::framework::widget::dialog_button::DialogButton;
use crate::framework::color::Color;
use crate::lawn::game_enums::*;
use crate::todlib::reanimator::Reanimation;

/// 对话框头部偏移常量
pub const DIALOG_HEADER_OFFSET: i32 = 45;

/// 重动画小部件（对应 C++ ReanimationWidget : Widget）
/// [TRANSLATION_NOTE]: Rust 组合 `Widget` 基座，`&mut self.widget as *mut Widget`
/// 可挂接 WidgetManager（对应 C++ AddWidget(mReanimation)）
#[repr(C)]
pub struct ReanimationWidget {
    /// Widget 基座
    pub widget: crate::framework::widget::widget::Widget,
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub reanim: Option<*mut Reanimation>,
    pub lawn_dialog: Option<*mut LawnDialog>,
    pub pos_x: f32,
    pub pos_y: f32,
}

impl std::ops::Deref for ReanimationWidget {
    type Target = crate::framework::widget::widget::Widget;
    fn deref(&self) -> &crate::framework::widget::widget::Widget {
        &self.widget
    }
}

impl std::ops::DerefMut for ReanimationWidget {
    fn deref_mut(&mut self) -> &mut crate::framework::widget::widget::Widget {
        &mut self.widget
    }
}

impl ReanimationWidget {
    pub fn new() -> Self {
        ReanimationWidget {
            widget: crate::framework::widget::widget::Widget::new(),
            app: None,
            reanim: None,
            lawn_dialog: None,
            pos_x: 0.0,
            pos_y: 0.0,
        }
    }

    pub fn dispose(&mut self) {
        // 对应 C++ Dispose：释放并清空动画
        self.reanim = None;
    }

    pub fn draw(&self, g: &mut Graphics) {
        // 对应 C++ Draw
        if let Some(r) = self.reanim {
            unsafe { (*r).draw(g); }
        }
    }

    pub fn update(&mut self) {
        // 对应 C++ Update
        if let Some(r) = self.reanim {
            unsafe { (*r).update(); }
        }
    }

    pub fn add_reanimation(&mut self, x: f32, y: f32, reanimation_type: ReanimationType) {
        // 对应 C++ AddReanimation
        self.reanim = None;
        self.pos_x = x;
        self.pos_y = y;
        if let Some(app) = self.app {
            unsafe {
                if let Some(r) = (*app).add_reanimation(x, y, 0, reanimation_type as i32) {
                    (*r).m_loop_type = crate::todlib::reanimator::ReanimLoopType::Loop;
                    (*r).m_is_attachment = true;
                    // C++ 中 TrackExists("anim_idle") 时设置该层帧
                    (*r).set_frames_for_layer("anim_idle");
                    self.reanim = Some(r);
                }
            }
        }
    }
}

impl Default for ReanimationWidget {
    fn default() -> Self {
        ReanimationWidget::new()
    }
}

/// 草坪对话框（对应 C++ LawnDialog，内嵌 Dialog 基类字段）
pub struct LawnDialog {
    pub app: Option<*mut crate::lawn::lawn_app::LawnApp>,
    pub button_delay: i32,
    pub reanimation: Option<*mut ReanimationWidget>,
    pub draw_standard_back: bool,
    pub lawn_yes_button: Option<*mut DialogButton>,
    pub lawn_no_button: Option<*mut DialogButton>,
    pub tall_bottom: bool,
    pub vertical_center_text: bool,
    // Dialog 基类字段（对应 C++ Dialog 成员）
    pub x: i32, pub y: i32, pub width: i32, pub height: i32,
    pub colors: Vec<Color>,
    pub id: i32,
    pub is_modal: bool,
    pub result: i32,
    pub button_height: i32,
    pub button_horz_spacing: i32,
    pub dialog_header: String,
    pub dialog_footer: String,
    pub dialog_lines: String,
    pub text_align: i32,
    pub line_spacing_offset: i32,
    pub space_after_header: i32,
    pub content_insets: crate::framework::widget::insets::Insets,
    pub background_insets: crate::framework::widget::insets::Insets,
    pub header_font: Option<Box<crate::framework::graphics::font::Font>>,
    pub lines_font: Option<Box<crate::framework::graphics::font::Font>>,
}

impl LawnDialog {
    pub fn new() -> Self {
        LawnDialog {
            app: None,
            button_delay: -1,
            reanimation: None,
            draw_standard_back: true,
            lawn_yes_button: None,
            lawn_no_button: None,
            tall_bottom: false,
            vertical_center_text: true,
            x: 0, y: 0, width: 0, height: 0,
            colors: vec![crate::framework::color::Color::new(0xE0, 0xBB, 0x62, 255); 7],            id: 0,
            is_modal: false,
            result: 0,
            button_height: 40,
            button_horz_spacing: 0,
            dialog_header: String::new(),
            dialog_footer: String::new(),
            dialog_lines: String::new(),
            text_align: 0,
            line_spacing_offset: 0,
            space_after_header: 0,
            content_insets: crate::framework::widget::insets::Insets::new(36, 35, 46, 36),
            background_insets: crate::framework::widget::insets::Insets::new(30, 30, 30, 30),
            header_font: None,
            lines_font: None,
        }
    }

    /// 获取左侧内容起点（对应 C++ GetLeft）
    pub fn get_left(&self) -> i32 {
        self.content_insets.left + self.background_insets.left
    }

    /// 获取内容宽度（对应 C++ GetWidth）
    pub fn get_width(&self) -> i32 {
        self.width - self.content_insets.left - self.content_insets.right
            - self.background_insets.left - self.background_insets.right
    }

    /// 获取内容顶部起点（对应 C++ GetTop）
    pub fn get_top(&self) -> i32 {
        self.content_insets.top + self.background_insets.top + 99
    }

    /// 设置按钮延迟（对应 C++ SetButtonDelay）
    pub fn set_button_delay(&mut self, delay: i32) {
        self.button_delay = delay;
        if let Some(btn) = self.lawn_yes_button {
            unsafe { (&mut *btn).disabled = true; }
        }
        if let Some(btn) = self.lawn_no_button {
            unsafe { (&mut *btn).disabled = true; }
        }
    }

    pub fn update(&mut self) {
        // 对应 C++ Update：延迟结束后启用按钮
        if self.button_delay == 0 {
            if let Some(btn) = self.lawn_yes_button {
                unsafe { (&mut *btn).disabled = false; }
            }
            if let Some(btn) = self.lawn_no_button {
                unsafe { (&mut *btn).disabled = false; }
            }
        }
    }

    /// 按钮按下（对应 C++ ButtonPress）
    pub fn button_press(&mut self, _id: i32) {
        // C++: (void)theId; mApp->PlaySample(SOUND_GRAVEBUTTON);
        if let Some(app) = self.app {
            unsafe {
                (*app).play_sample(crate::framework::resources::ResourceId::SoundGravebutton as i32);
            }
        }
    }

    pub fn button_depress(&mut self, _id: i32) {
        // 对应 C++ ButtonDepress：延迟结束前忽略
        if self.button_delay < 0 {
            // 无延迟则直接处理
        }
    }

    pub fn checkbox_checked(&mut self) {
        // 对应 C++ CheckboxChecked：mApp->PlaySample(SOUND_BUTTONCLICK)
        if let Some(app) = self.app {
            unsafe {
                (*app).play_sample(crate::framework::resources::ResourceId::SoundButtonclick as i32);
            }
        }
    }

    pub fn key_down(&mut self, key: KeyCode) {
        // 对应 C++ KeyDown：空格/回车=Yes，Esc/N=No（非图鉴对话框）
        if self.id != crate::lawn::game_enums::Dialogs::Almanac as i32 {
            if key == crate::framework::key_codes::KEYCODE_SPACE
                || key == crate::framework::key_codes::KEYCODE_RETURN
                || key == b'y' as i32
                || key == b'Y' as i32
            {
                self.result = crate::framework::widget::dialog::ID_YES;
            } else if key == crate::framework::key_codes::KEYCODE_ESCAPE
                || key == b'n' as i32
                || key == b'N' as i32
            {
                if self.lawn_no_button.is_some() {
                    self.result = crate::framework::widget::dialog::ID_NO;
                } else if key == crate::framework::key_codes::KEYCODE_ESCAPE && self.lawn_yes_button.is_some() {
                    self.result = crate::framework::widget::dialog::ID_YES;
                }
            }
        }
    }

    pub fn added_to_manager(&mut self, manager: &mut WidgetManager) {
        // 对应 C++ AddedToManager：AddWidget(mReanimation) / AddWidget(mLawnYesButton) / AddWidget(mLawnNoButton)
        // C++: Dialog::AddedToManager（Widget 树挂接）由 manager 管理
        if let Some(reanim) = self.reanimation {
            unsafe {
                manager.add_widget(&mut (*reanim).widget as *mut crate::framework::widget::widget::Widget);
            }
        }
        if let Some(btn) = self.lawn_yes_button {
            unsafe {
                manager.add_widget((&mut *btn).as_widget_ptr());
            }
        }
        if let Some(btn) = self.lawn_no_button {
            unsafe {
                manager.add_widget((&mut *btn).as_widget_ptr());
            }
        }
    }

    pub fn removed_from_manager(&mut self, manager: &mut WidgetManager) {
        // 对应 C++ RemovedFromManager：RemoveWidget(mLawnYesButton) / RemoveWidget(mLawnNoButton) / RemoveWidget(mReanimation)
        if let Some(btn) = self.lawn_yes_button {
            unsafe {
                manager.remove_widget((&mut *btn).as_widget_ptr());
            }
        }
        if let Some(btn) = self.lawn_no_button {
            unsafe {
                manager.remove_widget((&mut *btn).as_widget_ptr());
            }
        }
        if let Some(reanim) = self.reanimation {
            unsafe {
                manager.remove_widget(&mut (*reanim).widget as *mut crate::framework::widget::widget::Widget);
            }
        }
    }

    pub fn resize(&mut self, x: i32, y: i32, width: i32, height: i32) {
        // 对应 C++ Resize
        self.x = x;
        self.y = y;
        self.width = width;
        self.height = height;
    }

    pub fn draw(&self, g: &mut Graphics) {
        // 对应 C++ LawnDialog::Draw：九宫格组件图平铺 + 标题 + 正文
        if !self.draw_standard_back {
            return;
        }
        // 底部图：普通或高底（mTallBottom）
        let (bottom_left, bottom_middle, bottom_right) = if self.tall_bottom {
            ("dialog_bigbottomleft", "dialog_bigbottommiddle", "dialog_bigbottomright")
        } else {
            ("dialog_bottomleft", "dialog_bottommiddle", "dialog_bottomright")
        };
        let top_left = self.get_image("dialog_topleft");
        let top_middle = self.get_image("dialog_topmiddle");
        let top_right = self.get_image("dialog_topright");
        let center_left = self.get_image("dialog_centerleft");
        let center_middle = self.get_image("dialog_centermiddle");
        let center_right = self.get_image("dialog_centerright");
        let bottom_left_img = self.get_image(bottom_left);
        let bottom_middle_img = self.get_image(bottom_middle);
        let bottom_right_img = self.get_image(bottom_right);
        let header_img = self.get_image("dialog_header");
        if top_left.is_none() || top_middle.is_none() || top_right.is_none()
            || center_left.is_none() || center_middle.is_none() || center_right.is_none()
            || bottom_left_img.is_none() || bottom_middle_img.is_none() || bottom_right_img.is_none()
        {
            // [TRANSLATION_NOTE]: 对话框组件图未加载，回退为半透明背景矩形
            g.set_color(&Color::new(40, 40, 40, 230));
            g.fill_rect_xywh(self.x, self.y, self.width, self.height);
            return;
        }
        let (tl, tm, tr) = (top_left.unwrap(), top_middle.unwrap(), top_right.unwrap());
        let (cl, cm, cr) = (center_left.unwrap(), center_middle.unwrap(), center_right.unwrap());
        let (bl, bm, br) = (bottom_left_img.unwrap(), bottom_middle_img.unwrap(), bottom_right_img.unwrap());

        let a_repeat_x = (self.width - tr.width - tl.width) / tm.width.max(1);
        let a_repeat_y = (self.height - tl.height - bl.height - DIALOG_HEADER_OFFSET) / cl.height.max(1);

        let mut a_pos_x = 0;
        let mut a_pos_y = DIALOG_HEADER_OFFSET;
        g.draw_image_f_xy(tl, a_pos_x as f32, a_pos_y as f32);
        a_pos_x += tl.width;
        for _ in 0..a_repeat_x {
            g.draw_image_f_xy(tm, a_pos_x as f32, a_pos_y as f32);
            a_pos_x += tm.width;
        }
        g.draw_image_f_xy(tr, a_pos_x as f32, a_pos_y as f32);

        a_pos_y += tr.height;
        for _ in 0..a_repeat_y {
            a_pos_x = 0;
            g.draw_image_f_xy(cl, a_pos_x as f32, a_pos_y as f32);
            a_pos_x += cl.width;
            for _ in 0..a_repeat_x {
                g.draw_image_f_xy(cm, a_pos_x as f32, a_pos_y as f32);
                a_pos_x += cm.width;
            }
            g.draw_image_f_xy(cr, a_pos_x as f32, a_pos_y as f32);
            a_pos_y += cl.height;
        }

        a_pos_x = 0;
        g.draw_image_f_xy(bl, a_pos_x as f32, a_pos_y as f32);
        a_pos_x += bl.width;
        for _ in 0..a_repeat_x {
            g.draw_image_f_xy(bm, a_pos_x as f32, a_pos_y as f32);
            a_pos_x += bm.width;
        }
        g.draw_image_f_xy(br, a_pos_x as f32, a_pos_y as f32);
        if let Some(hd) = header_img {
            g.draw_image_f_xy(hd, ((self.width - hd.width) / 2 - 5) as f32, 0.0);
        }
        // [TRANSLATION_NOTE]: 标题与正文文字依赖字体/换行系统，后续补全
    }

    /// 按小写 id 获取对话框图片（经 resource_manager）
    pub(crate) fn get_image(&self, id: &str) -> Option<&crate::framework::graphics::image::Image> {
        let app = self.app?;
        let app_ref = unsafe { &*app };
        let rm = app_ref.base.resource_manager?;
        let shared = unsafe { (*rm).get_image(id) };
        unsafe {
            if !shared.unshared_image.is_null() {
                return Some(&(*(shared.unshared_image)).base);
            }
            if !shared.shared_image.is_null() {
                return Some(&(*(*(shared.shared_image)).image).base.base);
            }
        }
        None
    }

    /// 计算对话框尺寸（对应 C++ CalcSize）
    pub fn calc_size(&mut self, extra_x: i32, extra_y: i32) {
        let mut a_width = self.background_insets.left + self.background_insets.right
            + self.content_insets.left + self.content_insets.right + extra_x;
        if !self.dialog_header.is_empty() {
            if let Some(font) = &self.header_font {
                a_width += font.string_width(&self.dialog_header);
            }
        }
        // 最小宽度：对话框组件图（左上+右上+中上）
        const IMAGE_DIALOG_TOPLEFT_W: i32 = 36;
        const IMAGE_DIALOG_TOPRIGHT_W: i32 = 36;
        const IMAGE_DIALOG_TOPMIDDLE_W: i32 = 18;
        let a_image_width = IMAGE_DIALOG_TOPLEFT_W + IMAGE_DIALOG_TOPRIGHT_W + IMAGE_DIALOG_TOPMIDDLE_W;
        if a_width <= a_image_width {
            a_width = a_image_width;
        } else if IMAGE_DIALOG_TOPMIDDLE_W > 0 {
            let an_extra = (a_width - a_image_width) % IMAGE_DIALOG_TOPMIDDLE_W;
            if an_extra != 0 {
                a_width += IMAGE_DIALOG_TOPMIDDLE_W - an_extra;
            }
        }

        let mut a_height = self.background_insets.top + self.background_insets.bottom
            + self.content_insets.top + self.content_insets.bottom + extra_y + DIALOG_HEADER_OFFSET;
        if !self.dialog_header.is_empty() {
            if let Some(font) = &self.header_font {
                a_height += -font.get_ascent_padding() + font.get_height() + self.space_after_header;
            }
        }
        a_height += self.button_height;

        // 最小高度：对话框组件图
        const IMAGE_DIALOG_TOPLEFT_H: i32 = 36;
        const IMAGE_DIALOG_BOTTOMLEFT_H: i32 = 36;
        const IMAGE_DIALOG_CENTERLEFT_H: i32 = 24;
        let a_image_height = IMAGE_DIALOG_TOPLEFT_H + IMAGE_DIALOG_BOTTOMLEFT_H + DIALOG_HEADER_OFFSET;
        if a_height < a_image_height {
            a_height = a_image_height;
        } else {
            let an_extra = (a_height - a_image_height) % IMAGE_DIALOG_CENTERLEFT_H;
            if an_extra != 0 {
                a_height += IMAGE_DIALOG_CENTERLEFT_H - an_extra;
            }
        }

        self.resize(self.x, self.y, a_width, a_height);
    }
}

impl Default for LawnDialog {
    fn default() -> Self {
        LawnDialog::new()
    }
}
