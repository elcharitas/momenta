use alloc::{string::String, vec::Vec};
use core::{cmp::Ordering, fmt};
use smallvec::SmallVec;

type Captures<'p> = SmallVec<[&'p str; 2]>;
type Segments = SmallVec<[Segment; 4]>;
type ParamNames = SmallVec<[String; 2]>;

#[derive(Clone, Debug)]
enum RouteIndices {
    Empty,
    One(usize),
    Many(Vec<usize>),
}

impl RouteIndices {
    fn as_slice(&self) -> &[usize] {
        match self {
            Self::Empty => &[],
            Self::One(index) => core::slice::from_ref(index),
            Self::Many(indices) => indices,
        }
    }

    fn push(&mut self, index: usize) {
        match self {
            Self::Empty => *self = Self::One(index),
            Self::One(previous) => *self = Self::Many(alloc::vec![*previous, index]),
            Self::Many(indices) => indices.push(index),
        }
    }

    fn sort_by<T>(&mut self, routes: &[Route<T>]) {
        if let Self::Many(indices) = self {
            indices.sort_unstable_by(|a, b| compare_routes(&routes[*a], &routes[*b]));
        }
    }
}

#[derive(Clone, Debug)]
pub struct Router<T> {
    routes: Vec<Route<T>>,
    buckets: Vec<Option<Bucket>>,
    bucket_count: usize,
    sole_bucket_slot: usize,
    deep: ChildTable,
    deep_route_count: usize,
    tail_only: bool,
    tail_table: ChildTable,
    variable_first: Vec<usize>,
}

#[derive(Clone, Debug)]
struct Route<T> {
    pattern: String,
    segments: Segments,
    names: ParamNames,
    fast: FastMatch,
    value: T,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Segment {
    Static(InlineText),
    Param { prefix: InlineText },
    CatchAll,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct InlineText(SmallVec<[u8; 16]>);

impl InlineText {
    fn new(text: &str) -> Self {
        let mut bytes = SmallVec::new();
        bytes.extend_from_slice(text.as_bytes());
        Self(bytes)
    }

    fn as_str(&self) -> &str {
        core::str::from_utf8(&self.0).expect("validated route text")
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Copy, Debug)]
enum FastMatch {
    Exact,
    TailParam(usize),
    TailCatchAll(usize),
    General,
}

#[derive(Clone, Debug)]
struct Bucket {
    key: SmallVec<[u8; 16]>,
    routes: RouteIndices,
    promoted: bool,
}

#[derive(Clone, Debug)]
struct ChildTable {
    slots: Vec<Option<ChildBucket>>,
    count: usize,
}

#[derive(Clone, Debug)]
struct ChildBucket {
    key: SmallVec<[u8; 16]>,
    routes: RouteIndices,
}

impl<T> Default for Router<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Router<T> {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            buckets: Vec::new(),
            bucket_count: 0,
            sole_bucket_slot: 0,
            deep: ChildTable::new(),
            deep_route_count: 0,
            tail_only: true,
            tail_table: ChildTable::new(),
            variable_first: Vec::new(),
        }
    }

    pub fn insert(&mut self, pattern: impl Into<String>, value: T) -> Result<(), InsertError> {
        let pattern = pattern.into();
        if self.tail_only
            && let Some((prefix_len, name)) = parse_tail_fast(&pattern)
        {
            if let Some(bucket) = self.tail_table.find(&pattern.as_bytes()[..prefix_len]) {
                return Err(InsertError::Conflict {
                    with: self.routes[bucket.routes.as_slice()[0]].pattern.clone(),
                });
            }
            let index = self.routes.len();
            let mut names = ParamNames::new();
            names.push(name);
            self.routes.push(Route {
                pattern,
                segments: Segments::new(),
                names,
                fast: FastMatch::TailParam(prefix_len),
                value,
            });
            let prefix = &self.routes[index].pattern.as_bytes()[..prefix_len];
            self.tail_table.insert(prefix, index, &self.routes);
            return Ok(());
        }
        let (segments, names) = parse_pattern(&pattern)?;
        let fast = FastMatch::from_pattern(&pattern, &segments, names.len());
        if self.tail_only {
            self.tail_only = false;
            self.tail_table = ChildTable::new();
            for index in 0..self.routes.len() {
                self.routes[index].segments = parse_pattern(&self.routes[index].pattern)
                    .expect("previously validated route")
                    .0;
                let route = &self.routes[index];
                if first_static_segment(&route.pattern, &route.segments).is_some() {
                    self.insert_bucket(index);
                } else {
                    self.variable_first.push(index);
                }
            }
            let routes = &self.routes;
            self.variable_first
                .sort_unstable_by(|a, b| compare_routes(&routes[*a], &routes[*b]));
        }
        let first = first_static_segment(&pattern, &segments);
        let second = second_static_segment(&pattern, &segments);
        if let Some(route) = self.find_conflict(first, second, &segments) {
            return Err(InsertError::Conflict {
                with: route.pattern.clone(),
            });
        }

        let has_first = first.is_some();
        let index = self.routes.len();
        self.routes.push(Route {
            pattern,
            segments,
            names,
            fast,
            value,
        });
        if has_first {
            self.insert_bucket(index);
        } else {
            self.variable_first.push(index);
            let routes = &self.routes;
            self.variable_first
                .sort_unstable_by(|a, b| compare_routes(&routes[*a], &routes[*b]));
        }
        Ok(())
    }

