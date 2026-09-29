use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::fmt;
use path_tree::{Parser, Path, PathTree, Piece, Position};

#[derive(Clone, Debug)]
pub struct Router<T> {
    tree: PathTree<usize>,
    routes: Vec<Route<T>>,
}

#[derive(Clone, Debug)]
struct Route<T> {
    pattern: String,
    translated: String,
    signature: Vec<Piece>,
    names: Vec<String>,
    value: T,
}

impl<T> Default for Router<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Router<T> {
    pub fn new() -> Self {
        Self {
            tree: PathTree::new(),
            routes: Vec::new(),
        }
    }

    pub fn insert(&mut self, pattern: impl Into<String>, value: T) -> Result<(), InsertError> {
        let pattern = pattern.into();
        let (translated, names) = translate_pattern(&pattern)?;
        let mut pieces: Vec<_> = Parser::new(&translated).collect();
        for piece in &mut pieces {
            if let Piece::Parameter(Position::Named(name), _) = piece {
                name.clear();
            }
        }
        if let Some(route) = self.routes.iter().find(|route| route.signature == pieces) {
            return Err(InsertError::Conflict {
                with: route.pattern.clone(),
            });
        }

        let _ = self.tree.insert(&translated, self.routes.len());
        self.routes.push(Route {
            pattern,
            translated,
            signature: pieces,
            names,
            value,
        });
        Ok(())
    }

    pub fn at<'r, 'p>(&'r self, path: &'p str) -> Result<Match<'r, 'p, &'r T>, MatchError> {
        let (index, params) = self.tree.find(path).ok_or(MatchError::NotFound)?;
        if params.params_iter().any(|(_, value)| value.is_empty()) {
            return Err(MatchError::NotFound);
        }
        Ok(Match {
            value: &self.routes[*index].value,
            params: Params {
                path: params,
                names: &self.routes[*index].names,
            },
        })
    }

    pub fn at_mut<'r, 'p>(
        &'r mut self,
        path: &'p str,
    ) -> Result<Match<'r, 'p, &'r mut T>, MatchError> {
        let (index, params) = self.tree.find(path).ok_or(MatchError::NotFound)?;
        if params.params_iter().any(|(_, value)| value.is_empty()) {
            return Err(MatchError::NotFound);
        }
        let route = &mut self.routes[*index];
        Ok(Match {
            value: &mut route.value,
            params: Params {
                path: params,
                names: &route.names,
            },
        })
    }

    pub fn remove(&mut self, pattern: impl Into<String>) -> Option<T> {
        let pattern = pattern.into();
        let index = self
            .routes
            .iter()
            .position(|route| route.pattern == pattern)?;
        let route = self.routes.remove(index);
        self.tree = PathTree::new();
        for (index, route) in self.routes.iter().enumerate() {
            let _ = self.tree.insert(&route.translated, index);
        }
        Some(route.value)
    }
}

#[derive(Debug)]
pub struct Match<'r, 'p, T> {
    pub value: T,
    pub params: Params<'r, 'p>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Params<'r, 'p> {
    path: Path<'r, 'p>,
    names: &'r [String],
}

impl<'r, 'p> Params<'r, 'p> {
    pub fn get(&self, key: impl AsRef<str>) -> Option<&'p str> {
        self.names
            .iter()
            .zip(self.path.raws.iter().copied())
            .find(|(name, _)| *name == key.as_ref())
            .map(|(_, value)| value)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.names
            .iter()
            .map(String::as_str)
            .zip(self.path.raws.iter().copied())
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InsertError {
    Conflict { with: String },
    InvalidParamSegment,
    InvalidParam,
    InvalidCatchAll,
}

impl fmt::Display for InsertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Conflict { with } => write!(f, "route conflicts with {with}"),
            Self::InvalidParamSegment => f.write_str("invalid parameter segment"),
            Self::InvalidParam => f.write_str("invalid route parameter"),
            Self::InvalidCatchAll => f.write_str("catch-all must be at the end of a route"),
        }
    }
}

impl core::error::Error for InsertError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatchError {
    NotFound,
}

impl fmt::Display for MatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("matching route not found")
    }
}

impl core::error::Error for MatchError {}

