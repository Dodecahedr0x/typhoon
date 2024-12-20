pub mod macros {
    pub use {
        typhoon_account_macro::*, typhoon_context_macro::*, typhoon_handler_macro::*,
        typhoon_program_id_macro::*,
    };
}

pub mod program {
    pub use typhoon_program::*;
}

pub mod lib {
    pub use {typhoon_accounts::*, typhoon_context::*, typhoon_traits::*};
}

pub mod prelude {
    pub use {
        super::{lib::*, macros::*, program},
        program::{msg, program_error::ProgramError},
    };
}
