//! Kinetix bytecode opcodes.

/// All VM opcodes. Each fits in a single byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Op {
    // ── Constants ─────────────────────────────────────────────────────────
    /// Load constant pool entry into register: `LOAD_CONST  dest  const_idx`
    LoadConst  = 0x00,
    /// Load nil into register: `LOAD_NIL  dest`
    LoadNil    = 0x01,
    /// Load bool true: `LOAD_TRUE  dest`
    LoadTrue   = 0x02,
    /// Load bool false: `LOAD_FALSE  dest`
    LoadFalse  = 0x03,
    /// Load small integer (fits in 16 bits): `LOAD_INT  dest  imm16`
    LoadInt    = 0x04,
    /// Load small float (f32 approximation): `LOAD_FLOAT  dest  imm32`
    LoadFloat  = 0x05,

    // ── Registers ─────────────────────────────────────────────────────────
    /// Copy register: `MOV  dest  src`
    Mov        = 0x10,

    // ── Arithmetic ────────────────────────────────────────────────────────
    Add        = 0x20,
    Sub        = 0x21,
    Mul        = 0x22,
    Div        = 0x23,
    Rem        = 0x24,
    Pow        = 0x25,
    Neg        = 0x26,  // Unary negate: `NEG  dest  src`

    // ── Comparison ────────────────────────────────────────────────────────
    Eq         = 0x30,
    Ne         = 0x31,
    Lt         = 0x32,
    Le         = 0x33,
    Gt         = 0x34,
    Ge         = 0x35,

    // ── Logical ───────────────────────────────────────────────────────────
    Not        = 0x40,  // Boolean NOT
    And        = 0x41,
    Or         = 0x42,

    // ── Bitwise ───────────────────────────────────────────────────────────
    BitAnd     = 0x50,
    BitOr      = 0x51,
    BitXor     = 0x52,
    BitNot     = 0x53,
    Shl        = 0x54,
    Shr        = 0x55,

    // ── Memory / collections ──────────────────────────────────────────────
    /// Allocate new vec, push to register: `NEW_VEC  dest  len`
    NewVec     = 0x60,
    /// Push to vec: `VEC_PUSH  vec_reg  val_reg`
    VecPush    = 0x61,
    /// Get element: `VEC_GET  dest  vec_reg  idx_reg`
    VecGet     = 0x62,
    /// Set element: `VEC_SET  vec_reg  idx_reg  val_reg`
    VecSet     = 0x63,
    /// Allocate new matrix: `NEW_MATRIX  dest  rows  cols`
    NewMatrix  = 0x68,
    /// Matrix get: `MATRIX_GET  dest  mat  row  col`
    MatrixGet  = 0x69,
    /// Matrix set: `MATRIX_SET  mat  row  col  val`
    MatrixSet  = 0x6A,

    // ── Locals / Upvalues ─────────────────────────────────────────────────
    /// Load local slot into register: `LOAD_LOCAL  dest  slot`
    LoadLocal  = 0x70,
    /// Store register into local slot: `STORE_LOCAL  slot  src`
    StoreLocal = 0x71,
    /// Load captured (upvalue): `LOAD_UP  dest  idx`
    LoadUp     = 0x72,
    /// Store upvalue: `STORE_UP  idx  src`
    StoreUp    = 0x73,

    // ── Global ────────────────────────────────────────────────────────────
    /// `LOAD_GLOBAL  dest  name_const_idx`
    LoadGlobal  = 0x78,
    /// `STORE_GLOBAL  name_const_idx  src`
    StoreGlobal = 0x79,

    // ── Control flow ──────────────────────────────────────────────────────
    /// Unconditional jump: `JUMP  offset_i16`
    Jump       = 0x80,
    /// Jump if register is false: `JUMP_IF_FALSE  cond_reg  offset_i16`
    JumpFalse  = 0x81,
    /// Jump if register is true: `JUMP_IF_TRUE  cond_reg  offset_i16`
    JumpTrue   = 0x82,
    /// Jump if register is nil: `JUMP_NIL  cond_reg  offset_i16`
    JumpNil    = 0x83,

    // ── Calls ─────────────────────────────────────────────────────────────
    /// Call function: `CALL  dest  fn_reg  argc`
    Call       = 0x90,
    /// Tail call (optimize to loop): `TAIL_CALL  fn_reg  argc`
    TailCall   = 0x91,
    /// Return: `RETURN  src` (or `RETURN_NIL`)
    Return     = 0x92,
    ReturnNil  = 0x93,
    /// Call native (built-in) function: `CALL_NATIVE  dest  native_id  argc`
    CallNative = 0x94,

    // ── Object / struct ───────────────────────────────────────────────────
    /// Get field: `GET_FIELD  dest  obj_reg  field_const_idx`
    GetField   = 0xA0,
    /// Set field: `SET_FIELD  obj_reg  field_const_idx  val_reg`
    SetField   = 0xA1,
    /// Create struct: `NEW_STRUCT  dest  type_const_idx`
    NewStruct  = 0xA2,

    // ── Closures ──────────────────────────────────────────────────────────
    /// Create closure over current upvalues: `CLOSURE  dest  fn_id  upvalue_count`
    Closure    = 0xB0,

    // ── Type operations ───────────────────────────────────────────────────
    /// Type cast: `CAST  dest  src  type_id`
    Cast       = 0xC0,
    /// Type test: `IS_TYPE  dest  src  type_id`
    IsType     = 0xC1,

    // ── I/O intrinsics ────────────────────────────────────────────────────
    /// Print to stdout: `PRINT  src`
    Print      = 0xD0,

    // ── Debug ─────────────────────────────────────────────────────────────
    /// Breakpoint — trap into debugger: `BREAKPOINT`
    Breakpoint = 0xE0,

    // ── Special ───────────────────────────────────────────────────────────
    /// Panic with message: `PANIC  msg_reg`
    Panic      = 0xF0,
    /// No-op: `NOP`
    Nop        = 0xFF,
}

