use crate::prelude::*;

pub trait Search {
    fn search_contains(&self, target: &str, any_case: bool, skip_spaces: bool) -> bool;

    fn search(&self, targets: Vec<String>, any_case: bool, skip_spaces: bool) -> Option<String> {
        targets
            .into_iter()
            .find(|target| self.search_contains(target, any_case, skip_spaces))
    }
}

pub fn str_contains(source: &str, target: &str, any_case: bool, skip_spaces: bool) -> bool {
    let (mut source, mut target) = if any_case {
        (source.to_lowercase(), target.to_lowercase())
    } else {
        (source.to_string(), target.to_string())
    };
    if skip_spaces {
        source = source.replace([' ', '\t', '\n', '\r'], "");
        target = target.replace([' ', '\t', '\n', '\r'], "");
    }

    source.contains(&target)
}

impl Search for User {
    fn search_contains(&self, target: &str, any_case: bool, skip_spaces: bool) -> bool {
        str_contains(&self.tag(), target, any_case, skip_spaces)
            || str_contains(&self.face(), target, any_case, skip_spaces)
            || self
                .accent_colour
                .map(Color::hex)
                .is_some_and(|c| str_contains(&c, target, any_case, skip_spaces))
    }
}

impl Search for PartialMember {
    fn search_contains(&self, target: &str, any_case: bool, skip_spaces: bool) -> bool {
        self.user
            .as_ref()
            .is_some_and(|user| user.search_contains(target, any_case, skip_spaces))
            || self
                .nick
                .as_ref()
                .is_some_and(|nick| str_contains(nick, target, any_case, skip_spaces))
    }
}

impl Search for Message {
    fn search_contains(&self, target: &str, any_case: bool, skip_spaces: bool) -> bool {
        str_contains(&self.content, target, any_case, skip_spaces)
            || self.author.search_contains(target, any_case, skip_spaces)
            || self
                .member
                .as_ref()
                .is_some_and(|m| m.search_contains(target, any_case, skip_spaces))
    }
}
