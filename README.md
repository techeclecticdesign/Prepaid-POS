# Maine Department of Corrections Annex POS

A robust point-of-sale system that uses unit and integration tests to guarantee in-the-field stability.

It is designed for use in Maine State Prison to allow inmates to order items using money they have transferred to an annex account. Account funds are sent via report by accountant, parsed into the system, and once processed are available for use by prison residents.

It is built using the Clean Architecture model (https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html), which makes testing easier to set up and reduces the amount of refactoring involved with any future changes due to its modular and decoupled design.

---

## Project Layout

```
src/
├── main.rs             # App entrypoint + Tauri setup
├── commands.rs         # Tauri commands adapters
├── error.rs            # Central AppError enum (thiserror)
├── logger.rs           # fern-based logging config
├── domain/             # Core business models & repo traits
│   ├── models/
│   └── repos/
├── infrastructure/     # Concrete implementations
│   ├── db.rs           # SQLite connection + migrations
│   └── repos/
├── interface/          # Controllers & presenters
│   ├── controllers/
│   └── presenters/
├── services/           # Business-use-case orchestration
└── test_support/       # Mocks + test helpers
```

---

## Module Responsibilities

- **Domain**: defines models + generic traits.
- **Infrastructure**: holds SQLite connection logic + concrete Database Repositories.
- **Services**: contains services that orchestrate domain logic (create, list, update, etc).
- **Interface**:
  - **Controllers**: adapt service calls into simple methods.
  - **Presenters**: process data in the way it is expected by the frontend.
  - **Commands**: Tauri-facing functions.
- **Error Handling**: `error.rs` centralizes `AppError` variants (`Db`, `Migration`, `Parse`, `NotFound`, etc).
- **Logging**: `logger.rs` configures `fern` to output to console + file.
- **Test Support**: reusable mocks for isolated controller/service tests.

---

## Data & Call Flow

```
Frontend
   ⇅
Tauri Command
   ⇅
Interface Controller
   ⇅
Service
   ⇅
Repo Trait (Domain)
   ⇅
Repository (Infrastructure)
   ⇅
Database

```

---

## Adding New Models

1. Create `domain/models/YourModel.rs` + trait in `domain/repos`.
2. Add SQL migration in `migrations/xxxx_create_your_model.sql`.
3. Implement concrete repo in `infrastructure/repos`.
4. Add service orchestration in `services/your_service.rs`.
5. Expose via controller in `interface/`.
6. Wire Tauri command in `commands.rs`.
7. Write tests using `test_support/mock_your_repo`.
