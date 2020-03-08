use super::dfa::{Dfa, DfaState};
use super::ReadResult;

#[derive(Debug)]
pub struct ByteReader {
    dfa: Dfa,
    current_state: DfaState,
    output_pos: usize,
    has_read: bool,

    initial_state: DfaState,
}

impl ByteReader {
    pub fn with_dfa(dfa: Dfa, initial_state: DfaState) -> ByteReader {
        ByteReader {
            dfa,
            current_state: initial_state,
            output_pos: 0,
            has_read: false,
            initial_state,
        }
    }

    pub fn reset(&mut self) {
        self.current_state = self.initial_state;
        self.output_pos = 0;
        self.has_read = false;
    }

    /// parse `input` and copy to `output`, each column edge index to `edges`.
    ///
    /// access by output[start..edge]. edge is "until".
    ///
    pub fn read_record(
        &mut self,
        input: &[u8],
        output: &mut [u8],
        edges: &mut [usize],
    ) -> (ReadResult, usize, usize, usize) {
        let (input, bom) = self.strip_utf8_bom(input);
        let (r, in_pos, out_pos, column) = self.read_record_core(input, output, edges);
        self.has_read = true;
        (r, in_pos + bom, out_pos, column)
    }

    #[inline]
    fn read_record_core(
        &mut self,
        input: &[u8],
        output: &mut [u8],
        edges: &mut [usize],
    ) -> (ReadResult, usize, usize, usize) {
        if input.is_empty() {
            let s = self.dfa.transition_final_dfa(self.current_state);
            let res = self
                .dfa
                .new_read_record_result(s, true, false, false, false);
            return match res {
                ReadResult::Record => {
                    if edges.is_empty() {
                        return (ReadResult::OutputEdgeFull, 0, 0, 0);
                    }
                    self.current_state = s;
                    edges[0] = self.output_pos;
                    self.output_pos = 0;
                    (res, 0, 0, 1)
                }
                _ => {
                    self.current_state = s;
                    (res, 0, 0, 0)
                }
            };
        }
        if output.is_empty() {
            return (ReadResult::OutputFull, 0, 0, 0);
        }
        if edges.is_empty() {
            return (ReadResult::OutputEdgeFull, 0, 0, 0);
        }

        let (mut in_pos, mut out_pos, mut column) = (0, 0, 0);
        let mut state = self.current_state;

        while in_pos < input.len() && out_pos < output.len() && column < edges.len() {
            let (s, has_out) = self.dfa.get_output(state, input[in_pos]);
            state = s;
            if has_out {
                output[out_pos] = input[in_pos];
                out_pos += 1;
            }
            in_pos += 1;
            if state >= self.dfa.get_final_field() {
                edges[column] = self.output_pos + out_pos;
                column += 1;
                if state > self.dfa.get_final_field() {
                    break;
                }
            }
            self.dfa
                .consume_in_field(state, input, &mut in_pos, output, &mut out_pos);
        }

        let res = self.dfa.new_read_record_result(
            state,
            false,
            in_pos >= input.len(),
            out_pos >= output.len(),
            column >= edges.len(),
        );
        self.current_state = state;
        if res.is_record() {
            self.output_pos = 0;
        } else {
            self.output_pos += out_pos;
        }
        (res, in_pos, out_pos, column)
    }

    fn strip_utf8_bom<'a>(&self, input: &'a [u8]) -> (&'a [u8], usize) {
        let (input, nin) =
            if { !self.has_read && input.len() >= 3 && &input[0..3] == b"\xef\xbb\xbf" } {
                (&input[3..], 3)
            } else {
                (input, 0)
            };
        (input, nin)
    }
}
