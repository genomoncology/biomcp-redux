//! Preserve registry line structure within the escaped JSON text budget.
const BUDGET: usize = 6000;

pub(super) fn fragment(text: &str, offset: usize) -> (&str, bool) {
    let start = text
        .char_indices()
        .nth(offset)
        .map_or(text.len(), |(i, _)| i);
    let remaining = text.split_at(start).1;
    let mut lines = remaining.split_inclusive('\n').peekable();
    let mut end = 0;
    let mut bytes = 0;
    while let Some(line) = lines.next() {
        let mut group = line.len();
        let indent = indentation(line);
        while let Some(next) = lines.peek() {
            if !next.trim().is_empty() && indentation(next) <= indent {
                break;
            }
            group += next.len();
            lines.next();
        }
        let group_end = end + group;
        let cost =
            biomcp_adapters::jats::escaped_bytes(remaining.split_at(end).1.split_at(group).0);
        if bytes + cost > BUDGET {
            if end > 0 {
                return (remaining.split_at(end).0, false);
            }
            return oversized(remaining.split_at(group_end).0);
        }
        bytes += cost;
        end = group_end;
    }
    (remaining, false)
}

fn indentation(line: &str) -> usize {
    line.chars()
        .take_while(|ch| matches!(ch, ' ' | '\t'))
        .count()
}

fn oversized(group: &str) -> (&str, bool) {
    let mut end = 0;
    let mut bytes = 0;
    for line in group.split_inclusive('\n') {
        let cost = biomcp_adapters::jats::escaped_bytes(line);
        if bytes + cost > BUDGET {
            if end > 0 {
                return (group.split_at(end).0, true);
            }
            return split_line(line);
        }
        bytes += cost;
        end += line.len();
    }
    (group, false)
}

fn split_line(line: &str) -> (&str, bool) {
    let mut bytes = 0;
    let mut end = 0;
    for (index, ch) in line.char_indices() {
        let cost = biomcp_adapters::jats::escaped_bytes(ch.encode_utf8(&mut [0; 4]));
        if bytes + cost > BUDGET {
            break;
        }
        bytes += cost;
        end = index + ch.len_utf8();
    }
    (line.split_at(end).0, true)
}
