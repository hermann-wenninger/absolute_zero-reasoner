// 1. Der Mutex wird in einen Arc "gepackt"
//c but RUST

    let n = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // 2. Für jeden Thread wird ein neuer Zeiger (Klon) auf die Daten erstellt
        let n_clone = Arc::clone(&n);

        // 3. Wir benutzen das normale thread::spawn
        let handle = thread::spawn(move || {
            // Die Logik im Thread ist identisch
            let mut guard = n_clone.lock().unwrap();
            for _ in 0..100 {
                *guard += 1;
            }
            drop(guard);
            thread::sleep(Duration::from_millis(100));
        });
        handles.push(handle);
    }

    // 4. Wir müssen manuell auf alle Threads warten!
    //    Das macht der Scope automatisch.
    for handle in handles {
        handle.join().unwrap();
    }

    // 5. Um an den Wert zu kommen, ist es etwas komplizierter
    let final_n = Arc::try_unwrap(n).expect("Konnte Arc nicht entpacken");
    println!("Anzahl: {}", final_n.into_inner().unwrap());
    println!("Fertig!");