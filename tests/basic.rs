use cgns::*;

fn create_file(lib: &Library, path: &str) -> CgnsResult<()> {
    let file = lib.open_write(path)?;
    file.close()?;
    Ok(())
}

#[test]
fn lib_single_instance() {
    let lib = Library::take();
    drop(lib);
}

#[test]
fn lib_two_instances_sequential() {
    let lib = Library::take();
    drop(lib);

    let lib2 = Library::take();
    drop(lib2);
}

#[test]
#[should_panic]
fn lib_two_instances_parralel() {
    let lib = Library::take();
    let lib2 = Library::take();

    drop(lib);
    drop(lib2);
}

#[test]
fn open_file() {
    let lib = Library::new();

    let file = lib.open_write("test.cgns").expect("Failed to open file");

    file.close().expect("Failed to close file");
}

#[test]
fn get_cgio() {
    let lib = Library::new();

    let file = lib.open_write("test.cgns").expect("Failed to open file");

    let cgio = file.cgio().expect("Failed to get cgio");

    cgio.cgio_number();
    cgio.root_id();
}

#[test]
fn test_goto() {
    let lib = Library::new();

    create_file(&lib, "goto_test.cgns").expect("Failed to create file");

    let mut file = lib
        .open_modify("goto_test.cgns")
        .expect("failed to open file");

    let base_index = Base::write(
        &mut file,
        &base::BaseData {
            name: "New Base".into(),
            cell_dim: 3,
            phys_dim: 3,
        },
    )
    .expect("failed to write base");

    let base = file.get_base(base_index).expect("Failed to get base");

    let path = base.path();
    lib.goto(&path).expect("failed to goto path");
    assert_eq!(
        path,
        lib.current_path().expect("failed to get current path")
    );
}

#[test]
fn iter_bases() {
    let lib = Library::new();

    create_file(&lib, "iter_test.cgns").expect("Failed to create file");

    let mut file = lib
        .open_modify("iter_test.cgns")
        .expect("failed to open file");

    assert_eq!(
        0,
        Base::iter(&file)
            .expect("failed to read number of bases")
            .count()
    );

    Base::write(
        &mut file,
        &base::BaseData {
            name: "New Base 1".into(),
            cell_dim: 3,
            phys_dim: 3,
        },
    )
    .expect("failed to write base");

    Base::write(
        &mut file,
        &base::BaseData {
            name: "New Base 2".into(),
            cell_dim: 3,
            phys_dim: 3,
        },
    )
    .expect("failed to write base");

    Base::write(
        &mut file,
        &base::BaseData {
            name: "New Base 3".into(),
            cell_dim: 3,
            phys_dim: 3,
        },
    )
    .expect("failed to write base");

    let mut n = 0;
    for base in Base::iter(&file).expect("failed to read number of bases") {
        println!("base: {:#?}", base.read().expect("failed to read base"));
        n += 1;
    }

    assert_eq!(n, 3);
}

#[test]
fn read_write_base_and_descriptor() {
    let lib = Library::new();

    create_file(&lib, "base_test.cgns").expect("Failed to create file");

    let mut file = lib
        .open_modify("base_test.cgns")
        .expect("Failed to open file");

    let base_data = base::BaseData {
        name: "New Base".into(),
        cell_dim: 3,
        phys_dim: 3,
    };

    let base_index = Base::write(&mut file, &base_data).expect("failed to write base");

    let mut base = file.get_base(base_index).expect("failed to get base");

    let data = base.read().expect("failed to read base");

    assert_eq!(data, base_data);

    let descriptor_data = DescriptorData {
        name: "TestDescriptor".into(),
        value: "Test Value".into(),
    };

    base.set_descriptor(&descriptor_data)
        .expect("Failed to write descriptor");

    let descriptor = base
        .get_descriptor(1)
        .expect("Failed to read descriptor Node");

    assert_eq!(
        descriptor.read().expect("Failed to read descriptor"),
        descriptor_data
    );
}

#[test]
fn read_write_zone_and_descriptor() {
    let lib = Library::new();

    create_file(&lib, "desc_test.cgns").expect("Failed to create file");

    let mut file = lib
        .open_modify("desc_test.cgns")
        .expect("Failed to open file");

    let base_data = base::BaseData {
        name: "New Base".into(),
        cell_dim: 3,
        phys_dim: 3,
    };

    let base_index = Base::write(&mut file, &base_data).expect("failed to write base");

    let mut base = file.get_base(base_index).expect("failed to get base");

    let zone_data = ZoneData {
        name: "Zone_0001".into(),
        size: ZoneSize::Structured(StructuredZoneSize {
            n_cell: (9, 9, 9),
            n_vertex: (10, 10, 10),
        }),
    };

    let zone_index = Zone::write(&mut base, &zone_data).expect("Failed to write Zone");

    let mut zone = Zone::new(&base, zone_index).expect("Failed to read Zone Node");

    assert_eq!(zone.read().expect("Failed to read Zone"), zone_data);

    let descriptor_data = DescriptorData {
        name: "TestDescriptor".into(),
        value: "Test Value".into(),
    };

    zone.set_descriptor(&descriptor_data)
        .expect("Failed to write descriptor");

    let descriptor = zone
        .get_descriptor(1)
        .expect("Failed to read descriptor Node");

    assert_eq!(
        descriptor.read().expect("Failed to read descriptor"),
        descriptor_data
    );
}
