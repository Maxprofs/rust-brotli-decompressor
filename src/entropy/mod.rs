use super::{HuffmanCode, HuffmanTreeGroup};
use super::huffman::histogram::{ANSTable, HistogramSpec};
use super::BrotliResult;
use super::bit_reader;
use core::ops::AddAssign;
use alloc;
use alloc::Allocator;
use alloc::SliceWrapper;
use alloc::SliceWrapperMut;

pub trait BoolTrait {
  const VALUE: bool;
}
pub struct TrueBoolTrait {}
impl BoolTrait for TrueBoolTrait {
  const VALUE: bool = true;
}
pub struct FalseBoolTrait{}
impl BoolTrait for FalseBoolTrait {
  const VALUE: bool = false;
}

pub type Speculative = TrueBoolTrait;
pub type Unconditional = FalseBoolTrait;



pub trait EntropyEncoder {
    fn put<Symbol: Sized+Ord+AddAssign<Symbol>+From<u8> + Clone, AllocS:Allocator<Symbol>, AllocH: Allocator<u32>, AllocU32:Allocator<u32>,AllocHC:Allocator<HuffmanCode>, Spec:HistogramSpec>(&mut self, group:HuffmanTreeGroup<AllocU32, AllocHC>, prob: &ANSTable<u32, Symbol, AllocS, AllocH, Spec>, prior: u8, symbol: Symbol, output:&mut [u8], output_offset:&mut usize) -> BrotliResult;
  fn put_stationary<Symbol: Sized+Ord+AddAssign<Symbol>+From<u8> + Clone, AllocS:Allocator<Symbol>, AllocH: Allocator<u32>, Spec:HistogramSpec>(&mut self, group:&[HuffmanCode], prob: &ANSTable<u32, Symbol, AllocS, AllocH, Spec>, symbol: Symbol, output: &mut[u8], output_offset:&mut usize) -> BrotliResult;
  fn put_uniform(&mut self, nbits: u8, symbol: u16, output: &mut [u8], output_offset: &mut usize);
  fn flush(&mut self, output: &mut[u8], output_offset:&mut usize) -> BrotliResult;
}

pub trait EntropyDecoder {
  type SpeculativeState;
  fn set_active(&mut self);
  fn set_inactive(&mut self);
  fn bit_reader(&mut self) -> &mut bit_reader::BrotliBitReader;
  fn br(&self) -> &bit_reader::BrotliBitReader;
  fn warmup(&mut self, input:&[u8]) -> BrotliResult;
  fn begin_metablock(&mut self, input:&[u8]) -> BrotliResult;
  fn sufficient_bits(&mut self, nbits: u8) -> bool;
  fn placeholder(&self) -> Self::SpeculativeState;
  fn preload<Symbol: Sized+Ord+AddAssign<Symbol>+From<u8> + Clone,
             AllocS:Allocator<Symbol>,
             AllocH: Allocator<u32>,
             Spec:HistogramSpec>(&mut self,
                                   group:&[&[HuffmanCode];256],
                                   prob: &ANSTable<u32, Symbol, AllocS, AllocH, Spec>,
                                   prior: u8,
                                   input:&[u8]) -> (u32, u32);
  // precondition: input has at least 4 bytes
  fn get_preloaded<Symbol: Sized+Ord+AddAssign<Symbol>+From<u8> + Clone,
                   AllocS:Allocator<Symbol>,
                   AllocH: Allocator<u32>,
                   Spec:HistogramSpec>(&mut self,
                                          group:&[&[HuffmanCode];256],
                                          prob: &ANSTable<u32, Symbol, AllocS, AllocH, Spec>,
                                          prior: u8,
                                          preloaded: (u32, u32),
                                          input:&[u8]) -> Symbol;
  // precondition: input has at least 4 bytes
  fn get<Symbol: Sized+Ord+AddAssign<Symbol>+From<u8> + Clone,
         AllocS:Allocator<Symbol>,
         AllocH: Allocator<u32>,
         Spec:HistogramSpec,
         Speculative:BoolTrait>(&mut self,
                                group:&[&[HuffmanCode];256],
                                prob: &ANSTable<u32, Symbol, AllocS, AllocH, Spec>,
                                prior: u8,
                                input:&[u8],
                                is_speculative: Speculative) -> (Symbol, BrotliResult);
  // precondition: input has at least 4 bytes
    fn get_stationary<Symbol: Sized+Ord+AddAssign<Symbol>+From<u8> + Clone, AllocS:Allocator<Symbol>, AllocH: Allocator<u32>, Spec:HistogramSpec, Speculative:BoolTrait>(&mut self, group:&[HuffmanCode], prob: &ANSTable<u32, Symbol, AllocS, AllocH, Spec>, l1numbits: u8, input: &[u8], is_speculative: Speculative) -> (Symbol, BrotliResult);
    // precondition: input has at least 4 bytes
    fn get_uniform<Speculative:BoolTrait>(&mut self, nbits: u8, input: &[u8], is_speculative: Speculative) -> (u32, BrotliResult);
    fn begin_speculative(&mut self) -> Self::SpeculativeState;
    fn commit_speculative(&mut self);
    fn abort_speculative(&mut self, val:Self::SpeculativeState);
}


#[derive(Default)]
pub struct HuffmanDecoder {
  active: bool,
  br: bit_reader::BrotliBitReader,
}

