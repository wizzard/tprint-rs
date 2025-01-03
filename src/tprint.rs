use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;
use unicode_width::UnicodeWidthStr;

use crate::tprint_column::{TPrintColumn, TPrintAlign};
use crate::tprint_output::{TPrintOutput, TPrintOutputStdout};
use crate::tprint_borders::{TPrintBorders, TPrintBordersASCII, TPrintBordersType};

type TPrintOutputRef = Rc<RefCell<dyn TPrintOutput>>;
type TPrintBordersRef = Rc<RefCell<dyn TPrintBorders>>;

pub struct TPrint {
    output: TPrintOutputRef,
    borders: TPrintBordersRef,
    show_borders: bool,
    show_headers: bool,
    spaces_left: usize,
    spaces_between: usize,
    columns: Vec<TPrintColumn>,
    current_column_id: usize,
}

impl TPrint {
    pub fn new(show_borders: bool, show_headers: bool, spaces_left: usize, extra_spaces_between: usize) -> Self {
        TPrint {
            output: Rc::new(RefCell::new(TPrintOutputStdout {})),
            borders: Rc::new(RefCell::new(TPrintBordersASCII {})),
            show_borders,
            show_headers,
            spaces_left,
            spaces_between: extra_spaces_between,
            columns: Vec::new(),
            current_column_id: 0,
        }
    }

    pub fn new_with_output(output: TPrintOutputRef, show_borders: bool, show_headers: bool, spaces_left: usize, extra_spaces_between: usize) -> Self {
        TPrint {
            output,
            borders: Rc::new(RefCell::new(TPrintBordersASCII {})),
            show_borders,
            show_headers,
            spaces_left,
            spaces_between: extra_spaces_between,
            columns: Vec::new(),
            current_column_id: 0,
        }
    }

    pub fn new_with_borders(borders: TPrintBordersRef, show_borders: bool, show_headers: bool, spaces_left: usize, extra_spaces_between: usize) -> Self {
        TPrint {
            output: Rc::new(RefCell::new(TPrintOutputStdout {})),
            borders,
            show_borders,
            show_headers,
            spaces_left,
            spaces_between: extra_spaces_between,
            columns: Vec::new(),
            current_column_id: 0,
        }
    }

    pub fn new_with_borders_output(borders: TPrintBordersRef, output: TPrintOutputRef, show_borders: bool, show_headers: bool, spaces_left: usize, extra_spaces_between: usize) -> Self {
        TPrint {
            output,
            borders,
            show_borders,
            show_headers,
            spaces_left,
            spaces_between: extra_spaces_between,
            columns: Vec::new(),
            current_column_id: 0,
        }
    }

    pub fn column_add(&mut self, caption: &str, caption_align: TPrintAlign, data_align: TPrintAlign) -> &mut Self {
        self.columns.push(TPrintColumn::new(caption, caption_align, data_align));
        self
    }

    pub fn add_data<T: fmt::Display>(&mut self, value: T) -> &mut Self {
        let column_id = self.current_column_id;
        if let Some(column) = self.columns.get_mut(column_id) {
            column.add_data(value);

            if self.current_column_id + 1 >= self.columns.len() {
                self.current_column_id = 0;
            } else {
                self.current_column_id += 1;
            }
        }
        self
    }

    fn print_horizonal_border(&self, left: &TPrintBordersType, right: &TPrintBordersType, middle: &TPrintBordersType, line: &TPrintBordersType) {
        if !self.show_borders {
            return
        }

        if self.spaces_left > 0 {
            self.output.borrow_mut().print_str(&self.borders.borrow().get_border(&TPrintBordersType::WhiteSpace).repeat(self.spaces_left));
        }

        self.output.borrow_mut().print_str(self.borders.borrow().get_border(left));

        for i in 0..self.columns.len() {
            let c = &self.columns[i];

            self.output.borrow_mut().print_str(&self.borders.borrow().get_border(line).repeat(c.get_max_width() + 2 * self.spaces_between));

            if i < self.columns.len() - 1 {
                self.output.borrow_mut().print_str(self.borders.borrow().get_border(middle));
            }
        }

        self.output.borrow_mut().print_str(self.borders.borrow().get_border(right));
        self.output.borrow_mut().print_str(self.borders.borrow().get_border(&TPrintBordersType::NewLine));
    }

