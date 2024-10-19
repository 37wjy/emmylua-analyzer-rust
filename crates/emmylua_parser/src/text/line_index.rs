use rowan::TextSize;

pub struct LineIndex {
    line_offsets: Vec<TextSize>,
    line_only_ascii_vec: Vec<bool>,
}

impl LineIndex {
    pub fn parse(text: &str) -> LineIndex {
        let mut line_offsets = Vec::new();
        let mut line_only_ascii_vec = Vec::new();
        let mut offset = 0;

        line_offsets.push(TextSize::from(offset as u32));

        let mut is_line_only_ascii = true;
        for (i, c) in text.char_indices() {
            if c == '\n' {
                offset = i + 1; // 记录每行的字节偏移量
                line_offsets.push(TextSize::from(offset as u32));
                line_only_ascii_vec.push(is_line_only_ascii);
                is_line_only_ascii = true;
            } else {
                if !c.is_ascii() {
                    is_line_only_ascii = false;
                }
            }
        }

        line_only_ascii_vec.push(is_line_only_ascii);

        assert_eq!(line_offsets.len(), line_only_ascii_vec.len());
        LineIndex {
            line_offsets,
            line_only_ascii_vec,
        }
    }

    pub fn get_line_offset(&self, line: usize) -> Option<TextSize> {
        let line_index = line;
        if line_index < self.line_offsets.len() {
            let line_offset = self.line_offsets[line_index];
            Some(line_offset)
        } else {
            None
        }
    }

    pub fn get_line(&self, offset: TextSize) -> Option<usize> {
        let offset_value = usize::from(offset);
        match self
            .line_offsets
            .binary_search(&TextSize::from(offset_value as u32))
        {
            Ok(line) => Some(line),
            Err(line) => {
                if line > 0 {
                    Some(line - 1)
                } else {
                    None
                }
            }
        }
    }

    pub fn get_line_with_start_offset(&self, offset: TextSize) -> Option<(usize, TextSize)> {
        let line = self.get_line(offset)?;
        let start_offset = self.line_offsets[line];
        Some((line, start_offset))
    }

    pub fn is_line_only_ascii(&self, line: TextSize) -> bool {
        let line_index = usize::from(line);
        if line_index < self.line_only_ascii_vec.len() {
            self.line_only_ascii_vec[line_index]
        } else {
            false
        }
    }

    pub fn line_count(&self) -> usize {
        self.line_offsets.len()
    }
}
