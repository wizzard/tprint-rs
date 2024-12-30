#[derive(PartialEq)]
pub enum TPrintBordersType {
    WhiteSpace,
    NewLine,

    TopLeft,
    TopRight,
    TopMiddle,

    BottomLeft,
    BottomRight,
    BottomMiddle,

    MiddleLeft,
    MiddleRight,
    MiddleMiddle,

    TopHLine,
    BottomHLine,

    HeaderTopLeft,
    HeaderTopRight,
    HeaderTopMiddle,

    HeaderBottomLeft,
    HeaderBottomRight,
    HeaderBottomMiddle,

    HeaderHLine,
    HeaderVLine,

    MiddleHLine,

    MiddleLeftVLine,
    MiddleRightVLine,
    MiddleMiddleVLine,
}

pub trait TPrintBorders {
    fn get_intro(&self) -> &str;
    fn get_closing(&self) -> &str;
    fn get_border(&self, btype: &TPrintBordersType) -> &str;
}

struct TPrintBordersTable {
    border_type: TPrintBordersType,
    border: &'static str
}


pub struct TPrintBordersASCII {}

static TPRINT_BORDERS_ASCII_TABLE: &[TPrintBordersTable] = &[
    TPrintBordersTable { border_type: TPrintBordersType::WhiteSpace, border: " " },
    TPrintBordersTable { border_type: TPrintBordersType::NewLine, border: "\n" },

    TPrintBordersTable { border_type: TPrintBordersType::TopLeft, border: "+" },
    TPrintBordersTable { border_type: TPrintBordersType::TopRight, border: "+" },
    TPrintBordersTable { border_type: TPrintBordersType::TopMiddle, border: "+" },

    TPrintBordersTable { border_type: TPrintBordersType::BottomLeft, border: "+" },
    TPrintBordersTable { border_type: TPrintBordersType::BottomRight, border: "+" },
    TPrintBordersTable { border_type: TPrintBordersType::BottomMiddle, border: "+" },

    TPrintBordersTable { border_type: TPrintBordersType::MiddleLeft, border: "+" },
    TPrintBordersTable { border_type: TPrintBordersType::MiddleRight, border: "+" },
    TPrintBordersTable { border_type: TPrintBordersType::MiddleMiddle, border: "+" },

    TPrintBordersTable { border_type: TPrintBordersType::TopHLine, border: "-" },
    TPrintBordersTable { border_type: TPrintBordersType::BottomHLine, border: "-" },

    TPrintBordersTable { border_type: TPrintBordersType::HeaderTopLeft, border: "+" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderTopRight, border: "+" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderTopMiddle, border: "+" },

    TPrintBordersTable { border_type: TPrintBordersType::HeaderBottomLeft, border: "+" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderBottomRight, border: "+" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderBottomMiddle, border: "+" },

    TPrintBordersTable { border_type: TPrintBordersType::HeaderHLine, border: "=" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderVLine, border: "|" },

    TPrintBordersTable { border_type: TPrintBordersType::MiddleHLine, border: "-" },

    TPrintBordersTable { border_type: TPrintBordersType::MiddleLeftVLine, border: "|" },
    TPrintBordersTable { border_type: TPrintBordersType::MiddleRightVLine, border: "|" },
    TPrintBordersTable { border_type: TPrintBordersType::MiddleMiddleVLine, border: "|" },
];

impl TPrintBorders for TPrintBordersASCII {
    fn get_border(&self, btype: &TPrintBordersType) -> &str {
        for b in TPRINT_BORDERS_ASCII_TABLE {
            if b.border_type == *btype {
                return b.border;
            }
        }
        ""
    }

    fn get_intro(&self) -> &str {
        ""
    }
    fn get_closing(&self) -> &str {
        ""
    }
}

pub struct TPrintBordersUnicode {}

