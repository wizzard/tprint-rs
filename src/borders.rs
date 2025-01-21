/// Represents borders for tables
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
    HeaderLeftVLine,
    HeaderMiddleVLine,
    HeaderRightVLine,

    MiddleHLine,

    MiddleLeftVLine,
    MiddleRightVLine,
    MiddleMiddleVLine,
}

pub trait TPrintBorders {
    fn get_intro(&self, show_borders: bool, show_headers: bool) -> &str;
    fn get_closing(&self) -> &str;
    fn get_border(&self, btype: &TPrintBordersType) -> &str;
}

struct TPrintBordersTable {
    border_type: TPrintBordersType,
    border: &'static str
}

/// ASCII borders (used by default)
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
    TPrintBordersTable { border_type: TPrintBordersType::HeaderLeftVLine, border: "|" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderMiddleVLine, border: "|" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderRightVLine, border: "|" },

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

    fn get_intro(&self, _show_borders: bool, _show_headers: bool) -> &str {
        ""
    }
    fn get_closing(&self) -> &str {
        ""
    }
}

/// Unicode borders
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
    TPrintBordersTable { border_type: TPrintBordersType::HeaderLeftVLine, border: "┃" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderMiddleVLine, border: "┃" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderRightVLine, border: "┃" },

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

    fn get_intro(&self, _show_borders: bool, _show_headers: bool) -> &str {
        ""
    }
    fn get_closing(&self) -> &str {
        ""
    }
}

/// HTML borders
pub struct TPrintBordersHTML {}

static TPRINT_BORDERS_HTML_TABLE: &[TPrintBordersTable] = &[
    TPrintBordersTable { border_type: TPrintBordersType::WhiteSpace, border: "" },
    TPrintBordersTable { border_type: TPrintBordersType::NewLine, border: "" },

    TPrintBordersTable { border_type: TPrintBordersType::TopLeft, border: "<TBODY>" },
    TPrintBordersTable { border_type: TPrintBordersType::TopRight, border: "" },
    TPrintBordersTable { border_type: TPrintBordersType::TopMiddle, border: "<TH>" },

    TPrintBordersTable { border_type: TPrintBordersType::BottomLeft, border: "" },
    TPrintBordersTable { border_type: TPrintBordersType::BottomRight, border: "</TBODY>" },
    TPrintBordersTable { border_type: TPrintBordersType::BottomMiddle, border: "" },

    TPrintBordersTable { border_type: TPrintBordersType::MiddleLeft, border: "" },
    TPrintBordersTable { border_type: TPrintBordersType::MiddleRight, border: "" },

    TPrintBordersTable { border_type: TPrintBordersType::TopHLine, border: "" },
    TPrintBordersTable { border_type: TPrintBordersType::BottomHLine, border: "" },

    TPrintBordersTable { border_type: TPrintBordersType::HeaderTopLeft, border: "<THEAD>" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderTopRight, border: "" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderTopMiddle, border: "" },

    TPrintBordersTable { border_type: TPrintBordersType::HeaderBottomLeft, border: "" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderBottomRight, border: "</THEAD><TBODY>" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderBottomMiddle, border: "" },

    TPrintBordersTable { border_type: TPrintBordersType::HeaderHLine, border: "" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderLeftVLine, border: "<TR><TH>" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderMiddleVLine, border: "</TH><TH>" },
    TPrintBordersTable { border_type: TPrintBordersType::HeaderRightVLine, border: "</TH></TR>" },


    TPrintBordersTable { border_type: TPrintBordersType::MiddleHLine, border: "" },

    TPrintBordersTable { border_type: TPrintBordersType::MiddleLeftVLine, border: "<TR><TD>" },
    TPrintBordersTable { border_type: TPrintBordersType::MiddleRightVLine, border: "</TD></TR>" },
    TPrintBordersTable { border_type: TPrintBordersType::MiddleMiddleVLine, border: "</TD><TD>" },
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

    fn get_intro(&self, show_borders: bool, _show_headers: bool) -> &str {
        if show_borders {
            return "<TABLE border='1' cellpadding='1' cellspacing='1'>"
        }

        "<TABLE border='0' cellpadding='1' cellspacing='1'>"
    }
    fn get_closing(&self) -> &str {
        "</TABLE>"
    }
}