# decisol

`decisol` is a small Rust crate that provide FULLY TYPED Solana-style token amounts wrappers and math.

Most on-chain amounts are just `u64` + `u8`. That is fast and compact, but also hard to track whether `1_000_000` means `1 USDC`, `0.001 SOL`, or one million
base units of some token. `decisol` keeps the raw integer
storage model, but wraps it in types that remember the unit.

## What it solves

- **Raw lamports are easy to mix up.** `Lamports`, `Wsol`, `Usdc`, `Usdt`,
  `Usd`, `Sol`, and `TokenLamportsN` carry their decimal scale with them.
- **Generic token decimals are awkward.** `TokenLamports` can represent tokens
  from 0 to 18 decimals while still exposing the same `Decisol` methods.
- **Decimal math should stay integer-backed.** You can multiply or divide by
  `fastnum::UD128` prices and rates, then get back a typed amount.
- **Quote assets need normalization.** SOL-like quotes and USD-like quotes can
  be compared or converted through helper functions instead of hand-written
  conversion branches.
- **Safety policy should be explicit.** Feature flags let you choose between
  checked arithmetic, logging errors, or panicking for overflow/conversion
  failures.

## Highlights

- Typed wrappers over `u64` raw amounts.
- Built-in decimal scales for common Solana quote units:
  - `Lamports`, `Wsol`, `Sol`: 9 decimals
  - `Usdc`, `Usd1`, `Usdt`, `Usd`: 6 decimals
  - `TokenLamports0` through `TokenLamports18`
- Dynamic enums for places where the token kind is only known at runtime:
  - `TokenLamports`
  - `QuoteLamports`
  - `SolanaLamports`
- `serde` support on amount and kind types.
- Optional protobuf schema support behind the `proto` / `build-schemas`
  features.
- Re-exports of `fastnum` decimal types and macros: `UD128`, `D128`,
  `udec128!`, `dec128!`.

## Quick Start

```rust
use decisol::{Decisol, Lamports, TokenLamports, Usdc, udec128};

let sol = Lamports::from(udec128!(1.25));
assert_eq!(sol.amount(), 1_250_000_000);
assert_eq!(sol.decimals(), 9);
assert_eq!(format!("{sol:.2}"), "1.25");

let fee = sol * udec128!(0.003);
assert_eq!(fee.amount(), 3_750_000);

let token = TokenLamports::new_from_decimals(42_000_000_u64, 6);
assert_eq!(token.decimals(), 6);

let usdc = Usdc::from(udec128!(12.50));
assert_eq!(usdc.amount(), 12_500_000);
```

## Typed Amounts

Use concrete types when the unit is known at compile time:

```rust
use decisol::{Decisol, Lamports, Usdc};

let a = Lamports::new(1_000_000_000_u64);
let b = Lamports::one();
assert_eq!(a + b, Lamports::new(2_000_000_000_u64));

let quote = Usdc::ten();
assert_eq!(quote.amount(), 10_000_000);
```

Each typed amount provides constants and constructors such as `ZERO`, `ONE`,
`TEN`, `zero()`, `one()`, and `ten()`.

## Runtime Token Kinds

Use enum wrappers when the decimal scale or quote kind arrives from metadata,
configuration, or an external message:

```rust
use decisol::{Decisol, SolanaLamportsKind, TokenLamports};

let kind = SolanaLamportsKind::from_dec(6).unwrap();
let amount = TokenLamports::new_from_decimals(1_500_000_u64, kind.decimals());

assert_eq!(amount.decimals(), 6);
assert_eq!(format!("{amount:.2}"), "1.50");
```

The enum variants preserve the raw amount and expose the same `Decisol` trait as
the concrete wrappers, so most code can be generic over `impl Decisol`.

## Quote Helpers

`QuoteLamports` groups SOL-like and USD-like quote assets:

```rust
use decisol::{Decisol, QuoteLamports, QuoteLamportsKind, compare_quotes_usd, udec128};

let sol_quote = QuoteLamports::new(2_000_000_000_u64, QuoteLamportsKind::Lamports);
let usd_quote = QuoteLamports::new(300_000_000_u64, QuoteLamportsKind::Usdc);

let ordering = compare_quotes_usd(sol_quote, usd_quote, || udec128!(150));
assert_eq!(ordering, std::cmp::Ordering::Equal);

let normalized = QuoteLamportsKind::Usdc.normalize(
    sol_quote,
    || udec128!(150),
    || udec128!(0.006666666),
);
assert_eq!(normalized.kind(), QuoteLamportsKind::Usdc);
```

This is useful for AMMs, pricing code, indexers, order books, and any service
that has to compare liquidity or fees across SOL and stablecoin quotes.

## Feature Flags

Default features:

```toml
default = ["check_enum_kind", "conv_errors", "overflow_errors", "track_caller"]
```

Available safety and integration flags:

- `overflow_checks`: enable arithmetic overflow checks.
- `overflow_errors`: log overflow problems and continue with a fallback value.
- `overflow_panics`: panic on overflow.
- `conv_checks`: enable conversion checks.
- `conv_errors`: log conversion problems and continue with a fallback value.
- `conv_panics`: panic on conversion failure.
- `check_enum_kind`: check enum kind compatibility before enum arithmetic.
- `check_enum_decimals`: check decimal compatibility before enum arithmetic.
- `track_caller`: include caller locations in panic paths.
- `proto`: enable protobuf message integration.
- `build-schemas`: enable schema generation support.
