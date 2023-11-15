/* Type code for values. */
 pub const BOOL_VALUE: u32 = 0;
pub const INT_VALUE:u32 = BOOL_VALUE + 1;
pub const REAL_VALUE:u32 = INT_VALUE + 1;
pub const STRING_VALUE:u32 = REAL_VALUE + 1;
pub const RECORD_VALUE:u32 = STRING_VALUE + 1;
pub const SET_ENUM_VALUE:u32 = RECORD_VALUE + 1;
pub const SET_PRED_VALUE:u32 = SET_ENUM_VALUE + 1;
pub const TUPLE_VALUE:u32 = SET_PRED_VALUE + 1;
pub const FCN_LAMBDA_VALUE:u32 = TUPLE_VALUE + 1;
pub const FCN_RCD_VALUE:u32 = FCN_LAMBDA_VALUE + 1;
pub const OP_LAMBDA_VALUE:u32 = FCN_RCD_VALUE + 1;
pub const OP_RCD_VALUE:u32 = OP_LAMBDA_VALUE + 1;
pub const METHOD_VALUE:u32 = OP_RCD_VALUE + 1;
pub const SET_OF_FCNS_VALUE:u32 = METHOD_VALUE + 1;
pub const SET_OF_RCDS_VALUE:u32 = SET_OF_FCNS_VALUE + 1;
pub const SET_OF_TUPLES_VALUE:u32 = SET_OF_RCDS_VALUE + 1;
pub const SUBSET_VALUE:u32 = SET_OF_TUPLES_VALUE + 1;
pub const SET_DIFF_VALUE:u32 = SUBSET_VALUE + 1;
pub const SET_CAP_VALUE:u32 = SET_DIFF_VALUE + 1;
pub const SET_CUP_VALUE:u32 = SET_CAP_VALUE + 1;
pub const UNION_VALUE:u32 = SET_CUP_VALUE + 1;
pub const MODEL_VALUE:u32 = UNION_VALUE + 1;
#[allow(dead_code)]
pub const USER_VALUE:u32 = MODEL_VALUE + 1;
#[allow(dead_code)]
pub const INTER_VAL_VALUE:u32 = USER_VALUE + 1;
#[allow(dead_code)]
pub const UNDEF_VALUE:u32 = INTER_VAL_VALUE + 1;
#[allow(dead_code)]
pub const LAZY_VALUE:u32 = UNDEF_VALUE + 1;
#[allow(dead_code)]
pub const DUMMY_VALUE:u32 = LAZY_VALUE + 1;