import strsim.strsim as strsim
from strsim.strsim import *

__doc__ = strsim.__doc__

if hasattr(strsim, "__all__"):
    __all__ = strsim.__all__  # type: ignore