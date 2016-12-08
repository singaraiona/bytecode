// ByteCode with internal continuous store
//

use std::fmt;
use std::cell::UnsafeCell;
use std::ops::Deref;

const POOL_SIZE: usize = (!0 as u16) as usize;

#[derive(Debug)]
struct Store {
    lByteCode: usize,
    ptr: Vec<ByteCode>,
}

thread_local!(static BIN_POOL: UnsafeCell<Store> = UnsafeCell::new(Store {
            lByteCode: 0,
            ptr: Vec::with_capacity(POOL_SIZE),
        }));

#[derive(Debug)]
pub enum Error {
    NoSpace,
    NotImplemented,
}

pub struct Bin<'a> {
    ptr: &'a ByteCode,
}

impl<'a> Bin<'a> {
    pub fn new(x: ByteCode) -> Bin<'a> {
        let p = store(x);
        Bin { ptr: p }
    }
}

impl<'a> Deref for Bin<'a> {
    type Target = ByteCode;

    fn deref(&self) -> &ByteCode {
        self.ptr
    }
}

impl<'a> fmt::Display for Bin<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ptr)
    }
}

impl<'a> fmt::Debug for Bin<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.ptr)
    }
}

#[derive(Debug)]
pub enum ByteCode {
    Nil,
    Cons(Bin<'static>, Bin<'static>),
    List(Bin<'static>),
    Dict(Bin<'static>),
    Call(Bin<'static>, Bin<'static>),
    Lambda(Bin<'static>, Bin<'static>),
    Verb(u16, Bin<'static>, Bin<'static>),
    Adverb(u16, Bin<'static>, Bin<'static>),
    Ioverb(Bin<'static>),
    NameInt(Bin<'static>),
    SymbolInt(Bin<'static>),
    SequenceInt(Bin<'static>),
    Name(Bin<'static>),
    Number(i64),
    Hexlit(i64),
    Bool(bool),
    Symbol(u16),
    Sequence(Bin<'static>),
    Cell(Bin<'static>),
    Assign(Bin<'static>, Bin<'static>),
    Cond(Bin<'static>, Bin<'static>, Bin<'static>),
}

impl ByteCode {
    pub fn bin<'a>(self) -> Bin<'a> {
        Bin::new(self)
    }
}

impl fmt::Display for ByteCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ByteCode::Nil => write!(f, ""),
            ByteCode::Cons(ref a, ref b) => write!(f, "{} {}", *a, *b),
            ByteCode::List(ref a) => write!(f, "{}", *a),
            ByteCode::Dict(ref d) => write!(f, "[{};]", *d),
            ByteCode::Call(ref a, ref b) => write!(f, "{} {}", *a, *b),
            ByteCode::Lambda(ref a, ref b) => {
                match **a {
                    ByteCode::Nil => write!(f, "{{[x]{}}}", *b),
                    _ => {
                        let args = format!("{}", *a).replace(" ", ";");
                        write!(f, "{{[{}]{}}}", args, *b)
                    }
                }
            }
            ByteCode::Verb(ref v, ref a, ref b) => write!(f, "{}{}{}", a, *v, *b),
            ByteCode::Adverb(ref v, ref a, ref b) => write!(f, "{}{}{}", a, *v, *b),
            ByteCode::Ioverb(ref v) => write!(f, "{}", v),
            ByteCode::Number(n) => write!(f, "{}", n),
            ByteCode::Hexlit(h) => write!(f, "0x{}", h),
            ByteCode::Bool(b) => write!(f, "{:?}", b),
            ByteCode::Name(ref n) => write!(f, "{}", n),
            ByteCode::Symbol(ref s) => write!(f, "{}", s),
            ByteCode::Sequence(ref s) => write!(f, "{:?}", s),
            ByteCode::NameInt(ref n) => write!(f, "{}", n),
            ByteCode::SymbolInt(ref s) => write!(f, "{}", s),
            ByteCode::SequenceInt(ref s) => write!(f, "{:?}", s),
            ByteCode::Cell(ref c) => write!(f, "{}", *c),
            ByteCode::Assign(ref a, ref b) => write!(f, "{}:{}", *a, *b),
            ByteCode::Cond(ref c, ref a, ref b) => write!(f, "$[{};{};{}]", *c, *a, *b),
        }

    }
}

pub fn init_pool() {
    BIN_POOL.with(|p| {
        for i in 0..POOL_SIZE {
            let v = p.get();
            unsafe {
                (*v).ptr.push(ByteCode::Nil);
            };
        }
    });
}

fn store<'a>(b: ByteCode) -> &'a ByteCode {
    BIN_POOL.with(|p| {
        let v = p.get();
        unsafe {
            let l = (*v).lByteCode;
            (*v).ptr[l] = b;
            (*v).lByteCode += 1;
            &(*v).ptr[l]
        }
    })
}