    pub fn at<'r, 'p>(&'r self, path: &'p str) -> Result<Match<'r, 'p, &'r T>, MatchError> {
        let (index, values) = self.find(path).ok_or(MatchError::NotFound)?;
        let route = &self.routes[index];
        Ok(Match {
            value: &route.value,
            params: Params {
                names: &route.names,
                values,
            },
        })
    }

    pub fn at_mut<'r, 'p>(
        &'r mut self,
        path: &'p str,
    ) -> Result<Match<'r, 'p, &'r mut T>, MatchError> {
        let (index, values) = self.find(path).ok_or(MatchError::NotFound)?;
        let route = &mut self.routes[index];
        Ok(Match {
            value: &mut route.value,
            params: Params {
                names: &route.names,
                values,
            },
        })
    }

    pub fn remove(&mut self, pattern: impl Into<String>) -> Option<T> {
        let pattern = pattern.into();
        let index = self
            .routes
            .iter()
            .position(|route| route.pattern == pattern)?;
        let removed = self.routes.remove(index);
        self.buckets.clear();
        self.bucket_count = 0;
        self.sole_bucket_slot = 0;
        self.deep = ChildTable::new();
        self.deep_route_count = 0;
        self.tail_table = ChildTable::new();
        self.tail_only = self
            .routes
            .iter()
            .all(|route| tail_prefix_len(&route.pattern, route.fast).is_some());
        self.variable_first.clear();
        for index in 0..self.routes.len() {
            if self.tail_only {
                let route = &self.routes[index];
                let prefix_len = tail_prefix_len(&route.pattern, route.fast).expect("tail route");
                self.tail_table.insert(
                    &route.pattern.as_bytes()[..prefix_len],
                    index,
                    &self.routes,
                );
                continue;
            }
            let route = &self.routes[index];
            let first = first_static_segment(&route.pattern, &route.segments);
            if first.is_some() {
                self.insert_bucket(index);
            } else {
                self.variable_first.push(index);
            }
        }
        let routes = &self.routes;
        self.variable_first
            .sort_unstable_by(|a, b| compare_routes(&routes[*a], &routes[*b]));
        Some(removed.value)
    }

    #[inline]
    fn find<'p>(&self, path: &'p str) -> Option<(usize, Captures<'p>)> {
        if self.tail_only {
            let slash = path.rfind('/')?;
            let bucket = self.tail_table.find(&path.as_bytes()[..=slash])?;
            let index = bucket.routes.as_slice()[0];
            return self.routes[index]
                .match_path(path)
                .map(|values| (index, values));
        }
        if self.routes.len() <= 4 && self.bucket_count == self.routes.len() {
            for (index, route) in self.routes.iter().enumerate().rev() {
                if let Some(values) = route.match_path(path) {
                    return Some((index, values));
                }
            }
            return None;
        }
        if self.deep_route_count > 0 {
            if let Some(key) = first_two_path_segments(path)
                && let Some(bucket) = self.deep.find(key.as_bytes())
            {
                for &index in bucket.routes.as_slice() {
                    if let Some(values) = self.routes[index].match_path(path) {
                        return Some((index, values));
                    }
                }
            }
            if self.deep_route_count == self.routes.len() {
                return None;
            }
        }
        let key = first_path_segment(path);
        let bucket = if self.bucket_count == 1 {
            self.buckets[self.sole_bucket_slot]
                .as_ref()
                .filter(|bucket| bucket.key.as_slice() == key.as_bytes())
        } else {
            self.find_bucket(key)
        };
        if let Some(bucket) = bucket {
            for &index in bucket.routes.as_slice() {
                if let Some(values) = self.routes[index].match_path(path) {
                    return Some((index, values));
                }
            }
        }
        for &index in &self.variable_first {
            if let Some(values) = self.routes[index].match_path(path) {
                return Some((index, values));
            }
        }
        None
    }

    fn find_conflict(
        &self,
        first: Option<&str>,
        second: Option<&str>,
        segments: &[Segment],
    ) -> Option<&Route<T>> {
        let conflicting = |index: usize| {
            let route = &self.routes[index];
            routes_conflict(&route.segments, segments).then_some(route)
        };
        match first {
            Some(key) => {
                let bucket = self.find_bucket(key)?;
                bucket
                    .routes
                    .as_slice()
                    .iter()
                    .find_map(|&index| conflicting(index))
                    .or_else(|| {
                        self.deep
                            .find(&combined_key(key, second?))?
                            .routes
                            .as_slice()
                            .iter()
                            .find_map(|&index| conflicting(index))
                    })
            }
            None => self
                .variable_first
                .iter()
                .find_map(|&index| conflicting(index)),
        }
    }

    fn find_bucket(&self, key: &str) -> Option<&Bucket> {
        if self.buckets.is_empty() {
            return None;
        }
        let mask = self.buckets.len() - 1;
        let mut slot = hash_key(key) as usize & mask;
        loop {
            match &self.buckets[slot] {
                Some(bucket) if bucket.key.as_slice() == key.as_bytes() => return Some(bucket),
                Some(_) => slot = (slot + 1) & mask,
                None => return None,
            }
        }
    }

    fn insert_bucket(&mut self, index: usize) {
        self.ensure_capacity();
        let key = first_static_segment(&self.routes[index].pattern, &self.routes[index].segments)
            .expect("static route key");
        let mask = self.buckets.len() - 1;
        let mut slot = hash_key(key) as usize & mask;
        loop {
            match &mut self.buckets[slot] {
                Some(bucket) if bucket.key.as_slice() == key.as_bytes() => {
                    if !bucket.promoted {
                        let previous = core::mem::replace(&mut bucket.routes, RouteIndices::Empty);
                        for &previous_index in previous.as_slice() {
                            if let Some(second) = second_static_segment(
                                &self.routes[previous_index].pattern,
                                &self.routes[previous_index].segments,
                            ) {
                                let deep_key = combined_key(key, second);
                                self.deep.insert(&deep_key, previous_index, &self.routes);
                                self.deep_route_count += 1;
                            } else {
                                bucket.routes.push(previous_index);
                            }
                        }
                        bucket.promoted = true;
                    }
                    if let Some(second) = second_static_segment(
                        &self.routes[index].pattern,
                        &self.routes[index].segments,
                    ) {
                        let deep_key = combined_key(key, second);
                        self.deep.insert(&deep_key, index, &self.routes);
                        self.deep_route_count += 1;
                    } else {
                        bucket.routes.push(index);
                        bucket.routes.sort_by(&self.routes);
                    }
                    return;
                }
                Some(_) => slot = (slot + 1) & mask,
                empty @ None => {
                    let mut stored_key = SmallVec::new();
                    stored_key.extend_from_slice(key.as_bytes());
                    *empty = Some(Bucket {
                        key: stored_key,
                        routes: RouteIndices::One(index),
                        promoted: false,
                    });
                    self.bucket_count += 1;
                    if self.bucket_count == 1 {
                        self.sole_bucket_slot = slot;
                    }
                    return;
                }
            }
        }
    }

    fn ensure_capacity(&mut self) {
        if !self.buckets.is_empty() && (self.bucket_count + 1) * 2 < self.buckets.len() {
            return;
        }
        let capacity = if self.buckets.is_empty() {
            8
        } else {
            self.buckets.len() * 2
        };
        let mut next = Vec::with_capacity(capacity);
        next.resize_with(capacity, || None);
        for bucket in core::mem::take(&mut self.buckets).into_iter().flatten() {
            place_bucket(&mut next, bucket);
        }
        self.buckets = next;
    }
}

