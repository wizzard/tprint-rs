use std::{fmt, io};
use std::cell::RefCell;
use std::rc::Rc;

use crate::column::{TPrintColumn, TPrintAlign};
use crate::output::{TPrintOutput, TPrintOutputStdout};
use crate::borders::{TPrintBorders, TPrintBordersASCII, TPrintBordersType};
use crate::utils::get_string_width;

type TPrintOutputRef = Rc<RefCell<dyn TPrintOutput>>;
type TPrintBordersRef = Rc<RefCell<dyn TPrintBorders>>;

/// A struct to store the table data
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
    /// Creates a new TPrint object with the default ASCII borders and default output to stdout.
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
    /// Creates a new TPrint object with the default ASCII borders and the specified output.
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

    /// Creates a new TPrint object with the specified borders and default output to stdout.
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

    /// Creates a new TPrint object with the specified borders and output.
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

    /// Adds a new column with the specified caption text, caption alignment and cells alignment.
    /// Columns must be defined before adding cell data to the table.
    pub fn column_add(&mut self, caption: &str, caption_align: TPrintAlign, cell_align: TPrintAlign) -> &mut Self {
        self.columns.push(TPrintColumn::new(caption, caption_align, cell_align));
        self
    }

    /// Adds a cell data to the table. The data is added to columns in a consecutive order.
    ///
    /// # Examples
    ///
    /// ## Basic Usage
    /// ```
    /// use tprint::{TPrint, TPrintAlign};
    /// let mut tp = TPrint::new(true, true, 0, 1);
    /// tp.column_add("Name", TPrintAlign::Center, TPrintAlign::Left);
    /// tp.column_add("Count", TPrintAlign::Center, TPrintAlign::Left);
    /// tp.add_data("test");
    /// tp.add_data(42);
    /// tp.print()?;
    /// ```
    ///
    /// ## Chained Usage
    /// ```
    /// use tprint::{TPrint, TPrintAlign};
    /// TPrint::new(true, true, 0, 1).
    ///     column_add("Name", TPrintAlign::Center, TPrintAlign::Left).
    ///     column_add("Count", TPrintAlign::Center, TPrintAlign::Left).
    ///     add_data("test").add_data(42).
    ///     print()?;
    /// ```
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

    fn print_horizonal_border(&self, left: &TPrintBordersType, right: &TPrintBordersType, middle: &TPrintBordersType, line: &TPrintBordersType) -> io::Result<()> {
        if !self.show_borders {
            self.output.borrow_mut().print_str(self.borders.borrow().get_border(&TPrintBordersType::NewLine))?;
            return Ok(());
        }

        if self.spaces_left > 0 {
            self.output.borrow_mut().print_str(&self.borders.borrow().get_border(&TPrintBordersType::WhiteSpace).repeat(self.spaces_left))?;
        }

        self.output.borrow_mut().print_str(self.borders.borrow().get_border(left))?;

        for i in 0..self.columns.len() {
            let c = &self.columns[i];

            self.output.borrow_mut().print_str(&self.borders.borrow().get_border(line).repeat(c.get_max_width() + 2 * self.spaces_between))?;

            if i < self.columns.len() - 1 {
                self.output.borrow_mut().print_str(self.borders.borrow().get_border(middle))?;
            }
        }

        self.output.borrow_mut().print_str(self.borders.borrow().get_border(right))?;
        self.output.borrow_mut().print_str(self.borders.borrow().get_border(&TPrintBordersType::NewLine))?;

        Ok(())
    }

    fn print_left_border(&self, left_border: &TPrintBordersType) -> io::Result<()> {
        if !self.show_borders {
            return Ok(());
        }
        self.output.borrow_mut().print_str(self.borders.borrow().get_border(left_border))?;
        Ok(())
    }

    fn print_right_border(&self, right_border: &TPrintBordersType) -> io::Result<()> {
        if !self.show_borders {
            return Ok(());
        }
        self.output.borrow_mut().print_str(self.borders.borrow().get_border(right_border))?;
        self.output.borrow_mut().print_str(self.borders.borrow().get_border(&TPrintBordersType::NewLine))?;
        Ok(())
    }

    fn print_internal_border(&self, v_border: &TPrintBordersType) -> io::Result<()> {
        if !self.show_borders {
            return Ok(());
        }
        self.output.borrow_mut().print_str(self.borders.borrow().get_border(v_border))?;
        Ok(())
    }

    fn print_cell(&self, value: &str, max_width: usize, align: &TPrintAlign, whitespace: &str) -> io::Result<()> {
        let left_spaces;
        let right_spaces;

        let value_len = get_string_width(value);

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

        self.output.borrow_mut().print_str(&whitespace.repeat(left_spaces))?;
        self.output.borrow_mut().print_str(value)?;
        self.output.borrow_mut().print_str(&whitespace.repeat(right_spaces))?;
        Ok(())
    }

    /// Prints the table to the output.
    pub fn print(&self) -> io::Result<()> {
        let total_rows: usize = self.columns.iter().map(|c| c.get_rows_count()).max().unwrap_or(0);

        let border = self.borders.borrow();
        let whitespace = border.get_border(&TPrintBordersType::WhiteSpace);

        self.output.borrow_mut().print_str(border.get_intro(self.show_borders, self.show_headers))?;

        if self.show_headers {
            self.print_horizonal_border(&TPrintBordersType::HeaderTopLeft, &TPrintBordersType::HeaderTopRight, &TPrintBordersType::HeaderTopMiddle, &TPrintBordersType::HeaderHLine)?;

            if self.spaces_left > 0 {
                self.output.borrow_mut().print_str(&whitespace.repeat(self.spaces_left))?;
            }

            self.print_left_border(&TPrintBordersType::HeaderLeftVLine)?;

            for cid in 0..self.columns.len() {
                let c = &self.columns[cid];

                if cid > 0 {
                    self.print_internal_border(&TPrintBordersType::HeaderMiddleVLine)?;
                }

                self.print_cell(c.get_caption(), c.get_max_width(), c.get_caption_align(), whitespace)?;
            }

            self.print_right_border(&TPrintBordersType::HeaderRightVLine)?;

            self.print_horizonal_border(&TPrintBordersType::HeaderBottomLeft, &TPrintBordersType::HeaderBottomRight, &TPrintBordersType::HeaderBottomMiddle, &TPrintBordersType::HeaderHLine)?;

        } else {
            self.print_horizonal_border(&TPrintBordersType::TopLeft, &TPrintBordersType::TopRight, &TPrintBordersType::TopMiddle, &TPrintBordersType::TopHLine)?;
        }

        for r in 0..total_rows {
            if self.spaces_left > 0 {
                self.output.borrow_mut().print_str(&whitespace.repeat(self.spaces_left))?;
            }

            self.print_left_border(&TPrintBordersType::MiddleLeftVLine)?;

            for cid in 0..self.columns.len() {
                let c = &self.columns[cid];
                if cid > 0 {
                    self.print_internal_border(&TPrintBordersType::MiddleMiddleVLine)?;
                }

                self.print_cell(c.get_str(r), c.get_max_width(), c.get_cell_align(), whitespace)?;
            }
            self.print_right_border(&TPrintBordersType::MiddleRightVLine)?;

            if r < total_rows - 1 {
                self.print_horizonal_border(&TPrintBordersType::MiddleLeft, &TPrintBordersType::MiddleRight, &TPrintBordersType::MiddleMiddle, &TPrintBordersType::MiddleHLine)?;
            }
        }

        self.print_horizonal_border(&TPrintBordersType::BottomLeft, &TPrintBordersType::BottomRight, &TPrintBordersType::BottomMiddle, &TPrintBordersType::BottomHLine)?;

        self.output.borrow_mut().print_str(border.get_closing())
    }
}