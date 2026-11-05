"""Path-safety helpers shared by corpus metadata validators."""

from __future__ import annotations

from pathlib import Path


def has_symlink_component(path: Path, root: Path) -> bool:
    """Return whether path traverses a symlink below root.

    Check each lexical component before opening the final file. This rejects
    both links that escape the corpus and in-root aliases that could make
    manifest identity or duplicate detection ambiguous.
    """
    try:
        relative = path.relative_to(root)
    except ValueError:
        return True

    current = root
    for component in relative.parts:
        current = current / component
        try:
            if current.is_symlink():
                return True
        except OSError:
            return True
    return False