impl ChildTable {
    fn new() -> Self {
        Self {
            slots: Vec::new(),
            count: 0,
        }
    }

    #[inline]
    fn find(&self, key: &[u8]) -> Option<&ChildBucket> {
        if self.slots.is_empty() {
            return None;
        }
        let mask = self.slots.len() - 1;
        let mut slot = hash_bytes(key) as usize & mask;
        loop {
            match &self.slots[slot] {
                Some(bucket) if bucket.key.as_slice() == key => return Some(bucket),
                Some(_) => slot = (slot + 1) & mask,
                None => return None,
            }
        }
    }

    fn insert<T>(&mut self, key: &[u8], index: usize, routes: &[Route<T>]) {
        self.ensure_capacity();
        let mask = self.slots.len() - 1;
        let mut slot = hash_bytes(key) as usize & mask;
        loop {
            match &mut self.slots[slot] {
                Some(bucket) if bucket.key.as_slice() == key => {
                    bucket.routes.push(index);
                    bucket.routes.sort_by(routes);
                    return;
                }
                Some(_) => slot = (slot + 1) & mask,
                empty @ None => {
                    let mut stored_key = SmallVec::new();
                    stored_key.extend_from_slice(key);
                    *empty = Some(ChildBucket {
                        key: stored_key,
                        routes: RouteIndices::One(index),
                    });
                    self.count += 1;
                    return;
                }
            }
        }
    }

