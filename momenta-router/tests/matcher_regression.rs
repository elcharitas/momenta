use momenta_router::{InsertError, Router};

#[test]
fn static_routes_take_priority_over_parameters() {
    let mut router = Router::new();
    router.insert("/users/{id}", 1).unwrap();
    router.insert("/users/new", 2).unwrap();
    router.insert("/files/{*path}", 3).unwrap();

    assert_eq!(*router.at("/users/new").unwrap().value, 2);
    let user = router.at("/users/42").unwrap();
    assert_eq!(*user.value, 1);
    assert_eq!(user.params.get("id"), Some("42"));
    assert_eq!(
        router
            .at("/files/docs/readme.md")
            .unwrap()
            .params
            .get("path"),
        Some("docs/readme.md")
    );
    for path in ["/users/", "/users/42/extra", "/files/", "/missing"] {
        assert!(router.at(path).is_err(), "{path}");
    }
}

#[test]
fn nested_and_dynamic_first_routes_keep_their_parameters() {
    let mut router = Router::new();
    router.insert("/{area}/posts/{id}", 1).unwrap();
    router.insert("/admin/posts/new", 2).unwrap();
    router.insert("/admin/posts/{id}", 3).unwrap();
    router.insert("/{area}/files/{*path}", 4).unwrap();

    assert_eq!(*router.at("/admin/posts/new").unwrap().value, 2);
    assert_eq!(*router.at("/admin/posts/42").unwrap().value, 3);
    let post = router.at("/team/posts/42").unwrap();
    assert_eq!(*post.value, 1);
    assert_eq!(
        post.params.iter().collect::<Vec<_>>(),
        [("area", "team"), ("id", "42")]
    );
    let file = router.at("/team/files/a/b").unwrap();
    assert_eq!(*file.value, 4);
    assert_eq!(file.params.get("path"), Some("a/b"));
}

#[test]
fn partial_segments_escaped_braces_and_empty_middle_values_match() {
    let mut router = Router::new();
    router.insert("/files/file-{id}", 1).unwrap();
    router.insert("/{{literal}}", 2).unwrap();
    router.insert("/x/{first}/{second}", 3).unwrap();

    assert_eq!(
        router.at("/files/file-42").unwrap().params.get("id"),
        Some("42")
    );
    assert_eq!(*router.at("/{literal}").unwrap().value, 2);
    assert!(router.at("/files/file-").is_err());
    let empty = router.at("/x//b").unwrap();
    assert_eq!(*empty.value, 3);
    assert_eq!(
        empty.params.iter().collect::<Vec<_>>(),
        [("first", ""), ("second", "b")]
    );
    assert!(router.at("/x/a/").is_err());
}

#[test]
fn equivalent_parameters_and_catchalls_conflict() {
    let mut router = Router::new();
    router.insert("/a/{id}", 1).unwrap();
    assert_eq!(
        router.insert("/a/{name}", 2),
        Err(InsertError::Conflict {
            with: "/a/{id}".into()
        })
    );
    assert_eq!(
        router.insert("/a/{*rest}", 3),
        Err(InsertError::Conflict {
            with: "/a/{id}".into()
        })
    );
    assert!(router.insert("/a/{*rest}/tail", 4).is_err());
    assert!(router.insert("/a/{}", 5).is_err());
    assert_eq!(*router.at("/a/42").unwrap().value, 1);
}

#[test]
fn switching_from_tail_routes_to_mixed_routes_preserves_matches() {
    let mut router = Router::new();
    router.insert("/api/users/{id}", 1).unwrap();
    router.insert("/api/posts/{id}", 2).unwrap();
    router.insert("/api/users/new", 3).unwrap();

    assert_eq!(*router.at("/api/users/new").unwrap().value, 3);
    assert_eq!(
        router.at("/api/users/42").unwrap().params.get("id"),
        Some("42")
    );
    assert_eq!(*router.at("/api/posts/7").unwrap().value, 2);

    assert_eq!(router.remove("/api/users/new"), Some(3));
    router.insert("/api/comments/{id}", 4).unwrap();
    assert_eq!(*router.at("/api/comments/9").unwrap().value, 4);
    assert_eq!(*router.at("/api/users/42").unwrap().value, 1);
}

#[test]
fn dynamic_first_route_survives_index_transition() {
    let mut router = Router::new();
    router.insert("/{id}", 1).unwrap();
    router.insert("/static", 2).unwrap();

    assert_eq!(*router.at("/static").unwrap().value, 2);
    assert_eq!(router.at("/alpha").unwrap().params.get("id"), Some("alpha"));
    assert!(router.at("/alpha/extra").is_err());
}
