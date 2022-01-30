mod_include! {module "module.rs"}

#[macro_export]
macro_rules! mod_include {
    ( $PUB:vis $MODULE:ident $PATH:literal) => {
        $PUB mod $MODULE {include!($PATH);}
    };
}
