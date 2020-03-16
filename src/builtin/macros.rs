macro_rules! arg {
    ($args:ident, $idx:literal, $name:literal) => {
        match $args.remove(stringify!($idx)) {
            Some(v) => v.eval()?,
            None => match $args.remove($name) {
                Some(v) => v.eval()?,
                None => return Err(concat!("Missing argument $", $name, ".").into()),
            },
        };
    };
    ($args:ident, $idx:literal, $name:literal=$default:expr) => {
        match $args.remove(stringify!($idx)) {
            Some(v) => v.eval()?,
            None => match $args.remove($name) {
                Some(v) => v.eval()?,
                None => $default,
            },
        };
    };
}

macro_rules! max_args {
    ($args:ident, $count:literal) => {
        if $args.len() > $count {
            if $count > 1 {
                return Err(format!(
                    "Only {} arguments allowed, but {} were passed.",
                    $count,
                    $args.len()
                )
                .into());
            } else {
                return Err(format!(
                    "Only {} argument allowed, but {} were passed.",
                    $count,
                    $args.len()
                )
                .into());
            }
        }
    };
}

macro_rules! bound {
    ($name:literal, $arg:ident, $unit:ident, $low:literal, $high:literal) => {
        if $arg > Number::from($high) || $arg < Number::from($low) {
            return Err(format!(
                "${}: Expected {}{} to be within {}{} and {}{}.",
                $name, $arg, $unit, $low, $unit, $high, $unit,
            )
            .into());
        } else {
            $arg
        }
    };
    // HACK: we accept `$low` as an ident here in order to work around
    // a bug in the nightly compiler.
    // https://github.com/rust-lang/rust/issues/70050
    ($name:literal, $arg:ident, $unit:ident, $low:ident, $high:literal) => {
        if $arg > Number::from($high) || $arg < Number::from($low) {
            return Err(format!(
                "${}: Expected {}{} to be within {}{} and {}{}.",
                $name, $arg, $unit, $low, $unit, $high, $unit,
            )
            .into());
        } else {
            $arg
        }
    };
    ($name:literal, $arg:ident, $unit:path, $low:literal, $high:literal) => {
        if $arg > Number::from($high) || $arg < Number::from($low) {
            return Err(format!(
                "${}: Expected {}{} to be within {}{} and {}{}.",
                $name, $arg, $unit, $low, $unit, $high, $unit,
            )
            .into());
        } else {
            $arg
        }
    };
    // HACK: we accept `$low` as an ident here in order to work around
    // a bug in the nightly compiler.
    // https://github.com/rust-lang/rust/issues/70050
    ($name:literal, $arg:ident, $unit:path, $low:ident, $high:literal) => {
        if $arg > Number::from($high) || $arg < Number::from($low) {
            return Err(format!(
                "${}: Expected {}{} to be within {}{} and {}{}.",
                $name, $arg, $unit, $low, $unit, $high, $unit,
            )
            .into());
        } else {
            $arg
        }
    };
}