// https://www.utf8-chartable.de/unicode-utf8-table.pl?start=9472&unicodeinhtml=dec
static TPRINT_BORDERS_UNICODE_TABLE: &[TPrintBordersTable] = &[
    TPrintBordersTable { border_type: TPrintBordersType::WhiteSpace, border: " " },
    TPrintBordersTable { border_type: TPrintBordersType::NewLine, border: "\n" },

    TPrintBordersTable { border_type: TPrintBordersType::TopLeft, border: "┏" },
    TPrintBordersTable { border_type: TPrintBordersType::TopRight, border: "┓" },
    TPrintBordersTable { border_type: TPrintBordersType::TopMiddle, border: "┯" },

    TPrintBordersTable { border_type: TPrintBordersType::BottomLeft, border: "┗" },
    TPrintBordersTable { border_type: TPrintBordersType::BottomRight, border: "┛" },
    TPrintBordersTable { border_type: TPrintBordersType::BottomMiddle, border: "┷" },

    TPrintBordersTable { border_type: TPrintBordersType::MiddleLeft, border: "┠" },
    TPrintBordersTable { border_type: TPrintBordersType::MiddleRight, border: "┨" },
    TPrintBordersTable { border_type: TPrintBordersType::MiddleMiddle, border: "┼" },

    TPrintBordersTable { border_type: TPrintBordersType::TopHLine, border: "━" },
    TPrintBordersTable { border_type: TPrintBordersType::BottomHLine, border: "━" },

    TPrintBordersTable { border_type: TPrintBordersType::HeaderTopLeft, border: "┏" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderTopRight, border: "┓" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderTopMiddle, border: "┳" },

    TPrintBordersTable { border_type: TPrintBordersType::HeaderBottomLeft, border: "┣" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderBottomRight, border: "┫" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderBottomMiddle, border: "╇" },

    TPrintBordersTable { border_type: TPrintBordersType::HeaderHLine, border: "━" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderVLine, border: "┃" },

    TPrintBordersTable { border_type: TPrintBordersType::MiddleHLine, border: "─" },

    TPrintBordersTable { border_type: TPrintBordersType::MiddleLeftVLine, border: "┃" },
    TPrintBordersTable { border_type: TPrintBordersType::MiddleRightVLine, border: "┃" },
    TPrintBordersTable { border_type: TPrintBordersType::MiddleMiddleVLine, border: "│" },
];

impl TPrintBorders for TPrintBordersUnicode {
    fn get_border(&self, btype: &TPrintBordersType) -> &str {
        for b in TPRINT_BORDERS_UNICODE_TABLE {
            if b.border_type == *btype {
                return b.border;
            }
        }
        ""
    }

    fn get_intro(&self) -> &str {
        ""
    }
    fn get_closing(&self) -> &str {
        ""
    }
}


pub struct TPrintBordersHTML {}


static TPRINT_BORDERS_HTML_TABLE: &[TPrintBordersTable] = &[
    TPrintBordersTable { border_type: TPrintBordersType::WhiteSpace, border: "" },
    TPrintBordersTable { border_type: TPrintBordersType::NewLine, border: "" },

    TPrintBordersTable { border_type: TPrintBordersType::TopLeft, border: "<THEAD><TR>" },
    TPrintBordersTable { border_type: TPrintBordersType::TopRight, border: "" },
    TPrintBordersTable { border_type: TPrintBordersType::TopMiddle, border: "<TH>" },

    TPrintBordersTable { border_type: TPrintBordersType::BottomLeft, border: "<TBODY>" },
    TPrintBordersTable { border_type: TPrintBordersType::BottomRight, border: "" },
    TPrintBordersTable { border_type: TPrintBordersType::BottomMiddle, border: "<TD>" },

    TPrintBordersTable { border_type: TPrintBordersType::MiddleLeft, border: "||" },
    TPrintBordersTable { border_type: TPrintBordersType::MiddleRight, border: "||" },
];

impl TPrintBorders for TPrintBordersHTML {
    fn get_border(&self, btype: &TPrintBordersType) -> &str {
        for b in TPRINT_BORDERS_HTML_TABLE {
            if b.border_type == *btype {
                return b.border;
            }
        }
        ""
    }

    fn get_intro(&self) -> &str {
        "<TABLE border='1' cellpadding='1' cellspacing='1'>"
    }
    fn get_closing(&self) -> &str {
        "</TABLE>"
    }
}