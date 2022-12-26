use crate::prelude::*;

pub mod react;
pub mod speak;

macro_rules! get_fn {
    ($id:ident($inner:path) -> $ret:ty) => {
        #[allow(dead_code)]
        pub fn $id<'c>(o: &'c [ResolvedOption<'c>], n: &'c str) -> Result<$ret> {
            let resolved = o.iter().find(|r| r.name == n).map_or_else(
                || Err(anyhow!("missing data for \"{n}\"")),
                |r| Ok(&r.value),
            );

            match resolved? {
                $inner(v) => Ok(*v),
                _ => Err(anyhow!("invalid data type for \"{n}\"")),
            }
        }
    };
    ($id:ident($inner:path) -> ref $ret:ty) => {
        #[allow(dead_code)]
        pub fn $id<'c>(o: &'c [ResolvedOption<'c>], n: &'c str) -> Result<&'c $ret> {
            let resolved = o.iter().find(|r| r.name == n).map_or_else(
                || Err(anyhow!("missing data for \"{n}\"")),
                |r| Ok(&r.value),
            );

            match resolved? {
                $inner(v) => Ok(v),
                _ => Err(anyhow!("invalid data type for \"{n}\"")),
            }
        }
    };
}

get_fn!(get_bool(ResolvedValue::Boolean) -> bool);
get_fn!(get_i64(ResolvedValue::Integer) -> i64);
get_fn!(get_f64(ResolvedValue::Number) -> f64);
get_fn!(get_partial_channel(ResolvedValue::Channel) -> ref PartialChannel);
get_fn!(get_role(ResolvedValue::Role) -> ref Role);
get_fn!(get_str(ResolvedValue::String) -> ref str);
get_fn!(get_subcommand(ResolvedValue::SubCommand) -> ref [ResolvedOption<'c>]);
get_fn!(get_subcommand_group(ResolvedValue::SubCommandGroup) -> ref [ResolvedOption<'c>]);

#[allow(dead_code)]
pub fn get_user<'c>(
    o: &'c [ResolvedOption<'c>],
    n: &'c str,
) -> Result<(&'c User, Option<&'c PartialMember>)> {
    let resolved = o.iter().find(|r| r.name == n).map_or_else(
        || Err(anyhow!("missing data for \"{n}\"")),
        |r| Ok(&r.value),
    );

    match resolved? {
        ResolvedValue::User(u, m) => Ok((u, *m)),
        _ => Err(anyhow!("invalid data type for \"{n}\"")),
    }
}
