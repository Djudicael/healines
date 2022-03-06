


use eframe::egui::CentralPanel;
use eframe::epi::{App, Frame, Storage};
use egui::{
    Color32, Context, FontData, FontDefinitions, FontFamily, Hyperlink, Label, Layout, RichText,
    ScrollArea, Ui, Vec2, Separator,
};

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

pub struct NewsCardData {
    title: String,
    description: String,
    url: String,
}

pub struct Headlines {
    articles: Vec<NewsCardData>,
}

 impl Headlines {
    pub fn new() -> Self {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("title {}", a),
            description: format!("description {}", a),
            url: format!("https://example.com/{}", a),
        });
        Self {
            articles: Vec::from_iter(iter),
        }
    }

    fn render_news_cards(&self, ui: &mut Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);
            //render title
            let title = format!(">{}", a.title);
            ui.colored_label(WHITE, title);
            // render description
            ui.add_space(PADDING);
            let description = Label::new(
                RichText::new(&a.description).text_style(eframe::egui::TextStyle::Button),
            );
            ui.add(description);

            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.allocate_ui_with_layout(
                Vec2::new(ui.available_width(), 24.0),
                Layout::right_to_left(),
                |ui| {
                    ui.add(Hyperlink::from_label_and_url(
                        RichText::new("read more ^"),
                        &a.url,
                    ));
                },
            );
            ui.add(Separator::default());
        }
    }

    fn configure_fonts(&self, ctx: &Context) {
        // create font definition object
        let mut font_def = FontDefinitions::default();
        // then load up the font
        font_def.font_data.insert(
            "Oswald".to_string(),
            FontData::from_static(include_bytes!("../../assets/fonts/Oswald-Regular.ttf")),
        );
        // set the size of different text style
        
        font_def
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "Oswald".to_owned());
        // load the font using the context object
        ctx.set_fonts(font_def);
    }
}

impl App for Headlines {
    fn setup(&mut self, ctx: &egui::Context, _frame: &Frame, _storage: Option<&dyn Storage>) {
        self.configure_fonts(ctx);
    }

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                self.render_news_cards(ui);
            });
        });
    }

    fn name(&self) -> &str {
        "News Headlines"
    }
}