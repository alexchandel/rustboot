// based on define_flags by lexs
#![macro_escape]

macro_rules! define_flags (
    (
        $name:ident {
            $($flag:ident = $value:expr),+
        }
    ) => {
        define_flags!($name: uint { $($flag = $value),+ })
    };

    (
        $name:ident: $T:ty {
            $($flag:ident $(= $v:expr)*),*
        }
    ) => {
        #[allow(dead_code)]
        pub struct $name($T);

        #[allow(dead_code)]
        impl $name {
            /// Return the value.
            pub fn get(self) -> $T {
                match self { $name(x) => x }
            }

            /// Maps by applying a function to a contained value.
            pub fn map(self, f: |$T| -> $T) -> $name {
                match self { $name(x) => $name(f(x)) }
            }

            /// Returns `true` if no flags are currently stored.
            pub fn is_zero(&self) -> bool {
                match *self {
                    $name(0) => true,
                    _ => false
                }
            }

            pub fn zero() -> $name {
                $name(0)
            }
        }

        impl_ops!($name, $name)

        impl core::ops::Not<$name> for $name {
            #[inline(always)]
            fn not(&self) -> $name {
                match self {
                    &$name(flags) => $name(!flags)
                }
            }
        }

        define_flags_rec!($name, 1, $( $flag $(= $v)* ),+)
    };
)
macro_rules! define_flags_rec (
    // full list (original behavior)
    (
        $name:ident,
        $default:expr,
        $($flag:ident = $value:expr),*
    ) => (
        $( #[allow(dead_code)] pub static $flag: $name = $name($value); )+
    );
    // ----------
    // only one default value
    (
        $name:ident,
        $default:expr,
        $flag:ident
    ) => (
        #[allow(dead_code)]
        pub static $flag: $name = $name($default);
    );
    // only one value
    (
        $name:ident,
        $default:expr,
        $flag:ident = $value:expr
    ) => (
        #[allow(dead_code)]
        pub static $flag: $name = $name($value);
    );
    // ----------
    // without value (default)
    (
        $name:ident,
        $default:expr,
        $flag:ident,
        $($f:ident $(= $v:expr)*),*
    ) => (
        #[allow(dead_code)]
        pub static $flag: $name = $name($default);
        define_flags_rec!($name, $default << 1, $($f $(= $v)*),+)
    );
    // with value
    (
        $name:ident,
        $default:expr,
        $flag:ident = $value:expr,
        $($f:ident $(= $v:expr)*),*
    ) => (
        #[allow(dead_code)]
        pub static $flag: $name = $name($value);
        define_flags_rec!($name, $value << 1, $( $f $(= $v)* ),+)
    );
)

macro_rules! define_reg (
    (
        $Reg:ident, $flags:ident: $T:ty {
            $($flag:ident $(= $v:expr)*),*
        }
    ) => (
        define_flags!($flags: $T { $( $flag $(= $v)* ),* })

        pub struct $Reg;

        impl_ops!($Reg, $flags, $flags, $Reg::read(), $flags)
    )
)

macro_rules! impl_ops (
    ($T:ident, $RHS:ident) => (
        impl core::ops::BitOr<$RHS, $T> for $T {
            #[inline(always)]
            fn bitor(&self, other: &$RHS) -> $T {
                match (*self, other) {
                    ($T(p), &$RHS(f)) => $T(p | f)
                }
            }
        }

        impl core::ops::BitAnd<$RHS, $T> for $T {
            #[inline(always)]
            fn bitand(&self, other: &$RHS) -> $T {
                match (*self, other) {
                    ($T(p), &$RHS(f)) => $T(p & f)
                }
            }
        }

        impl core::ops::Sub<$RHS, $T> for $T {
            #[inline(always)]
            fn sub(&self, other: &$RHS) -> $T {
                match (*self, other) {
                    ($T(flags1), &$RHS(flags2)) => $T(flags1 & !flags2)
                }
            }
        }
    );

    ($LHS:ident, $RHS:ident, $T:ident, $e:expr, $X:ident) => (
        impl core::ops::BitOr<$RHS, $T> for $LHS {
            #[inline(always)]
            fn bitor(&self, other: &$RHS) -> $T {
                match ($e, other) {
                    ($X(p), &$RHS(f)) => $T(p | f)
                }
            }
        }

        impl core::ops::BitAnd<$RHS, $T> for $LHS {
            #[inline(always)]
            fn bitand(&self, other: &$RHS) -> $T {
                match ($e, other) {
                    ($X(p), &$RHS(f)) => $T(p & f)
                }
            }
        }

        impl core::ops::Sub<$RHS, $T> for $LHS {
            #[inline(always)]
            fn sub(&self, other: &$RHS) -> $T {
                match ($e, other) {
                    ($X(flags1), &$RHS(flags2)) => $T(flags1 & !flags2)
                }
            }
        }
    )
)

macro_rules! print(
    ($($arg:tt)*) => (format_args!(::platform::io::print_args, $($arg)*))
)

macro_rules! println(
    ($($arg:tt)*) => (format_args!(::platform::io::println_args, $($arg)*))
)