impl Op {
    /// Human-readable name for disassembly output.
    pub fn name(self) -> &'static str {
        match self {
            Op::LoadConst   => "LOAD_CONST",
            Op::LoadNil     => "LOAD_NIL",
            Op::LoadTrue    => "LOAD_TRUE",
            Op::LoadFalse   => "LOAD_FALSE",
            Op::LoadInt     => "LOAD_INT",
            Op::LoadFloat   => "LOAD_FLOAT",
            Op::Mov         => "MOV",
            Op::Add         => "ADD",
            Op::Sub         => "SUB",
            Op::Mul         => "MUL",
            Op::Div         => "DIV",
            Op::Rem         => "REM",
            Op::Pow         => "POW",
            Op::Neg         => "NEG",
            Op::Eq          => "EQ",
            Op::Ne          => "NE",
            Op::Lt          => "LT",
            Op::Le          => "LE",
            Op::Gt          => "GT",
            Op::Ge          => "GE",
            Op::Not         => "NOT",
            Op::And         => "AND",
            Op::Or          => "OR",
            Op::BitAnd      => "BAND",
            Op::BitOr       => "BOR",
            Op::BitXor      => "BXOR",
            Op::BitNot      => "BNOT",
            Op::Shl         => "SHL",
            Op::Shr         => "SHR",
            Op::NewVec      => "NEW_VEC",
            Op::VecPush     => "VEC_PUSH",
            Op::VecGet      => "VEC_GET",
            Op::VecSet      => "VEC_SET",
            Op::NewMatrix   => "NEW_MATRIX",
            Op::MatrixGet   => "MATRIX_GET",
            Op::MatrixSet   => "MATRIX_SET",
            Op::LoadLocal   => "LOAD_LOCAL",
            Op::StoreLocal  => "STORE_LOCAL",
            Op::LoadUp      => "LOAD_UP",
            Op::StoreUp     => "STORE_UP",
            Op::LoadGlobal  => "LOAD_GLOBAL",
            Op::StoreGlobal => "STORE_GLOBAL",
            Op::Jump        => "JUMP",
            Op::JumpFalse   => "JUMP_FALSE",
            Op::JumpTrue    => "JUMP_TRUE",
            Op::JumpNil     => "JUMP_NIL",
            Op::Call        => "CALL",
            Op::TailCall    => "TAIL_CALL",
            Op::Return      => "RETURN",
            Op::ReturnNil   => "RETURN_NIL",
            Op::CallNative  => "CALL_NATIVE",
            Op::GetField    => "GET_FIELD",
            Op::SetField    => "SET_FIELD",
            Op::NewStruct   => "NEW_STRUCT",
            Op::Closure     => "CLOSURE",
            Op::Cast        => "CAST",
            Op::IsType      => "IS_TYPE",
            Op::Print       => "PRINT",
            Op::Breakpoint  => "BREAKPOINT",
            Op::Panic       => "PANIC",
            Op::Nop         => "NOP",
        }
    }
}
