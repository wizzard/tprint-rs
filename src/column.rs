use crate::utils::get_string_width;

/// Represents the alignment of the text in the column.
pub enum TPrintAlign {
    /// Left text alignment for captions / cell data
    Left,
    /// Rigth text alignment for captions / cell data
    Right,
    /// Center text alignment for captions / cell data
    Center,
}

pub(crate) struct TPrintColumn {
    caption: String,
    caption_align: TPrintAlign,
    cell_align: TPrintAlign,
    cells: Vec<String>,
    max_width: usize,
}

/// Represents a column in the table.
impl TPrintColumn {
    pub(crate) fn new(caption: &str, caption_align: TPrintAlign, cell_align: TPrintAlign) -> TPrintColumn {
        TPrintColumn {
            caption: caption.to_string(),
            caption_align,
            cell_align,
            cells: Vec::new(),
            max_width: get_string_width(caption)
        }
    }

    pub fn add_data<T: ToString>(&mut self, value: T) {
        self.cells.push(value.to_string());

        let value_len = get_string_width(&value.to_string());

        if value_len > self.max_width {
            self.max_width = value_len;
        }
    }

    pub fn get_str(&self, row_id: usize) -> &str {
        if self.cells.len() <= row_id {
            return ""
        }
        &self.cells[row_id]
    }

    pub fn get_caption(&self) -> &str {
        &self.caption
    }

    pub fn get_caption_align(&self) -> &TPrintAlign {
        &self.caption_align
    }

    pub fn get_cell_align(&self) -> &TPrintAlign {
        &self.cell_align
    }
    pub fn get_max_width(&self) -> usize {
        self.max_width
    }

    pub fn get_rows_count(&self) -> usize {
        self.cells.len()
    }
}