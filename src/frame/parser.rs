use nom::{
  be_u16,
  IResult,
  ErrorCode, Err,
};

use frame::Footer;

fn blocking_strategy(input: &[u8]) -> IResult<&[u8], bool> {
  match take!(input, 2) {
    IResult::Done(i, bytes)   => {
      let sync_code = ((bytes[0] as u16) << 6) +
                      (bytes[1] as u16) >> 2;
      let is_valid  = ((bytes[1] >> 1) & 0b01) == 0;

      if sync_code == 0b11111111111110 && is_valid {
        let is_variable_block_size = (bytes[1] & 0b01) == 1;

        IResult::Done(i, is_variable_block_size)
      } else {
        IResult::Error(Err::Position(ErrorCode::Digit as u32, input))
      }
    }
    IResult::Error(error)     => IResult::Error(error),
    IResult::Incomplete(need) => IResult::Incomplete(need),
  }
}

fn block_sample(input: &[u8]) -> IResult<&[u8], (u32, u32)> {
  match take!(input, 1) {
    IResult::Done(i, bytes)   => {
      let sample_rate = bytes[0] & 0x0f;

      if sample_rate != 0x0f {
        let block_size = bytes[0] >> 4;

        IResult::Done(i, (block_size as u32, sample_rate as u32))
      } else {
        IResult::Error(Err::Position(ErrorCode::Digit as u32, input))
      }
    }
    IResult::Error(error)     => IResult::Error(error),
    IResult::Incomplete(need) => IResult::Incomplete(need),
  }
}

named!(footer <&[u8], Footer>, map!(be_u16, Footer));