    fn ensure_capacity(&mut self) {
        if !self.slots.is_empty() && (self.count < 4 || (self.count + 1) * 2 < self.slots.len()) {
            return;
        }
        let capacity = if self.slots.is_empty() {
            8
        } else {
            self.slots.len() * 2
        };
        let mut next = Vec::with_capacity(capacity);
        next.resize_with(capacity, || None);
        for bucket in core::mem::take(&mut self.slots).into_iter().flatten() {
            let mask = next.len() - 1;
            let mut slot = hash_bytes(&bucket.key) as usize & mask;
            while next[slot].is_some() {
                slot = (slot + 1) & mask;
            }
            next[slot] = Some(bucket);
        }
        self.slots = next;
    }
}

impl<T> Route<T> {
    #[inline]
    fn match_path<'p>(&self, path: &'p str) -> Option<Captures<'p>> {
        match self.fast {
            FastMatch::Exact => (path == self.pattern).then(Captures::new),
            FastMatch::TailParam(prefix_len) => {
                let value = path.strip_prefix(&self.pattern[..prefix_len])?;
                if value.is_empty() || value.contains('/') {
                    return None;
                }
                let mut values = Captures::new();
                values.push(value);
                Some(values)
            }
            FastMatch::TailCatchAll(prefix_len) => {
                let value = path.strip_prefix(&self.pattern[..prefix_len])?;
                if value.is_empty() {
                    return None;
                }
                let mut values = Captures::new();
                values.push(value);
                Some(values)
            }
            FastMatch::General => self.match_general(path),
        }
    }

    fn match_general<'p>(&self, path: &'p str) -> Option<Captures<'p>> {
        let mut remaining = path;
        let mut values = Captures::new();
        for (position, segment) in self.segments.iter().enumerate() {
            if matches!(segment, Segment::CatchAll) {
                if remaining.is_empty() {
                    return None;
                }
                values.push(remaining);
                return Some(values);
            }
            let (part, rest, has_slash) = match remaining.find('/') {
                Some(at) => (&remaining[..at], &remaining[at + 1..], true),
                None => (remaining, "", false),
            };
            match segment {
                Segment::Static(expected) if part != expected.as_str() => return None,
                Segment::Param { prefix } => {
                    let value = part.strip_prefix(prefix.as_str())?;
                    if value.is_empty() && position + 1 == self.segments.len() {
                        return None;
                    }
                    values.push(value);
                }
                _ => {}
            }
            if position + 1 < self.segments.len() {
                if !has_slash {
                    return None;
                }
                remaining = rest;
            } else if has_slash {
                return None;
            }
        }
        Some(values)
    }
}

impl FastMatch {
    fn from_pattern(pattern: &str, segments: &[Segment], count: usize) -> Self {
        if !pattern.contains('{') && !pattern.contains('}') {
            return Self::Exact;
        }
        if count == 1 && !pattern.contains("{{") && !pattern.contains("}}") {
            let prefix_len = pattern.find('{').unwrap_or(0);
            return match segments.last() {
                Some(Segment::Param { .. }) => Self::TailParam(prefix_len),
                Some(Segment::CatchAll) => Self::TailCatchAll(prefix_len),
                _ => Self::General,
            };
        }
        Self::General
    }
}

