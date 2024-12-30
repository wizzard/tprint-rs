use unicode_width::UnicodeWidthStr;

pub enum TPrintAlign {
    Left,
    Right,
    Center,
}

pub struct TPrintColumn {
    caption: String,
    caption_align: TPrintAlign,
    data_align: TPrintAlign,
    data: Vec<String>,
    max_width: usize,
}

impl TPrintColumn {
    pub fn new(caption: &str, caption_align: TPrintAlign, data_align: TPrintAlign) -> TPrintColumn {
        TPrintColumn {
            caption: caption.to_string(),
            caption_align,
            data_align,
            data: Vec::new(),
            max_width: caption.width()
        }
    }

    pub fn add_data<T: ToString>(&mut self, value: T) {
        self.data.push(value.to_string());

        let value_len = value.to_string().width();

        if value_len > self.max_width {
            self.max_width = value_len;
        }
    }

    pub fn get_str(&self, row_id: usize) -> &str {
        if self.data.len() <= row_id {
            return ""
        }
        &self.data[row_id]
    }

    pub fn get_caption(&self) -> &str {
        &self.caption
    }

    pub fn get_caption_align(&self) -> &TPrintAlign {
        &self.caption_align
    }

    pub fn get_data_align(&self) -> &TPrintAlign {
        &self.data_align
    }
    pub fn get_max_width(&self) -> usize {
        self.max_width
    }

    pub fn get_rows_count(&self) -> usize {
        self.data.len()
    }
}