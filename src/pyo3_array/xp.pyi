from __future__ import annotations

from typing import TYPE_CHECKING, Any

if TYPE_CHECKING:
    Array = Any
    dtype = Any
    device = Any

def eye(
    n_rows: int, n_cols: int | None, /, *,
    k: int | None, dtype: dtype | None, device: device | None,
) -> Array: ...