    fn print_left_border(&self, left_border: &TPrintBordersType) {
        if !self.show_borders {
            return
        }
        self.output.borrow_mut().print_str(self.borders.borrow().get_border(left_border));
    }

    fn print_right_border(&self, right_border: &TPrintBordersType) {
        if !self.show_borders {
            return
        }
        self.output.borrow_mut().print_str(self.borders.borrow().get_border(right_border));
        self.output.borrow_mut().print_str(self.borders.borrow().get_border(&TPrintBordersType::NewLine));
    }

    fn print_internal_border(&self, v_border: &TPrintBordersType) {
        if !self.show_borders {
            return
        }
        self.output.borrow_mut().print_str(self.borders.borrow().get_border(v_border));
    }

    fn print_cell(&self, value: &str, max_width: usize, align: &TPrintAlign) {
        let left_spaces;
        let right_spaces;

        let value_len = value.width();

        match align {
            TPrintAlign::Left => {
                left_spaces = self.spaces_between;
                right_spaces = (self.spaces_between * 2 + max_width) - (left_spaces + value_len);
            }
            TPrintAlign::Center => {
                left_spaces = self.spaces_between + (max_width - value_len) / 2;
                right_spaces = (self.spaces_between * 2 + max_width) - (left_spaces + value_len);
            }
            TPrintAlign::Right => {
                right_spaces = self.spaces_between;
                left_spaces = (self.spaces_between * 2 + max_width) - (right_spaces + value_len);
            }
        }

        self.output.borrow_mut().print_str(&" ".repeat(left_spaces));
        self.output.borrow_mut().print_str(value);
        self.output.borrow_mut().print_str(&" ".repeat(right_spaces));
    }

    pub fn print(&self) {
        let total_rows: usize = self.columns.iter().map(|c| c.get_rows_count()).max().unwrap_or(0);

        self.output.borrow_mut().print_str(self.borders.borrow().get_intro());

        if self.show_headers {
            self.print_horizonal_border(&TPrintBordersType::HeaderTopLeft, &TPrintBordersType::HeaderTopRight, &TPrintBordersType::HeaderTopMiddle, &TPrintBordersType::HeaderHLine);

            if self.spaces_left > 0 {
                self.output.borrow_mut().print_str(&" ".repeat(self.spaces_left));
            }

            self.print_left_border(&TPrintBordersType::HeaderVLine);

            for cid in 0..self.columns.len() {
                let c = &self.columns[cid];

                if cid > 0 {
                    self.print_internal_border(&TPrintBordersType::HeaderVLine);
                }

                self.print_cell(c.get_caption(), c.get_max_width(), c.get_caption_align());
            }

            self.print_right_border(&TPrintBordersType::HeaderVLine);

            self.print_horizonal_border(&TPrintBordersType::HeaderBottomLeft, &TPrintBordersType::HeaderBottomRight, &TPrintBordersType::HeaderBottomMiddle, &TPrintBordersType::HeaderHLine);

        } else {
            self.print_horizonal_border(&TPrintBordersType::TopLeft, &TPrintBordersType::TopRight, &TPrintBordersType::TopMiddle, &TPrintBordersType::TopHLine);
        }

        for r in 0..total_rows {
            if self.spaces_left > 0 {
                self.output.borrow_mut().print_str(&" ".repeat(self.spaces_left));
            }

            self.print_left_border(&TPrintBordersType::MiddleLeftVLine);

            for cid in 0..self.columns.len() {
                let c = &self.columns[cid];
                if cid > 0 {
                    self.print_internal_border(&TPrintBordersType::MiddleMiddleVLine);
                }

                self.print_cell(c.get_str(r), c.get_max_width(), c.get_data_align());
            }
            self.print_right_border(&TPrintBordersType::MiddleRightVLine);

            if r < total_rows - 1 {
                self.print_horizonal_border(&TPrintBordersType::MiddleLeft, &TPrintBordersType::MiddleRight, &TPrintBordersType::MiddleMiddle, &TPrintBordersType::MiddleHLine);
            }
        }

        self.print_horizonal_border(&TPrintBordersType::BottomLeft, &TPrintBordersType::BottomRight, &TPrintBordersType::BottomMiddle, &TPrintBordersType::BottomHLine);
    }
}