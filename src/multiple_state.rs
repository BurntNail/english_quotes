use tui::{
    buffer::Buffer,
    layout::{Corner, Rect},
    style::Style,
    widgets::{Block, StatefulWidget, Widget},
};
use tui::text::Text;
use unicode_width::UnicodeWidthStr;

#[derive(Clone, Default)]
pub struct MultipleListState {
    offset: usize,
    selected: Vec<usize>,
    highlighted: Option<usize>
}

impl MultipleListState {
    pub fn selected(&self) -> Option<Vec<usize>> {
        if self.selected.is_empty() {
            None
        } else {
            Some(self.selected.clone())
        }
    }

    pub fn select(&mut self, index: usize) {
        let current_index = self.selected.iter().position(|x| x == &index);
        
        if let Some(p) = current_index {
            self.selected.remove(p);
        } else {
            self.selected.push(index);
        }
    }
    
    pub fn highlight (&mut self, index: Option<usize>) {
        self.highlighted = index;
    }
    
    pub fn highlighted (&self) -> Option<usize> {
        self.highlighted
    }
}

#[derive(Debug, Clone)]
pub struct MultipleList<'a> {
    block: Option<Block<'a>>,
    items: Vec<MultipleListItem<'a>>,
    /// Style used as a base style for the widget
    style: Style,
    start_corner: Corner,
    /// Style used to render selected items
    highlight_style: Style,
    /// Style used to render highlighted item
    non_select_style: Style,
    /// Style used to render when item is highlighted and selected
    both_style: Style,
    /// Symbol in front of the selected item (Shift all items to the right)
    highlight_symbol: Option<&'a str>,
}

impl<'a> MultipleList<'a> {
    pub fn new<T>(items: T) -> MultipleList<'a>
    where
        T: Into<Vec<MultipleListItem<'a>>>,
    {
        Self {
            block: None,
            style: Style::default(),
            items: items.into(),
            start_corner: Corner::TopLeft,
            highlight_style: Style::default(),
            non_select_style: Style::default(),
            both_style: Style::default(),
            highlight_symbol: None,
        }
    }

    pub fn block(mut self, block: Block<'a>) -> MultipleList<'a> {
        self.block = Some(block);
        self
    }

    pub fn style(mut self, style: Style) -> MultipleList<'a> {
        self.style = style;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> MultipleList<'a> {
        self.highlight_symbol = Some(highlight_symbol);
        self
    }
    
    pub fn both_style (mut self, style: Style) -> MultipleList<'a> {
        self.both_style = style;
        self
    }
    
    pub fn non_select_style(mut self, style: Style) -> MultipleList<'a> {
        self.non_select_style = style;
        self
    }

    pub fn highlight_style(mut self, style: Style) -> MultipleList<'a> {
        self.highlight_style = style;
        self
    }

    pub fn start_corner(mut self, corner: Corner) -> MultipleList<'a> {
        self.start_corner = corner;
        self
    }
}

impl<'a> StatefulWidget for MultipleList<'a> {
    type State = MultipleListState;

    fn render(mut self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);
        let list_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        if list_area.width < 1 || list_area.height < 1 {
            return;
        }

        if self.items.is_empty() {
            return;
        }
        let list_height = list_area.height as usize;

        let mut start = state.offset;
        let mut end = state.offset;
        let mut height = 0;
        for item in self.items.iter().skip(state.offset) {
            if height + item.height() > list_height {
                break;
            }
            height += item.height();
            end += 1;
        }
	    
	    for selected in state.selected.clone() {
		    while selected >= end {
			    height = height.saturating_add(self.items[end].height());
			    end += 1;
			    while height > list_height {
				    height = height.saturating_sub(self.items[start].height());
				    start += 1;
			    }
		    }
		    while selected < start {
			    start -= 1;
			    height = height.saturating_add(self.items[start].height());
			    while height > list_height {
				    end -= 1;
				    height = height.saturating_sub(self.items[end].height());
			    }
		    }
		    state.offset = start;
	    }

        let highlight_symbol = self.highlight_symbol.unwrap_or("");
        let blank_symbol = " ".repeat(highlight_symbol.width());

        let mut current_height = 0;
        let has_selection = !state.selected.is_empty();
        for (i, item) in self
            .items
            .iter_mut()
            .enumerate()
            .skip(state.offset)
            .take(end - start)
        {
            let (x, y) = match self.start_corner {
                Corner::BottomLeft => {
                    current_height += item.height() as u16;
                    (list_area.left(), list_area.bottom() - current_height)
                }
                _ => {
                    let pos = (list_area.left(), list_area.top() + current_height);
                    current_height += item.height() as u16;
                    pos
                }
            };
            let area = Rect {
                x,
                y,
                width: list_area.width,
                height: item.height() as u16,
            };
            let item_style = self.style.patch(item.style);
            buf.set_style(area, item_style);

            let is_selected = state.selected.contains(&i);
            let is_highlighted = state.highlighted.map(|x| x == i).unwrap_or(false);
            
            let elem_x = if has_selection {
                let symbol = if is_selected || is_highlighted {
                    highlight_symbol
                } else {
                    &blank_symbol
                };
                let (x, _) = buf.set_stringn(x, y, symbol, list_area.width as usize, item_style);
                x
            } else {
                x
            };

            let max_element_width = (list_area.width - (elem_x - x)) as usize;
            for (j, line) in item.content.lines.iter().enumerate() {
                buf.set_spans(elem_x, y + j as u16, line, max_element_width as u16);
            }
            
            if is_highlighted && is_selected {
                buf.set_style(area, self.both_style)
            } else if is_selected {
                buf.set_style(area, self.highlight_style);
            } else if is_highlighted {
                buf.set_style(area, self.non_select_style);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MultipleListItem<'a> {
    content: Text<'a>,
    style: Style,
}

impl<'a> MultipleListItem<'a> {
    pub fn new<T>(content: T) -> MultipleListItem<'a>
        where
            T: Into<Text<'a>>,
    {
        MultipleListItem {
            content: content.into(),
            style: Style::default(),
        }
    }
    
    pub fn style(mut self, style: Style) -> MultipleListItem<'a> {
        self.style = style;
        self
    }
    
    pub fn height(&self) -> usize {
        self.content.height()
    }
}