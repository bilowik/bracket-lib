mod simple_console_backing;
pub(crate) use simple_console_backing::*;
mod sparse_console_backing;
pub(crate) use sparse_console_backing::*;
mod fancy_console_backing;
pub(crate) use fancy_console_backing::*;
mod sprite_console_backing;
pub(crate) use sprite_console_backing::*;
mod shared_main_loop;
pub(crate) use shared_main_loop::*;

pub enum ConsoleBacking {
    Simple { backing: SimpleConsoleBackend },
    Sparse { backing: SparseConsoleBackend },
    Fancy { backing: FancyConsoleBackend },
    Sprite { backing: SpriteConsoleBackend },
}