fn tail_prefix_len(pattern: &str, fast: FastMatch) -> Option<usize> {
    match fast {
        FastMatch::TailParam(len) if pattern.as_bytes().get(len.wrapping_sub(1)) == Some(&b'/') => {
            Some(len)
        }
        _ => None,
    }
}

fn parse_tail_fast(pattern: &str) -> Option<(usize, String)> {
    let start = pattern.rfind("/{")?;
    let prefix_len = start + 1;
    if !pattern.ends_with('}') || pattern[..prefix_len].contains(['{', '}']) {
        return None;
    }
    let name = &pattern[prefix_len + 1..pattern.len() - 1];
    if name.is_empty() || name.contains(['{', '}', '/', '*']) {
        return None;
    }
    Some((prefix_len, name.into()))
}

fn parse_pattern(pattern: &str) -> Result<(Segments, ParamNames), InsertError> {
    let mut segments = SmallVec::new();
    let mut names = SmallVec::new();
    let mut parts = pattern.split('/').peekable();
    while let Some(part) = parts.next() {
        let (segment, name) = parse_segment(part, parts.peek().is_none())?;
        if let Some(name) = name {
            names.push(name);
        }
        segments.push(segment);
    }
    Ok((segments, names))
}

fn parse_segment(raw: &str, last: bool) -> Result<(Segment, Option<String>), InsertError> {
    if !raw
        .as_bytes()
        .iter()
        .any(|&byte| byte == b'{' || byte == b'}')
    {
        return Ok((Segment::Static(InlineText::new(raw)), None));
    }
    if let Some(name) = raw
        .strip_prefix('{')
        .and_then(|raw| raw.strip_suffix('}'))
        .filter(|name| !name.contains(['{', '}']))
    {
        let (name, catch_all) = match name.strip_prefix('*') {
            Some(name) => (name, true),
            None => (name, false),
        };
        if name.is_empty() || name.contains(['{', '}', '/', '*']) {
            return Err(InsertError::InvalidParam);
        }
        if catch_all && !last {
            return Err(InsertError::InvalidCatchAll);
        }
        let segment = if catch_all {
            Segment::CatchAll
        } else {
            Segment::Param {
                prefix: InlineText::new(""),
            }
        };
        return Ok((segment, Some(name.into())));
    }
    let mut chars = raw.chars().peekable();
    let mut static_part = String::new();
    while let Some(ch) = chars.next() {
        match ch {
            '{' if chars.peek() == Some(&'{') => {
                chars.next();
                static_part.push('{');
            }
            '}' if chars.peek() == Some(&'}') => {
                chars.next();
                static_part.push('}');
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
                if catch_all && (!last || !static_part.is_empty()) {
                    return Err(InsertError::InvalidCatchAll);
                }
                if chars.peek().is_some() {
                    return Err(InsertError::InvalidParamSegment);
                }
                let segment = if catch_all {
                    Segment::CatchAll
                } else {
                    Segment::Param {
                        prefix: InlineText::new(&static_part),
                    }
                };
                return Ok((segment, Some(name.into())));
            }
            '}' => return Err(InsertError::InvalidParam),
            _ => static_part.push(ch),
        }
    }
    Ok((Segment::Static(InlineText::new(&static_part)), None))
}

fn first_static_segment<'a>(pattern: &str, segments: &'a [Segment]) -> Option<&'a str> {
    let index = usize::from(pattern.starts_with('/'));
    match segments.get(index) {
        Some(Segment::Static(value)) => Some(value.as_str()),
        _ => None,
    }
}

fn second_static_segment<'a>(pattern: &str, segments: &'a [Segment]) -> Option<&'a str> {
    let index = usize::from(pattern.starts_with('/')) + 1;
    match segments.get(index) {
        Some(Segment::Static(value)) => Some(value.as_str()),
        _ => None,
    }
}

fn first_path_segment(path: &str) -> &str {
    path.strip_prefix('/')
        .unwrap_or(path)
        .split('/')
        .next()
        .unwrap_or("")
}

fn first_two_path_segments(path: &str) -> Option<&str> {
    let path = path.strip_prefix('/').unwrap_or(path);
    let (first, tail) = path.split_once('/')?;
    let end = first.len() + 1 + tail.find('/').unwrap_or(tail.len());
    Some(&path[..end])
}

