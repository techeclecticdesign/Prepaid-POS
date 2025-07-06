# Maine Department of Corrections Annex POS

A robust point-of-sale system that uses frontend/backend validation and unit/integration tests to guarantee in-the-field stability.

It is designed for use in Maine State Prison to allow inmates to order items using money they have transferred to an annex account. Account funds are sent via report by accountant, parsed into the system, and once processed are available for use by prison residents.

It is built using the Clean Architecture model (https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html), which makes testing easier to set up and reduces the amount of refactoring involved with any future changes due to its modular and decoupled design.

---

## Project Layout

```
src/
├── main.rs           # App entrypoint + Tauri setup
├── application/      # Business logic and use-case orchestration
├── common            # Common libs like auth, error and logger
├── domain/           # Core business models & repo traits
│   ├── models/         # The shape of the data
│   └── repos/          # The shape of the repos
├── infrastructure/   # Concrete implementations
│   ├── db.rs           # SQLite connection + migrations
│   └── repo            # SQL operations
├── interface/        # Boundary of front and back end.
│   ├── controller      # Type conversion etc of data coming from frontend
|   ├── dto             # The shape of the data going to and from the frontend
│   └── presenter       # Type conversion etc of data going to frontend
└── test_support/     # Mocks + test helpers
```

---

## Responsibilities

- **Domain**: defines models + generic traits.
- **Infrastructure**: concrete Database Repositories containing SQLite logic.
- **Application**: services that orchestrate business logic (sale_transaction, stock_items, etc).
- **Interface**:
  - **Controllers**: adapt service calls into simple methods.
  - **Presenters**: process data in the way it is expected by the frontend.
  - **Commands**: Tauri-facing functions.
- **Error Handling**: `error.rs` centralizes `AppError` variants (`Db`, `Migration`, `Parse`, `NotFound`, etc).
- **Logging**: `logger.rs` configures `fern` to output to console + file.
- **Auth**: `auth.rs` and associated commands handle log in/out, timeout etc.
- **Test Support**: reusable mocks for isolated controller/service tests.

---

## Logical Flow

```
Frontend
   ⇅
Tauri Command
   ⇅
  DTO
   ⇅
Interface Controller
   ⇅
Application
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
