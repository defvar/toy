use std::fmt;
use std::ops::Mul;

use super::states::{Action, ReadResult, State};
use crate::Terminator;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum TransClass {
    FieldDelimiter = 1,
    Terminator = 2,
    CRorLF = 3,
    QuoteByte = 4,
    EscapeByte = 5,
    CommentByte = 6,
    Value = 7,
}

const TRANS_CLASSES: usize = 7;
const DFA_STATES: usize = 10;
const TRANS_SIZE: usize = TRANS_CLASSES * DFA_STATES;
const CLASS_SIZE: usize = 256;

const STATES: &'static [State] = &[
    State::StartRecord,
    State::StartField,
    State::EndFieldDelimiter,
    State::InField,
    State::InQuotedField,
    State::InEscapedQuote,
    State::InDoubleEscapedQuote,
    State::InComment,
    State::EndRecord,
    State::CRLF,
];

pub struct Dfa {
    delimiter: Option<u8>,
    quote: u8,
    quoting: bool,
    terminator: Terminator,
    escape: Option<u8>,
    double_quote: bool,
    comment: Option<u8>,

    trans: [DfaState; TRANS_SIZE],
    has_output: [bool; TRANS_SIZE],
    classes: DfaClasses,
    in_field: DfaState,
    in_quoted: DfaState,
    final_field: DfaState,
    final_record: DfaState,
}

impl Dfa {
    pub fn with_prop(
        delimiter: Option<u8>,
        quote: u8,
        quoting: bool,
        terminator: Terminator,
        escape: Option<u8>,
        double_quote: bool,
        comment: Option<u8>,
    ) -> Dfa {
        Dfa {
            delimiter,
            quote,
            quoting,
            terminator,
            escape,
            double_quote,
            comment,

            trans: [DfaState(0); TRANS_SIZE],
            has_output: [false; TRANS_SIZE],
            classes: DfaClasses::new(),
            in_field: DfaState(0),
            in_quoted: DfaState(0),
            final_field: DfaState(0),
            final_record: DfaState(0),
        }
    }

    pub fn build(&mut self) -> DfaState {
        if let Some(delimiter) = self.delimiter {
            self.classes.add(TransClass::FieldDelimiter, delimiter);
        }

        if self.quoting {
            self.classes.add(TransClass::QuoteByte, self.quote);
            if let Some(escape) = self.escape {
                self.classes.add(TransClass::EscapeByte, escape);
            }
        }

        if let Some(comment) = self.comment {
            self.classes.add(TransClass::CommentByte, comment);
        }

        match self.terminator {
            Terminator::Any(b) => self.classes.add(TransClass::Terminator, b),
            Terminator::CRLF => {
                self.classes.add(TransClass::Terminator, b'\r');
                self.classes.add(TransClass::CRorLF, b'\n');
            }
        }

        for &state in STATES {
            for c in (0..256).map(|c| c as u8) {
                let mut res = (state, Action::None);

                while res.0 != State::End && res.1 == Action::None {
                    res = self.transition(res.0, c);
                }

                let f = self.new_state_of_value_class(state);
                let t = self.new_state_of_value_class(res.0);
                self.set(f, c, t, res.1 == Action::ToOutput);
            }
        }
        self.finish();
        self.new_state_of_value_class(State::StartRecord)
    }

