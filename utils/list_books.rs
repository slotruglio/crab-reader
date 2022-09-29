let mut main_view = Flex::column();

    for item in std::fs::read_dir("/Users/slotruglio/pds/crab-reader/src/assets/books/").unwrap() {
        let item = item.unwrap();
        let path = item.path();
        let path = path.to_str().unwrap();
        let book = Book::new(path);

        main_view.add_flex_child(book.get_widget_library(), 1.0);
        main_view.add_flex_child(Scroll::new(book.get_widget_chapter()), 2.0);
    }

    main_view