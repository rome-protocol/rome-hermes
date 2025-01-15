use test_strategy::proptest;

use super::*;

#[proptest]
fn ptb_conversion(ptb: ProgrammableTransaction) {
    let builder: ProgrammableTransactionBuilder = ptb.clone().try_into().expect("into builder");
    let ptb_: ProgrammableTransaction = builder.into();
    assert_eq!(ptb, ptb_);
}
