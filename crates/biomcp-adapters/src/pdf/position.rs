//! Where text sits on a PDF page and where that position calls for a break.
use super::state::{Matrix, State};

/// A gap wider than a tenth of an em is a word break.
pub(super) const WORD_GAP: f32 = 0.1;
/// A baseline move wider than half an em starts another line.
const LINE_GAP: f32 = 0.5;

/// A glyph position in page space, with the font height on the page.
#[derive(Clone, Copy)]
pub(super) struct Position {
    pub(super) page: Matrix,
    pub(super) height: f32,
}

/// Where the last shown glyph ended, or the array fallback when its width is unknown.
#[derive(Clone, Copy)]
pub(super) struct End {
    position: Position,
    pub(super) measured: bool,
    array: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Place {
    Joined,
    Word,
    Line,
    RaisedDigit,
}

impl Position {
    pub(super) fn new(matrix: Matrix, state: &State<'_>) -> Self {
        let page = matrix.moved(0.0, state.rise).times(state.ctm);
        Self {
            page,
            height: state.size * page.c.hypot(page.d),
        }
    }

    pub(super) fn end(self, measured: bool, array: bool) -> End {
        End {
            position: self,
            measured,
            array,
        }
    }

    pub(super) fn place(self, end: End, digits_touch: bool) -> Place {
        if !end.measured {
            return if end.array {
                Place::Word
            } else {
                Place::Joined
            };
        }
        let prior = end.position;
        let height = prior.height.max(self.height);
        let length = self.page.a.hypot(self.page.b);
        let (x, y) = (self.page.e - prior.page.e, self.page.f - prior.page.f);
        let (along_x, along_y) = (self.page.a / length, self.page.b / length);
        let along = x * along_x + y * along_y;
        let rise = y * along_x - x * along_y;
        // A degenerate or non-finite matrix places nothing, so keep the words apart.
        if height <= 0.0
            || length <= 0.0
            || !(height.is_finite() && along.is_finite() && rise.is_finite())
        {
            Place::Word
        } else if digits_touch
            && along.abs() <= WORD_GAP * height
            && self.raised_digit(prior, rise, height)
        {
            Place::RaisedDigit
        } else if rise.abs() > LINE_GAP * height {
            Place::Line
        } else if along > WORD_GAP * height || along < -height {
            Place::Word
        } else {
            Place::Joined
        }
    }

    fn raised_digit(self, prior: Self, rise: f32, height: f32) -> bool {
        let (small, raised_by) = if self.height < prior.height {
            (self.height, rise)
        } else {
            (prior.height, -rise)
        };
        small > 0.0 && small <= 0.8 * height && (0.15 * height..=0.65 * height).contains(&raised_by)
    }
}