    #[inline(always)]
    fn transition(&self, state: State, c: u8) -> (State, Action) {
        match state {
            State::End => (State::End, Action::None),
            State::StartRecord => {
                if self.terminator.equals(c) {
                    (State::StartRecord, Action::Discard)
                } else {
                    (State::StartField, Action::None)
                }
            }
            State::EndRecord => (State::StartRecord, Action::None),
            State::StartField => {
                if self.quoting && self.quote == c {
                    (State::InQuotedField, Action::Discard)
                } else if self.delimiter == Some(c) {
                    (State::EndFieldDelimiter, Action::Discard)
                } else if self.terminator.equals(c) {
                    (State::EndFieldTerminator, Action::None)
                } else if self.comment == Some(c) {
                    (State::InComment, Action::Discard)
                } else {
                    (State::InField, Action::ToOutput)
                }
            }
            State::EndFieldDelimiter => (State::StartField, Action::None),
            State::EndFieldTerminator => (State::InRecordTerminator, Action::None),
            State::InField => {
                if self.delimiter == Some(c) {
                    (State::EndFieldDelimiter, Action::Discard)
                } else if self.terminator.equals(c) {
                    (State::EndFieldTerminator, Action::None)
                } else {
                    (State::InField, Action::ToOutput)
                }
            }
            State::InQuotedField => {
                if self.quoting && self.quote == c {
                    (State::InDoubleEscapedQuote, Action::Discard)
                } else if self.quoting && self.escape == Some(c) {
                    (State::InEscapedQuote, Action::Discard)
                } else {
                    (State::InQuotedField, Action::ToOutput)
                }
            }
            State::InEscapedQuote => (State::InQuotedField, Action::ToOutput),
            State::InDoubleEscapedQuote => {
                if self.quoting && self.double_quote && self.quote == c {
                    (State::InQuotedField, Action::ToOutput)
                } else if self.delimiter == Some(c) {
                    (State::EndFieldDelimiter, Action::Discard)
                } else if self.terminator.equals(c) {
                    (State::EndFieldTerminator, Action::None)
                } else {
                    (State::InField, Action::ToOutput)
                }
            }
            State::InComment => {
                if b'\n' == c {
                    (State::StartRecord, Action::Discard)
                } else {
                    (State::InComment, Action::Discard)
                }
            }
            State::InRecordTerminator => {
                if self.terminator.is_crlf() && b'\r' == c {
                    (State::CRLF, Action::Discard)
                } else {
                    (State::EndRecord, Action::Discard)
                }
            }
            State::CRLF => {
                if b'\n' == c {
                    (State::StartRecord, Action::Discard)
                } else {
                    (State::StartRecord, Action::None)
                }
            }
        }
    }

    fn new_state_of_value_class(&self, state: State) -> DfaState {
        let n = TransClass::Value as u8;
        let idx = (state as u8).mul(n);
        DfaState(idx)
    }

    fn set(&mut self, from: DfaState, c: u8, to: DfaState, output: bool) {
        let cls = self.classes.0[c as usize];
        let idx = from.0 as usize + cls as usize;
        self.trans[idx] = to;
        self.has_output[idx] = output;
    }

    pub fn get_output(&self, state: DfaState, c: u8) -> (DfaState, bool) {
        let cls = self.classes.0[c as usize];
        let idx = state.0 as usize + cls as usize;
        (self.trans[idx], self.has_output[idx])
    }

    fn finish(&mut self) {
        self.in_field = self.new_state_of_value_class(State::InField);
        self.in_quoted = self.new_state_of_value_class(State::InQuotedField);
        self.final_field = self.new_state_of_value_class(State::EndFieldDelimiter);
        self.final_record = self.new_state_of_value_class(State::EndRecord);
    }

    #[inline(always)]
    pub fn get_final_field(&self) -> DfaState {
        self.final_field
    }

    #[inline(always)]
    pub fn consume_in_field(
        &self,
        state: DfaState,
        input: &[u8],
        in_pos: &mut usize,
        output: &mut [u8],
        out_pos: &mut usize,
    ) {
        if state == self.in_field || state == self.in_quoted {
            while *in_pos < input.len()
                && *out_pos < output.len()
                && self.classes.0[input[*in_pos] as usize] == 0
            {
                output[*out_pos] = input[*in_pos];
                *in_pos += 1;
                *out_pos += 1;
            }
        }
    }

    pub fn new_read_record_result(
        &self,
        state: DfaState,
        is_final_trans: bool,
        in_done: bool,
        out_done: bool,
        idx_done: bool,
    ) -> ReadResult {
        if state >= self.final_record {
            ReadResult::Record
        } else if is_final_trans && state.is_start() {
            ReadResult::End
        } else {
            debug_assert!(state < self.final_record);
            if !in_done && out_done {
                ReadResult::OutputFull
            } else if !in_done && idx_done {
                ReadResult::OutputEdgeFull
            } else {
                ReadResult::InputEmpty
            }
        }
    }

    pub fn transition_final_dfa(&self, state: DfaState) -> DfaState {
        if state >= self.final_record || state.is_start() {
            self.new_state_of_value_class(State::StartRecord)
        } else {
            self.new_state_of_value_class(State::EndRecord)
        }
    }
}

impl fmt::Debug for Dfa {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dfa({:?})", self.classes)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DfaState(u8);

impl DfaState {
    pub fn start() -> DfaState {
        DfaState(0)
    }

    fn is_start(&self) -> bool {
        self.0 == 0
    }
}

pub struct DfaClasses([u8; CLASS_SIZE]);

impl DfaClasses {
    fn new() -> DfaClasses {
        DfaClasses([0; CLASS_SIZE])
    }

    fn add(&mut self, class: TransClass, b: u8) {
        self.0[b as usize] = class as u8;
    }
}

impl fmt::Debug for DfaClasses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DfaClasses(size:{:?})", self.0.len())
    }
}