impl EntropyDecoder  for HuffmanDecoder {
  fn set_active(&mut self) {
    self.active = true;
  }
  fn set_inactive(&mut self) {
    self.active = false;
  }
  fn bit_reader(&mut self) -> &mut bit_reader::BrotliBitReader {
    &mut self.br
  }
  fn br(&self) -> &bit_reader::BrotliBitReader {
    &self.br
  }
  fn warmup(&mut self, input:&[u8]) -> BrotliResult{
    if self.active {
      if (!bit_reader::BrotliWarmupBitReader(&mut self.br,
                                             input)) {
        return BrotliResult::NeedsMoreInput;
      }
    }
    BrotliResult::ResultSuccess
  }
  fn begin_metablock(&mut self, input:&[u8]) -> BrotliResult{
    // nothing to do for standard huffman-coded items
    BrotliResult::ResultSuccess
  }
  fn sufficient_bits(&mut self, nbits: u8) -> bool{
    true
  }
    fn preload<Symbol: Sized+Ord+AddAssign<Symbol>+From<u8> + Clone,
               AllocS:Allocator<Symbol>,
               AllocH: Allocator<u32>,
               Spec:HistogramSpec>(&mut self,
                                   group:&[&[HuffmanCode];256],
                                   prob: &ANSTable<u32, Symbol, AllocS, AllocH, Spec>,
                                   prior: u8,
                                   input:&[u8]) -> (u32, u32){
        (0,0)
    }
    // precondition: input has at least 4 bytes
  fn get_preloaded<Symbol: Sized+Ord+AddAssign<Symbol>+From<u8> + Clone,
         AllocS:Allocator<Symbol>,
         AllocH: Allocator<u32>,
         Spec:HistogramSpec>(&mut self,
                                group:&[&[HuffmanCode];256],
                                prob: &ANSTable<u32, Symbol, AllocS, AllocH, Spec>,
                                prior: u8,
                                preloaded: (u32, u32),
                                input:&[u8]) -> Symbol {
    Symbol::from(0)
  }
  fn get<Symbol: Sized+Ord+AddAssign<Symbol>+From<u8> + Clone,
         AllocS:Allocator<Symbol>,
         AllocH: Allocator<u32>,
         Spec:HistogramSpec,
         Speculative:BoolTrait>(&mut self,
                                group:&[&[HuffmanCode];256],
                                prob: &ANSTable<u32, Symbol, AllocS, AllocH, Spec>,
                                prior: u8,
                                input:&[u8],
                                is_speculative: Speculative) -> (Symbol, BrotliResult) {
    (Symbol::from(0), BrotliResult::ResultSuccess)
  }
  // precondition: input has at least 4 bytes
  fn get_stationary<Symbol: Sized+Ord+AddAssign<Symbol>+From<u8> + Clone, AllocS:Allocator<Symbol>, AllocH: Allocator<u32>, Spec:HistogramSpec, Speculative:BoolTrait>(&mut self, group:&[HuffmanCode], prob: &ANSTable<u32, Symbol, AllocS, AllocH, Spec>, l1numbits: u8, input: &[u8], is_speculative: Speculative) -> (Symbol, BrotliResult){
    (Symbol::from(0u8), BrotliResult::ResultSuccess)
  }
  // precondition: input has at least 4 bytes
  fn get_uniform<Speculative:BoolTrait>(&mut self, nbits: u8, input: &[u8], is_speculative:Speculative) -> (u32, BrotliResult){
    let mut ix = 0u32;
    if self.active {
      if !bit_reader::BrotliSafeReadBits(&mut self.br, u32::from(nbits), &mut ix, input) {
        
        return (0, BrotliResult::NeedsMoreInput)
      }
    }
    (ix, BrotliResult::ResultSuccess)
  }
  type SpeculativeState = ();
  fn placeholder(&self) -> Self::SpeculativeState{
    ()
  }
  fn begin_speculative(&mut self) -> Self::SpeculativeState{
    ()
  }
  fn commit_speculative(&mut self){}
  fn abort_speculative(&mut self, val:Self::SpeculativeState){}
}

#[derive(Default)]
pub struct NopEncoder {
}

impl EntropyEncoder for NopEncoder {
  fn put<Symbol: Sized+Ord+AddAssign<Symbol>+From<u8> + Clone, AllocS:Allocator<Symbol>, AllocH: Allocator<u32>, AllocU32:Allocator<u32>,AllocHC:Allocator<HuffmanCode>, Spec:HistogramSpec>(&mut self, group:HuffmanTreeGroup<AllocU32, AllocHC>, prob: &ANSTable<u32, Symbol, AllocS, AllocH, Spec>, prior: u8, symbol: Symbol, output:&mut [u8], output_offset:&mut usize) -> BrotliResult {
    BrotliResult::ResultSuccess
  }
  fn put_stationary<Symbol: Sized+Ord+AddAssign<Symbol>+From<u8> + Clone, AllocS:Allocator<Symbol>, AllocH: Allocator<u32>, Spec:HistogramSpec>(&mut self, group:&[HuffmanCode], prob: &ANSTable<u32, Symbol, AllocS, AllocH, Spec>, symbol: Symbol, output: &mut[u8], output_offset:&mut usize) -> BrotliResult {
    BrotliResult::ResultSuccess
  }
  fn put_uniform(&mut self, nbits: u8, symbol: u16, output: &mut [u8], output_offset: &mut usize){}
  fn flush(&mut self, output: &mut[u8], output_offset:&mut usize) -> BrotliResult {
    BrotliResult::ResultSuccess
  }
}