fn translate_pattern(pattern: &str) -> Result<(String, Vec<String>), InsertError> {
    let mut translated = String::new();
    let mut names = Vec::new();
    let mut chars = pattern.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '{' if chars.peek() == Some(&'{') => {
                chars.next();
                translated.push('{');
            }
            '{' => {
                let mut name = String::new();
                loop {
                    match chars.next() {
                        Some('}') => break,
                        Some(c) => name.push(c),
                        None => return Err(InsertError::InvalidParam),
                    }
                }
                let (name, catch_all) = match name.strip_prefix('*') {
                    Some(name) => (name, true),
                    None => (name.as_str(), false),
                };
                if name.is_empty() || name.chars().any(|c| matches!(c, '{' | '}' | '/' | '*')) {
                    return Err(InsertError::InvalidParam);
                }
                if catch_all && chars.peek().is_some() {
                    return Err(InsertError::InvalidCatchAll);
                }
                if chars.peek().is_some_and(|ch| *ch != '/') {
                    return Err(InsertError::InvalidParamSegment);
                }
                translated.push(':');
                translated.push_str("__momenta_param");
                translated.push_str(&names.len().to_string());
                if catch_all {
                    translated.push('*');
                }
                names.push(name.into());
            }
            '}' if chars.peek() == Some(&'}') => {
                chars.next();
                translated.push('}');
            }
            '}' => return Err(InsertError::InvalidParam),
            ':' | '+' | '*' | '?' | '\\' => {
                translated.push('\\');
                translated.push(ch);
            }
            _ => translated.push(ch),
        }
    }

    Ok((translated, names))
}

#[cfg(test)]
mod tests {
    use super::{InsertError, Router};
    use alloc::vec::Vec;

    #[test]
    fn matches_static_dynamic_and_catch_all_routes() {
        let mut router = Router::new();
        router.insert("/users/{id}", "user").unwrap();
        router.insert("/users/new", "new user").unwrap();
        router.insert("/files/{*path}", "file").unwrap();

        assert_eq!(*router.at("/users/new").unwrap().value, "new user");
        let matched = router.at("/users/42").unwrap();
        assert_eq!(*matched.value, "user");
        assert_eq!(matched.params.get("id"), Some("42"));
        assert_eq!(matched.params.iter().collect::<Vec<_>>(), [("id", "42")]);
        assert_eq!(
            router
                .at("/files/docs/readme.md")
                .unwrap()
                .params
                .get("path"),
            Some("docs/readme.md")
        );
        assert!(router.at("/files/").is_err());
        assert!(router.at("/users").is_err());
    }

    #[test]
    fn rejects_conflicting_and_invalid_routes() {
        let mut router = Router::new();
        router.insert("/users/{id}", 1).unwrap();
        assert_eq!(
            router.insert("/users/{name}", 2),
            Err(InsertError::Conflict {
                with: "/users/{id}".into()
            })
        );
        assert_eq!(
            router.insert("/users/{id", 2),
            Err(InsertError::InvalidParam)
        );
        assert_eq!(*router.at("/users/42").unwrap().value, 1);
    }

    #[test]
    fn updates_a_matched_value() {
        let mut router = Router::new();
        router.insert("/users/{id}", 1).unwrap();
        *router.at_mut("/users/42").unwrap().value = 2;
        assert_eq!(*router.at("/users/42").unwrap().value, 2);
    }

    #[test]
    fn preserves_parameter_names() {
        let mut router = Router::new();
        router
            .insert("/users/{user-id}/posts/{post.id}", 1)
            .unwrap();
        let matched = router.at("/users/42/posts/7").unwrap();
        assert_eq!(matched.params.get("user-id"), Some("42"));
        assert_eq!(matched.params.get("post.id"), Some("7"));
        assert_eq!(matched.params.len(), 2);
    }

    #[test]
    fn removes_only_the_named_route() {
        let mut router = Router::new();
        router.insert("/users/{id}", 1).unwrap();
        router.insert("/posts/{id}", 2).unwrap();
        assert_eq!(router.remove("/users/{name}"), None);
        assert_eq!(router.remove("/users/{id}"), Some(1));
        assert!(router.at("/users/42").is_err());
        assert_eq!(*router.at("/posts/42").unwrap().value, 2);
    }
}
