// ByteCode with internal continuous store
//

use std::fmt;
use std::cell::UnsafeCell;
use std::ops::Deref;

const POOL_SIZE: usize = (!0 as u16) as usize;

#[derive(Debug)]
pub struct Handle<'a> {
    last: usize,
    ptr: Vec<ByteCode<'a>>,
}

impl<'a> Handle<'a> {
    fn store(&'a mut self, b: ByteCode<'a>) -> usize {
        self.ptr.push(b);
        &self.ptr.len() - 1
    }

    #[inline]
    pub fn split(&'a mut self) -> (&'a mut Handle<'a>, &'a mut Handle<'a>) {
        let f: *mut Handle<'a> = self;
        let uf: &mut Handle<'a> = unsafe { &mut *f };
        let us: &mut Handle<'a> = unsafe { &mut *f };
        (uf, us)
    }
}

#[derive(Debug)]
pub enum Error {
    NoSpace,
    NotImplemented,
}

pub struct Bin<'a> {
    id: usize,
    hdl: &'a Handle<'a>,
}

impl<'a> Bin<'a> {
    pub fn new(h: &'a mut Handle<'a>, x: ByteCode<'a>) -> Bin<'a> {
        let (h1, h2) = h.split();
        Bin {
            hdl: h1,
            id: h2.store(x),
        }
    }

    #[inline]
    fn bc(&self) -> &ByteCode<'a> {
        &self.hdl.ptr[self.id]
    }
}

impl<'a> Deref for Bin<'a> {
    type Target = ByteCode<'a>;

    fn deref(&self) -> &Self::Target {
        self.bc()
    }
}

impl<'a> fmt::Display for Bin<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.bc())
    }
}

impl<'a> fmt::Debug for Bin<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.bc())
    }
}

#[derive(Debug)]
pub enum ByteCode<'a> {
    Nil,
    Cons(Bin<'a>, Bin<'a>),
    List(Bin<'a>),
    Dict(Bin<'a>),
    Call(Bin<'a>, Bin<'a>),
    Lambda(Bin<'a>, Bin<'a>),
    Verb(u16, Bin<'a>, Bin<'a>),
    Adverb(u16, Bin<'a>, Bin<'a>),
    Ioverb(Bin<'a>),
    NameInt(Bin<'a>),
    SymbolInt(Bin<'a>),
    SequenceInt(Bin<'a>),
    Name(Bin<'a>),
    Number(i64),
    Hexlit(i64),
    Bool(bool),
    Symbol(u16),
    Sequence(Bin<'a>),
    Cell(Bin<'a>),
    Assign(Bin<'a>, Bin<'a>),
    Cond(Bin<'a>, Bin<'a>, Bin<'a>),
}

impl<'a> ByteCode<'a> {
    pub fn bin(self, h: &'a UnsafeCell<Handle<'a>>) -> Bin<'a> {
        let ptr = unsafe { &mut *h.get() };
        Bin::new(ptr, self)
    }
}

impl<'a> fmt::Display for ByteCode<'a> {
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

pub fn handle<'a>() -> UnsafeCell<Handle<'a>> {
    UnsafeCell::new(Handle {
        last: 0,
        ptr: Vec::with_capacity(POOL_SIZE),
    })
}
