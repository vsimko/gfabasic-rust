"""Validation helpers for repository-relative metadata paths."""

from __future__ import annotations

from pathlib import Path, PurePosixPath


def repository_relative_path(value: object) -> PurePosixPath | None:
    """Parse a canonical repository-relative POSIX path.

    Git paths use forward slashes on every host. Reject aliases and platform
    absolute-path forms before converting the components to a local Path.
    """
    if not isinstance(value, str) or not value or "\\" in value:
        return None

    relative = PurePosixPath(value)
    if (
        relative.is_absolute()
        or not relative.parts
        or relative.as_posix() != value
        or any(part in {"", ".", ".."} for part in relative.parts)
        or ":" in relative.parts[0]
    ):
        return None
    return relative


def has_symlink_component(path: Path, root: Path) -> bool:
    """Return whether a repository-relative path traverses a symlink."""
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
