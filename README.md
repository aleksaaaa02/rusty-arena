# Rusty Arena

Ocena: 10 i diplomski

## Kratak opis

RustyArena predstavlja jednostavnu multiplayer igru koja za cilj ima da prikaže mogućnosti i prednosti korišćenja programskog jezika Rust u razvoju mrežnih aplikacija i video igara.

Projekat je zamišljen kao mala demonstracija osnovnih mehanizama real-time sinhronizacije između više klijenata koji se povezuju na zajednički server.

Klijentska strana igre se razvija pomoću Godot engine-a, dok je serverski deo implementiran u Rust-u. Time se naglašava kontrola na mrežnom komunikacijom, asihronim radom i konkretnim protokolom komunikacije. Takođe pokušao bih integraciju Rust-a unutar same klijentske strane korišćenjem Rust binding-a za Godot4 (Konkretan alat naveden ispod)

## Zašto

Rust je relativno nov programski jezik poznat po sigurnosti memorije, visokim perfomansama i kontrolama niskog nivoa. 

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

## Protokol

Kako se radi o igri u realnom vremenu, gde je brzina ključna, odlučio sam se za korišćenje [UDP](https://sh.wikipedia.org/wiki/UDP_(protokol)) protokola za prenos komandi od kljuenta ka serveru i stanja igre od servera ka klijentima. Za razliku od [TCP](https://sh.wikipedia.org/wiki/Transmisioni_kontrolni_protokol) protokola UDP se ne trudi da obezbedi pouzdan transfer podataka i često se za njega može pronaći izraz "_fire and forget_". 

TCP protokol je pogodan za procese u kojima je bitan redosled paketa koje šaljemo kao i da svi paketi stignu na zadatu adresu. Dobar primer gde se može upotrebiti je kada vršimo autorizaciju i autentifikaciju sa serverima ili saljenje poruka (_live chat_). Konkretno u ovom projektu je upotrebljeno kod dobavljanja identifiktora. Klijent prvo otvara TCP konekciju i šalje zahtev serveru za dobavljanje identifikatora koji je potreban radi kontrolisanja igrača u video igri. Često je da se proces autorizacije odvija na potpuno drugom serveru, koji služi za dobavljanju tokena, povezivanju na konkretne game-server gde autorizacioni server igra i ulogu _load balacer-a_. 

## Arhitektura 

Sama aplikacija prati client-server arhitekturu. Server igra autoritativnu ulogu, jedini je izvor istine (_soruce of truth_), dok klijentu ne mogu direktno da menjaju stanje igre već samo posredstvom komandi. Server takođe dodeljuje identifikator korisniku preko TCP konekcije jer je potrebno da odgovor sigurno stigne do klijenta, nakon čega korisnik može da započne igru.

- Projekat se treuntno sastoji iz sledećih celina:
    - Client - klijentski deo napisan u rust korišćenjem Godot Ekstenzija
    - Server - game server (trenutno i "autorizacioni") koji simulira tok igre
    - Common - zajedničke strukture (modeli) na koje se oslanjaju klijent i server
    - Godot - godot resursi, scene, čvorovi

## Modeli
```rust
// Komande koje korisnik salje serveru

#[derive(Encode, Decode, Clone, Debug)]
pub enum InputAction {
    RotateLeft,
    RotateRight,
    Shoot,
    Thrust,
}

#[derive(Encode, Decode, Clone, Debug)]
pub struct PlayerInput {
    pub id: u32,
    pub action: InputAction,
}
```

```rust
// modeli koji se koriste za komunikaciju kao i za tok igre

pub struct GameWorld {
    pub players: HashMap<u32, Player>,
    pub bullets: Vec<Bullet>,
    pub asteroids: Vec<Asteroid>,
    pub width: f32,
    pub height: f32,
    pub bullet_id_counter: u32,
    pub asteroid_id_counter: u32,
    pub last_spawn_asteroid: u64,
}

pub struct Player {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub vx: f32,
    pub vy: f32,
    pub hp: u16,
    pub last_shot_ms: u64,
    pub fire_rate_ms: u64,
}

pub struct Bullet {
    pub id: u32,
    pub owner_id: u32,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub distance_traveled: f32,
}

pub struct Asteroid {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub radius: f32,
    pub distance_travaled: f32,
}

```

Konkretno struktura GameWorld sadrži stanje igre i njen tip implementira update na osnovu jedne iteracije _game loop_-a.
Server je multi-threaded koristi kanale za komunikaciju kako prijem poruka ne bi blokirao čitavu igru.


## Moguća proširenja za diplomski
- Razdvajanje autorizacije, omogućavanje korisniku da se registruje
- Praćenje sesija i otklanjanja onih koje više nisu aktivne
- Primena neke tehnike za prividno smanjenje kašnjenja odgovora - pobljašavanje UX-a ([link](https://www.gabrielgambetta.com/client-server-game-architecture.html)):
    - Client-side prediction and Server Reconceliation
    - Entity Interpolation
    - Lag Compensation
- Pokretanje više instanci game servera:
    - Server može da sadrži različite partije, autorizacioni server rutira ka tom server-u ili
    - Na jednom serveru se igra jedna igra, autorizacioni server rutira ka tom server-u
- Dodavanje novih funkcionalnosti - pojačnja (power ups)...


## Pokretanje

```sh
TODO
```

## Alati
- Rust (Cargo)
- Godot Engine 4.x (Linux build) 
- Git/Github 
- Tokio
- bincode
- [gdext](https://github.com/godot-rust/gdext) - GDExtensions (Rust bindings for Godot 4)

    
- Napomena: Dokument će biti proširen kako teče implementacija