fn combined_key(first: &str, second: &str) -> SmallVec<[u8; 16]> {
    let mut key = SmallVec::new();
    key.extend_from_slice(first.as_bytes());
    key.push(b'/');
    key.extend_from_slice(second.as_bytes());
    key
}

fn compare_routes<T>(left: &Route<T>, right: &Route<T>) -> Ordering {
    for (a, b) in left.segments.iter().zip(&right.segments) {
        let rank = |segment: &Segment| match segment {
            Segment::Static(_) => 3,
            Segment::Param { prefix } if !prefix.is_empty() => 2,
            Segment::Param { .. } => 1,
            Segment::CatchAll => 0,
        };
        let by_rank = rank(b).cmp(&rank(a));
        if by_rank != Ordering::Equal {
            return by_rank;
        }
        if let (Segment::Param { prefix: a }, Segment::Param { prefix: b }) = (a, b) {
            let by_prefix = b.len().cmp(&a.len());
            if by_prefix != Ordering::Equal {
                return by_prefix;
            }
        }
    }
    right.segments.len().cmp(&left.segments.len())
}

fn routes_conflict(left: &[Segment], right: &[Segment]) -> bool {
    if left == right {
        return true;
    }
    for (a, b) in left.iter().zip(right) {
        match (a, b) {
            (Segment::Param { prefix }, Segment::CatchAll)
            | (Segment::CatchAll, Segment::Param { prefix })
                if prefix.is_empty() =>
            {
                return true;
            }
            _ if a == b => {}
            _ => return false,
        }
    }
    false
}

fn hash_key(key: &str) -> u64 {
    hash_bytes(key.as_bytes())
}

#[inline]
fn hash_bytes(bytes: &[u8]) -> u64 {
    if (17..=24).contains(&bytes.len()) {
        let first = u64::from_le_bytes(bytes[..8].try_into().expect("eight bytes"));
        let last = u64::from_le_bytes(bytes[bytes.len() - 8..].try_into().expect("eight bytes"));
        let mut middle = [0u8; 8];
        middle[..bytes.len() - 16].copy_from_slice(&bytes[8..bytes.len() - 8]);
        let middle = u64::from_le_bytes(middle);
        let mut hash = first.wrapping_mul(0x9e3779b185ebca87)
            ^ last.wrapping_mul(0xc2b2ae3d27d4eb4f)
            ^ middle.rotate_left(13)
            ^ bytes.len() as u64;
        hash ^= hash >> 33;
        hash = hash.wrapping_mul(0xff51afd7ed558ccd);
        return hash ^ (hash >> 33);
    }
    if bytes.len() <= 16 {
        let mut short = [0u8; 8];
        let (first, last) = if bytes.len() >= 8 {
            (
                u64::from_le_bytes(bytes[..8].try_into().expect("eight bytes")),
                u64::from_le_bytes(bytes[bytes.len() - 8..].try_into().expect("eight bytes")),
            )
        } else {
            short[..bytes.len()].copy_from_slice(bytes);
            (u64::from_le_bytes(short), 0)
        };
        let mut hash = first ^ last.rotate_left(17) ^ bytes.len() as u64;
        hash ^= hash >> 33;
        hash = hash.wrapping_mul(0xff51afd7ed558ccd);
        return hash ^ (hash >> 33);
    }
    let mut hash = 0xcbf29ce484222325u64;
    for &byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash ^ (hash >> 12)
}

fn place_bucket(table: &mut [Option<Bucket>], bucket: Bucket) {
    let mask = table.len() - 1;
    let mut slot = hash_bytes(&bucket.key) as usize & mask;
    while table[slot].is_some() {
        slot = (slot + 1) & mask;
    }
    table[slot] = Some(bucket);
}

#[derive(Debug)]
pub struct Match<'r, 'p, T> {
    pub value: T,
    pub params: Params<'r, 'p>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Params<'r, 'p> {
    names: &'r [String],
    values: Captures<'p>,
}

impl<'r, 'p> Params<'r, 'p> {
    #[inline]
    pub fn get(&self, key: impl AsRef<str>) -> Option<&'p str> {
        self.names
            .iter()
            .zip(self.values.iter().copied())
            .find(|(name, _)| *name == key.as_ref())
            .map(|(_, value)| value)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.names
            .iter()
            .map(String::as_str)
            .zip(self.values.iter().copied())
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
        assert!(router.at("/users/").is_err());
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
