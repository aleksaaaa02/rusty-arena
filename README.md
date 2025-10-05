# Rusty Arena

## Kratak opis

RustyArena predstavlja jednostavnu multiplayer igru koja za cilj ima prikaz mogućnosti i prednosti korišćenja programskog jezika Rust u razvoju mrežnih aplikacija i video igara.

Projekat je zamišljen kao mala demonstracija osnovnih mehanizama real-time sinhronizacije između više klijenata koji se povezuju na zajednički server.

Klijentska strana igre se razvija pomoću Godot engine-a, dok je serverski deo implementiran u Rust-u. Time se naglašava kontrola na mrežnom komunikacijom, asihronim radom i konkretnim protokolom komunikacije. Takođe pokušao bih integraciju Rust-a unutar same klijentske strane korišćenjem Rust binding-a za Godot4 (Konkretan alat naveden ispod)

## Zašto

Rust relativno nov programski jezik poznat zbog sigurnosti memorije, performansi i kontrole niskog nivoa. 

Godot je besplatan i open-source engine, za razliku od Unity-a i nekih drugih game engine-a, koji omogućava jednostavan razvoj video igara.

[Are we game yet?](https://arewegameyet.rs) - veb sajt koji sadrži informacije o trenutnom stanju razvoja video igara u Rust-u. 

## Opis igre

- Osnovne stvari (apstraktni gameloop)
    - Igrač se povezuje na server
    - Server dodeljuje spawn poziciju
    - Igrač se kreće po areni
    - Igrač može da napad
    - Pogodak smanjuje protivnikov health
    - Smrt i respawn 
    - Rezultati igre

- Osnovne mehanike:
    - Kretanje
    - Napad
    - Health sistem
    - Bodovanje/Rezultat

## Alati
- Rust (Cargo)
- Godot Engine 4.x (Linux build) 
- Git/Github 
- [gdext](https://github.com/godot-rust/gdext) - GDExtensions (Rust bindings for Godot 4)
    
- Napomena: Dokument će biti proširen kako teče implementacija